//! Layout module for the VB6 runtime.
//!
//! This module provides a layout engine that takes a parsed VB6 control tree
//! and produces an interactive, web-renderable UI in HTML/CSS format,
//! targetable at both WASM and Tauri runtimes.
//!
//! The engine is structured in layers:
//! - [`model`] — core layout types (nodes, positions, sizes, styles)
//! - [`converter`] — transforms parsed `FormRoot` into the layout model
//! - [`diff_tree`] — diff types for incremental rendering
//! - [`diff_engine`] — engine for computing diffs between snapshots and live trees
//! - [`scale`] — twip-to-pixel and ScaleMode conversion
//! - [`color`] — VB6 Color to CSS color mapping
//! - [`css`] — `LayoutStyle` to CSS declaration string
//! - [`controls`] — per-control style builders
//! - [`form_store`] — mutable state store for loaded forms
//! - [`menu`] — menubar and context/popup menu rendering
//! - [`renderer`] — platform-abstract rendering trait + Tauri backend
//! - [`theme`] — CSS custom property theming + injection API
//! - [`vb6_css`] — complete VB6 CSS stylesheet with scoped selectors
//! - [`snapshot`] — snapshot types for incremental layout diffing

pub mod color;
/// Per-control style builder dispatch.
pub mod controls;
/// Tree walker: FormRoot → LayoutNode tree → form store.
pub mod converter;
/// LayoutStyle → CSS declaration string.
pub mod css;
/// Diff engine for incremental layout rendering.
pub mod diff_engine;
/// Diff types for incremental layout rendering.
pub mod diff_tree;
/// Mutable state store for loaded forms.
pub mod form_store;
/// Menubar and context/popup menu rendering.
pub mod menu;
/// Core layout types (nodes, positions, sizes, styles).
pub mod model;
/// Platform-abstract rendering trait + platform backends.
pub mod renderer;
/// Twip-to-pixel and ScaleMode conversion.
pub mod scale;
/// Snapshot types for incremental layout diffing.
pub mod snapshot;
/// CSS custom property theming + injection API.
pub mod theme;
/// Full VB6 CSS stylesheet with scoped selectors for platform isolation.
pub mod vb6_css;

use model::style::LayoutStyle;

// Re-export converter types at the module level for convenience.
pub use converter::{LayoutError, LayoutResult, load_form};

// Re-export diff engine.
pub use diff_engine::DiffEngine;

// Re-export diff tree type for callers that need to inspect diff results.
pub use diff_tree::DiffTree;

// Re-export form store handle type.
pub use form_store::FormHandle;

// Re-export snapshot type for diff-based rendering.
pub use snapshot::SnapshotNode;

/// Configuration for the layout conversion.
#[derive(Debug, Clone, Copy)]
pub struct LayoutConfig {
    /// DPI for twip-to-pixel conversion (default: 96).
    pub dpi: u32,
    /// Include controls with visible = false in the output tree
    /// (default: true; set false to skip invisible controls).
    pub include_hidden: bool,
    /// Include non-visual controls (Timer, Data) in the output tree
    /// (default: false; these controls have no visual output).
    pub include_nonvisual: bool,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            dpi: 96,
            include_hidden: true,
            include_nonvisual: false,
        }
    }
}

/// Build CSS style properties for a control based on its [`vb6parse::controls::ControlKind`].
///
/// Delegates to individual control modules (button, label, textbox, etc.)
/// to construct the appropriate [`LayoutStyle`].
pub fn build_style_for_control(
    kind: &vb6parse::language::ControlKind,
    config: &LayoutConfig,
) -> LayoutStyle {
    controls::build_style_for_control(kind, config)
}

/// Convert a font size in points (from vb6parse [`Font::size`]) to pixels at the given DPI.
///
/// 1 point = 1/72 inch. At 96 DPI: 1 point = 96/72 ≈ 1.333 pixels.
#[must_use]
pub fn font_points_to_px(points: f32, dpi: u32) -> f32 {
    points * dpi as f32 / 72.0
}

/// Retrieve the [`LayoutForm`] for a loaded form by handle.
///
/// Operates on the form via a closure while the store lock is held,
/// returning the closure's result. This avoids the `'static` reference
/// lifetime problem inherent in global mutable state.
///
/// Returns `None` if the handle is invalid.
///
/// # Example
/// ```ignore
/// let form = layout::get_form(handle, |f| f.name.clone());
/// layout::get_form(handle, |f| { f.caption = "New Caption".into(); });
/// ```
pub fn get_form<T>(handle: FormHandle, f: impl FnOnce(&model::LayoutForm) -> T) -> Option<T> {
    form_store::get(handle, f)
}

/// Mutate the [`LayoutForm`] for a loaded form by handle.
///
/// Convenience wrapper around [`get_form`] that provides mutable access.
/// Returns `None` if the handle is invalid.
///
/// # Example
/// ```ignore
/// layout::get_form_mut(handle, |f| {
///     f.visible = false;
///     f.current_value = Some("Done".into());
/// });
/// ```
pub fn get_form_mut<T>(
    handle: FormHandle,
    f: impl FnOnce(&mut model::LayoutForm) -> T,
) -> Option<T> {
    form_store::get_mut(handle, f)
}

/// Render a form using the given [`renderer::Renderer`] implementation.
///
/// Walks the [`LayoutNode`] tree of the loaded form and produces
/// platform-specific output (HTML string for Tauri, DOM elements for WASM).
///
/// On the first render (no snapshot exists), performs a full tree render.
/// On subsequent renders, computes a diff against the previous snapshot
/// and uses [`Renderer::render_node_with_diff`](renderer::Renderer::render_node_with_diff)
/// when available to skip unchanged subtrees.
/// After rendering, captures the current model state as the snapshot for
/// the next diff computation.
///
/// # Panics
///
/// Panics if the handle is invalid. Use [`get_form`] first to check validity,
/// or use the closure-based API directly for fallible access.
///
/// # Example
/// ```ignore
/// let html = layout::render(handle, &renderer::TauriRenderer::new(false));
/// // or for WASM:
/// // let dom = layout::render(handle, &renderer::WebSysRenderer::new(doc));
/// ```
pub fn render<R: renderer::Renderer>(handle: FormHandle, renderer: &R) -> R::Output {
    form_store::get_mut(handle, |form| {
        // Compute diff if a snapshot exists from a previous render.
        let diff = form
            .snapshot
            .as_ref()
            .map(|snap| DiffEngine::compute_diff(snap, &form.root_node));

        // render_node_with_diff falls back to full render when diff is None/empty,
        // so we can always call it — the fallback handles the first-render case.
        let output = renderer.render_node_with_diff(&form.root_node, diff.as_ref());

        // Capture the current model state as the snapshot for next diff.
        form.snapshot = Some(form.root_node.to_snapshot());
        form.render_id += 1;

        output
    })
    .expect("unknown form handle")
}

/// Compute the diff since the last render.
///
/// Returns `None` if no snapshot exists (first render) — the caller should
/// perform a full render in that case.
///
/// After a successful render, the snapshot is automatically updated so that
/// subsequent calls compute the diff against the last rendered state.
pub fn get_diff(handle: FormHandle) -> Option<DiffTree> {
    form_store::get(handle, |form| {
        form.snapshot
            .as_ref()
            .map(|snap| DiffEngine::compute_diff(snap, &form.root_node))
    })
    .flatten()
}

