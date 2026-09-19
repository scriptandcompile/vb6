//! Tree walker: converts a parsed `FormRoot` into a `LayoutNode` tree.
//!
//! Recursively processes controls, rejects unsupported types (Custom/Ole), and builds
//! style and position data via the converter pipeline.
//!
//! # Conversion Pipeline
//!
//! 1. `load_form` accepts a `FormRoot` (Form or MDIForm) and returns a `FormHandle`
//! 2. Form properties (size, colors, caption) are converted to `LayoutForm`
//! 3. Each control is recursively converted via `convert_control`
//! 4. Frame/PictureBox children are nested inside their parent containers
//! 5. Custom/Ole controls cause immediate rejection with `LayoutError::UnsupportedControl`
//! 6. Timer/Data controls are skipped when `include_nonvisual` is false
//! 7. Positions/sizes are converted from twips (or form scale mode) to pixels
//! 8. The `LayoutForm` is stored in the form store and the handle is returned

use std::collections::HashMap;

use base64::{Engine as _, engine::general_purpose::STANDARD};
use image::DynamicImage;
use vb6parse::language::{
    Activation, BorderStyle, Control, ControlKind, Form, FormBorderStyle, MDIForm, MultiLine,
    ReferenceOrValue, ScaleMode, ScrollBars, TabStop, TextDirection, Visibility,
};
use vb6parse::parsers::{ConcreteSyntaxTree, SyntaxKind};

use super::model::LayoutControlType;
use super::model::{
    EventProcedure, LayoutContainer, LayoutForm, LayoutLeaf, LayoutNode, LayoutPosition,
    LayoutSize, LayoutStyle, NodeId,
};
use super::scale::{scale_mode_to_pixels, twips_to_pixels};
use super::{
    LayoutConfig,
    color::{color_to_css, mouse_pointer_css},
    font_points_to_px,
};

use super::form_store::{self, FormHandle};

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

/// Error type for layout conversion failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutError {
    /// A Custom or OLE control was encountered but is not supported in Phase 1.
    UnsupportedControl {
        /// The name of the unsupported control.
        name: String,
        /// The kind of control (`"Custom"` or `"OLE"`).
        kind: String,
        /// A human-readable explanation.
        message: String,
    },
    /// A position/size or property conversion error.
    ConversionError {
        /// The name of the control that caused the error.
        control: String,
        /// A description of what went wrong.
        reason: String,
    },
}

impl std::fmt::Display for LayoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayoutError::UnsupportedControl {
                name,
                kind,
                message,
            } => {
                write!(f, "Unsupported control '{name}': {kind} ({message})")
            }
            LayoutError::ConversionError { control, reason } => {
                write!(f, "Conversion error for '{control}': {reason}")
            }
        }
    }
}

impl std::error::Error for LayoutError {}

/// Result type for conversion operations.
pub type LayoutResult<T> = Result<T, LayoutError>;

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Load a parsed VB6 `FormRoot` into the layout engine.
///
/// Walks the control tree, converts positions/sizes from twips to pixels,
/// builds styles for each control, and stores the result in the form store.
/// Event procedures are populated from the provided bindings.
///
/// # Arguments
///
/// * `root` - The parsed form root (Form or MDIForm).
/// * `event_procedures` - Pre-computed event procedure bindings in `(control_name, event_name, procedure_name)` format.
/// * `config` - Layout configuration.
///
/// # Errors
///
/// Returns `Err` if a Custom or OLE control is encountered (not supported in Phase 1).
pub fn load_form(
    root: &vb6parse::language::FormRoot,
    event_procedures: Vec<(String, String, String)>,
    config: &LayoutConfig,
) -> FormHandle {
    let form = convert_form(root, event_procedures, config).expect("form conversion failed");
    form_store::insert(form)
}

/// Convert a parsed VB6 `FormRoot` into a [`LayoutForm`].
///
/// This is the entry point for the converter. It dispatches to either
/// [`convert_form_impl`] or [`convert_mdi_form_impl`] depending on the root type.
/// Event procedures are populated from the provided bindings.
///
/// # Arguments
///
/// * `root` - The parsed form root (Form or MDIForm).
/// * `event_procedures` - Pre-computed event procedure bindings in `(control_name, event_name, procedure_name)` format.
/// * `config` - Layout configuration.
///
/// # Errors
///
/// Returns `Err` if a Custom or OLE control is encountered.
pub fn convert_form(
    root: &vb6parse::language::FormRoot,
    event_procedures: Vec<(String, String, String)>,
    config: &LayoutConfig,
) -> LayoutResult<LayoutForm> {
    match root {
        vb6parse::language::FormRoot::Form(form) => {
            convert_form_impl(form, event_procedures, config)
        }
        vb6parse::language::FormRoot::MDIForm(mdi) => {
            convert_mdi_form_impl(mdi, Vec::new(), config)
        }
    }
}

// ---------------------------------------------------------------------------
// Form / MDIForm conversion
// ---------------------------------------------------------------------------

/// Convert a parsed VB6 [`Form`] into a [`LayoutForm`].
fn convert_form_impl(
    form: &Form,
    event_procedures: Vec<(String, String, String)>,
    config: &LayoutConfig,
) -> LayoutResult<LayoutForm> {
    let dpi = config.dpi;
    let scale_mode = form.properties.scale_mode;

    // Convert form dimensions to pixels — use client area dimensions
    // (ClientWidth/ClientHeight). In pixel scale mode these are pixel values;
    // in all other modes they're twips.
    let (width, height) = if form.properties.scale_mode == ScaleMode::Pixel {
        (
            form.properties.client_width as f32,
            form.properties.client_height as f32,
        )
    } else {
        (
            twips_to_pixels(form.properties.client_width, dpi),
            twips_to_pixels(form.properties.client_height, dpi),
        )
    };

    // Build a placeholder parent container for position lookups
    let client_area = LayoutContainer {
        name: form.name.clone(),
        control_type: LayoutControlType::Form,
        size: LayoutSize { width, height },
        ..LayoutContainer::default()
    };

    // Convert child controls
    let mut children = Vec::new();
    for ctrl in &form.controls {
        if let Some(node) = convert_control(ctrl, &client_area, scale_mode, dpi, config)? {
            children.push(node);
        }
    }

    // Build form-level style
    let style = build_form_style(&form.properties, dpi);

    // Build the root container (represents the form's client area)
    let root_container = LayoutContainer {
        name: form.name.clone(),
        control_type: LayoutControlType::Form,
        size: LayoutSize { width, height },
        children,
        caption: Some(form.properties.caption.clone()),
        visible: form.properties.visible == Visibility::Visible,
        enabled: form.properties.enabled == Activation::Enabled,
        style: style.clone(),
        ..LayoutContainer::default()
    };

    // Form-level position (screen coordinates, in twips → pixels)
    let position = LayoutPosition {
        left: twips_to_pixels(form.properties.left, dpi),
        top: twips_to_pixels(form.properties.top, dpi),
    };

    // Build a name -> control type -> index -> NodeId map from the layout tree
    let mut name_to_nodes: HashMap<String, Vec<NodeId>> = HashMap::new();
    collect_node_ids_from_tree(&root_container, &mut name_to_nodes);

    // Populate node_ids in event procedures
    let event_procedures: Vec<EventProcedure> = event_procedures
        .into_iter()
        .filter_map(|(control, event, procedure)| {
            // Find the first matching NodeId for this control name
            let node_id = name_to_nodes
                .get(&control)
                .and_then(|nodes| nodes.first())
                .cloned()?;
            Some(EventProcedure {
                node_id,
                control,
                event,
                procedure,
            })
        })
        .collect();

    Ok(LayoutForm {
        name: form.name.clone(),
        control_type: LayoutControlType::Form,
        index: form.index,
        position,
        size: LayoutSize { width, height },
        style,
        root_node: LayoutNode::Container(root_container),
        caption: form.properties.caption.clone(),
        visible: form.properties.visible == Visibility::Visible,
        enabled: form.properties.enabled == Activation::Enabled,
        current_value: None,
        snapshot: None,
        render_id: 0,
        event_procedures,
    })
}

/// Convert a parsed VB6 [`MDIForm`] into a [`LayoutForm`].
///
/// MDIForms do not have `scale_mode`, `scale_width`, `scale_height`, or
/// `border_style` fields — dimensions are always in twips.
fn convert_mdi_form_impl(
    mdi: &MDIForm,
    _event_procedures: Vec<EventProcedure>,
    config: &LayoutConfig,
) -> LayoutResult<LayoutForm> {
    let dpi = config.dpi;

    // MDIForm dimensions are in twips (no scale_mode field)
    let width = twips_to_pixels(mdi.properties.width, dpi);
    let height = twips_to_pixels(mdi.properties.height, dpi);

    let client_area = LayoutContainer {
        name: mdi.name.clone(),
        control_type: LayoutControlType::Form,
        size: LayoutSize { width, height },
        ..LayoutContainer::default()
    };

    let mut children = Vec::new();
    for ctrl in &mdi.controls {
        if let Some(node) = convert_control(ctrl, &client_area, ScaleMode::Twip, dpi, config)? {
            children.push(node);
        }
    }

    // Build form-level style
    let style = build_mdi_form_style(&mdi.properties, dpi);

    let root_container = LayoutContainer {
        name: mdi.name.clone(),
        control_type: LayoutControlType::Form,
        size: LayoutSize { width, height },
        children,
        caption: Some(mdi.properties.caption.clone()),
        visible: mdi.properties.visible == Visibility::Visible,
        enabled: mdi.properties.enabled == Activation::Enabled,
        style: style.clone(),
        ..LayoutContainer::default()
    };

    let position = LayoutPosition {
        left: twips_to_pixels(mdi.properties.left, dpi),
        top: twips_to_pixels(mdi.properties.top, dpi),
    };

    Ok(LayoutForm {
        name: mdi.name.clone(),
        control_type: LayoutControlType::Form,
        index: mdi.index,
        position,
        size: LayoutSize { width, height },
        style,
        root_node: LayoutNode::Container(root_container),
        caption: mdi.properties.caption.clone(),
        visible: mdi.properties.visible == Visibility::Visible,
        enabled: mdi.properties.enabled == Activation::Enabled,
        current_value: None,
        snapshot: None,
        render_id: 0,
        event_procedures: vec![],
    })
}

// ---------------------------------------------------------------------------
// Event procedure helpers
// ---------------------------------------------------------------------------

/// Common VB6 event names used when matching procedure names to controls.
const VB6_EVENTS: &[&str] = &[
    "Click",
    "DblClick",
    "MouseDown",
    "MouseMove",
    "MouseUp",
    "Change",
    "GotFocus",
    "LostFocus",
    "KeyPress",
    "KeyDown",
    "KeyUp",
    "BeforeUpdate",
    "AfterUpdate",
    "OnDirty",
    "Dirty",
    "Enter",
    "Exit",
    "Error",
    "AddNew",
    "WriteComplete",
    "WantsAccelerator",
    "Validate",
    "Paint",
    "Resize",
    "Initialize",
    "Terminate",
    "SelectionChange",
];

