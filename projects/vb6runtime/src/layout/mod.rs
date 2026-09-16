//! Layout module for the VB6 runtime.
//!
//! This module provides a layout engine that takes a parsed VB6 control tree
//! and produces an interactive, web-renderable UI in HTML/CSS format,
//! targetable at both WASM and Tauri runtimes.
//!
//! The engine is structured in layers:
//! - [`model`] — core layout types (nodes, positions, sizes, styles)
//! - [`converter`] — transforms parsed `FormRoot` into the layout model
//! - [`scale`] — twip-to-pixel and ScaleMode conversion
//! - [`color`] — VB6 Color to CSS color mapping
//! - [`css`] — `LayoutStyle` to CSS declaration string
//! - [`controls`] — per-control style builders
//! - [`form_store`] — mutable state store for loaded forms
//! - [`menu`] — menubar and context/popup menu rendering
//! - [`renderer`] — platform-abstract rendering trait + Tauri backend
//! - [`theme`] — CSS custom property theming + injection API
//! - [`vb6_css`] — complete VB6 CSS stylesheet with scoped selectors

pub mod color;
/// Per-control style builder dispatch.
pub mod controls;
/// Tree walker: FormRoot → LayoutNode tree → form store.
pub mod converter;
/// LayoutStyle → CSS declaration string.
pub mod css;
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
/// CSS custom property theming + injection API.
pub mod theme;
/// Full VB6 CSS stylesheet with scoped selectors for platform isolation.
pub mod vb6_css;

use model::style::LayoutStyle;

// Re-export converter types at the module level for convenience.
pub use converter::{load_form, LayoutError, LayoutResult};

// Re-export form store handle type.
pub use form_store::FormHandle;

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
pub fn build_style_for_control(kind: &vb6parse::language::ControlKind, config: &LayoutConfig) -> LayoutStyle {
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
pub fn get_form_mut<T>(handle: FormHandle, f: impl FnOnce(&mut model::LayoutForm) -> T) -> Option<T> {
    form_store::get_mut(handle, f)
}

/// Render a form using the given [`renderer::Renderer`] implementation.
///
/// Walks the [`LayoutNode`] tree of the loaded form and produces
/// platform-specific output (HTML string for Tauri, DOM elements for WASM).
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
    let form = form_store::get(handle, |f| f.clone()).expect("unknown form handle");
    renderer.render_node(&form.root_node)
}

#[cfg(test)]
mod tests {
    use super::*;
    use vb6parse::language::{Activation, FormRoot, Form, Visibility};

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
        use vb6parse::language::{Control, ControlKind, CommandButtonProperties};
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
        let handle = load_form(&FormRoot::Form(form), &config);
        let name = get_form(handle, |f| f.name.clone());
        assert_eq!(name, Some("Form1".into()));
    }

    #[test]
    fn test_get_form_mut() {
        let _lock = lock_test();
        let form = make_test_form_with_button();
        let config = LayoutConfig::default();
        let handle = load_form(&FormRoot::Form(form), &config);
        let changed = get_form_mut(handle, |f| {
            f.caption = "Modified".into();
            f.caption.clone()
        });
        assert_eq!(changed, Some("Modified".into()));
        let check = get_form(handle, |f| f.caption.clone());
        assert_eq!(check, Some("Modified".into()));
    }

    #[test]
    fn render_with_tauri_renderer() {
        let _lock = lock_test();
        let form = make_test_form_with_label();
        let config = LayoutConfig::default();
        let handle = load_form(&FormRoot::Form(form), &config);
        let renderer = renderer::TauriRenderer::new(false);
        let html = render(handle, &renderer);
        assert!(!html.is_empty(), "html was empty: {html}");
        assert!(html.contains("vb6-form"), "html missing vb6-form: {html}");
        assert!(html.contains("vb6-label"), "html missing vb6-label: {html}");
    }

    #[test]
    fn render_with_scope() {
        let _lock = lock_test();
        let form = make_test_form_with_label();
        let config = LayoutConfig::default();
        let handle = load_form(&FormRoot::Form(form), &config);
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
        let handle1 = load_form(&FormRoot::Form(form1), &config);
        let handle2 = load_form(&FormRoot::Form(form2), &config);
        let renderer = renderer::TauriRenderer::new(false);
        let html1 = render(handle1, &renderer);
        let html2 = render(handle2, &renderer);
        assert!(html1.contains("Form1"));
        assert!(html2.contains("Form1"));
        assert!(html1.contains("vb6-label"));
        assert!(html2.contains("vb6-commandbutton"));
    }
}