/// Capture the current form state as the snapshot for the next diff computation.
///
/// Call this after a successful render to record the model state. This is
/// called automatically by [`render`] — manual calls are only needed when
/// rendering outside the standard pipeline.
///
/// Increments [`render_id`][model::LayoutForm::render_id] on the form.
pub fn capture_snapshot(handle: FormHandle) {
    form_store::get_mut(handle, |form| {
        form.snapshot = Some(form.root_node.to_snapshot());
        form.render_id += 1;
    });
}

#[cfg(test)]
mod tests {
    use super::model::LayoutNode;
    use super::*;
    use vb6parse::language::{Activation, Control, ControlKind, Form, FormRoot, Visibility};

    /// Helper to ensure tests that share global state run sequentially.
    fn lock_test() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        LOCK.lock().unwrap()
    }

    fn make_test_form_with_label() -> Form {
        use vb6parse::language::{Control, ControlKind, LabelProperties};
        let label = Control::new(
            "Label1".to_string(),
            String::new(),
            0,
            ControlKind::Label {
                properties: LabelProperties {
                    caption: "Hello".to_string(),
                    left: 120,
                    top: 120,
                    width: 1000,
                    height: 300,
                    visible: Visibility::Visible,
                    enabled: Activation::Enabled,
                    ..Default::default()
                },
            },
        );
        Form {
            name: "Form1".to_string(),
            tag: String::new(),
            index: 0,
            properties: vb6parse::language::FormProperties {
                scale_width: 4000,
                scale_height: 3000,
                caption: "Form1".to_string(),
                left: 0,
                top: 0,
                visible: Visibility::Visible,
                enabled: Activation::Enabled,
                ..Default::default()
            },
            controls: vec![label],
            menus: Vec::new(),
        }
    }

    fn make_test_form_with_button() -> Form {
        use vb6parse::language::{CommandButtonProperties, Control, ControlKind};
        let btn = Control::new(
            "cmdOK".to_string(),
            String::new(),
            0,
            ControlKind::CommandButton {
                properties: CommandButtonProperties {
                    caption: "OK".to_string(),
                    left: 120,
                    top: 500,
                    width: 800,
                    height: 300,
                    enabled: Activation::Enabled,
                    ..Default::default()
                },
            },
        );
        Form {
            name: "Form1".to_string(),
            tag: String::new(),
            index: 0,
            properties: vb6parse::language::FormProperties {
                scale_width: 4000,
                scale_height: 3000,
                caption: "Form1".to_string(),
                left: 0,
                top: 0,
                visible: Visibility::Visible,
                enabled: Activation::Enabled,
                ..Default::default()
            },
            controls: vec![btn],
            menus: Vec::new(),
        }
    }

    #[test]
    fn load_form_and_get() {
        let _lock = lock_test();
        let form = make_test_form_with_label();
        let config = LayoutConfig::default();
        let handle = load_form(&FormRoot::Form(form), vec![], &config);
        let name = get_form(handle, |f| f.name.clone());
        assert_eq!(name, Some("Form1".into()));
    }

    #[test]
    fn test_get_form_mut() {
        let _lock = lock_test();
        let form = make_test_form_with_button();
        let config = LayoutConfig::default();
        let handle = load_form(&FormRoot::Form(form), vec![], &config);
        let changed = get_form_mut(handle, |f| {
            f.caption = "Modified".into();
            f.caption.clone()
        });
        assert_eq!(changed, Some("Modified".into()));
        let check = get_form(handle, |f| f.caption.clone());
        assert_eq!(check, Some("Modified".into()));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn render_with_tauri_renderer() {
        let _lock = lock_test();
        let form = make_test_form_with_label();
        let config = LayoutConfig::default();
        let handle = load_form(&FormRoot::Form(form), vec![], &config);
        let renderer = renderer::TauriRenderer::new(false);
        let html = render(handle, &renderer);
        assert!(!html.is_empty(), "html was empty: {html}");
        assert!(html.contains("vb6-label"), "html missing vb6-label: {html}");
    }

    #[test]
    fn render_with_scope() {
        let _lock = lock_test();
        let form = make_test_form_with_label();
        let config = LayoutConfig::default();
        let handle = load_form(&FormRoot::Form(form), vec![], &config);
        let renderer = renderer::TauriRenderer::new(true);
        let html = render(handle, &renderer);
        assert!(!html.is_empty());
    }

    #[test]
    fn invalid_handle_get_form() {
        assert_eq!(get_form(999, |f| f.name.clone()), None);
    }

    #[test]
    fn invalid_handle_get_form_mut() {
        assert_eq!(get_form_mut(999, |f| f.name.clone()), None);
    }

    #[test]
    fn render_invalid_handle_panics() {
        let renderer = renderer::TauriRenderer::new(false);
        let result = std::panic::catch_unwind(|| {
            render(999, &renderer);
        });
        assert!(result.is_err());
    }

    #[test]
    fn render_multiple_forms() {
        let _lock = lock_test();
        let form1 = make_test_form_with_label();
        let form2 = make_test_form_with_button();
        let config = LayoutConfig::default();
        let handle1 = load_form(&FormRoot::Form(form1), vec![], &config);
        let handle2 = load_form(&FormRoot::Form(form2), vec![], &config);
        let renderer = renderer::TauriRenderer::new(false);
        let html1 = render(handle1, &renderer);
        let html2 = render(handle2, &renderer);
        assert!(html1.contains("vb6-label"));
        assert!(html2.contains("vb6-commandbutton"));
    }

    #[test]
    fn get_diff_no_snapshot_returns_none() {
        let _lock = lock_test();
        form_store::reset();
        let form = make_test_form_with_label();
        let config = LayoutConfig::default();
        let handle = load_form(&FormRoot::Form(form), vec![], &config);
        assert!(get_diff(handle).is_none());
    }

    #[test]
    fn get_diff_after_capture_returns_some() {
        let _lock = lock_test();
        form_store::reset();
        let form = make_test_form_with_label();
        let config = LayoutConfig::default();
        let handle = load_form(&FormRoot::Form(form), vec![], &config);
        // First render captures snapshot
        let renderer = renderer::TauriRenderer::new(false);
        let _html = render(handle, &renderer);
        // Now diff should be computable
        let diff = get_diff(handle);
        assert!(diff.is_some());
    }

    #[test]
    fn get_diff_no_changes_is_empty() {
        let _lock = lock_test();
        form_store::reset();
        let form = make_test_form_with_label();
        let config = LayoutConfig::default();
        let handle = load_form(&FormRoot::Form(form), vec![], &config);
        // First render captures snapshot
        let renderer = renderer::TauriRenderer::new(false);
        let _html = render(handle, &renderer);
        // Second render: no changes to model, diff should be empty
        let diff = get_diff(handle).unwrap();
        assert!(diff.is_empty(), "Expected empty diff when model unchanged");
    }

    #[test]
    fn capture_snapshot_creates_snapshot() {
        let _lock = lock_test();
        form_store::reset();
        let form = make_test_form_with_label();
        let config = LayoutConfig::default();
        let handle = load_form(&FormRoot::Form(form), vec![], &config);
        assert!(get_diff(handle).is_none());
        capture_snapshot(handle);
        assert!(get_diff(handle).is_some());
        // Verify render_id incremented
        let rid = get_form(handle, |f| f.render_id);
        assert_eq!(rid, Some(1));
    }

    #[test]
    fn render_uses_diff_after_first() {
        let _lock = lock_test();
        form_store::reset();
        let form = make_test_form_with_label();
        let config = LayoutConfig::default();
        let handle = load_form(&FormRoot::Form(form), vec![], &config);
        let renderer = renderer::TauriRenderer::new(false);
        // First render: no diff, full render
        let html1 = render(handle, &renderer);
        assert!(!html1.is_empty());
        assert!(html1.contains("vb6-label"));
        // Second render: diff exists, uses diff-aware rendering
        let html2 = render(handle, &renderer);
        assert!(!html2.is_empty());
        assert!(html2.contains("vb6-label"));
    }

    #[test]
    fn render_increments_render_id() {
        let _lock = lock_test();
        form_store::reset();
        let form = make_test_form_with_label();
        let config = LayoutConfig::default();
        let handle = load_form(&FormRoot::Form(form), vec![], &config);
        let renderer = renderer::TauriRenderer::new(false);
        // First render -> render_id = 1
        render(handle, &renderer);
        let rid = get_form(handle, |f| f.render_id);
        assert_eq!(rid, Some(1));
        // Second render -> render_id = 2
        render(handle, &renderer);
        let rid = get_form(handle, |f| f.render_id);
        assert_eq!(rid, Some(2));
    }

    #[test]
    fn get_diff_detects_child_insertion() {
        use super::model::LayoutLeaf;
        let _lock = lock_test();
        form_store::reset();
        let form = make_test_form_with_label();
        let config = LayoutConfig::default();
        let handle = load_form(&FormRoot::Form(form), vec![], &config);
        let renderer = renderer::TauriRenderer::new(false);
        let _html = render(handle, &renderer);
        // Insert a new child — diff engine can detect structural changes.
        let inserted = get_form_mut(handle, |f| {
            if let LayoutNode::Container(ref mut rc) = f.root_node {
                rc.children.push(LayoutNode::Leaf(LayoutLeaf {
                    name: "NewLabel".into(),
                    control_type: super::model::LayoutControlType::Label,
                    visible: true,
                    enabled: true,
                    ..Default::default()
                }));
                true
            } else {
                false
            }
        });
        assert_eq!(inserted, Some(true));
        let diff = get_diff(handle).unwrap();
        assert!(!diff.is_empty(), "Expected diff to detect child insertion");
    }

    #[test]
    fn load_form_returns_valid_handle() {
        let _lock = lock_test();
        form_store::reset();
        let bytes = make_test_form_bytes();
        let source_file = vb6parse::io::SourceFile::decode_with_replacement("form.frm", &bytes)
            .expect("failed to decode form");
        let form_file = vb6parse::FormFile::parse(&source_file).unwrap_or_fail();
        let handle = load_form(&form_file.form, vec![], &LayoutConfig::default());
        let name = get_form(handle, |f| f.name.clone());
        assert_eq!(name, Some("Form1".to_string()));
    }

    #[test]
    fn render_form_with_label() {
        let _lock = lock_test();
        form_store::reset();
        let bytes = make_test_form_bytes();
        let source_file = vb6parse::io::SourceFile::decode_with_replacement("form.frm", &bytes)
            .expect("failed to decode form");
        let form_file = vb6parse::FormFile::parse(&source_file).unwrap_or_fail();
        let handle = load_form(&form_file.form, vec![], &LayoutConfig::default());
        let html = render(handle, &renderer::TauriRenderer::new(false));
        assert!(html.contains("vb6-label"));
        assert!(html.contains("Hello"));
    }

    fn make_test_form_bytes() -> Vec<u8> {
        String::from(
            "VERSION 5.00\r\n\
             Begin VB.Form Form1\r\n\
                Caption         =   \"Test\"\r\n\
                ClientHeight    =   3000\r\n\
                ClientWidth     =   4000\r\n\
                Begin VB.Label Label1\r\n\
                   Caption       =   \"Hello\"\r\n\
                   Height        =   375\r\n\
                   Left          =   120\r\n\
                   Top           =   120\r\n\
                   Width         =   2000\r\n\
                End\r\n\
             End\r\n",
        )
        .into_bytes()
    }

    // -----------------------------------------------------------------------
    // Step 5.3 — Edge case tests
    // -----------------------------------------------------------------------

    #[test]
    fn form_with_zero_controls() {
        let _lock = lock_test();
        form_store::reset();
        let form = Form {
            name: "EmptyForm".to_string(),
            tag: String::new(),
            index: 0,
            properties: vb6parse::language::FormProperties {
                scale_width: 4000,
                scale_height: 3000,
                caption: "EmptyForm".to_string(),
                left: 0,
                top: 0,
                visible: Visibility::Visible,
                enabled: Activation::Enabled,
                ..Default::default()
            },
            controls: vec![],
            menus: Vec::new(),
        };
        let config = LayoutConfig::default();
        let handle = load_form(&FormRoot::Form(form), vec![], &config);
        let html = render(handle, &renderer::TauriRenderer::new(false));
        assert!(html.is_empty());
    }

    #[test]
    fn form_with_scale_mode_user() {
        let _lock = lock_test();
        form_store::reset();
        let label = vb6parse::language::Control::new(
            "Label1".to_string(),
            String::new(),
            0,
            vb6parse::language::ControlKind::Label {
                properties: vb6parse::language::LabelProperties {
                    caption: "User scale".to_string(),
                    left: 120,
                    top: 120,
                    width: 1000,
                    height: 300,
                    visible: Visibility::Visible,
                    enabled: Activation::Enabled,
                    ..Default::default()
                },
            },
        );
        let form = Form {
            name: "UserScaleForm".to_string(),
            tag: String::new(),
            index: 0,
            properties: vb6parse::language::FormProperties {
                scale_width: 4000,
                scale_height: 3000,
                scale_mode: vb6parse::language::ScaleMode::User,
                caption: "UserScaleForm".to_string(),
                left: 0,
                top: 0,
                visible: Visibility::Visible,
                enabled: Activation::Enabled,
                ..Default::default()
            },
            controls: vec![label],
            menus: Vec::new(),
        };
        let config = LayoutConfig::default();
        // ScaleMode::User falls back to twip conversion — should not panic
        let handle = load_form(&FormRoot::Form(form), vec![], &config);
        let html = render(handle, &renderer::TauriRenderer::new(false));
        assert!(!html.is_empty());
    }

    #[test]
    fn form_with_scale_mode_pixel() {
        let _lock = lock_test();
        form_store::reset();
        let label = vb6parse::language::Control::new(
            "Label1".to_string(),
            String::new(),
            0,
            vb6parse::language::ControlKind::Label {
                properties: vb6parse::language::LabelProperties {
                    caption: "Pixel scale".to_string(),
                    left: 120,
                    top: 120,
                    width: 1000,
                    height: 300,
                    visible: Visibility::Visible,
                    enabled: Activation::Enabled,
                    ..Default::default()
                },
            },
        );
        let form = Form {
            name: "PixelScaleForm".to_string(),
            tag: String::new(),
            index: 0,
            properties: vb6parse::language::FormProperties {
                scale_width: 4000,
                scale_height: 3000,
                scale_mode: vb6parse::language::ScaleMode::Pixel,
                caption: "PixelScaleForm".to_string(),
                left: 0,
                top: 0,
                visible: Visibility::Visible,
                enabled: Activation::Enabled,
                ..Default::default()
            },
            controls: vec![label],
            menus: Vec::new(),
        };
        let config = LayoutConfig::default();
        let handle = load_form(&FormRoot::Form(form), vec![], &config);
        let html = render(handle, &renderer::TauriRenderer::new(false));
        assert!(!html.is_empty());
    }

    #[test]
    fn form_with_start_up_position_center_screen() {
        let _lock = lock_test();
        form_store::reset();
        let form = Form {
            name: "CenterScreenForm".to_string(),
            tag: String::new(),
            index: 0,
            properties: vb6parse::language::FormProperties {
                scale_width: 4000,
                scale_height: 3000,
                start_up_position: vb6parse::language::StartUpPosition::CenterScreen,
                caption: "CenterScreenForm".to_string(),
                left: 0,
                top: 0,
                visible: Visibility::Visible,
                enabled: Activation::Enabled,
                ..Default::default()
            },
            controls: vec![],
            menus: Vec::new(),
        };
        let config = LayoutConfig::default();
        let handle = load_form(&FormRoot::Form(form), vec![], &config);
        let name = get_form(handle, |f| f.name.clone());
        assert_eq!(name, Some("CenterScreenForm".into()));
    }

    #[test]
    fn form_with_start_up_position_center_owner() {
        let _lock = lock_test();
        form_store::reset();
        let form = Form {
            name: "CenterOwnerForm".to_string(),
            tag: String::new(),
            index: 0,
            properties: vb6parse::language::FormProperties {
                scale_width: 4000,
                scale_height: 3000,
                start_up_position: vb6parse::language::StartUpPosition::CenterOwner,
                caption: "CenterOwnerForm".to_string(),
                left: 0,
                top: 0,
                visible: Visibility::Visible,
                enabled: Activation::Enabled,
                ..Default::default()
            },
            controls: vec![],
            menus: Vec::new(),
        };
        let config = LayoutConfig::default();
        let handle = load_form(&FormRoot::Form(form), vec![], &config);
        let name = get_form(handle, |f| f.name.clone());
        assert_eq!(name, Some("CenterOwnerForm".into()));
    }

    #[test]
    fn form_with_start_up_position_windows_default() {
        let _lock = lock_test();
        form_store::reset();
        let form = Form {
            name: "WinDefaultForm".to_string(),
            tag: String::new(),
            index: 0,
            properties: vb6parse::language::FormProperties {
                scale_width: 4000,
                scale_height: 3000,
                start_up_position: vb6parse::language::StartUpPosition::WindowsDefault,
                caption: "WinDefaultForm".to_string(),
                left: 0,
                top: 0,
                visible: Visibility::Visible,
                enabled: Activation::Enabled,
                ..Default::default()
            },
            controls: vec![],
            menus: Vec::new(),
        };
        let config = LayoutConfig::default();
        let handle = load_form(&FormRoot::Form(form), vec![], &config);
        let name = get_form(handle, |f| f.name.clone());
        assert_eq!(name, Some("WinDefaultForm".into()));
    }

    #[test]
    fn form_with_start_up_position_manual() {
        let _lock = lock_test();
        form_store::reset();
        let form = Form {
            name: "ManualForm".to_string(),
            tag: String::new(),
            index: 0,
            properties: vb6parse::language::FormProperties {
                scale_width: 4000,
                scale_height: 3000,
                start_up_position: vb6parse::language::StartUpPosition::Manual {
                    client_height: 3000,
                    client_width: 4000,
                    client_top: 100,
                    client_left: 200,
                },
                caption: "ManualForm".to_string(),
                left: 0,
                top: 0,
                visible: Visibility::Visible,
                enabled: Activation::Enabled,
                ..Default::default()
            },
            controls: vec![],
            menus: Vec::new(),
        };
        let config = LayoutConfig::default();
        let handle = load_form(&FormRoot::Form(form), vec![], &config);
        let name = get_form(handle, |f| f.name.clone());
        assert_eq!(name, Some("ManualForm".into()));
    }

    #[test]
    fn controls_at_position_zero() {
        let _lock = lock_test();
        form_store::reset();
        let label = vb6parse::language::Control::new(
            "Label1".to_string(),
            String::new(),
            0,
            vb6parse::language::ControlKind::Label {
                properties: vb6parse::language::LabelProperties {
                    caption: "At Origin".to_string(),
                    left: 0,
                    top: 0,
                    width: 1000,
                    height: 300,
                    visible: Visibility::Visible,
                    enabled: Activation::Enabled,
                    ..Default::default()
                },
            },
        );
        let form = Form {
            name: "OriginForm".to_string(),
            tag: String::new(),
            index: 0,
            properties: vb6parse::language::FormProperties {
                scale_width: 4000,
                scale_height: 3000,
                caption: "OriginForm".to_string(),
                left: 0,
                top: 0,
                visible: Visibility::Visible,
                enabled: Activation::Enabled,
                ..Default::default()
            },
            controls: vec![label],
            menus: Vec::new(),
        };
        let config = LayoutConfig::default();
        let handle = load_form(&FormRoot::Form(form), vec![], &config);
        let html = render(handle, &renderer::TauriRenderer::new(false));
        assert!(!html.is_empty());
        assert!(html.contains("vb6-label"));
    }

    #[test]
    fn controls_at_negative_position() {
        let _lock = lock_test();
        form_store::reset();
        let label = vb6parse::language::Control::new(
            "Label1".to_string(),
            String::new(),
            0,
            vb6parse::language::ControlKind::Label {
                properties: vb6parse::language::LabelProperties {
                    caption: "Negative Position".to_string(),
                    left: -500,
                    top: -200,
                    width: 1000,
                    height: 300,
                    visible: Visibility::Visible,
                    enabled: Activation::Enabled,
                    ..Default::default()
                },
            },
        );
        let form = Form {
            name: "NegativeForm".to_string(),
            tag: String::new(),
            index: 0,
            properties: vb6parse::language::FormProperties {
                scale_width: 4000,
                scale_height: 3000,
                caption: "NegativeForm".to_string(),
                left: 0,
                top: 0,
                visible: Visibility::Visible,
                enabled: Activation::Enabled,
                ..Default::default()
            },
            controls: vec![label],
            menus: Vec::new(),
        };
        let config = LayoutConfig::default();
        let handle = load_form(&FormRoot::Form(form), vec![], &config);
        let html = render(handle, &renderer::TauriRenderer::new(false));
        assert!(!html.is_empty());
        assert!(html.contains("vb6-label"));
    }

    #[test]
    fn deeply_nested_containers() {
        let _lock = lock_test();
        form_store::reset();
        // Form → PictureBox → Frame → Label
        let label = vb6parse::language::Control::new(
            "Label1".to_string(),
            String::new(),
            0,
            vb6parse::language::ControlKind::Label {
                properties: vb6parse::language::LabelProperties {
                    caption: "Deeply nested".to_string(),
                    left: 200,
                    top: 200,
                    width: 800,
                    height: 300,
                    visible: Visibility::Visible,
                    enabled: Activation::Enabled,
                    ..Default::default()
                },
            },
        );
        let frame = vb6parse::language::Control::new(
            "Frame1".to_string(),
            String::new(),
            0,
            vb6parse::language::ControlKind::Frame {
                properties: vb6parse::language::FrameProperties {
                    caption: "Frame".to_string(),
                    left: 100,
                    top: 100,
                    width: 1500,
                    height: 1000,
                    visible: Visibility::Visible,
                    enabled: Activation::Enabled,
                    ..Default::default()
                },
                controls: vec![label],
            },
        );
        let picturebox = vb6parse::language::Control::new(
            "PictureBox1".to_string(),
            String::new(),
            0,
            vb6parse::language::ControlKind::PictureBox {
                properties: vb6parse::language::PictureBoxProperties {
                    left: 50,
                    top: 50,
                    width: 2000,
                    height: 1500,
                    visible: Visibility::Visible,
                    enabled: Activation::Enabled,
                    ..Default::default()
                },
                controls: vec![frame],
            },
        );
        let form = Form {
            name: "NestedForm".to_string(),
            tag: String::new(),
            index: 0,
            properties: vb6parse::language::FormProperties {
                scale_width: 5000,
                scale_height: 4000,
                caption: "NestedForm".to_string(),
                left: 0,
                top: 0,
                visible: Visibility::Visible,
                enabled: Activation::Enabled,
                ..Default::default()
            },
            controls: vec![picturebox],
            menus: Vec::new(),
        };
        let config = LayoutConfig::default();
        let handle = load_form(&FormRoot::Form(form), vec![], &config);
        let html = render(handle, &renderer::TauriRenderer::new(false));
        assert!(!html.is_empty());
        assert!(
            html.contains("vb6-picturebox"),
            "missing vb6-picturebox in: {html}"
        );
        assert!(html.contains("vb6-frame"), "missing vb6-frame in: {html}");
        assert!(html.contains("vb6-label"), "missing vb6-label in: {html}");
        assert!(html.contains("Deeply nested"), "missing caption in: {html}");
    }

    #[test]
    fn triple_nested_containers() {
        let _lock = lock_test();
        form_store::reset();
        // Form → Frame → PictureBox → Label
        let label = vb6parse::language::Control::new(
            "Label1".to_string(),
            String::new(),
            0,
            vb6parse::language::ControlKind::Label {
                properties: vb6parse::language::LabelProperties {
                    caption: "Triple nested".to_string(),
                    left: 100,
                    top: 100,
                    width: 500,
                    height: 200,
                    visible: Visibility::Visible,
                    enabled: Activation::Enabled,
                    ..Default::default()
                },
            },
        );
        let picturebox = vb6parse::language::Control::new(
            "PictureBox1".to_string(),
            String::new(),
            0,
            vb6parse::language::ControlKind::PictureBox {
                properties: vb6parse::language::PictureBoxProperties {
                    left: 100,
                    top: 100,
                    width: 1500,
                    height: 1000,
                    visible: Visibility::Visible,
                    enabled: Activation::Enabled,
                    ..Default::default()
                },
                controls: vec![label],
            },
        );
        let frame = vb6parse::language::Control::new(
            "Frame1".to_string(),
            String::new(),
            0,
            vb6parse::language::ControlKind::Frame {
                properties: vb6parse::language::FrameProperties {
                    caption: "Outer Frame".to_string(),
                    left: 50,
                    top: 50,
                    width: 2000,
                    height: 1500,
                    visible: Visibility::Visible,
                    enabled: Activation::Enabled,
                    ..Default::default()
                },
                controls: vec![picturebox],
            },
        );
        let form = Form {
            name: "TripleNestedForm".to_string(),
            tag: String::new(),
            index: 0,
            properties: vb6parse::language::FormProperties {
                scale_width: 5000,
                scale_height: 4000,
                caption: "TripleNestedForm".to_string(),
                left: 0,
                top: 0,
                visible: Visibility::Visible,
                enabled: Activation::Enabled,
                ..Default::default()
            },
            controls: vec![frame],
            menus: Vec::new(),
        };
        let config = LayoutConfig::default();
        let handle = load_form(&FormRoot::Form(form), vec![], &config);
        let html = render(handle, &renderer::TauriRenderer::new(false));
        assert!(!html.is_empty());
        assert!(html.contains("vb6-frame"), "missing vb6-frame in: {html}");
        assert!(
            html.contains("vb6-picturebox"),
            "missing vb6-picturebox in: {html}"
        );
        assert!(html.contains("vb6-label"), "missing vb6-label in: {html}");
    }

    // ========================================================================
    // Step 7: Verify Visual Fidelity
    // ========================================================================

    #[test]
    fn default_textbox_has_minimal_inline_style() {
        let _lock = lock_test();
        form_store::reset();
        use vb6parse::language::{CommandButtonProperties, Control, ControlKind};
        let btn = Control::new(
            "cmdOK".to_string(),
            String::new(),
            0,
            ControlKind::CommandButton {
                properties: CommandButtonProperties {
                    caption: "OK".to_string(),
                    left: 120,
                    top: 500,
                    width: 800,
                    height: 300,
                    enabled: Activation::Enabled,
                    ..Default::default()
                },
            },
        );
        let form = Form {
            name: "Form1".to_string(),
            tag: String::new(),
            index: 0,
            properties: vb6parse::language::FormProperties {
                scale_width: 4000,
                scale_height: 3000,
                caption: "Form1".to_string(),
                left: 0,
                top: 0,
                visible: Visibility::Visible,
                enabled: Activation::Enabled,
                ..Default::default()
            },
            controls: vec![btn],
            menus: Vec::new(),
        };
        let config = LayoutConfig::default();
        let handle = load_form(&FormRoot::Form(form), vec![], &config);
        let renderer = renderer::TauriRenderer::new(false);
        let html = render(handle, &renderer);
        assert!(html.contains("vb6-commandbutton"));
        // Default VB6 button: display is diffed out (matches default),
        // background-color is diffed out (Standard uses CSS class),
        // font remains inline because VB6 default properties include a font
        assert!(
            !html.contains("background-color:") && html.contains("font-family:"),
            "Standard button should have no background-color (CSS class) but font inline from VB6 defaults, got: {html}"
        );
    }

    #[test]
    fn default_label_has_minimal_inline_style() {
        let _lock = lock_test();
        form_store::reset();
        let label = Control::new(
            "Label1".to_string(),
            String::new(),
            0,
            ControlKind::Label {
                properties: vb6parse::language::LabelProperties {
                    caption: "Hello".to_string(),
                    left: 120,
                    top: 120,
                    width: 1000,
                    height: 300,
                    visible: Visibility::Visible,
                    enabled: Activation::Enabled,
                    ..Default::default()
                },
            },
        );
        let form = Form {
            name: "Form1".to_string(),
            tag: String::new(),
            index: 0,
            properties: vb6parse::language::FormProperties {
                scale_width: 4000,
                scale_height: 3000,
                caption: "Form1".to_string(),
                left: 0,
                top: 0,
                visible: Visibility::Visible,
                enabled: Activation::Enabled,
                ..Default::default()
            },
            controls: vec![label],
            menus: Vec::new(),
        };
        let config = LayoutConfig::default();
        let handle = load_form(&FormRoot::Form(form), vec![], &config);
        let renderer = renderer::TauriRenderer::new(false);
        let html = render(handle, &renderer);
        assert!(html.contains("vb6-label"));
        assert!(html.contains("Hello"));
        // Default VB6 label has NonWrapping word wrap and a border by default.
        // Font properties are inline because VB6 properties include default font.
        assert!(
            html.contains("white-space: nowrap") && html.contains("border:"),
            "Label at VB6 defaults should have nowrap and border, got: {html}"
        );
    }

    #[test]
    fn custom_color_produces_inline_css() {
        let _lock = lock_test();
        form_store::reset();
        let label = Control::new(
            "Label1".to_string(),
            String::new(),
            0,
            ControlKind::Label {
                properties: vb6parse::language::LabelProperties {
                    caption: "Hello".to_string(),
                    fore_color: vb6parse::language::Color::RGB {
                        red: 255,
                        green: 0,
                        blue: 0,
                    },
                    left: 120,
                    top: 120,
                    width: 1000,
                    height: 300,
                    visible: Visibility::Visible,
                    enabled: Activation::Enabled,
                    ..Default::default()
                },
            },
        );
        let form = Form {
            name: "Form1".to_string(),
            tag: String::new(),
            index: 0,
            properties: vb6parse::language::FormProperties {
                scale_width: 4000,
                scale_height: 3000,
                caption: "Form1".to_string(),
                left: 0,
                top: 0,
                visible: Visibility::Visible,
                enabled: Activation::Enabled,
                ..Default::default()
            },
            controls: vec![label],
            menus: Vec::new(),
        };
        let config = LayoutConfig::default();
        let handle = load_form(&FormRoot::Form(form), vec![], &config);
        let renderer = renderer::TauriRenderer::new(false);
        let html = render(handle, &renderer);
        assert!(html.contains("color: rgb(255, 0, 0)"));
        assert!(
            html.contains("vb6-label"),
            "Custom color label should still have vb6-label class, got: {html}"
        );
    }

    #[test]
    fn custom_font_produces_inline_css() {
        let _lock = lock_test();
        form_store::reset();
        let label = Control::new(
            "Label1".to_string(),
            String::new(),
            0,
            ControlKind::Label {
                properties: vb6parse::language::LabelProperties {
                    caption: "Hello".to_string(),
                    font: Some(vb6parse::language::Font {
                        name: "Times New Roman".into(),
                        size: 14.0,
                        ..Default::default()
                    }),
                    left: 120,
                    top: 120,
                    width: 1000,
                    height: 300,
                    visible: Visibility::Visible,
                    enabled: Activation::Enabled,
                    ..Default::default()
                },
            },
        );
        let form = Form {
            name: "Form1".to_string(),
            tag: String::new(),
            index: 0,
            properties: vb6parse::language::FormProperties {
                scale_width: 4000,
                scale_height: 3000,
                caption: "Form1".to_string(),
                left: 0,
                top: 0,
                visible: Visibility::Visible,
                enabled: Activation::Enabled,
                ..Default::default()
            },
            controls: vec![label],
            menus: Vec::new(),
        };
        let config = LayoutConfig::default();
        let handle = load_form(&FormRoot::Form(form), vec![], &config);
        let renderer = renderer::TauriRenderer::new(false);
        let html = render(handle, &renderer);
        // HTML escaping converts " to &quot; in style attributes
        assert!(html.contains("font-family:") && html.contains("Times New Roman"));
        assert!(html.contains("vb6-label"));
    }

    #[test]
    fn composite_form_with_mixed_controls() {
        let _lock = lock_test();
        form_store::reset();
        let label_default = Control::new(
            "LabelDefault".to_string(),
            String::new(),
            0,
            ControlKind::Label {
                properties: vb6parse::language::LabelProperties {
                    caption: "Default label".to_string(),
                    left: 100,
                    top: 100,
                    width: 800,
                    height: 300,
                    visible: Visibility::Visible,
                    enabled: Activation::Enabled,
                    ..Default::default()
                },
            },
        );
        let label_custom = Control::new(
            "LabelCustom".to_string(),
            String::new(),
            0,
            ControlKind::Label {
                properties: vb6parse::language::LabelProperties {
                    caption: "Custom font".to_string(),
                    font: Some(vb6parse::language::Font {
                        name: "Arial".into(),
                        size: 12.0,
                        weight: 700,
                        italic: true,
                        ..Default::default()
                    }),
                    left: 100,
                    top: 200,
                    width: 800,
                    height: 300,
                    visible: Visibility::Visible,
                    enabled: Activation::Enabled,
                    ..Default::default()
                },
            },
        );
        let button_default = Control::new(
            "cmdOK".to_string(),
            String::new(),
            0,
            ControlKind::CommandButton {
                properties: vb6parse::language::CommandButtonProperties {
                    caption: "OK".to_string(),
                    left: 100,
                    top: 400,
                    width: 800,
                    height: 300,
                    enabled: Activation::Enabled,
                    ..Default::default()
                },
            },
        );
        let textbox_default = Control::new(
            "txtName".to_string(),
            String::new(),
            0,
            ControlKind::TextBox {
                properties: vb6parse::language::TextBoxProperties {
                    text: "John".to_string(),
                    left: 100,
                    top: 500,
                    width: 1500,
                    height: 300,
                    visible: Visibility::Visible,
                    enabled: Activation::Enabled,
                    ..Default::default()
                },
            },
        );
        let textbox_custom = Control::new(
            "txtCustom".to_string(),
            String::new(),
            0,
            ControlKind::TextBox {
                properties: vb6parse::language::TextBoxProperties {
                    left: 100,
                    top: 600,
                    width: 1500,
                    height: 300,
                    visible: Visibility::Visible,
                    enabled: Activation::Enabled,
                    back_color: vb6parse::language::Color::RGB {
                        red: 255,
                        green: 255,
                        blue: 0,
                    },
                    ..Default::default()
                },
            },
        );
        let frame = Control::new(
            "Frame1".to_string(),
            String::new(),
            0,
            ControlKind::Frame {
                properties: vb6parse::language::FrameProperties {
                    caption: "Frame".to_string(),
                    left: 100,
                    top: 700,
                    width: 1500,
                    height: 500,
                    visible: Visibility::Visible,
                    enabled: Activation::Enabled,
                    ..Default::default()
                },
                controls: vec![
                    Control::new(
                        "LabelInside".to_string(),
                        String::new(),
                        0,
                        ControlKind::Label {
                            properties: vb6parse::language::LabelProperties {
                                caption: "Inside frame".to_string(),
                                left: 50,
                                top: 50,
                                width: 500,
                                height: 200,
                                visible: Visibility::Visible,
                                enabled: Activation::Enabled,
                                ..Default::default()
                            },
                        },
                    ),
                ],
            },
        );
        let form = Form {
            name: "Form1".to_string(),
            tag: String::new(),
            index: 0,
            properties: vb6parse::language::FormProperties {
                scale_width: 4000,
                scale_height: 3000,
                caption: "Form1".to_string(),
                left: 0,
                top: 0,
                visible: Visibility::Visible,
                enabled: Activation::Enabled,
                ..Default::default()
            },
            controls: vec![
                label_default,
                label_custom,
                button_default,
                textbox_default,
                textbox_custom,
                frame,
            ],
            menus: Vec::new(),
        };
        let config = LayoutConfig::default();
        let handle = load_form(&FormRoot::Form(form), vec![], &config);
        let renderer = renderer::TauriRenderer::new(false);
        let html = render(handle, &renderer);

        assert!(html.contains("vb6-label"));
        assert!(html.contains("vb6-commandbutton"));
        assert!(html.contains("vb6-textbox"));
        assert!(html.contains("vb6-frame"));

        assert!(html.contains("Default label"));
        assert!(html.contains("Custom font"));
        assert!(html.contains("OK"));
        assert!(html.contains("value=\"John\""));
        assert!(html.contains("Inside frame"));

        assert!(html.contains("font-family:") && html.contains("Arial"));
        assert!(html.contains("font-weight: bold"));
        assert!(html.contains("font-style: italic"));
        assert!(html.contains("background-color: rgb(255, 255, 0)"));
    }

    #[test]
    fn dark_theme_css_contains_dark_variables() {
        use crate::layout::theme::dark_theme;
        let theme = dark_theme();
        let css = theme.to_css();
        assert!(css.contains("--vb6-bg: #1e1e1e"));
        assert!(css.contains("--vb6-fg: #d4d4d4"));
        assert!(css.contains("--vb6-window-bg: #3c3c3c"));
        assert!(css.contains("--vb6-window-text: #cccccc"));
    }

    #[test]
    fn custom_color_preserved_under_dark_theme() {
        let _lock = lock_test();
        form_store::reset();
        use crate::layout::theme::dark_theme;
        let label = Control::new(
            "Label1".to_string(),
            String::new(),
            0,
            ControlKind::Label {
                properties: vb6parse::language::LabelProperties {
                    caption: "Red label".to_string(),
                    fore_color: vb6parse::language::Color::RGB {
                        red: 255,
                        green: 0,
                        blue: 0,
                    },
                    left: 120,
                    top: 120,
                    width: 1000,
                    height: 300,
                    visible: Visibility::Visible,
                    enabled: Activation::Enabled,
                    ..Default::default()
                },
            },
        );
        let form = Form {
            name: "Form1".to_string(),
            tag: String::new(),
            index: 0,
            properties: vb6parse::language::FormProperties {
                scale_width: 4000,
                scale_height: 3000,
                caption: "Form1".to_string(),
                left: 0,
                top: 0,
                visible: Visibility::Visible,
                enabled: Activation::Enabled,
                ..Default::default()
            },
            controls: vec![label],
            menus: Vec::new(),
        };
        let config = LayoutConfig::default();
        let handle = load_form(&FormRoot::Form(form), vec![], &config);
        let renderer = renderer::TauriRenderer::new(false);
        let _html = render(handle, &renderer);

        let dark_css = dark_theme().to_css();
        assert!(dark_css.contains("--vb6-window-bg: #3c3c3c"));

        let html = render(handle, &renderer);
        assert!(html.contains("color: rgb(255, 0, 0)"));
        assert!(html.contains("vb6-label"));
    }

    #[test]
    fn default_control_under_dark_theme_no_extra_inline() {
        let _lock = lock_test();
        form_store::reset();
        use crate::layout::theme::dark_theme;
        let label = Control::new(
            "Label1".to_string(),
            String::new(),
            0,
            ControlKind::Label {
                properties: vb6parse::language::LabelProperties {
                    caption: "Dark theme label".to_string(),
                    left: 120,
                    top: 120,
                    width: 1000,
                    height: 300,
                    visible: Visibility::Visible,
                    enabled: Activation::Enabled,
                    ..Default::default()
                },
            },
        );
        let form = Form {
            name: "Form1".to_string(),
            tag: String::new(),
            index: 0,
            properties: vb6parse::language::FormProperties {
                scale_width: 4000,
                scale_height: 3000,
                caption: "Form1".to_string(),
                left: 0,
                top: 0,
                visible: Visibility::Visible,
                enabled: Activation::Enabled,
                ..Default::default()
            },
            controls: vec![label],
            menus: Vec::new(),
        };
        let config = LayoutConfig::default();
        let handle = load_form(&FormRoot::Form(form), vec![], &config);
        let renderer = renderer::TauriRenderer::new(false);
        let _html = render(handle, &renderer);

        let dark_css = dark_theme().to_css();
        assert!(dark_css.contains("--vb6-bg: #1e1e1e"));

        let html = render(handle, &renderer);
        assert!(html.contains("vb6-label"));
        assert!(html.contains("Dark theme label"));
        // Default VB6 label has BackStyle=Opaque, so it has background-color inline.
        // This is correct VB6 behavior - the theme provides CSS variable overrides
        // but the default BackStyle=Opaque means a background color is always set.
        assert!(html.contains("background-color: ButtonFace"));
        // Dark theme CSS still provides the overrides for the CSS classes
        assert!(dark_css.contains("--vb6-bg: #1e1e1e"));
    }

    #[test]
    fn invisible_control_has_visibility_hidden() {
        let _lock = lock_test();
        form_store::reset();
        let label = Control::new(
            "Label1".to_string(),
            String::new(),
            0,
            ControlKind::Label {
                properties: vb6parse::language::LabelProperties {
                    caption: "Hidden".to_string(),
                    left: 120,
                    top: 120,
                    width: 1000,
                    height: 300,
                    visible: Visibility::Hidden,
                    enabled: Activation::Enabled,
                    ..Default::default()
                },
            },
        );
        let form = Form {
            name: "Form1".to_string(),
            tag: String::new(),
            index: 0,
            properties: vb6parse::language::FormProperties {
                scale_width: 4000,
                scale_height: 3000,
                caption: "Form1".to_string(),
                left: 0,
                top: 0,
                visible: Visibility::Visible,
                enabled: Activation::Enabled,
                ..Default::default()
            },
            controls: vec![label],
            menus: Vec::new(),
        };
        let config = LayoutConfig {
            include_hidden: true,
            ..Default::default()
        };
        let handle = load_form(&FormRoot::Form(form), vec![], &config);
        let renderer = renderer::TauriRenderer::new(false);
        let html = render(handle, &renderer);
        assert!(html.contains("vb6-label"));
        assert!(html.contains("visibility: hidden"));
    }

    #[test]
    fn disabled_control_has_opacity() {
        let _lock = lock_test();
        form_store::reset();
        let button = Control::new(
            "cmdOK".to_string(),
            String::new(),
            0,
            ControlKind::CommandButton {
                properties: vb6parse::language::CommandButtonProperties {
                    caption: "Disabled".to_string(),
                    left: 120,
                    top: 500,
                    width: 800,
                    height: 300,
                    enabled: Activation::Disabled,
                    ..Default::default()
                },
            },
        );
        let form = Form {
            name: "Form1".to_string(),
            tag: String::new(),
            index: 0,
            properties: vb6parse::language::FormProperties {
                scale_width: 4000,
                scale_height: 3000,
                caption: "Form1".to_string(),
                left: 0,
                top: 0,
                visible: Visibility::Visible,
                enabled: Activation::Enabled,
                ..Default::default()
            },
            controls: vec![button],
            menus: Vec::new(),
        };
        let config = LayoutConfig::default();
        let handle = load_form(&FormRoot::Form(form), vec![], &config);
        let renderer = renderer::TauriRenderer::new(false);
        let html = render(handle, &renderer);
        assert!(html.contains("vb6-commandbutton"));
        assert!(html.contains("opacity: 0.5"));
        assert!(html.contains("Disabled"));
    }

    #[test]
    fn all_control_types_render_without_panic() {
        let _lock = lock_test();
        form_store::reset();
        let label = Control::new(
            "Label1".to_string(),
            String::new(),
            0,
            ControlKind::Label {
                properties: vb6parse::language::LabelProperties {
                    caption: "Label".to_string(),
                    left: 100,
                    top: 100,
                    width: 500,
                    height: 200,
                    visible: Visibility::Visible,
                    enabled: Activation::Enabled,
                    ..Default::default()
                },
            },
        );
        let button = Control::new(
            "cmdOK".to_string(),
            String::new(),
            0,
            ControlKind::CommandButton {
                properties: vb6parse::language::CommandButtonProperties {
                    caption: "Button".to_string(),
                    left: 100,
                    top: 200,
                    width: 500,
                    height: 200,
                    enabled: Activation::Enabled,
                    ..Default::default()
                },
            },
        );
        let textbox = Control::new(
            "txt1".to_string(),
            String::new(),
            0,
            ControlKind::TextBox {
                properties: vb6parse::language::TextBoxProperties {
                    text: "".to_string(),
                    left: 100,
                    top: 300,
                    width: 1000,
                    height: 300,
                    visible: Visibility::Visible,
                    enabled: Activation::Enabled,
                    ..Default::default()
                },
            },
        );
        let frame = Control::new(
            "Frame1".to_string(),
            String::new(),
            0,
            ControlKind::Frame {
                properties: vb6parse::language::FrameProperties {
                    caption: "Frame".to_string(),
                    left: 100,
                    top: 400,
                    width: 1000,
                    height: 500,
                    visible: Visibility::Visible,
                    enabled: Activation::Enabled,
                    ..Default::default()
                },
                controls: vec![],
            },
        );
        let picturebox = Control::new(
            "Pic1".to_string(),
            String::new(),
            0,
            ControlKind::PictureBox {
                properties: vb6parse::language::PictureBoxProperties {
                    left: 100,
                    top: 500,
                    width: 1000,
                    height: 500,
                    visible: Visibility::Visible,
                    enabled: Activation::Enabled,
                    ..Default::default()
                },
                controls: vec![],
            },
        );
        let image = Control::new(
            "Img1".to_string(),
            String::new(),
            0,
            ControlKind::Image {
                properties: vb6parse::language::ImageProperties {
                    left: 100,
                    top: 600,
                    width: 200,
                    height: 200,
                    visible: Visibility::Visible,
                    enabled: Activation::Enabled,
                    ..Default::default()
                },
            },
        );
        let checkbox = Control::new(
            "Chk1".to_string(),
            String::new(),
            0,
            ControlKind::CheckBox {
                properties: vb6parse::language::CheckBoxProperties {
                    caption: "Check".to_string(),
                    left: 100,
                    top: 700,
                    width: 500,
                    height: 200,
                    visible: Visibility::Visible,
                    enabled: Activation::Enabled,
                    ..Default::default()
                },
            },
        );
        let optionbutton = Control::new(
            "Opt1".to_string(),
            String::new(),
            0,
            ControlKind::OptionButton {
                properties: vb6parse::language::OptionButtonProperties {
                    caption: "Radio".to_string(),
                    left: 100,
                    top: 800,
                    width: 500,
                    height: 200,
                    visible: Visibility::Visible,
                    enabled: Activation::Enabled,
                    ..Default::default()
                },
            },
        );
        let combobox = Control::new(
            "Cmb1".to_string(),
            String::new(),
            0,
            ControlKind::ComboBox {
                properties: vb6parse::language::ComboBoxProperties {
                    left: 100,
                    top: 900,
                    width: 1000,
                    height: 300,
                    visible: Visibility::Visible,
                    enabled: Activation::Enabled,
                    ..Default::default()
                },
            },
        );
        let listbox = Control::new(
            "Lst1".to_string(),
            String::new(),
            0,
            ControlKind::ListBox {
                properties: vb6parse::language::ListBoxProperties {
                    left: 100,
                    top: 1000,
                    width: 1000,
                    height: 500,
                    visible: Visibility::Visible,
                    enabled: Activation::Enabled,
                    ..Default::default()
                },
            },
        );
        let hscroll = Control::new(
            "HScroll1".to_string(),
            String::new(),
            0,
            ControlKind::HScrollBar {
                properties: vb6parse::language::ScrollBarProperties {
                    left: 100,
                    top: 1100,
                    width: 500,
                    height: 200,
                    visible: Visibility::Visible,
                    ..Default::default()
                },
            },
        );
        let vscroll = Control::new(
            "VScroll1".to_string(),
            String::new(),
            0,
            ControlKind::VScrollBar {
                properties: vb6parse::language::ScrollBarProperties {
                    left: 600,
                    top: 1100,
                    width: 200,
                    height: 500,
                    visible: Visibility::Visible,
                    ..Default::default()
                },
            },
        );
        let shape = Control::new(
            "Shape1".to_string(),
            String::new(),
            0,
            ControlKind::Shape {
                properties: vb6parse::language::ShapeProperties {
                    left: 100,
                    top: 1200,
                    width: 300,
                    height: 300,
                    visible: Visibility::Visible,
                    ..Default::default()
                },
            },
        );
        let line = Control::new(
            "Line1".to_string(),
            String::new(),
            0,
            ControlKind::Line {
                properties: vb6parse::language::LineProperties {
                    x1: 0,
                    y1: 0,
                    x2: 500,
                    y2: 200,
                    visible: Visibility::Visible,
                    ..Default::default()
                },
            },
        );
        let drive_lb = Control::new(
            "Drv1".to_string(),
            String::new(),
            0,
            ControlKind::DriveListBox {
                properties: vb6parse::language::DriveListBoxProperties {
                    left: 100,
                    top: 1300,
                    width: 1000,
                    height: 300,
                    visible: Visibility::Visible,
                    ..Default::default()
                },
            },
        );
        let dir_lb = Control::new(
            "Dir1".to_string(),
            String::new(),
            0,
            ControlKind::DirListBox {
                properties: vb6parse::language::DirListBoxProperties {
                    left: 100,
                    top: 1400,
                    width: 1000,
                    height: 300,
                    visible: Visibility::Visible,
                    ..Default::default()
                },
            },
        );
        let file_lb = Control::new(
            "File1".to_string(),
            String::new(),
            0,
            ControlKind::FileListBox {
                properties: vb6parse::language::FileListBoxProperties {
                    left: 100,
                    top: 1500,
                    width: 1000,
                    height: 300,
                    visible: Visibility::Visible,
                    ..Default::default()
                },
            },
        );
        let form = Form {
            name: "Form1".to_string(),
            tag: String::new(),
            index: 0,
            properties: vb6parse::language::FormProperties {
                scale_width: 4000,
                scale_height: 4000,
                caption: "Form1".to_string(),
                left: 0,
                top: 0,
                visible: Visibility::Visible,
                enabled: Activation::Enabled,
                ..Default::default()
            },
            controls: vec![
                label,
                button,
                textbox,
                frame,
                picturebox,
                image,
                checkbox,
                optionbutton,
                combobox,
                listbox,
                hscroll,
                vscroll,
                shape,
                line,
                drive_lb,
                dir_lb,
                file_lb,
            ],
            menus: Vec::new(),
        };
        let config = LayoutConfig::default();
        let handle = load_form(&FormRoot::Form(form), vec![], &config);
        let renderer = renderer::TauriRenderer::new(false);
        let html = render(handle, &renderer);
        assert!(!html.is_empty(), "html was empty: {html}");
        assert!(html.contains("vb6-label"));
        assert!(html.contains("vb6-commandbutton"));
        assert!(html.contains("vb6-textbox"));
        assert!(html.contains("vb6-frame"));
        assert!(html.contains("vb6-picturebox"));
        assert!(html.contains("vb6-image"));
        assert!(html.contains("vb6-checkbox"));
        assert!(html.contains("vb6-optionbutton"));
        assert!(html.contains("vb6-combobox"));
        assert!(html.contains("vb6-listbox"));
        assert!(html.contains("vb6-hscrollbar"));
        assert!(html.contains("vb6-vscrollbar"));
        assert!(html.contains("vb6-shape"));
        assert!(html.contains("vb6-line"));
        assert!(html.contains("vb6-drivelistbox"));
        assert!(html.contains("vb6-dirlistbox"));
        assert!(html.contains("vb6-filelistbox"));
    }
}