/// Recursively collect node IDs from a layout container, grouped by name.
fn collect_node_ids_from_tree(container: &LayoutContainer, map: &mut HashMap<String, Vec<NodeId>>) {
    let nid = container.node_id();
    map.entry(nid.name.clone()).or_default().push(nid);
    for child in &container.children {
        match child {
            LayoutNode::Container(c) => collect_node_ids_from_tree(c, map),
            LayoutNode::Leaf(l) => {
                let nid = l.node_id();
                map.entry(nid.name.clone()).or_default().push(nid);
            }
        }
    }
}

/// Check if a procedure name matches a control event binding.
/// Returns `(control_name, event_name)` if the procedure follows the `controlName_eventName` pattern.
fn find_event_binding(
    proc_name: &str,
    control_names: &[String],
    form_name: &str,
) -> Option<(String, String)> {
    for event in VB6_EVENTS {
        let pattern = format!("_{}", event);
        if proc_name.ends_with(&pattern) {
            let ctrl_name = &proc_name[..proc_name.len() - pattern.len()];
            if ctrl_name.is_empty() {
                continue;
            }
            if control_names.iter().any(|cn| cn.as_str() == ctrl_name) {
                return Some((ctrl_name.to_string(), event.to_string()));
            }
        }
    }

    // Handle form-level events (no control prefix)
    if proc_name.starts_with(&format!("{}_", form_name)) {
        let event_part = &proc_name[form_name.len() + 1..];
        if event_part.contains(|c: char| c.is_ascii_uppercase()) {
            return Some((form_name.to_string(), event_part.to_string()));
        }
    }

    None
}

/// Extract the procedure name from a Sub or Function statement CST node.
fn procedure_name(node: &vb6parse::parsers::cst::CstNode) -> String {
    node.significant_children()
        .find(|c| c.kind() == SyntaxKind::Identifier)
        .map(|c| c.text().trim().to_string())
        .unwrap_or_default()
}

/// Build a list of event bindings from a `FormRoot` and its CST.
///
/// This extracts control names from the form structure and matches them against
/// procedure declarations in the CST to find event handlers in the format
/// `controlName_eventName` (e.g., `cmdOK_Click`).
///
/// Returns a vector of `(control_name, event_name, procedure_name)` tuples.
pub fn build_event_bindings(
    root: &vb6parse::language::FormRoot,
    cst: &ConcreteSyntaxTree,
) -> Vec<(String, String, String)> {
    let mut bindings: HashMap<(String, String), String> = HashMap::new();

    // Get the form name for form-level event detection
    let form_name = match root {
        vb6parse::language::FormRoot::Form(f) => f.name.clone(),
        vb6parse::language::FormRoot::MDIForm(m) => m.name.clone(),
    };

    // Collect all control names recursively
    let all_control_names: Vec<String> = match root {
        vb6parse::language::FormRoot::Form(form) => {
            let mut names = Vec::new();
            for ctrl in &form.controls {
                collect_layout_control_names(ctrl, &mut names);
            }
            names
        }
        vb6parse::language::FormRoot::MDIForm(mdi) => {
            let mut names = Vec::new();
            for ctrl in &mdi.controls {
                collect_layout_control_names(ctrl, &mut names);
            }
            names
        }
    };

    // Get procedure names from the CST
    let root_node = cst.to_root_node();
    for child in root_node.children() {
        match child.kind() {
            SyntaxKind::SubStatement | SyntaxKind::FunctionStatement => {
                let proc_name = procedure_name(child);
                if proc_name.is_empty() {
                    continue;
                }

                if let Some((ctrl, event)) =
                    find_event_binding(&proc_name, &all_control_names, &form_name)
                {
                    bindings.insert((ctrl, event), proc_name);
                }
            }
            _ => {}
        }
    }

    bindings
        .into_iter()
        .map(|((control, event), procedure)| (control, event, procedure))
        .collect()
}

/// Recursively collect control names from a parsed control tree.
fn collect_layout_control_names(control: &Control, names: &mut Vec<String>) {
    names.push(control.name().to_string());
    match control.kind() {
        ControlKind::Frame { controls, .. } | ControlKind::PictureBox { controls, .. } => {
            for child in controls {
                collect_layout_control_names(child, names);
            }
        }
        _ => {}
    }
}

// ---------------------------------------------------------------------------
// Style builders for forms
// ---------------------------------------------------------------------------

fn build_form_style(props: &vb6parse::language::FormProperties, dpi: u32) -> LayoutStyle {
    let mut style = LayoutStyle {
        background_color: Some(color_to_css(&props.back_color)),
        color: Some(color_to_css(&props.fore_color)),
        ..Default::default()
    };

    if let Some(ref font) = props.font {
        style.font_family = Some(font.name.clone());
        style.font_size = Some(font_points_to_px(font.size, dpi));
        style.font_weight = font_weight_css(font.weight);
        style.font_style = Some(font_style_css(font.italic));
        style.text_decoration = Some(text_decoration_css(font.underline));
    }

    style.border = form_border_style_css(props.border_style);
    style.cursor = mouse_pointer_css(props.mouse_pointer);
    style.direction = if matches!(props.right_to_left, TextDirection::RightToLeft) {
        Some("rtl".to_string())
    } else {
        None
    };
    style
}

fn build_mdi_form_style(props: &vb6parse::language::MDIFormProperties, dpi: u32) -> LayoutStyle {
    let mut style = LayoutStyle {
        background_color: Some(color_to_css(&props.back_color)),
        ..Default::default()
    };

    if let Some(ref font) = props.font {
        style.font_family = Some(font.name.clone());
        style.font_size = Some(font_points_to_px(font.size, dpi));
        style.font_weight = font_weight_css(font.weight);
        style.font_style = Some(font_style_css(font.italic));
        style.text_decoration = Some(text_decoration_css(font.underline));
    }

    style.cursor = mouse_pointer_css(props.mouse_pointer);
    style.direction = if matches!(props.right_to_left, TextDirection::RightToLeft) {
        Some("rtl".to_string())
    } else {
        None
    };
    style
}

// ---------------------------------------------------------------------------
// Control conversion
// ---------------------------------------------------------------------------

/// Convert a single VB6 [`Control`] into a [`LayoutNode`].
///
/// Returns `Some(node)` for visual controls and `None` for non-visual controls
/// that should be skipped (Timer, Data) when `config.include_nonvisual` is false.
///
/// # Errors
///
/// Returns `Err` if the control is Custom or OLE (not supported in Phase 1).
fn convert_control(
    control: &Control,
    parent: &LayoutContainer,
    scale_mode: ScaleMode,
    dpi: u32,
    config: &LayoutConfig,
) -> LayoutResult<Option<LayoutNode>> {
    // Reject Custom/Ole immediately
    match control.kind() {
        ControlKind::Custom { .. } | ControlKind::Ole { .. } => {
            return Err(LayoutError::UnsupportedControl {
                name: control.name().to_string(),
                kind: control_kind_name(control.kind()),
                message: "Custom and OLE controls are not supported in Phase 1. \
                          vb6library will provide this capability in a future phase."
                    .to_string(),
            });
        }
        _ => {}
    }

    // Skip non-visual controls when not requested
    match control.kind() {
        ControlKind::Timer { .. } | ControlKind::Data { .. } if !config.include_nonvisual => {
            return Ok(None);
        }
        _ => {}
    }

    // Build style for this control
    let mut style = super::build_style_for_control(control.kind(), config);

    // Option buttons are grouped by their container (form or frame) so that
    // selecting one deselects the others in the same group.
    if matches!(control.kind(), ControlKind::OptionButton { .. }) {
        style.group = Some(parent.name.clone());
    }

    // Determine visibility and enabled state
    let visible = control_visible(control.kind());
    let enabled = control_enabled(control.kind());

    // Convert position/size and determine layout control type
    let (position, size, layout_type) = extract_position_size_type(control.kind(), scale_mode, dpi);

    // Build the node — containers recurse, leaves are terminal
    match control.kind() {
        // Container: Frame
        ControlKind::Frame {
            properties,
            controls,
        } => {
            let frame_container = LayoutContainer {
                name: control.name().to_string(),
                control_type: layout_type,
                index: control.index(),
                position,
                size,
                style,
                caption: Some(properties.caption.clone()),
                visible,
                enabled,
                ..Default::default()
            };

            let mut child_nodes = Vec::new();
            for child in controls {
                if let Some(node) =
                    convert_control(child, &frame_container, scale_mode, dpi, config)?
                {
                    child_nodes.push(node);
                }
            }

            Ok(Some(LayoutNode::Container(LayoutContainer {
                children: child_nodes,
                ..frame_container
            })))
        }

        // Container: PictureBox
        ControlKind::PictureBox {
            properties,
            controls,
        } => {
            let picture_container = LayoutContainer {
                name: control.name().to_string(),
                control_type: layout_type,
                index: control.index(),
                position,
                size,
                style,
                visible,
                enabled,
                ..Default::default()
            };

            let mut child_nodes = Vec::new();
            for child in controls {
                // PictureBox may have its own scale_mode; use it if available,
                // otherwise fall back to the parent form's scale_mode.
                let child_scale_mode = properties.scale_mode;
                if let Some(node) =
                    convert_control(child, &picture_container, child_scale_mode, dpi, config)?
                {
                    child_nodes.push(node);
                }
            }

            Ok(Some(LayoutNode::Container(LayoutContainer {
                children: child_nodes,
                ..picture_container
            })))
        }

        ControlKind::HScrollBar { .. } | ControlKind::VScrollBar { .. } => {
            let sb_leaf = extract_scrollbar_leaf(
                control.kind(),
                control.name().to_string(),
                control.index(),
                position,
                size,
                style,
            );
            Ok(Some(LayoutNode::Leaf(sb_leaf)))
        }

        ControlKind::Image { .. } => {
            let image_src = extract_image_leaf(control.kind());
            Ok(Some(LayoutNode::Leaf(LayoutLeaf {
                name: control.name().to_string(),
                control_type: layout_type,
                index: control.index(),
                position,
                size,
                style,
                value: extract_value(control.kind()),
                visible,
                enabled,
                tooltip: extract_tooltip(control.kind()),
                tabindex: extract_tabindex(control.kind()),
                is_default: extract_is_default(control.kind()),
                is_cancel: extract_is_cancel(control.kind()),
                combo_style: extract_combo_style(control.kind()),
                image_src,
                ..Default::default()
            })))
        }

        ControlKind::TextBox { .. } => {
            let (locked, max_length, password_char, hide_selection, scroll_bars) =
                extract_textbox_leaf(control.kind());
            Ok(Some(LayoutNode::Leaf(LayoutLeaf {
                name: control.name().to_string(),
                control_type: layout_type,
                index: control.index(),
                position,
                size,
                style,
                value: extract_value(control.kind()),
                visible,
                enabled,
                tooltip: extract_tooltip(control.kind()),
                tabindex: extract_tabindex(control.kind()),
                is_default: extract_is_default(control.kind()),
                is_cancel: extract_is_cancel(control.kind()),
                combo_style: extract_combo_style(control.kind()),
                use_mnemonic: extract_use_mnemonic(control.kind()),
                is_locked: locked,
                max_length,
                password_char,
                hide_selection,
                scroll_bars,
                ..Default::default()
            })))
        }

        // Leaf controls
        _ => Ok(Some(LayoutNode::Leaf(LayoutLeaf {
            name: control.name().to_string(),
            control_type: layout_type,
            index: control.index(),
            position,
            size,
            style,
            value: extract_value(control.kind()),
            visible,
            enabled,
            tooltip: extract_tooltip(control.kind()),
            tabindex: extract_tabindex(control.kind()),
            is_default: extract_is_default(control.kind()),
            is_cancel: extract_is_cancel(control.kind()),
            combo_style: extract_combo_style(control.kind()),
            use_mnemonic: extract_use_mnemonic(control.kind()),
            ..Default::default()
        }))),
    }
}

/// Extract ScrollBar-specific range values into a [`LayoutLeaf`].
fn extract_scrollbar_leaf(
    kind: &ControlKind,
    name: String,
    index: i32,
    position: LayoutPosition,
    size: LayoutSize,
    style: LayoutStyle,
) -> LayoutLeaf {
    let properties = match kind {
        ControlKind::HScrollBar { properties, .. } | ControlKind::VScrollBar { properties, .. } => {
            properties
        }
        _ => unreachable!("extract_scrollbar_leaf called with non-scrollbar control"),
    };

    let tabindex = match properties.tab_stop {
        TabStop::Included => Some(0),
        TabStop::ProgrammaticOnly => Some(-1),
    };

    LayoutLeaf {
        name,
        control_type: layout_type_from_kind(kind),
        index,
        position,
        size,
        style,
        value: Some(properties.value.to_string()),
        visible: properties.visible == Visibility::Visible,
        enabled: properties.enabled == Activation::Enabled,
        tooltip: None,
        tabindex,
        range_min: Some(properties.min),
        range_max: Some(properties.max),
        range_step: Some(properties.small_change),
        ..Default::default()
    }
}

// ---------------------------------------------------------------------------
// Position / size extraction
// ---------------------------------------------------------------------------

/// Extract position, size, and layout control type from a [`ControlKind`].
///
/// Each control kind has a different properties struct with different field
/// names for position and size. This function maps them all to the common
/// [`LayoutPosition`]/[`LayoutSize`] types.
fn extract_position_size_type(
    kind: &ControlKind,
    scale_mode: ScaleMode,
    dpi: u32,
) -> (LayoutPosition, LayoutSize, LayoutControlType) {
    let lt = layout_type_from_kind(kind);

    let (left, top, width, height) = match kind {
        ControlKind::CommandButton { properties, .. } => (
            properties.left,
            properties.top,
            properties.width,
            properties.height,
        ),
        ControlKind::TextBox { properties, .. } => (
            properties.left,
            properties.top,
            properties.width,
            properties.height,
        ),
        ControlKind::Label { properties, .. } => (
            properties.left,
            properties.top,
            properties.width,
            properties.height,
        ),
        ControlKind::Frame { properties, .. } => (
            properties.left,
            properties.top,
            properties.width,
            properties.height,
        ),
        ControlKind::PictureBox { properties, .. } => (
            properties.left,
            properties.top,
            properties.width,
            properties.height,
        ),
        ControlKind::Image { properties, .. } => (
            properties.left,
            properties.top,
            properties.width,
            properties.height,
        ),
        ControlKind::CheckBox { properties, .. } => (
            properties.left,
            properties.top,
            properties.width,
            properties.height,
        ),
        ControlKind::OptionButton { properties, .. } => (
            properties.left,
            properties.top,
            properties.width,
            properties.height,
        ),
        ControlKind::ComboBox { properties, .. } => (
            properties.left,
            properties.top,
            properties.width,
            properties.height,
        ),
        ControlKind::ListBox { properties, .. } => (
            properties.left,
            properties.top,
            properties.width,
            properties.height,
        ),
        ControlKind::HScrollBar { properties, .. } | ControlKind::VScrollBar { properties, .. } => {
            (
                properties.left,
                properties.top,
                properties.width,
                properties.height,
            )
        }
        ControlKind::Shape { properties, .. } => (
            properties.left,
            properties.top,
            properties.width,
            properties.height,
        ),
        ControlKind::Line { properties, .. } => {
            // Line: position is (x1,y1), size is (x2-x1, y2-y1)
            let x1 = properties.x1;
            let y1 = properties.y1;
            return (
                LayoutPosition {
                    left: scale_mode_to_pixels(x1, scale_mode, dpi),
                    top: scale_mode_to_pixels(y1, scale_mode, dpi),
                },
                LayoutSize {
                    width: scale_mode_to_pixels(properties.x2 - x1, scale_mode, dpi),
                    height: scale_mode_to_pixels(properties.y2 - y1, scale_mode, dpi),
                },
                lt,
            );
        }
        ControlKind::Timer { properties, .. } => (properties.left, properties.top, 0, 0),
        ControlKind::Data { properties, .. } => (
            properties.left,
            properties.top,
            properties.width,
            properties.height,
        ),
        ControlKind::DriveListBox { properties, .. } => (
            properties.left,
            properties.top,
            properties.width,
            properties.height,
        ),
        ControlKind::DirListBox { properties, .. } => (
            properties.left,
            properties.top,
            properties.width,
            properties.height,
        ),
        ControlKind::FileListBox { properties, .. } => (
            properties.left,
            properties.top,
            properties.width,
            properties.height,
        ),
        ControlKind::Custom { .. } | ControlKind::Ole { .. } | ControlKind::Menu { .. } => {
            unreachable!("Custom/Ole/Menu controls should be rejected before this point")
        }
    };

    (
        LayoutPosition {
            left: scale_mode_to_pixels(left, scale_mode, dpi),
            top: scale_mode_to_pixels(top, scale_mode, dpi),
        },
        LayoutSize {
            width: scale_mode_to_pixels(width, scale_mode, dpi),
            height: scale_mode_to_pixels(height, scale_mode, dpi),
        },
        lt,
    )
}

/// Map a vb6parse [`ControlKind`] to a layout [`LayoutControlType`].
fn layout_type_from_kind(kind: &ControlKind) -> LayoutControlType {
    match kind {
        ControlKind::CommandButton { .. } => LayoutControlType::CommandButton,
        ControlKind::TextBox { .. } => LayoutControlType::TextBox,
        ControlKind::Label { .. } => LayoutControlType::Label,
        ControlKind::Frame { .. } => LayoutControlType::Frame,
        ControlKind::PictureBox { .. } => LayoutControlType::PictureBox,
        ControlKind::Image { .. } => LayoutControlType::Image,
        ControlKind::CheckBox { .. } => LayoutControlType::CheckBox,
        ControlKind::OptionButton { .. } => LayoutControlType::OptionButton,
        ControlKind::ComboBox { .. } => LayoutControlType::ComboBox,
        ControlKind::ListBox { .. } => LayoutControlType::ListBox,
        ControlKind::HScrollBar { .. } => LayoutControlType::HScrollBar,
        ControlKind::VScrollBar { .. } => LayoutControlType::VScrollBar,
        ControlKind::Timer { .. } => LayoutControlType::Timer,
        ControlKind::Shape { .. } => LayoutControlType::Shape,
        ControlKind::Line { .. } => LayoutControlType::Line,
        ControlKind::Data { .. } => LayoutControlType::Data,
        ControlKind::DriveListBox { .. } => LayoutControlType::DriveListBox,
        ControlKind::DirListBox { .. } => LayoutControlType::DirListBox,
        ControlKind::FileListBox { .. } => LayoutControlType::FileListBox,
        ControlKind::Custom { .. } => LayoutControlType::Custom,
        ControlKind::Ole { .. } => LayoutControlType::Custom,
        ControlKind::Menu { .. } => LayoutControlType::Custom,
    }
}

/// Return a human-readable name for a control kind (for error messages).
fn control_kind_name(kind: &ControlKind) -> String {
    match kind {
        ControlKind::Custom { .. } => "Custom".to_string(),
        ControlKind::Ole { .. } => "OLE".to_string(),
        _ => "Unknown".to_string(),
    }
}

// ---------------------------------------------------------------------------
// Visibility / enabled extraction
// ---------------------------------------------------------------------------

/// Extract visibility from a [`ControlKind`].
fn control_visible(kind: &ControlKind) -> bool {
    match kind {
        ControlKind::CommandButton { .. } => {
            // CommandButton doesn't have a visible property
            true
        }
        ControlKind::TextBox { properties, .. } => properties.visible == Visibility::Visible,
        ControlKind::Label { properties, .. } => properties.visible == Visibility::Visible,
        ControlKind::Frame { properties, .. } => properties.visible == Visibility::Visible,
        ControlKind::PictureBox { properties, .. } => properties.visible == Visibility::Visible,
        ControlKind::Image { properties, .. } => properties.visible == Visibility::Visible,
        ControlKind::CheckBox { properties, .. } => properties.visible == Visibility::Visible,
        ControlKind::OptionButton { properties, .. } => properties.visible == Visibility::Visible,
        ControlKind::ComboBox { properties, .. } => properties.visible == Visibility::Visible,
        ControlKind::ListBox { properties, .. } => properties.visible == Visibility::Visible,
        ControlKind::HScrollBar { properties, .. } | ControlKind::VScrollBar { properties, .. } => {
            properties.visible == Visibility::Visible
        }
        ControlKind::Shape { properties, .. } => properties.visible == Visibility::Visible,
        ControlKind::Line { properties, .. } => properties.visible == Visibility::Visible,
        ControlKind::Timer { .. } => {
            // Timer doesn't have a visible property
            true
        }
        ControlKind::Data { properties, .. } => properties.visible == Visibility::Visible,
        ControlKind::DriveListBox { properties, .. } => properties.visible == Visibility::Visible,
        ControlKind::DirListBox { properties, .. } => properties.visible == Visibility::Visible,
        ControlKind::FileListBox { properties, .. } => properties.visible == Visibility::Visible,
        ControlKind::Custom { .. } | ControlKind::Ole { .. } | ControlKind::Menu { .. } => true,
    }
}

/// Extract enabled state from a [`ControlKind`].
fn control_enabled(kind: &ControlKind) -> bool {
    match kind {
        ControlKind::CommandButton { properties, .. } => properties.enabled == Activation::Enabled,
        ControlKind::TextBox { properties, .. } => properties.enabled == Activation::Enabled,
        ControlKind::Label { properties, .. } => properties.enabled == Activation::Enabled,
        ControlKind::Frame { properties, .. } => properties.enabled == Activation::Enabled,
        ControlKind::PictureBox { properties, .. } => properties.enabled == Activation::Enabled,
        ControlKind::Image { properties, .. } => properties.enabled == Activation::Enabled,
        ControlKind::CheckBox { properties, .. } => properties.enabled == Activation::Enabled,
        ControlKind::OptionButton { properties, .. } => properties.enabled == Activation::Enabled,
        ControlKind::ComboBox { properties, .. } => properties.enabled == Activation::Enabled,
        ControlKind::ListBox { properties, .. } => properties.enabled == Activation::Enabled,
        ControlKind::HScrollBar { properties, .. } | ControlKind::VScrollBar { properties, .. } => {
            properties.enabled == Activation::Enabled
        }
        ControlKind::Shape { .. } => {
            // Shape doesn't have an enabled property
            true
        }
        ControlKind::Line { .. } => {
            // Line doesn't have an enabled property
            true
        }
        ControlKind::Timer { properties, .. } => properties.enabled == Activation::Enabled,
        ControlKind::Data { properties, .. } => properties.enabled == Activation::Enabled,
        ControlKind::DriveListBox { properties, .. } => properties.enabled == Activation::Enabled,
        ControlKind::DirListBox { properties, .. } => properties.enabled == Activation::Enabled,
        ControlKind::FileListBox { properties, .. } => properties.enabled == Activation::Enabled,
        ControlKind::Custom { .. } | ControlKind::Ole { .. } | ControlKind::Menu { .. } => true,
    }
}

// ---------------------------------------------------------------------------
// Value extraction (for leaf controls)
// ---------------------------------------------------------------------------

/// Extract the runtime-writable value from a [`ControlKind`].
///
/// This represents the control's current "value" — text content, caption,
/// checked state, scroll position, etc.
fn extract_value(kind: &ControlKind) -> Option<String> {
    match kind {
        ControlKind::Label { properties, .. } => Some(properties.caption.clone()),
        ControlKind::TextBox { properties, .. } => Some(properties.text.clone()),
        ControlKind::CommandButton { properties, .. } => Some(properties.caption.clone()),
        ControlKind::CheckBox { properties, .. } => Some(
            match properties.value {
                vb6parse::language::CheckBoxValue::Unchecked => "False",
                vb6parse::language::CheckBoxValue::Checked => "True",
                vb6parse::language::CheckBoxValue::Grayed => "Grayed",
            }
            .to_string(),
        ),
        ControlKind::OptionButton { properties, .. } => Some(
            if properties.value == vb6parse::language::OptionButtonValue::Selected {
                "True".to_string()
            } else {
                "False".to_string()
            },
        ),
        ControlKind::Frame { properties, .. } => Some(properties.caption.clone()),
        ControlKind::PictureBox { properties, .. } => {
            properties.picture.as_ref().map(|p| format!("{:?}", p))
        }
        ControlKind::Image { properties, .. } => {
            // Image source is handled separately via extract_image_src.
            // The `value` field is intentionally empty for Image controls.
            properties.picture.as_ref().map(|_| String::new())
        }
        ControlKind::HScrollBar { properties, .. } | ControlKind::VScrollBar { properties, .. } => {
            Some(properties.value.to_string())
        }
        ControlKind::ComboBox { properties, .. } => Some(properties.text.clone()),
        ControlKind::ListBox { properties, .. } => {
            // Return first list item or empty string
            // ReferenceOrValue is not re-exported, use Display as fallback
            let list_str = properties.list.to_string();
            if list_str == "Value" {
                // It's a Value - try to get first item via Display
                None // Value display shows "Value", we'd need direct access
            } else {
                // It's a Reference
                Some(format!("[Reference: {list_str}]"))
            }
        }
        ControlKind::Timer { properties, .. } => Some(properties.interval.to_string()),
        ControlKind::Data { properties, .. } => Some(properties.connection.to_string()),
        ControlKind::DriveListBox { .. } => {
            // No path field in vb6parse - return empty
            Some(String::new())
        }
        ControlKind::DirListBox { .. } => {
            // No path field in vb6parse - return empty
            Some(String::new())
        }
        ControlKind::FileListBox { .. } => {
            // No path field in vb6parse - return empty
            Some(String::new())
        }
        ControlKind::Shape { .. } => None,
        ControlKind::Line { .. } => None,
        ControlKind::Custom { .. } | ControlKind::Ole { .. } | ControlKind::Menu { .. } => None,
    }
}

/// Extract tooltip text from a [`ControlKind`] for the HTML `title` attribute.
///
/// Returns `None` when the control has no tooltip or the tooltip is empty.
fn extract_tooltip(kind: &ControlKind) -> Option<String> {
    match kind {
        ControlKind::Label { properties, .. } => tooltip_text(&properties.tool_tip_text),
        ControlKind::TextBox { properties, .. } => tooltip_text(&properties.tool_tip_text),
        ControlKind::CommandButton { properties, .. } => tooltip_text(&properties.tool_tip_text),
        ControlKind::CheckBox { properties, .. } => tooltip_text(&properties.tool_tip_text),
        ControlKind::OptionButton { properties, .. } => tooltip_text(&properties.tool_tip_text),
        ControlKind::ComboBox { properties, .. } => tooltip_text(&properties.tool_tip_text),
        ControlKind::ListBox { properties, .. } => tooltip_text(&properties.tool_tip_text),
        ControlKind::Frame { properties, .. } => tooltip_text(&properties.tool_tip_text),
        ControlKind::PictureBox { properties, .. } => tooltip_text(&properties.tool_tip_text),
        ControlKind::Image { properties, .. } => tooltip_text(&properties.tool_tip_text),
        ControlKind::DriveListBox { properties, .. } => tooltip_text(&properties.tool_tip_text),
        ControlKind::DirListBox { properties, .. } => tooltip_text(&properties.tool_tip_text),
        ControlKind::FileListBox { properties, .. } => tooltip_text(&properties.tool_tip_text),
        ControlKind::Data { properties, .. } => tooltip_text(&properties.tool_tip_text),
        ControlKind::HScrollBar { .. }
        | ControlKind::VScrollBar { .. }
        | ControlKind::Shape { .. }
        | ControlKind::Line { .. }
        | ControlKind::Timer { .. }
        | ControlKind::Custom { .. }
        | ControlKind::Ole { .. }
        | ControlKind::Menu { .. } => None,
    }
}

/// Convert a string to an optional tooltip: empty strings become `None`.
fn tooltip_text(s: &str) -> Option<String> {
    if s.is_empty() {
        None
    } else {
        Some(s.to_string())
    }
}

/// Extract whether a [`ControlKind`] is the default button.
///
/// Returns `true` for `CommandButton { default: true }`, `false` otherwise.
fn extract_is_default(kind: &ControlKind) -> bool {
    matches!(kind, ControlKind::CommandButton { properties } if properties.default)
}

/// Extract whether a [`ControlKind`] is the cancel button.
///
/// Returns `true` for `CommandButton { cancel: true }`, `false` otherwise.
fn extract_is_cancel(kind: &ControlKind) -> bool {
    matches!(kind, ControlKind::CommandButton { properties } if properties.cancel)
}

/// Extract tabindex from a [`ControlKind`] for the HTML `tabindex` attribute.
///
/// Returns `Some(0)` for `TabStop::Included`, `Some(-1)` for
/// `TabStop::ProgrammaticOnly`, and `None` for controls that don't have
/// a `tab_stop` property.
fn extract_tabindex(kind: &ControlKind) -> Option<i32> {
    match kind {
        ControlKind::CommandButton { properties, .. } => match properties.tab_stop {
            TabStop::Included => Some(0),
            TabStop::ProgrammaticOnly => Some(-1),
        },
        ControlKind::TextBox { properties, .. } => match properties.tab_stop {
            TabStop::Included => Some(0),
            TabStop::ProgrammaticOnly => Some(-1),
        },
        ControlKind::CheckBox { properties, .. } => match properties.tab_stop {
            TabStop::Included => Some(0),
            TabStop::ProgrammaticOnly => Some(-1),
        },
        ControlKind::OptionButton { properties, .. } => match properties.tab_stop {
            TabStop::Included => Some(0),
            TabStop::ProgrammaticOnly => Some(-1),
        },
        ControlKind::ComboBox { properties, .. } => match properties.tab_stop {
            TabStop::Included => Some(0),
            TabStop::ProgrammaticOnly => Some(-1),
        },
        ControlKind::ListBox { properties, .. } => match properties.tab_stop {
            TabStop::Included => Some(0),
            TabStop::ProgrammaticOnly => Some(-1),
        },
        ControlKind::PictureBox { properties, .. } => match properties.tab_stop {
            TabStop::Included => Some(0),
            TabStop::ProgrammaticOnly => Some(-1),
        },
        ControlKind::HScrollBar { properties, .. } | ControlKind::VScrollBar { properties, .. } => {
            match properties.tab_stop {
                TabStop::Included => Some(0),
                TabStop::ProgrammaticOnly => Some(-1),
            }
        }
        ControlKind::DriveListBox { properties, .. } => match properties.tab_stop {
            TabStop::Included => Some(0),
            TabStop::ProgrammaticOnly => Some(-1),
        },
        ControlKind::DirListBox { properties, .. } => match properties.tab_stop {
            TabStop::Included => Some(0),
            TabStop::ProgrammaticOnly => Some(-1),
        },
        ControlKind::FileListBox { properties, .. } => match properties.tab_stop {
            TabStop::Included => Some(0),
            TabStop::ProgrammaticOnly => Some(-1),
        },
        ControlKind::Label { .. }
        | ControlKind::Frame { .. }
        | ControlKind::Image { .. }
        | ControlKind::Shape { .. }
        | ControlKind::Line { .. }
        | ControlKind::Timer { .. }
        | ControlKind::Data { .. }
        | ControlKind::Custom { .. }
        | ControlKind::Ole { .. }
        | ControlKind::Menu { .. } => None,
    }
}

/// Extract ComboBox style from a [`ControlKind`].
///
/// Returns a string describing how the ComboBox should be rendered:
/// - `"dropdown"` — editable dropdown (default, `DropDownCombo`)
/// - `"dropdown-readonly"` — non-editable dropdown (`DropDownList`)
/// - `"simple"` — always-visible list with editable text above (`SimpleCombo`)
fn extract_combo_style(kind: &ControlKind) -> Option<String> {
    match kind {
        ControlKind::ComboBox { properties, .. } => Some(match properties.style {
            vb6parse::language::ComboBoxStyle::DropDownCombo => "dropdown".to_string(),
            vb6parse::language::ComboBoxStyle::DropDownList => "dropdown-readonly".to_string(),
            vb6parse::language::ComboBoxStyle::SimpleCombo => "simple".to_string(),
        }),
        _ => None,
    }
}

/// Extract the `use_mnemonic` property from a [`ControlKind`].
///
/// Returns `false` for controls that don't support mnemonics.
/// For Label controls, returns the `use_mnemonic` property value.
fn extract_use_mnemonic(kind: &ControlKind) -> bool {
    match kind {
        ControlKind::Label { properties, .. } => properties.use_mnemonic,
        _ => false,
    }
}

/// Convert a `DynamicImage` to a base64-encoded data URL.
///
/// Encodes the image as PNG and returns a data URL in the format
/// `data:image/png;base64,...`.
fn image_to_data_url(image: &DynamicImage) -> String {
    let mut buffer = Vec::new();
    if image
        .write_to(
            &mut std::io::Cursor::new(&mut buffer),
            image::ImageFormat::Png,
        )
        .is_ok()
    {
        let encoded = STANDARD.encode(&buffer);
        format!("data:image/png;base64,{encoded}")
    } else {
        String::new()
    }
}

/// Extract the image data URL from an `ImageProperties` picture field.
///
/// Converts `ReferenceOrValue<DynamicImage>` to a base64 data URL.
/// For `Reference` variants (external .frx resource), returns `None` since
/// the image data is not embedded in the layout model.
pub(crate) fn extract_image_src(
    picture: &Option<ReferenceOrValue<DynamicImage>>,
) -> Option<String> {
    match picture {
        Some(ReferenceOrValue::Value(img)) => {
            let url = image_to_data_url(img);
            if url.is_empty() { None } else { Some(url) }
        }
        Some(ReferenceOrValue::Reference { .. }) => {
            // External .frx reference - data URL cannot be generated here.
            // vb6interpret can resolve this at runtime.
            None
        }
        None => None,
    }
}

/// Extract Image-specific leaf fields from a [`ControlKind`].
///
/// Returns the base64 image data URL for the Image control.
/// For non-Image controls, returns `None`.
fn extract_image_leaf(kind: &ControlKind) -> Option<String> {
    match kind {
        ControlKind::Image { properties, .. } => extract_image_src(&properties.picture),
        _ => None,
    }
}

/// Extract TextBox-specific leaf fields from a [`ControlKind`].
///
/// Returns `(locked, max_length, password_char, hide_selection, scroll_bars)` for the TextBox control.
/// For non-TextBox controls, returns `(false, None, None, false, None)`.
fn extract_textbox_leaf(
    kind: &ControlKind,
) -> (bool, Option<i32>, Option<char>, bool, Option<String>) {
    match kind {
        ControlKind::TextBox { properties, .. } => {
            let max_length = if properties.max_length == 0 {
                None
            } else {
                Some(properties.max_length)
            };
            let scroll_bars = match (properties.multi_line, properties.scroll_bars) {
                (MultiLine::SingleLine, ScrollBars::None) => None,
                (MultiLine::SingleLine, ScrollBars::Horizontal) => Some("auto".to_string()),
                (MultiLine::SingleLine, ScrollBars::Vertical)
                | (MultiLine::SingleLine, ScrollBars::Both) => Some("hidden".to_string()),
                (MultiLine::MultiLine, ScrollBars::None) => Some("hidden".to_string()),
                (MultiLine::MultiLine, ScrollBars::Horizontal) => Some("auto".to_string()),
                (MultiLine::MultiLine, ScrollBars::Vertical) => Some("auto".to_string()),
                (MultiLine::MultiLine, ScrollBars::Both) => Some("auto".to_string()),
            };
            (
                properties.locked,
                max_length,
                properties.password_char,
                properties.hide_selection,
                scroll_bars,
            )
        }
        _ => (false, None, None, false, None),
    }
}

// ---------------------------------------------------------------------------
// CSS helper functions
// ---------------------------------------------------------------------------

fn font_weight_css(weight: i32) -> Option<String> {
    match weight {
        100 => Some("100".to_string()),
        200 => Some("200".to_string()),
        300 => Some("300".to_string()),
        400 => Some("normal".to_string()),
        500 => Some("500".to_string()),
        600 => Some("600".to_string()),
        700 => Some("bold".to_string()),
        800 => Some("800".to_string()),
        900 => Some("900".to_string()),
        _ => None,
    }
}

fn font_style_css(italic: bool) -> String {
    if italic {
        "italic".to_string()
    } else {
        "normal".to_string()
    }
}

fn text_decoration_css(underline: bool) -> String {
    if underline {
        "underline".to_string()
    } else {
        "none".to_string()
    }
}

fn form_border_style_css(style: FormBorderStyle) -> Option<String> {
    match style {
        FormBorderStyle::None => Some("none".to_string()),
        FormBorderStyle::FixedSingle => Some("1px solid rgb(120, 120, 120)".to_string()),
        FormBorderStyle::Sizable => Some("1px solid rgb(120, 120, 120)".to_string()),
        FormBorderStyle::FixedDialog => Some("2px solid rgb(120, 120, 120)".to_string()),
        FormBorderStyle::FixedToolWindow => Some("1px solid rgb(160, 160, 160)".to_string()),
        FormBorderStyle::SizableToolWindow => Some("1px solid rgb(160, 160, 160)".to_string()),
    }
}

#[allow(dead_code)]
fn border_style_css(style: BorderStyle) -> Option<String> {
    match style {
        BorderStyle::None => Some("none".to_string()),
        BorderStyle::FixedSingle => Some("1px solid rgb(120, 120, 120)".to_string()),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::form_store;
    use std::collections::HashMap;
    use vb6parse::language::{
        CheckBoxProperties, CheckBoxValue, CommandButtonProperties, CustomControlProperties,
        DataProperties, FrameProperties, LabelProperties, LineProperties, OLEProperties,
        ScrollBarProperties, TextBoxProperties, TimerProperties,
    };

    /// Lock to serialize tests that share the global form store.
    fn lock_test() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        LOCK.lock().unwrap()
    }

    fn test_form(name: &str) -> Form {
        Form {
            name: name.to_string(),
            tag: String::new(),
            index: 0,
            properties: vb6parse::language::FormProperties {
                scale_width: 4000,
                scale_height: 3000,
                left: 0,
                top: 0,
                visible: Visibility::Visible,
                enabled: Activation::Enabled,
                ..Default::default()
            },
            controls: Vec::new(),
            menus: Vec::new(),
        }
    }

    fn test_label(name: &str) -> Control {
        Control::new(
            name.to_string(),
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
        )
    }

    fn test_button(name: &str) -> Control {
        Control::new(
            name.to_string(),
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
        )
    }

    fn test_textbox(name: &str) -> Control {
        Control::new(
            name.to_string(),
            String::new(),
            0,
            ControlKind::TextBox {
                properties: TextBoxProperties {
                    text: "".to_string(),
                    left: 120,
                    top: 120,
                    width: 2000,
                    height: 300,
                    visible: Visibility::Visible,
                    enabled: Activation::Enabled,
                    ..Default::default()
                },
            },
        )
    }

    fn test_frame(name: &str) -> Control {
        Control::new(
            name.to_string(),
            String::new(),
            0,
            ControlKind::Frame {
                properties: FrameProperties {
                    caption: "Group".to_string(),
                    left: 120,
                    top: 120,
                    width: 2000,
                    height: 1500,
                    visible: Visibility::Visible,
                    enabled: Activation::Enabled,
                    ..Default::default()
                },
                controls: Vec::new(),
            },
        )
    }

    fn test_timer(name: &str) -> Control {
        Control::new(
            name.to_string(),
            String::new(),
            0,
            ControlKind::Timer {
                properties: TimerProperties {
                    interval: 1000,
                    left: 0,
                    top: 0,
                    enabled: Activation::Enabled,
                },
            },
        )
    }

    fn test_data(name: &str) -> Control {
        Control::new(
            name.to_string(),
            String::new(),
            0,
            ControlKind::Data {
                properties: DataProperties {
                    left: 0,
                    top: 0,
                    width: 0,
                    height: 0,
                    visible: Visibility::Visible,
                    enabled: Activation::Enabled,
                    ..Default::default()
                },
            },
        )
    }

    fn test_custom(name: &str) -> Control {
        Control::new(
            name.to_string(),
            String::new(),
            0,
            ControlKind::Custom {
                properties: CustomControlProperties {
                    property_store: HashMap::new(),
                },
                property_groups: Vec::new(),
            },
        )
    }

    fn test_ole(name: &str) -> Control {
        Control::new(
            name.to_string(),
            String::new(),
            0,
            ControlKind::Ole {
                properties: OLEProperties {
                    left: 120,
                    top: 120,
                    width: 500,
                    height: 500,
                    ..Default::default()
                },
            },
        )
    }

    fn create_test_form() -> Form {
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
            controls: vec![test_label("Label1")],
            menus: Vec::new(),
        }
    }

    fn create_test_form_with_button() -> Form {
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
            controls: vec![test_button("cmdOK")],
            menus: Vec::new(),
        }
    }

    fn create_test_form_with_frame() -> Form {
        let _label_in_frame = Control::new(
            "Label1".to_string(),
            String::new(),
            0,
            ControlKind::Label {
                properties: LabelProperties {
                    caption: "Inside Frame".to_string(),
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
            controls: vec![test_frame("Frame1")],
            menus: Vec::new(),
        }
    }

    fn create_test_form_with_timer() -> Form {
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
            controls: vec![test_timer("Timer1")],
            menus: Vec::new(),
        }
    }

    fn create_test_form_with_data() -> Form {
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
            controls: vec![test_data("Data1")],
            menus: Vec::new(),
        }
    }

    fn create_test_form_with_custom() -> Form {
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
            controls: vec![test_custom("MSFlexGrid1")],
            menus: Vec::new(),
        }
    }

    fn create_test_form_with_ole() -> Form {
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
            controls: vec![test_ole("Ole1")],
            menus: Vec::new(),
        }
    }

    fn create_test_form_with_sized_control() -> Form {
        Form {
            name: "Form1".to_string(),
            tag: String::new(),
            index: 0,
            properties: vb6parse::language::FormProperties {
                client_width: 4000,
                client_height: 3000,
                scale_width: 4000,
                scale_height: 3000,
                caption: "Form1".to_string(),
                left: 0,
                top: 0,
                visible: Visibility::Visible,
                enabled: Activation::Enabled,
                ..Default::default()
            },
            controls: vec![test_label("Label1")],
            menus: Vec::new(),
        }
    }

    fn create_test_form_with_textbox() -> Form {
        Form {
            name: "Form1".to_string(),
            tag: String::new(),
            index: 0,
            properties: vb6parse::language::FormProperties {
                client_width: 4000,
                client_height: 3000,
                scale_width: 4000,
                scale_height: 3000,
                caption: "Form1".to_string(),
                left: 0,
                top: 0,
                visible: Visibility::Visible,
                enabled: Activation::Enabled,
                ..Default::default()
            },
            controls: vec![test_textbox("Text1")],
            menus: Vec::new(),
        }
    }

    // --- Test cases ---

    #[test]
    fn convert_simple_form() {
        let form = create_test_form();
        let config = LayoutConfig::default();
        let layout_form =
            convert_form(&vb6parse::language::FormRoot::Form(form), vec![], &config).unwrap();

        assert_eq!(layout_form.name, "Form1");
        let children: Vec<_> = layout_form.visible_children().collect();
        assert_eq!(children.len(), 1);
        assert!(matches!(children[0], LayoutNode::Leaf(_)));
    }

    #[test]
    fn convert_form_with_button() {
        let form = create_test_form_with_button();
        let config = LayoutConfig::default();
        let layout_form =
            convert_form(&vb6parse::language::FormRoot::Form(form), vec![], &config).unwrap();

        assert_eq!(layout_form.name, "Form1");
        let children: Vec<_> = layout_form.visible_children().collect();
        assert_eq!(children.len(), 1);
        if let LayoutNode::Leaf(leaf) = &children[0] {
            assert_eq!(leaf.name, "cmdOK");
            assert_eq!(leaf.control_type, LayoutControlType::CommandButton);
        } else {
            panic!("Expected Leaf");
        }
    }

    #[test]
    fn convert_form_with_textbox() {
        let form = create_test_form_with_textbox();
        let config = LayoutConfig::default();
        let layout_form =
            convert_form(&vb6parse::language::FormRoot::Form(form), vec![], &config).unwrap();

        assert_eq!(layout_form.name, "Form1");
        let children: Vec<_> = layout_form.visible_children().collect();
        assert_eq!(children.len(), 1);
        if let LayoutNode::Leaf(leaf) = &children[0] {
            assert_eq!(leaf.name, "Text1");
            assert_eq!(leaf.control_type, LayoutControlType::TextBox);
        } else {
            panic!("Expected Leaf");
        }
    }

    #[test]
    fn convert_nested_containers() {
        // Form → Frame → Label
        let form = create_test_form_with_frame();
        let config = LayoutConfig::default();
        let layout_form =
            convert_form(&vb6parse::language::FormRoot::Form(form), vec![], &config).unwrap();

        let children: Vec<_> = layout_form.visible_children().collect();
        assert_eq!(children.len(), 1);
        if let LayoutNode::Container(frame) = &children[0] {
            assert_eq!(frame.name, "Frame1");
            assert_eq!(frame.control_type, LayoutControlType::Frame);
            assert_eq!(frame.children.len(), 0); // Frame has no children in test data
        } else {
            panic!("Expected Container (Frame)");
        }
    }

    #[test]
    fn skip_timer_controls() {
        let form = create_test_form_with_timer();
        let config = LayoutConfig {
            include_nonvisual: false,
            ..Default::default()
        };
        let layout_form =
            convert_form(&vb6parse::language::FormRoot::Form(form), vec![], &config).unwrap();

        let children: Vec<_> = layout_form.visible_children().collect();
        assert!(
            children.is_empty(),
            "Timer should be skipped when include_nonvisual=false"
        );
    }

    #[test]
    fn include_timer_controls() {
        let form = create_test_form_with_timer();
        let config = LayoutConfig {
            include_nonvisual: true,
            ..Default::default()
        };
        let layout_form =
            convert_form(&vb6parse::language::FormRoot::Form(form), vec![], &config).unwrap();

        let children: Vec<_> = layout_form.visible_children().collect();
        assert_eq!(children.len(), 1);
        if let LayoutNode::Leaf(leaf) = &children[0] {
            assert_eq!(leaf.control_type, LayoutControlType::Timer);
        } else {
            panic!("Expected Leaf (Timer)");
        }
    }

    #[test]
    fn skip_data_controls() {
        let form = create_test_form_with_data();
        let config = LayoutConfig {
            include_nonvisual: false,
            ..Default::default()
        };
        let layout_form =
            convert_form(&vb6parse::language::FormRoot::Form(form), vec![], &config).unwrap();

        let children: Vec<_> = layout_form.visible_children().collect();
        assert!(
            children.is_empty(),
            "Data should be skipped when include_nonvisual=false"
        );
    }

    #[test]
    fn reject_custom_control() {
        let form = create_test_form_with_custom();
        let config = LayoutConfig::default();
        let result = convert_form(&vb6parse::language::FormRoot::Form(form), vec![], &config);
        assert!(
            matches!(result, Err(LayoutError::UnsupportedControl { kind, .. }) if kind == "Custom")
        );
    }

    #[test]
    fn reject_ole_control() {
        let form = create_test_form_with_ole();
        let config = LayoutConfig::default();
        let result = convert_form(&vb6parse::language::FormRoot::Form(form), vec![], &config);
        assert!(
            matches!(result, Err(LayoutError::UnsupportedControl { kind, .. }) if kind == "OLE")
        );
    }

    #[test]
    fn twip_conversion_in_converter() {
        let form = create_test_form_with_sized_control();
        let config = LayoutConfig {
            dpi: 96,
            ..Default::default()
        };
        let layout_form =
            convert_form(&vb6parse::language::FormRoot::Form(form), vec![], &config).unwrap();

        // client_width = 4000 twips at 96 DPI
        // 4000 * 96 / 1440 = 266.666... px
        assert!((layout_form.size.width - 266.67).abs() < 0.1);
        // client_height = 3000 twips at 96 DPI
        // 3000 * 96 / 1440 = 200.0 px
        assert!((layout_form.size.height - 200.0).abs() < 0.1);
    }

    #[test]
    fn mdi_form_conversion() {
        let mdi = create_test_mdi_form();
        let config = LayoutConfig::default();
        let layout_form =
            convert_form(&vb6parse::language::FormRoot::MDIForm(mdi), vec![], &config).unwrap();

        assert_eq!(layout_form.name, "MDIMain");
        // Width = 4800 twips at 96 DPI = 4800 * 96 / 1440 = 320 px
        assert!((layout_form.size.width - 320.0).abs() < 0.1);
        // Height = 3600 twips at 96 DPI = 3600 * 96 / 1440 = 240 px
        assert!((layout_form.size.height - 240.0).abs() < 0.1);
    }

    fn create_test_mdi_form() -> MDIForm {
        MDIForm {
            name: "MDIMain".to_string(),
            tag: String::new(),
            index: 0,
            properties: vb6parse::language::MDIFormProperties {
                width: 4800,
                height: 3600,
                caption: "MDI Main".to_string(),
                left: 0,
                top: 0,
                visible: Visibility::Visible,
                enabled: Activation::Enabled,
                ..Default::default()
            },
            controls: Vec::new(),
            menus: Vec::new(),
        }
    }

    #[test]
    fn convert_form_with_mixed_controls() {
        // Form with Label, TextBox, Button, Timer, and Frame (nested Label)
        let label = test_label("Label1");
        let textbox = test_textbox("Text1");
        let button = test_button("cmdOK");
        let timer = test_timer("Timer1");
        let frame = test_frame("Frame1");

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
            controls: vec![label, textbox, button, timer, frame],
            menus: Vec::new(),
        };

        // Without non-visual: 5 controls - 1 timer = 4
        let config_skip = LayoutConfig {
            include_nonvisual: false,
            ..Default::default()
        };
        let layout = convert_form(
            &vb6parse::language::FormRoot::Form(form.clone()),
            vec![],
            &config_skip,
        )
        .unwrap();
        let children: Vec<_> = layout.visible_children().collect();
        assert_eq!(children.len(), 4);

        // With non-visual: all 5
        let config_include = LayoutConfig {
            include_nonvisual: true,
            ..Default::default()
        };
        let layout = convert_form(
            &vb6parse::language::FormRoot::Form(form),
            vec![],
            &config_include,
        )
        .unwrap();
        let children: Vec<_> = layout.visible_children().collect();
        assert_eq!(children.len(), 5);
    }

    #[test]
    fn convert_control_position_conversion() {
        let form = Form {
            name: "Form1".to_string(),
            tag: String::new(),
            index: 0,
            properties: vb6parse::language::FormProperties {
                scale_width: 4000,
                scale_height: 3000,
                caption: "Form1".to_string(),
                left: 100,
                top: 200,
                visible: Visibility::Visible,
                enabled: Activation::Enabled,
                ..Default::default()
            },
            controls: vec![test_label("Label1")],
            menus: Vec::new(),
        };

        let config = LayoutConfig {
            dpi: 96,
            ..Default::default()
        };
        let layout =
            convert_form(&vb6parse::language::FormRoot::Form(form), vec![], &config).unwrap();

        // Form position: left=100 twips, top=200 twips at 96 DPI
        // 100 * 96 / 1440 = 6.67 px, 200 * 96 / 1440 = 13.33 px
        assert!((layout.position.left - 6.67).abs() < 0.1);
        assert!((layout.position.top - 13.33).abs() < 0.1);
    }

    #[test]
    fn convert_control_with_different_dpi() {
        let label = Control::new(
            "Label1".to_string(),
            String::new(),
            0,
            ControlKind::Label {
                properties: LabelProperties {
                    caption: "Test".to_string(),
                    left: 1440, // 1 inch in twips
                    top: 1440,
                    width: 1440,
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

        // At 120 DPI: 1440 twips = 1440 * 120 / 1440 = 120 px
        let config = LayoutConfig {
            dpi: 120,
            ..Default::default()
        };
        let layout =
            convert_form(&vb6parse::language::FormRoot::Form(form), vec![], &config).unwrap();

        let children: Vec<_> = layout.visible_children().collect();
        assert_eq!(children.len(), 1);
        if let LayoutNode::Leaf(leaf) = &children[0] {
            assert!((leaf.position.left - 120.0).abs() < 0.1);
        } else {
            panic!("Expected Leaf");
        }
    }

    #[test]
    fn convert_form_with_scale_mode_pixel() {
        let form = Form {
            name: "Form1".to_string(),
            tag: String::new(),
            index: 0,
            properties: vb6parse::language::FormProperties {
                scale_mode: ScaleMode::Pixel,
                client_width: 400,
                client_height: 300,
                scale_width: 400,
                scale_height: 300,
                caption: "Form1".to_string(),
                left: 0,
                top: 0,
                visible: Visibility::Visible,
                enabled: Activation::Enabled,
                ..Default::default()
            },
            controls: vec![test_label("Label1")],
            menus: Vec::new(),
        };

        let config = LayoutConfig {
            dpi: 96,
            ..Default::default()
        };
        let layout =
            convert_form(&vb6parse::language::FormRoot::Form(form), vec![], &config).unwrap();

        // Pixel mode: client dimensions are used as pixel values directly
        assert_eq!(layout.size.width, 400.0);
        assert_eq!(layout.size.height, 300.0);
    }

    #[test]
    fn test_layout_type_from_kind() {
        assert_eq!(
            layout_type_from_kind(&ControlKind::CommandButton {
                properties: Default::default(),
            }),
            LayoutControlType::CommandButton
        );
        assert_eq!(
            layout_type_from_kind(&ControlKind::Label {
                properties: Default::default(),
            }),
            LayoutControlType::Label
        );
        assert_eq!(
            layout_type_from_kind(&ControlKind::TextBox {
                properties: Default::default(),
            }),
            LayoutControlType::TextBox
        );
        assert_eq!(
            layout_type_from_kind(&ControlKind::Frame {
                properties: Default::default(),
                controls: vec![],
            }),
            LayoutControlType::Frame
        );
        assert_eq!(
            layout_type_from_kind(&ControlKind::PictureBox {
                properties: Default::default(),
                controls: vec![],
            }),
            LayoutControlType::PictureBox
        );
        assert_eq!(
            layout_type_from_kind(&ControlKind::Timer {
                properties: Default::default(),
            }),
            LayoutControlType::Timer
        );
        assert_eq!(
            layout_type_from_kind(&ControlKind::Data {
                properties: Default::default(),
            }),
            LayoutControlType::Data
        );
        assert_eq!(
            layout_type_from_kind(&ControlKind::Custom {
                properties: Default::default(),
                property_groups: vec![],
            }),
            LayoutControlType::Custom
        );
        assert_eq!(
            layout_type_from_kind(&ControlKind::Ole {
                properties: Default::default(),
            }),
            LayoutControlType::Custom
        );
    }

    #[test]
    fn extract_value_for_labels() {
        let kind = ControlKind::Label {
            properties: LabelProperties {
                caption: "Hello World".to_string(),
                ..Default::default()
            },
        };
        assert_eq!(extract_value(&kind), Some("Hello World".to_string()));
    }

    #[test]
    fn extract_value_for_textbox() {
        let kind = ControlKind::TextBox {
            properties: TextBoxProperties {
                text: "input text".to_string(),
                ..Default::default()
            },
        };
        assert_eq!(extract_value(&kind), Some("input text".to_string()));
    }

    #[test]
    fn extract_value_for_checkbox() {
        let checked = ControlKind::CheckBox {
            properties: CheckBoxProperties {
                value: CheckBoxValue::Checked,
                ..Default::default()
            },
        };
        assert_eq!(extract_value(&checked), Some("True".to_string()));

        let unchecked = ControlKind::CheckBox {
            properties: CheckBoxProperties {
                value: CheckBoxValue::Unchecked,
                ..Default::default()
            },
        };
        assert_eq!(extract_value(&unchecked), Some("False".to_string()));

        let grayed = ControlKind::CheckBox {
            properties: CheckBoxProperties {
                value: CheckBoxValue::Grayed,
                ..Default::default()
            },
        };
        assert_eq!(extract_value(&grayed), Some("Grayed".to_string()));
    }

    #[test]
    fn extract_value_for_scrollbar() {
        let kind = ControlKind::HScrollBar {
            properties: ScrollBarProperties {
                value: 42,
                ..Default::default()
            },
        };
        assert_eq!(extract_value(&kind), Some("42".to_string()));
    }

    #[test]
    fn extract_value_for_timer() {
        let kind = ControlKind::Timer {
            properties: TimerProperties {
                interval: 5000,
                ..Default::default()
            },
        };
        assert_eq!(extract_value(&kind), Some("5000".to_string()));
    }

    #[test]
    fn extract_value_for_shape_is_none() {
        let kind = ControlKind::Shape {
            properties: Default::default(),
        };
        assert_eq!(extract_value(&kind), None);
    }

    #[test]
    fn extract_value_for_line_is_none() {
        let kind = ControlKind::Line {
            properties: Default::default(),
        };
        assert_eq!(extract_value(&kind), None);
    }

    #[test]
    fn form_background_from_back_color() {
        // The form renders its own BackColor inline so custom form colors
        // (e.g. white forms) are honored instead of always using CSS var().
        let form = create_test_form();
        let config = LayoutConfig::default();
        let layout =
            convert_form(&vb6parse::language::FormRoot::Form(form), vec![], &config).unwrap();

        assert!(layout.style.background_color.is_some());
        assert!(layout.style.color.is_some());
    }

    #[test]
    fn leaf_value_is_set() {
        let form = create_test_form();
        let config = LayoutConfig::default();
        let layout =
            convert_form(&vb6parse::language::FormRoot::Form(form), vec![], &config).unwrap();

        let children: Vec<_> = layout.visible_children().collect();
        if let LayoutNode::Leaf(leaf) = &children[0] {
            assert_eq!(leaf.value, Some("Hello".to_string()));
        } else {
            panic!("Expected Leaf");
        }
    }

    #[test]
    fn control_position_conversion_15_twips_per_pixel() {
        // 15 twips = 1 pixel at 96 DPI
        let label = Control::new(
            "Label1".to_string(),
            String::new(),
            0,
            ControlKind::Label {
                properties: LabelProperties {
                    caption: "Test".to_string(),
                    left: 150,   // 150 twips = 10 pixels at 96 DPI
                    top: 300,    // 300 twips = 20 pixels at 96 DPI
                    width: 450,  // 450 twips = 30 pixels at 96 DPI
                    height: 150, // 150 twips = 10 pixels at 96 DPI
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

        let config = LayoutConfig {
            dpi: 96,
            ..Default::default()
        };
        let layout =
            convert_form(&vb6parse::language::FormRoot::Form(form), vec![], &config).unwrap();

        let children: Vec<_> = layout.visible_children().collect();
        assert_eq!(children.len(), 1);
        if let LayoutNode::Leaf(leaf) = &children[0] {
            assert_eq!(leaf.position.left, 10.0);
            assert_eq!(leaf.position.top, 20.0);
            assert_eq!(leaf.size.width, 30.0);
            assert_eq!(leaf.size.height, 10.0);
        } else {
            panic!("Expected Leaf");
        }
    }

    #[test]
    fn load_form_returns_handle() {
        let _lock = lock_test();
        let form = create_test_form();
        let config = LayoutConfig::default();
        let root = vb6parse::language::FormRoot::Form(form);
        let handle = load_form(&root, vec![], &config);

        // Handle should be valid and form should be retrievable
        let name = form_store::get(handle, |f| f.name.clone());
        assert_eq!(name, Some("Form1".to_string()));
    }

    #[test]
    fn load_multiple_forms_incrementing_handles() {
        let _lock = lock_test();
        let f1 = create_test_form();
        let f2 = Form {
            name: "Form2".to_string(),
            ..test_form("Form2")
        };

        let config = LayoutConfig::default();
        let h1 = load_form(&vb6parse::language::FormRoot::Form(f1), vec![], &config);
        let h2 = load_form(&vb6parse::language::FormRoot::Form(f2), vec![], &config);

        // Handles should be incrementing, starting from 0 after reset
        assert_eq!(h2, h1 + 1);
    }

    #[test]
    fn layout_error_display() {
        let err = LayoutError::UnsupportedControl {
            name: "MSFlexGrid1".to_string(),
            kind: "Custom".to_string(),
            message: "not supported".to_string(),
        };
        let display = format!("{err}");
        assert!(display.contains("MSFlexGrid1"));
        assert!(display.contains("Custom"));
    }

    #[test]
    fn layout_error_clone() {
        let err1 = LayoutError::ConversionError {
            control: "Label1".to_string(),
            reason: "bad value".to_string(),
        };
        let err2 = err1.clone();
        assert_eq!(err1, err2);
    }

    #[test]
    fn control_visible_extraction() {
        let visible_label = ControlKind::Label {
            properties: LabelProperties {
                visible: Visibility::Visible,
                ..Default::default()
            },
        };
        let hidden_label = ControlKind::Label {
            properties: LabelProperties {
                visible: Visibility::Hidden,
                ..Default::default()
            },
        };

        assert!(control_visible(&visible_label));
        assert!(!control_visible(&hidden_label));
    }

    #[test]
    fn control_enabled_extraction() {
        let enabled_button = ControlKind::CommandButton {
            properties: CommandButtonProperties {
                enabled: Activation::Enabled,
                ..Default::default()
            },
        };
        let disabled_button = ControlKind::CommandButton {
            properties: CommandButtonProperties {
                enabled: Activation::Disabled,
                ..Default::default()
            },
        };

        assert!(control_enabled(&enabled_button));
        assert!(!control_enabled(&disabled_button));
    }

    #[test]
    fn line_control_position_size() {
        let kind = ControlKind::Line {
            properties: LineProperties {
                x1: 0,
                y1: 0,
                x2: 150, // 150 twips = 10 px at 96 DPI
                y2: 300, // 300 twips = 20 px at 96 DPI
                ..Default::default()
            },
        };
        let (_, size, _) = extract_position_size_type(&kind, ScaleMode::Twip, 96);
        assert_eq!(size.width, 10.0);
        assert_eq!(size.height, 20.0);
    }

    #[test]
    fn build_event_bindings_detects_control_click() {
        let input = b"VERSION 5.00\r\n\
                      Begin VB.CommandButton cmdOK \r\n\
                         Caption         =   \"OK\"\r\n\
                      End\r\n\
                      Attribute VB_Name = \"cmdOK\"\r\n\
                      Attribute VB_GlobalNameSpace = False\r\n\
                      Attribute VB_Creatable = False\r\n\
                      Attribute VB_PredeclaredId = True\r\n\
                      Attribute VB_Exposed = False\r\n\
                      Private Sub cmdOK_Click()\r\n\
                      End Sub\r\n";

        let source = vb6parse::io::SourceFile::decode_with_replacement("test.frm", input).unwrap();
        let form_file = vb6parse::FormFile::parse(&source).unwrap_or_fail();

        let bindings = build_event_bindings(&form_file.form, &form_file.cst);
        assert!(
            bindings.iter().any(|(ctrl, evt, proc)| {
                ctrl == "cmdOK" && evt == "Click" && proc == "cmdOK_Click"
            }),
            "Expected cmdOK_Click binding, got: {:?}",
            bindings
        );
    }

    #[test]
    fn build_event_bindings_excludes_form_level_events() {
        let input = b"VERSION 5.00\r\n\
                      Begin VB.Form Form1 \r\n\
                         Caption         =   \"Form1\"\r\n\
                      End\r\n\
                      Attribute VB_Name = \"Form1\"\r\n\
                      Attribute VB_GlobalNameSpace = False\r\n\
                      Attribute VB_Creatable = False\r\n\
                      Attribute VB_PredeclaredId = True\r\n\
                      Attribute VB_Exposed = False\r\n\
                      Private Sub Form_Load()\r\n\
                      End Sub\r\n";

        let source = vb6parse::io::SourceFile::decode_with_replacement("test.frm", input).unwrap();
        let form_file = vb6parse::FormFile::parse(&source).unwrap_or_fail();

        let bindings = build_event_bindings(&form_file.form, &form_file.cst);
        assert!(
            !bindings.iter().any(|(_, _, proc)| proc == "Form_Load"),
            "Form_Load should not be included in control event bindings"
        );
    }

    #[test]
    fn load_form_populates_event_procedures_for_control_click() {
        let _lock = lock_test();
        let input = b"VERSION 5.00\r\n\
Begin VB.Form Form1 \r\n\
   Caption         =   \"Form1\"\r\n\
   ClientHeight    =   3000\r\n\
   ClientLeft      =   60\r\n\
   ClientTop       =   345\r\n\
   ClientWidth     =   4500\r\n\
   LinkTopic       =   \"Form1\"\r\n\
   ScaleHeight     =   3000\r\n\
   ScaleWidth      =   4500\r\n\
   Begin VB.CommandButton Command1 \r\n\
      Caption         =   \"OK\"\r\n\
      Height          =   375\r\n\
      Left            =   1200\r\n\
      TabIndex        =   0\r\n\
      Top             =   600\r\n\
      Width           =   1095\r\n\
   End\r\n\
End\r\n\
Attribute VB_Name = \"Form1\"\r\n\
Attribute VB_GlobalNameSpace = False\r\n\
Attribute VB_Creatable = False\r\n\
Attribute VB_PredeclaredId = True\r\n\
Attribute VB_Exposed = False\r\n\
Private Sub Command1_Click()\r\n\
End Sub\r\n";

        let source = vb6parse::io::SourceFile::decode_with_replacement("test.frm", input).unwrap();
        let form_file = vb6parse::FormFile::parse(&source).unwrap_or_fail();

        let bindings = build_event_bindings(&form_file.form, &form_file.cst);
        let event_procedures: Vec<_> = bindings.into_iter().collect();

        let handle = load_form(&form_file.form, event_procedures, &LayoutConfig::default());

        let form = form_store::get(handle, |f| f.event_procedures.clone()).unwrap();
        assert!(
            form.iter()
                .any(|ep| ep.control == "Command1" && ep.event == "Click"),
            "Expected Command1_Click binding, got: {:?}",
            form
        );
    }

    #[test]
    fn load_form_includes_nested_control_events() {
        let _lock = lock_test();
        let input = b"VERSION 5.00\r\n\
Begin VB.Form Form1 \r\n\
   Caption         =   \"Form1\"\r\n\
   ClientHeight    =   3000\r\n\
   ClientLeft      =   60\r\n\
   ClientTop       =   345\r\n\
   ClientWidth     =   4500\r\n\
   LinkTopic       =   \"Form1\"\r\n\
   ScaleHeight     =   3000\r\n\
   ScaleWidth      =   4500\r\n\
   Begin VB.Frame Frame1 \r\n\
      Caption         =   \"Group\"\r\n\
      Height          =   1000\r\n\
      Left            =   300\r\n\
      TabIndex        =   1\r\n\
      Top             =   300\r\n\
      Width           =   3900\r\n\
      Begin VB.CommandButton cmdOK \r\n\
         Caption         =   \"OK\"\r\n\
         Height          =   375\r\n\
         Left            =   600\r\n\
         TabIndex        =   0\r\n\
         Top             =   200\r\n\
         Width           =   1095\r\n\
      End\r\n\
   End\r\n\
End\r\n\
Attribute VB_Name = \"Form1\"\r\n\
Attribute VB_GlobalNameSpace = False\r\n\
Attribute VB_Creatable = False\r\n\
Attribute VB_PredeclaredId = True\r\n\
Attribute VB_Exposed = False\r\n\
Private Sub cmdOK_Click()\r\n\
End Sub\r\n";

        let source = vb6parse::io::SourceFile::decode_with_replacement("test.frm", input).unwrap();
        let form_file = vb6parse::FormFile::parse(&source).unwrap_or_fail();

        let bindings = build_event_bindings(&form_file.form, &form_file.cst);
        let event_procedures: Vec<_> = bindings.into_iter().collect();

        let handle = load_form(&form_file.form, event_procedures, &LayoutConfig::default());

        let form = form_store::get(handle, |f| f.event_procedures.clone()).unwrap();
        assert!(
            form.iter().any(|ep| ep.control == "cmdOK"),
            "Expected cmdOK event procedure for nested control, got: {:?}",
            form
        );
    }

    #[test]
    fn extract_tabindex_included_returns_zero() {
        let kind = ControlKind::CommandButton {
            properties: CommandButtonProperties {
                tab_stop: vb6parse::language::TabStop::Included,
                ..Default::default()
            },
        };
        assert_eq!(extract_tabindex(&kind), Some(0));
    }

    #[test]
    fn extract_tabindex_programmatic_only_returns_neg_one() {
        let kind = ControlKind::CommandButton {
            properties: CommandButtonProperties {
                tab_stop: vb6parse::language::TabStop::ProgrammaticOnly,
                ..Default::default()
            },
        };
        assert_eq!(extract_tabindex(&kind), Some(-1));
    }

    #[test]
    fn extract_tabindex_label_returns_none() {
        let kind = ControlKind::Label {
            properties: LabelProperties::default(),
        };
        assert_eq!(extract_tabindex(&kind), None);
    }

    #[test]
    fn extract_tabindex_textbox_included() {
        let kind = ControlKind::TextBox {
            properties: TextBoxProperties {
                tab_stop: vb6parse::language::TabStop::Included,
                ..Default::default()
            },
        };
        assert_eq!(extract_tabindex(&kind), Some(0));
    }

    #[test]
    fn extract_tabindex_checkbox_programmatic_only() {
        let kind = ControlKind::CheckBox {
            properties: CheckBoxProperties {
                tab_stop: vb6parse::language::TabStop::ProgrammaticOnly,
                ..Default::default()
            },
        };
        assert_eq!(extract_tabindex(&kind), Some(-1));
    }

    #[test]
    fn extract_tabindex_scrollbar_included() {
        let kind = ControlKind::HScrollBar {
            properties: ScrollBarProperties {
                tab_stop: vb6parse::language::TabStop::Included,
                ..Default::default()
            },
        };
        assert_eq!(extract_tabindex(&kind), Some(0));
    }

    #[test]
    fn extract_tabindex_combo_box_programmatic_only() {
        let kind = ControlKind::ComboBox {
            properties: vb6parse::language::ComboBoxProperties {
                tab_stop: vb6parse::language::TabStop::ProgrammaticOnly,
                ..Default::default()
            },
        };
        assert_eq!(extract_tabindex(&kind), Some(-1));
    }

    #[test]
    fn extract_tabindex_list_box_included() {
        let kind = ControlKind::ListBox {
            properties: vb6parse::language::ListBoxProperties {
                tab_stop: vb6parse::language::TabStop::Included,
                ..Default::default()
            },
        };
        assert_eq!(extract_tabindex(&kind), Some(0));
    }

    #[test]
    fn extract_is_default_true() {
        let kind = ControlKind::CommandButton {
            properties: CommandButtonProperties {
                default: true,
                ..Default::default()
            },
        };
        assert!(extract_is_default(&kind));
    }

    #[test]
    fn extract_is_default_false() {
        let kind = ControlKind::CommandButton {
            properties: CommandButtonProperties {
                default: false,
                ..Default::default()
            },
        };
        assert!(!extract_is_default(&kind));
    }

    #[test]
    fn extract_is_default_non_button_returns_false() {
        let kind = ControlKind::Label {
            properties: LabelProperties::default(),
        };
        assert!(!extract_is_default(&kind));
    }

    #[test]
    fn extract_is_cancel_true() {
        let kind = ControlKind::CommandButton {
            properties: CommandButtonProperties {
                cancel: true,
                ..Default::default()
            },
        };
        assert!(extract_is_cancel(&kind));
    }

    #[test]
    fn extract_is_cancel_false() {
        let kind = ControlKind::CommandButton {
            properties: CommandButtonProperties {
                cancel: false,
                ..Default::default()
            },
        };
        assert!(!extract_is_cancel(&kind));
    }

    #[test]
    fn extract_is_cancel_non_button_returns_false() {
        let kind = ControlKind::TextBox {
            properties: TextBoxProperties::default(),
        };
        assert!(!extract_is_cancel(&kind));
    }

    #[test]
    fn extract_textbox_locked() {
        let kind = ControlKind::TextBox {
            properties: TextBoxProperties {
                locked: true,
                ..Default::default()
            },
        };
        let (locked, _, _, _, _) = extract_textbox_leaf(&kind);
        assert!(locked);
    }

    #[test]
    fn extract_textbox_max_length() {
        let kind = ControlKind::TextBox {
            properties: TextBoxProperties {
                max_length: 10,
                ..Default::default()
            },
        };
        let (_, max_length, _, _, _) = extract_textbox_leaf(&kind);
        assert_eq!(max_length, Some(10));
    }

    #[test]
    fn extract_textbox_max_length_zero_is_none() {
        let kind = ControlKind::TextBox {
            properties: TextBoxProperties {
                max_length: 0,
                ..Default::default()
            },
        };
        let (_, max_length, _, _, _) = extract_textbox_leaf(&kind);
        assert_eq!(max_length, None);
    }

    #[test]
    fn extract_textbox_password_char() {
        let kind = ControlKind::TextBox {
            properties: TextBoxProperties {
                password_char: Some('*'),
                ..Default::default()
            },
        };
        let (_, _, password_char, _, _) = extract_textbox_leaf(&kind);
        assert_eq!(password_char, Some('*'));
    }

    #[test]
    fn extract_textbox_hide_selection() {
        let kind = ControlKind::TextBox {
            properties: TextBoxProperties {
                hide_selection: false,
                ..Default::default()
            },
        };
        let (_, _, _, hide_selection, _) = extract_textbox_leaf(&kind);
        assert!(!hide_selection);
    }

    #[test]
    fn extract_textbox_scroll_bars_singleline_horizontal() {
        let kind = ControlKind::TextBox {
            properties: TextBoxProperties {
                multi_line: MultiLine::SingleLine,
                scroll_bars: ScrollBars::Horizontal,
                ..Default::default()
            },
        };
        let (_, _, _, _, scroll_bars) = extract_textbox_leaf(&kind);
        assert_eq!(scroll_bars, Some("auto".to_string()));
    }

    #[test]
    fn extract_textbox_scroll_bars_multiline_both() {
        let kind = ControlKind::TextBox {
            properties: TextBoxProperties {
                multi_line: MultiLine::MultiLine,
                scroll_bars: ScrollBars::Both,
                ..Default::default()
            },
        };
        let (_, _, _, _, scroll_bars) = extract_textbox_leaf(&kind);
        assert_eq!(scroll_bars, Some("auto".to_string()));
    }

    #[test]
    fn extract_textbox_scroll_bars_singleline_none() {
        let kind = ControlKind::TextBox {
            properties: TextBoxProperties {
                multi_line: MultiLine::SingleLine,
                scroll_bars: ScrollBars::None,
                ..Default::default()
            },
        };
        let (_, _, _, _, scroll_bars) = extract_textbox_leaf(&kind);
        assert_eq!(scroll_bars, None);
    }

    #[test]
    fn extract_textbox_non_textbox_returns_defaults() {
        let kind = ControlKind::Label {
            properties: LabelProperties::default(),
        };
        let (locked, max_length, password_char, hide_selection, scroll_bars) =
            extract_textbox_leaf(&kind);
        assert!(!locked);
        assert_eq!(max_length, None);
        assert_eq!(password_char, None);
        assert!(!hide_selection);
        assert_eq!(scroll_bars, None);
    }
}
