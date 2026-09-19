//! Tauri renderer — produces HTML fragment strings for webview injection.
//!
//! Generates self-contained HTML with inline styles from the [`LayoutNode`] tree.
//! Used by Tauri to inject VB6-rendered forms into a webview via `webview.eval()`.
//!
//! # Control-to-HTML Mapping
//!
//! | VB6 Control | HTML Element |
//! |-------------|-------------|
//! | Form | *children only (no wrapper)* |
//! | Label | `<div class="vb6-label">` |
//! | TextBox | `<input type="text">` or `<textarea>` |
//! | CommandButton | `<button class="vb6-commandbutton">` |
//! | Frame | `<fieldset class="vb6-frame">` with `<legend>` |
//! | PictureBox | `<div class="vb6-picturebox">` |
//! | Image | `<img class="vb6-image">` |
//! | CheckBox | `<input type="checkbox" class="vb6-checkbox">` |
//! | OptionButton | `<input type="radio" class="vb6-optionbutton">` |
//! | ComboBox | `<select class="vb6-combobox">` |
//! | ListBox | `<select class="vb6-listbox">` |
//! | HScrollBar/VScrollBar | `<input type="range" class="vb6-<type>scrollbar">` |
//! | Shape | `<div class="vb6-shape">` |
//! | Line | `<svg class="vb6-line">` |
//! | DriveListBox | `<select class="vb6-drivelistbox">` |
//! | DirListBox | `<select class="vb6-dirlistbox">` |
//! | FileListBox | `<select class="vb6-filelistbox">` |
//! | Data/Custom | *(omitted — no visual output / not supported)* |
//! | Timer | *(omitted — no visual output)* |

use crate::layout::model::{LayoutContainer, LayoutLeaf, LayoutNode};
use crate::layout::renderer::Renderer;

use crate::layout::diff_tree::{DiffChange, DiffKind, DiffTree};
use crate::layout::model::LayoutControlType;

/// Tauri renderer that produces HTML fragment strings for webview injection.
///
/// Form elements are rendered with inline styles derived from [`LayoutStyle`].
/// Visibility and enabled state are appended as runtime overrides.
///
/// # Scope Root
///
/// By default, Tauri renderer does NOT wrap output in a `.vb6-app` scope root
/// since the webview owns the full DOM. Set `with_scope` to `true` if you want
/// CSS isolation (e.g. for embedding in a larger page).
#[derive(Debug, Clone, Default)]
pub struct TauriRenderer {
    /// Whether to wrap output in `.vb6-app` scope root.
    pub with_scope: bool,
}

impl TauriRenderer {
    /// Create a new `TauriRenderer` instance.
    ///
    /// # Arguments
    /// * `with_scope` — If `true`, wrap output in `.vb6-app` scope root.
    #[must_use]
    pub fn new(with_scope: bool) -> Self {
        Self { with_scope }
    }

    /// Process a VB6 caption string to handle ampersand mnemonics.
    ///
    /// VB6 uses `&` to mark mnemonic/accelerator keys:
    /// - `&OK` → `<span class="vb6-mnemonic">O</span>K`
    /// - `&&` → `&` (escaped ampersand)
    fn process_mnemonic(caption: &str) -> String {
        let mut result = String::with_capacity(caption.len() + 8);
        let mut chars = caption.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '&' {
                if let Some(&next) = chars.peek() {
                    if next == '&' {
                        result.push('&');
                        chars.next();
                    } else {
                        chars.next();
                        result.push_str("<span class=\"vb6-mnemonic\">");
                        result.push(next);
                        result.push_str("</span>");
                    }
                } else {
                    result.push('&');
                }
            } else {
                result.push(c);
            }
        }

        result
    }

    /// Generate the opening tag for the scope root, if enabled.
    #[must_use]
    pub fn root_open(&self) -> String {
        if self.with_scope {
            r#"<div class="vb6-app" id="vb6-container">"#.to_string()
        } else {
            String::new()
        }
    }

    /// Generate the closing tag for the scope root, if enabled.
    #[must_use]
    pub fn root_close(&self) -> String {
        if self.with_scope {
            "</div>".to_string()
        } else {
            String::new()
        }
    }

    /// Extract the form's natural width and height from the layout model.
    ///
    /// Returns `(0.0, 0.0)` if the handle is unknown or the root node is not a
    /// container.
    #[must_use]
    pub fn form_dimensions(handle: u32) -> (f64, f64) {
        super::super::form_store::get(handle, |form| {
            if let LayoutNode::Container(c) = &form.root_node {
                return (c.size.width as f64, c.size.height as f64);
            }
            (0.0, 0.0)
        })
        .unwrap_or((0.0, 0.0))
    }
}

impl Renderer for TauriRenderer {
    type Output = String;

    fn render_node(&self, node: &LayoutNode) -> String {
        match node {
            LayoutNode::Container(c) => self.render_container(c),
            LayoutNode::Leaf(l) => self.render_leaf(l),
        }
    }

    fn render_leaf(&self, leaf: &LayoutLeaf) -> String {
        let style = self.style_attr(&leaf.style, leaf.visible, leaf.enabled);
        let value = leaf.value.as_deref().unwrap_or("");
        // Interactive form controls get the disabled attribute instead of just
        // an opacity hint; non-interactive controls keep the style-only hint.
        let disabled = if leaf.enabled { "" } else { " disabled" };

        match leaf.control_type {
            LayoutControlType::TextBox => {
                let input_type = if leaf.password_char.is_some() {
                    "password"
                } else {
                    "text"
                };
                let readonly = if leaf.is_locked { " readonly" } else { "" };
                let maxlength = leaf
                    .max_length
                    .map(|n| format!(" maxlength=\"{}\"", n))
                    .unwrap_or_default();
                // Multi-line text boxes render as <textarea> so vertical
                // scrolling and wrapped text behave like VB6.
                if leaf.style.multi_line {
                    format!(
                        r#"<textarea id="{}" class="vb6-textbox" style="{}"{}{}{}{}{}>{}</textarea>"#,
                        html_escape(&leaf.name),
                        html_escape(&style),
                        disabled,
                        readonly,
                        maxlength,
                        title_attr(leaf),
                        tabindex_attr(leaf),
                        html_escape(value)
                    )
                } else {
                    format!(
                        r#"<input id="{}" class="vb6-textbox" type="{}" value="{}" style="{}"{}{}{}{}{}>"#,
                        html_escape(&leaf.name),
                        html_escape(input_type),
                        html_escape(value),
                        html_escape(&style),
                        disabled,
                        readonly,
                        maxlength,
                        title_attr(leaf),
                        tabindex_attr(leaf)
                    )
                }
            }
            LayoutControlType::CheckBox => {
                let checked = leaf.value.as_deref() == Some("True");
                let grayed_style = if leaf.value.as_deref() == Some("Grayed") {
                    " opacity: 0.5"
                } else {
                    ""
                };
                format!(
                    r#"<input id="{}" class="vb6-checkbox" type="checkbox" {} style="{}{}"{}{}{}>"#,
                    html_escape(&leaf.name),
                    if checked { "checked" } else { "" },
                    html_escape(&style),
                    html_escape(grayed_style),
                    disabled,
                    title_attr(leaf),
                    tabindex_attr(leaf)
                )
            }
            LayoutControlType::OptionButton => {
                let checked = leaf.value.as_deref() == Some("True");
                // Radio buttons are grouped by their container (form or frame)
                // so selecting one clears the others in the same group.
                let group = leaf.style.group.as_deref().unwrap_or(&leaf.name);
                format!(
                    r#"<input id="{}" class="vb6-optionbutton" type="radio" name="{}" {} style="{}"{}{}{}>"#,
                    html_escape(&leaf.name),
                    html_escape(group),
                    if checked { "checked" } else { "" },
                    html_escape(&style),
                    disabled,
                    title_attr(leaf),
                    tabindex_attr(leaf)
                )
            }
            LayoutControlType::CommandButton => {
                // CommandButton always processes mnemonics (no use_mnemonic property)
                let caption = TauriRenderer::process_mnemonic(value);
                let inner = if caption.contains("<span") {
                    caption
                } else {
                    html_escape(value).to_string()
                };
                format!(
                    r#"<button id="{}" class="vb6-commandbutton" style="{}"{}{}{}{}{}>{}</button>"#,
                    html_escape(&leaf.name),
                    html_escape(&style),
                    disabled,
                    default_attr(leaf),
                    cancel_attr(leaf),
                    title_attr(leaf),
                    tabindex_attr(leaf),
                    inner
                )
            }
            LayoutControlType::HScrollBar | LayoutControlType::VScrollBar => {
                let min = leaf.range_min.unwrap_or(0);
                let max = leaf.range_max.unwrap_or(100);
                let step = leaf.range_step.unwrap_or(1);
                let step_attr = if step != 1 {
                    format!(" step=\"{}\"", step)
                } else {
                    String::new()
                };
                format!(
                    r#"<input id="{}" class="vb6-{}" type="range" value="{}" min="{}" max="{}"{} style="{}"{}{}{}>"#,
                    html_escape(&leaf.name),
                    leaf.control_type.css_class(),
                    html_escape(value),
                    min,
                    max,
                    step_attr,
                    html_escape(&style),
                    disabled,
                    title_attr(leaf),
                    tabindex_attr(leaf)
                )
            }
            LayoutControlType::ComboBox => {
                let readonly = leaf.combo_style.as_deref() == Some("dropdown-readonly");
                let readonly_attr = if readonly { " disabled" } else { "" };
                let mut options = String::new();
                for item in &leaf.combo_items {
                    let escaped_item = html_escape(item);
                    options.push_str(&format!(r#"<option>{}</option>"#, escaped_item));
                }
                format!(
                    r#"<select id="{}" class="vb6-combobox" style="{}"{}{}{}{}>{}</select>"#,
                    html_escape(&leaf.name),
                    html_escape(&style),
                    disabled,
                    readonly_attr,
                    title_attr(leaf),
                    tabindex_attr(leaf),
                    options
                )
            }
            LayoutControlType::ListBox => {
                if leaf.listbox_style.as_deref() == Some("checkbox") {
                    let disabled_attr = if !leaf.enabled { " disabled" } else { "" };
                    let mut items = String::new();
                    for (i, item) in leaf.list_items.iter().enumerate() {
                        let escaped_item = html_escape(item);
                        items.push_str(&format!(
                            r#"<label><input type="checkbox" id="{}_item{}"{}>{}</input> {}</label>"#,
                            html_escape(&leaf.name),
                            i,
                            disabled_attr,
                            escaped_item,
                            escaped_item
                        ));
                    }
                    format!(
                        r#"<div id="{}" class="vb6-listbox vb6-listbox-checkbox" style="{}"{}>{}</div>"#,
                        html_escape(&leaf.name),
                        html_escape(&style),
                        title_attr(leaf),
                        items
                    )
                } else {
                    format!(
                        r#"<select id="{}" class="vb6-listbox" style="{}"{}{}{}></select>"#,
                        html_escape(&leaf.name),
                        html_escape(&style),
                        disabled,
                        title_attr(leaf),
                        tabindex_attr(leaf)
                    )
                }
            }
            LayoutControlType::Line => {
                // Line control renders as SVG
                let x1 = leaf.style.line_x1.unwrap_or(0.0);
                let y1 = leaf.style.line_y1.unwrap_or(0.0);
                let x2 = leaf.style.line_x2.unwrap_or(0.0);
                let y2 = leaf.style.line_y2.unwrap_or(0.0);
                let color = leaf
                    .style
                    .line_color
                    .as_ref()
                    .map(|c| c.to_css_string())
                    .unwrap_or_else(|| "rgb(0, 0, 0)".to_string());
                let width = leaf.style.line_width.unwrap_or(1.0);
                format!(
                    r#"<svg id="{}" class="vb6-line" style="{}" width="{}" height="{}" viewBox="0 0 {} {}"{}{}><line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="{}" /></svg>"#,
                    html_escape(&leaf.name),
                    html_escape(&style),
                    leaf.size.width,
                    leaf.size.height,
                    leaf.size.width,
                    leaf.size.height,
                    title_attr(leaf),
                    tabindex_attr(leaf),
                    x1,
                    y1,
                    x2,
                    y2,
                    color,
                    width
                )
            }
            LayoutControlType::Image => {
                // Image uses <img> element for proper web semantics and object-fit styling.
                // The `image_src` field holds the base64 data URL for the picture.
                let src = leaf.image_src.as_deref().unwrap_or("");
                format!(
                    r#"<img id="{}" class="vb6-image" src="{}" style="{}"{}{}>"#,
                    html_escape(&leaf.name),
                    html_escape(src),
                    html_escape(&style),
                    title_attr(leaf),
                    tabindex_attr(leaf)
                )
            }
            LayoutControlType::DriveListBox
            | LayoutControlType::DirListBox
            | LayoutControlType::FileListBox => {
                format!(
                    r#"<select id="{}" class="vb6-{}" style="{}"{}{}{}></select>"#,
                    html_escape(&leaf.name),
                    leaf.control_type.css_class(),
                    html_escape(&style),
                    disabled,
                    title_attr(leaf),
                    tabindex_attr(leaf)
                )
            }
            LayoutControlType::Shape => {
                let mut s = style;
                if let Some(radius) = leaf.style.border_radius {
                    if !s.is_empty() {
                        s.push_str(&format!("; border-radius: {radius}px"));
                    } else {
                        s.push_str(&format!("border-radius: {radius}px"));
                    }
                }
                format!(
                    r#"<div id="{}" class="vb6-shape" style="{}"{}{}></div>"#,
                    html_escape(&leaf.name),
                    html_escape(&s),
                    title_attr(leaf),
                    tabindex_attr(leaf)
                )
            }
            LayoutControlType::Timer => {
                // Timer has no visual output — omit from DOM tree entirely.
                // The converter should already filter these when include_nonvisual=false,
                // but the renderer handles them gracefully if they reach this point.
                String::new()
            }
            _ => {
                let processed = if leaf.use_mnemonic {
                    TauriRenderer::process_mnemonic(value)
                } else {
                    html_escape(value).to_string()
                };
                let inner = if processed.contains("<span") {
                    processed
                } else {
                    html_escape(value).to_string()
                };
                format!(
                    r#"<div id="{}" class="vb6-{}" style="{}"{}{}>{}</div>"#,
                    html_escape(&leaf.name),
                    leaf.control_type.css_class(),
                    html_escape(&style),
                    title_attr(leaf),
                    tabindex_attr(leaf),
                    inner
                )
            }
        }
    }

    fn render_container(&self, container: &LayoutContainer) -> String {
        let style = self.style_attr(&container.style, container.visible, container.enabled);
        let caption = container.caption.as_deref().unwrap_or("");

        let mut html = String::new();

        match container.control_type {
            LayoutControlType::Form => {
                // Form renders its children directly — no wrapper div.
                // Dimensions and visibility are applied by the call site
                // (e.g. Tauri webview body, WASM container) rather than a
                // nested element.
                for child in &container.children {
                    html.push_str(&self.render_node(child));
                }
            }
            LayoutControlType::Frame => {
                let mut frame_style = style;
                if container.size.width > 0.0 {
                    if !frame_style.is_empty() {
                        frame_style.push_str("; ");
                    }
                    frame_style.push_str(&format!("width: {:.1}px", container.size.width));
                }
                if container.size.height > 0.0 {
                    if !frame_style.is_empty() {
                        frame_style.push_str("; ");
                    }
                    frame_style.push_str(&format!("height: {:.1}px", container.size.height));
                }
                html.push_str(&format!(
                    r#"<fieldset id="{}" class="vb6-frame" style="{}">"#,
                    html_escape(&container.name),
                    html_escape(&frame_style)
                ));
                let legend_caption = TauriRenderer::process_mnemonic(caption);
                html.push_str(&format!("<legend>{}</legend>", legend_caption));
            }
            _ => {
                let mut container_style = style;
                if container.size.width > 0.0 {
                    if !container_style.is_empty() {
                        container_style.push_str("; ");
                    }
                    container_style.push_str(&format!("width: {:.1}px", container.size.width));
                }
                if container.size.height > 0.0 {
                    if !container_style.is_empty() {
                        container_style.push_str("; ");
                    }
                    container_style.push_str(&format!("height: {:.1}px", container.size.height));
                }
                html.push_str(&format!(
                    r#"<div id="{}" class="vb6-{}" style="{}">"#,
                    html_escape(&container.name),
                    container.control_type.css_class(),
                    html_escape(&container_style)
                ));
            }
        }

        match container.control_type {
            LayoutControlType::Form => {} // children already rendered above
            _ => {
                for child in &container.children {
                    html.push_str(&self.render_node(child));
                }
            }
        }

        html.push_str(match container.control_type {
            LayoutControlType::Form => "", // no wrapper
            LayoutControlType::Frame => "</fieldset>",
            _ => "</div>",
        });

        html
    }

    fn render_children(&self, children: &[LayoutNode]) -> Vec<String> {
        children
            .iter()
            .filter(|n| n.visible())
            .map(|n| self.render_node(n))
            .collect()
    }

    fn render_node_with_diff(&self, node: &LayoutNode, diff: Option<&DiffTree>) -> String {
        match diff {
            Some(d) if !d.is_empty() => {
                let node_id = node.node_id();
                if let Some(change) = d.get_change(&node_id) {
                    match &change.kind {
                        DiffKind::Same => String::new(),
                        DiffKind::Inserted { node } => self.render_node(node.as_ref()),
                        _ => self.render_node(node),
                    }
                } else {
                    self.render_node(node)
                }
            }
            _ => self.render_node(node),
        }
    }

    fn apply_diff(&self, node: &LayoutNode, change: &DiffChange) -> String {
        match &change.kind {
            DiffKind::Same => String::new(),
            DiffKind::Inserted { node } => self.render_node(node.as_ref()),
            DiffKind::Removed => String::new(),
            _ => self.render_node(node),
        }
    }
}

impl crate::layout::theme::ThemeRenderer for TauriRenderer {
    /// Inject theme CSS into the Tauri webview.
    ///
    /// Creates a `<style>` element containing the theme's CSS custom properties
    /// and injects it into the `<head>` of the webview via `webview.eval()`.
    fn inject_theme(&self, theme: &crate::layout::theme::Vb6Theme) {
        let css = theme.to_css();
        let js = format!(
            "document.head.insertAdjacentHTML('beforeend', '<style>{}</style>');",
            css.replace('\\', "\\\\").replace('\'', "\\'")
        );
        let _ = js;
        // In actual Tauri, this would call self.webview.eval(&js)
        // For the pure-Rust renderer, we just return the CSS string
    }
}

impl crate::layout::theme::CssInjector for TauriRenderer {
    /// Inject raw CSS into the Tauri webview.
    ///
    /// Creates a `<style>` element containing the provided CSS
    /// and injects it into the `<head>` of the webview.
    fn inject_css(&self, css: &str) {
        let js = format!(
            "document.head.insertAdjacentHTML('beforeend', '<style>{}</style>');",
            css.replace('\\', "\\\\").replace('\'', "\\'")
        );
        let _ = js;
        // In actual Tauri, this would call self.webview.eval(&js)
    }
}

/// Escape special HTML characters in a string.
///
/// Handles `&`, `<`, `>`, `"`, and `'` to prevent XSS when injecting
/// untrusted content into HTML.
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

/// Build a `title` attribute string from a leaf's tooltip.
fn title_attr(leaf: &LayoutLeaf) -> String {
    leaf.tooltip
        .as_deref()
        .map(|t| format!(r#" title="{}""#, html_escape(t)))
        .unwrap_or_default()
}

/// Build a `tabindex` attribute string from a leaf's tabindex value.
fn tabindex_attr(leaf: &LayoutLeaf) -> String {
    leaf.tabindex
        .map(|t| format!(r#" tabindex="{}""#, t))
        .unwrap_or_default()
}

/// Build `autofocus` and `type="submit"` attribute strings for default buttons.
fn default_attr(leaf: &LayoutLeaf) -> String {
    if leaf.is_default {
        r#" autofocus type="submit""#.to_string()
    } else {
        String::new()
    }
}

/// Build `type="submit"` and `data-cancel` attribute strings for cancel buttons.
fn cancel_attr(leaf: &LayoutLeaf) -> String {
    if leaf.is_cancel {
        r#" type="submit" data-cancel="true""#.to_string()
    } else {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::model::{LayoutPosition, LayoutSize, LayoutStyle, NodeId};

    fn make_leaf(name: &str, control_type: LayoutControlType, value: Option<String>) -> LayoutLeaf {
        LayoutLeaf {
            name: name.into(),
            control_type,
            index: 0,
            position: LayoutPosition::default(),
            size: LayoutSize::default(),
            style: LayoutStyle::default(),
            value,
            visible: true,
            enabled: true,
            ..Default::default()
        }
    }

    fn make_container(name: &str, control_type: LayoutControlType) -> LayoutContainer {
        LayoutContainer {
            name: name.into(),
            control_type,
            index: 0,
            position: LayoutPosition::default(),
            size: LayoutSize {
                width: 400.0,
                height: 300.0,
            },
            style: LayoutStyle::default(),
            children: vec![],
            caption: Some("Test".into()),
            visible: true,
            enabled: true,
            current_value: None,
        }
    }

    #[test]
    fn render_label_leaf() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf("lblTest", LayoutControlType::Label, Some("Hello".into()));
        let html = renderer.render_leaf(&leaf);
        assert!(html.contains("vb6-label"));
        assert!(html.contains("Hello"));
    }

    #[test]
    fn render_button_leaf() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf(
            "cmdOK",
            LayoutControlType::CommandButton,
            Some("&OK".into()),
        );
        let html = renderer.render_leaf(&leaf);
        assert!(html.contains("<button"));
        assert!(html.contains("vb6-commandbutton"));
        assert!(html.contains("vb6-mnemonic"));
        assert!(html.contains(">O</span>K"));
    }

    #[test]
    fn render_frame_container() {
        let renderer = TauriRenderer::new(false);
        let container = make_container("fraGroup", LayoutControlType::Frame);
        let html = renderer.render_container(&container);
        assert!(html.contains("<fieldset"));
        assert!(html.contains("<legend>Test</legend>"));
    }

    #[test]
    fn render_textbox_input() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf("txtName", LayoutControlType::TextBox, Some("John".into()));
        let html = renderer.render_leaf(&leaf);
        assert!(html.contains("<input"));
        assert!(html.contains(r#"type="text""#));
        assert!(html.contains("value=\"John\""));
    }

    #[test]
    fn render_checkbox_checked() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf("chkAgree", LayoutControlType::CheckBox, Some("True".into()));
        let html = renderer.render_leaf(&leaf);
        assert!(html.contains(r#"type="checkbox""#));
        assert!(html.contains("checked"));
    }

    #[test]
    fn render_checkbox_unchecked() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf(
            "chkAgree",
            LayoutControlType::CheckBox,
            Some("False".into()),
        );
        let html = renderer.render_leaf(&leaf);
        assert!(html.contains(r#"type="checkbox""#));
        assert!(!html.contains("checked"));
    }

    #[test]
    fn render_checkbox_grayed() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf(
            "chkMixed",
            LayoutControlType::CheckBox,
            Some("Grayed".into()),
        );
        let html = renderer.render_leaf(&leaf);
        assert!(html.contains(r#"type="checkbox""#));
        assert!(html.contains("opacity: 0.5"));
        assert!(!html.contains("checked"));
    }

    #[test]
    fn invisible_control_includes_style() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf("lblHidden", LayoutControlType::Label, None);
        let leaf = LayoutLeaf {
            visible: false,
            ..leaf
        };
        let html = renderer.render_leaf(&leaf);
        assert!(html.contains("visibility: hidden"));
    }

    #[test]
    fn disabled_control_includes_opacity() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf("cmdDisabled", LayoutControlType::CommandButton, None);
        let leaf = LayoutLeaf {
            enabled: false,
            ..leaf
        };
        let html = renderer.render_leaf(&leaf);
        assert!(html.contains("opacity: 0.5"));
    }

    #[test]
    fn html_escape_special_chars() {
        assert!(html_escape("&").contains("&amp;"));
        assert!(html_escape("<").contains("&lt;"));
        assert!(html_escape(">").contains("&gt;"));
        assert!(html_escape("\"").contains("&quot;"));
        assert!(html_escape("'").contains("&#x27;"));
    }

    #[test]
    fn tooltip_rendered_on_label() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf("Label1", LayoutControlType::Label, Some("Hello".into()));
        let leaf = LayoutLeaf {
            tooltip: Some("Tooltip text".into()),
            ..leaf
        };
        let html = renderer.render_leaf(&leaf);
        assert!(html.contains(r#"title="Tooltip text""#));
    }

    #[test]
    fn tooltip_rendered_on_button() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf("cmdOK", LayoutControlType::CommandButton, Some("OK".into()));
        let leaf = LayoutLeaf {
            tooltip: Some("Click OK".into()),
            ..leaf
        };
        let html = renderer.render_leaf(&leaf);
        assert!(html.contains(r#"title="Click OK""#));
    }

    #[test]
    fn tooltip_rendered_on_textbox() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf("txtName", LayoutControlType::TextBox, Some("".into()));
        let leaf = LayoutLeaf {
            tooltip: Some("Enter your name".into()),
            ..leaf
        };
        let html = renderer.render_leaf(&leaf);
        assert!(html.contains(r#"title="Enter your name""#));
    }

    #[test]
    fn no_tooltip_means_no_title_attr() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf(
            "cmdNoTip",
            LayoutControlType::CommandButton,
            Some("Button".into()),
        );
        let html = renderer.render_leaf(&leaf);
        assert!(!html.contains("title="));
    }

    #[test]
    fn tooltip_html_escaping() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf(
            "cmdEsc",
            LayoutControlType::CommandButton,
            Some("Esc".into()),
        );
        let leaf = LayoutLeaf {
            tooltip: Some("A \"safe\" tip".into()),
            ..leaf
        };
        let html = renderer.render_leaf(&leaf);
        assert!(html.contains(r#"title="A &quot;safe&quot; tip""#));
    }

    #[test]
    fn tabindex_rendered_on_button() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf("cmdOK", LayoutControlType::CommandButton, Some("OK".into()));
        let leaf = LayoutLeaf {
            tabindex: Some(0),
            ..leaf
        };
        let html = renderer.render_leaf(&leaf);
        assert!(html.contains(r#"tabindex="0""#));
    }

    #[test]
    fn tabindex_rendered_on_textbox() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf("txtName", LayoutControlType::TextBox, Some("".into()));
        let leaf = LayoutLeaf {
            tabindex: Some(-1),
            ..leaf
        };
        let html = renderer.render_leaf(&leaf);
        assert!(html.contains(r#"tabindex="-1""#));
    }

    #[test]
    fn tabindex_rendered_on_checkbox() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf("chkAgree", LayoutControlType::CheckBox, Some("True".into()));
        let leaf = LayoutLeaf {
            tabindex: Some(0),
            ..leaf
        };
        let html = renderer.render_leaf(&leaf);
        assert!(html.contains(r#"tabindex="0""#));
    }

    #[test]
    fn tabindex_rendered_on_optionbutton() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf(
            "optChoice",
            LayoutControlType::OptionButton,
            Some("True".into()),
        );
        let leaf = LayoutLeaf {
            tabindex: Some(0),
            ..leaf
        };
        let html = renderer.render_leaf(&leaf);
        assert!(html.contains(r#"tabindex="0""#));
    }

    #[test]
    fn tabindex_rendered_on_combobox() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf("cmbList", LayoutControlType::ComboBox, Some("".into()));
        let leaf = LayoutLeaf {
            tabindex: Some(0),
            ..leaf
        };
        let html = renderer.render_leaf(&leaf);
        assert!(html.contains(r#"tabindex="0""#));
    }

    #[test]
    fn tabindex_rendered_on_listbox() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf("lstItems", LayoutControlType::ListBox, Some("".into()));
        let leaf = LayoutLeaf {
            tabindex: Some(0),
            ..leaf
        };
        let html = renderer.render_leaf(&leaf);
        assert!(html.contains(r#"tabindex="0""#));
    }

    #[test]
    fn tabindex_rendered_on_scrollbar() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf("scrValue", LayoutControlType::HScrollBar, Some("50".into()));
        let leaf = LayoutLeaf {
            tabindex: Some(0),
            ..leaf
        };
        let html = renderer.render_leaf(&leaf);
        assert!(html.contains(r#"tabindex="0""#));
    }

    #[test]
    fn default_button_has_autofocus_and_submit() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf("cmdOK", LayoutControlType::CommandButton, Some("OK".into()));
        let leaf = LayoutLeaf {
            is_default: true,
            ..leaf
        };
        let html = renderer.render_leaf(&leaf);
        assert!(html.contains(r#"autofocus"#));
        assert!(html.contains(r#"type="submit""#));
    }

    #[test]
    fn cancel_button_has_submit_and_data_cancel() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf(
            "cmdCancel",
            LayoutControlType::CommandButton,
            Some("Cancel".into()),
        );
        let leaf = LayoutLeaf {
            is_cancel: true,
            ..leaf
        };
        let html = renderer.render_leaf(&leaf);
        assert!(html.contains(r#"type="submit""#));
        assert!(html.contains(r#"data-cancel="true""#));
    }

    #[test]
    fn default_and_cancel_button_has_both_attributes() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf(
            "cmdDefaultCancel",
            LayoutControlType::CommandButton,
            Some("OK".into()),
        );
        let leaf = LayoutLeaf {
            is_default: true,
            is_cancel: true,
            ..leaf
        };
        let html = renderer.render_leaf(&leaf);
        assert!(html.contains(r#"autofocus"#));
        assert!(html.contains(r#"type="submit""#));
        assert!(html.contains(r#"data-cancel="true""#));
    }

    #[test]
    fn regular_button_has_no_default_cancel_attributes() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf(
            "cmdRegular",
            LayoutControlType::CommandButton,
            Some("Click".into()),
        );
        let html = renderer.render_leaf(&leaf);
        assert!(!html.contains("autofocus"));
        assert!(!html.contains("data-cancel"));
    }

    #[test]
    fn no_tabindex_means_no_tabindex_attr() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf("lblNoTab", LayoutControlType::Label, Some("Label".into()));
        let html = renderer.render_leaf(&leaf);
        assert!(!html.contains("tabindex="));
    }

    #[test]
    fn render_children_filters_invisible() {
        let renderer = TauriRenderer::new(false);
        let visible = LayoutNode::Leaf(make_leaf("visible", LayoutControlType::Label, None));
        let hidden = LayoutNode::Leaf(make_leaf("hidden", LayoutControlType::Label, None));
        let hidden = {
            let mut h = hidden;
            if let LayoutNode::Leaf(ref mut l) = h {
                l.visible = false;
            }
            h
        };
        let children = vec![visible, hidden];
        let rendered = renderer.render_children(&children);
        assert_eq!(rendered.len(), 1);
        assert!(rendered[0].contains("visible"));
    }

    #[test]
    fn render_image_leaf() {
        let renderer = TauriRenderer::new(false);
        let leaf = LayoutLeaf {
            name: "imgLogo".into(),
            control_type: LayoutControlType::Image,
            index: 0,
            position: LayoutPosition::default(),
            size: LayoutSize::default(),
            style: LayoutStyle::default(),
            value: None,
            image_src: Some("data:image/png;base64,abc123".into()),
            visible: true,
            enabled: true,
            ..Default::default()
        };
        let html = renderer.render_leaf(&leaf);
        assert!(html.contains("<img"));
        assert!(html.contains("vb6-image"));
        assert!(html.contains("src=\"data:image/png;base64,abc123\""));
    }

    #[test]
    fn render_image_leaf_empty_src() {
        let renderer = TauriRenderer::new(false);
        let leaf = LayoutLeaf {
            name: "imgEmpty".into(),
            control_type: LayoutControlType::Image,
            index: 0,
            position: LayoutPosition::default(),
            size: LayoutSize::default(),
            style: LayoutStyle::default(),
            value: None,
            image_src: None,
            visible: true,
            enabled: true,
            ..Default::default()
        };
        let html = renderer.render_leaf(&leaf);
        assert!(html.contains("<img"));
        assert!(html.contains("src=\"\""));
    }

    #[test]
    fn render_timer_omitted() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf("tmrTick", LayoutControlType::Timer, None);
        let html = renderer.render_leaf(&leaf);
        assert!(html.is_empty());
    }

    #[test]
    fn render_drive_listbox() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf("drv drives", LayoutControlType::DriveListBox, None);
        let html = renderer.render_leaf(&leaf);
        assert!(html.contains("<select"));
        assert!(html.contains("vb6-drivelistbox"));
    }

    #[test]
    fn render_dir_listbox() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf("dir1", LayoutControlType::DirListBox, None);
        let html = renderer.render_leaf(&leaf);
        assert!(html.contains("<select"));
        assert!(html.contains("vb6-dirlistbox"));
    }

    #[test]
    fn render_file_listbox() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf("fil1", LayoutControlType::FileListBox, None);
        let html = renderer.render_leaf(&leaf);
        assert!(html.contains("<select"));
        assert!(html.contains("vb6-filelistbox"));
    }

    #[test]
    fn render_picture_box_container() {
        let renderer = TauriRenderer::new(false);
        let container = make_container("picBox", LayoutControlType::PictureBox);
        let html = renderer.render_container(&container);
        assert!(html.contains("<div"));
        assert!(html.contains("vb6-picturebox"));
        assert!(html.contains("</div>"));
    }

    #[test]
    fn render_optionbutton_radio() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf(
            "optChoice",
            LayoutControlType::OptionButton,
            Some("True".into()),
        );
        let html = renderer.render_leaf(&leaf);
        assert!(html.contains(r#"type="radio""#));
        assert!(html.contains("vb6-optionbutton"));
        assert!(html.contains("checked"));
    }

    #[test]
    fn render_container_with_children() {
        let renderer = TauriRenderer::new(false);
        let mut container = make_container("fraGroup", LayoutControlType::Frame);
        container.children.push(LayoutNode::Leaf(make_leaf(
            "lblInside",
            LayoutControlType::Label,
            Some("Inside frame".into()),
        )));
        container.children.push(LayoutNode::Leaf(make_leaf(
            "cmdInside",
            LayoutControlType::CommandButton,
            Some("Click".into()),
        )));
        let html = renderer.render_container(&container);
        assert!(html.contains("<fieldset"));
        assert!(html.contains("<legend>Test</legend>"));
        assert!(html.contains("vb6-label"));
        assert!(html.contains("Inside frame"));
        assert!(html.contains("vb6-commandbutton"));
        assert!(html.contains("Click"));
        assert!(html.contains("</fieldset>"));
    }

    #[test]
    fn incremental_render_same_returns_empty() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf("lbl1", LayoutControlType::Label, Some("Hello".into()));
        let diff = DiffTree {
            leaf_changes: vec![DiffChange {
                id: leaf.node_id(),
                kind: DiffKind::Same,
                child_diff: DiffTree::default(),
            }],
            unchanged_containers: vec![],
        };
        let result = renderer.render_node_with_diff(&LayoutNode::Leaf(leaf), Some(&diff));
        assert!(result.is_empty());
    }

    #[test]
    fn incremental_render_insert_returns_full_html() {
        let renderer = TauriRenderer::new(false);
        let new_node = LayoutNode::Leaf(make_leaf(
            "new_label",
            LayoutControlType::Label,
            Some("Inserted".into()),
        ));
        let diff = DiffTree {
            leaf_changes: vec![DiffChange {
                id: new_node.node_id(),
                kind: DiffKind::Inserted {
                    node: Box::new(new_node.clone()),
                },
                child_diff: DiffTree::default(),
            }],
            unchanged_containers: vec![],
        };
        let result = renderer.render_node_with_diff(&new_node, Some(&diff));
        assert!(!result.is_empty());
        assert!(result.contains("Inserted"));
        assert!(result.contains("vb6-label"));
    }

    #[test]
    fn incremental_render_unknown_change_falls_back_to_full() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf("lbl1", LayoutControlType::Label, Some("Updated".into()));
        let diff = DiffTree {
            leaf_changes: vec![DiffChange {
                id: leaf.node_id(),
                kind: DiffKind::ValueChanged {
                    old_value: Some("Old".into()),
                    new_value: Some("Updated".into()),
                },
                child_diff: DiffTree::default(),
            }],
            unchanged_containers: vec![],
        };
        let result = renderer.render_node_with_diff(&LayoutNode::Leaf(leaf), Some(&diff));
        assert!(!result.is_empty());
        assert!(result.contains("Updated"));
    }

    #[test]
    fn incremental_render_no_diff_performs_full_render() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf("lbl1", LayoutControlType::Label, Some("Hello".into()));
        let result = renderer.render_node_with_diff(&LayoutNode::Leaf(leaf), None);
        assert!(!result.is_empty());
        assert!(result.contains("Hello"));
    }

    #[test]
    fn incremental_render_empty_diff_performs_full_render() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf("lbl1", LayoutControlType::Label, Some("Hello".into()));
        let empty_diff = DiffTree::default();
        let result = renderer.render_node_with_diff(&LayoutNode::Leaf(leaf), Some(&empty_diff));
        assert!(!result.is_empty());
        assert!(result.contains("Hello"));
    }

    #[test]
    fn incremental_render_node_id_not_in_diff_performs_full_render() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf("lbl1", LayoutControlType::Label, Some("Hello".into()));
        let diff = DiffTree {
            leaf_changes: vec![DiffChange {
                id: NodeId {
                    name: "other".into(),
                    kind: LayoutControlType::TextBox,
                    index: 0,
                },
                kind: DiffKind::Same,
                child_diff: DiffTree::default(),
            }],
            unchanged_containers: vec![],
        };
        let result = renderer.render_node_with_diff(&LayoutNode::Leaf(leaf), Some(&diff));
        assert!(!result.is_empty());
        assert!(result.contains("Hello"));
    }

    #[test]
    fn incremental_apply_diff_same_returns_empty() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf("lbl1", LayoutControlType::Label, Some("Hello".into()));
        let change = DiffChange {
            id: leaf.node_id(),
            kind: DiffKind::Same,
            child_diff: DiffTree::default(),
        };
        let result = renderer.apply_diff(&LayoutNode::Leaf(leaf.clone()), &change);
        assert!(result.is_empty());
    }

    #[test]
    fn incremental_apply_diff_insert_returns_full_html() {
        let renderer = TauriRenderer::new(false);
        let inserted_node = LayoutNode::Leaf(make_leaf(
            "new_ctrl",
            LayoutControlType::CommandButton,
            Some("New Button".into()),
        ));
        let change = DiffChange {
            id: inserted_node.node_id(),
            kind: DiffKind::Inserted {
                node: Box::new(inserted_node.clone()),
            },
            child_diff: DiffTree::default(),
        };
        let result = renderer.apply_diff(&inserted_node, &change);
        assert!(!result.is_empty());
        assert!(result.contains("New Button"));
        assert!(result.contains("vb6-commandbutton"));
    }

    #[test]
    fn incremental_apply_diff_removed_returns_empty() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf("lbl1", LayoutControlType::Label, Some("Hello".into()));
        let change = DiffChange {
            id: leaf.node_id(),
            kind: DiffKind::Removed,
            child_diff: DiffTree::default(),
        };
        let result = renderer.apply_diff(&LayoutNode::Leaf(leaf), &change);
        assert!(result.is_empty());
    }

    #[test]
    fn incremental_apply_diff_value_changed_returns_full_html() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf(
            "lbl1",
            LayoutControlType::Label,
            Some("Updated Value".into()),
        );
        let change = DiffChange {
            id: leaf.node_id(),
            kind: DiffKind::ValueChanged {
                old_value: Some("Old".into()),
                new_value: Some("Updated Value".into()),
            },
            child_diff: DiffTree::default(),
        };
        let result = renderer.apply_diff(&LayoutNode::Leaf(leaf), &change);
        assert!(!result.is_empty());
        assert!(result.contains("Updated Value"));
    }

    #[test]
    fn incremental_apply_diff_visibility_changed_returns_full_html() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf("lbl1", LayoutControlType::Label, Some("Hello".into()));
        let leaf = LayoutLeaf {
            visible: false,
            ..leaf
        };
        let change = DiffChange {
            id: leaf.node_id(),
            kind: DiffKind::VisibilityChanged {
                old_visible: true,
                new_visible: false,
            },
            child_diff: DiffTree::default(),
        };
        let result = renderer.apply_diff(&LayoutNode::Leaf(leaf), &change);
        assert!(!result.is_empty());
        assert!(result.contains("visibility: hidden"));
    }

    #[test]
    fn incremental_apply_diff_enabled_changed_returns_full_html() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf(
            "cmd1",
            LayoutControlType::CommandButton,
            Some("Click".into()),
        );
        let leaf = LayoutLeaf {
            enabled: false,
            ..leaf
        };
        let change = DiffChange {
            id: leaf.node_id(),
            kind: DiffKind::EnabledChanged {
                old_enabled: true,
                new_enabled: false,
            },
            child_diff: DiffTree::default(),
        };
        let result = renderer.apply_diff(&LayoutNode::Leaf(leaf), &change);
        assert!(!result.is_empty());
        assert!(result.contains("opacity: 0.5"));
    }

    #[test]
    fn incremental_apply_diff_children_changed_returns_full_html() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf("lbl1", LayoutControlType::Label, Some("Hello".into()));
        let change = DiffChange {
            id: leaf.node_id(),
            kind: DiffKind::ChildrenChanged { inserts: vec![] },
            child_diff: DiffTree::default(),
        };
        let result = renderer.apply_diff(&LayoutNode::Leaf(leaf), &change);
        assert!(!result.is_empty());
    }

    #[test]
    fn incremental_render_container_same_returns_empty() {
        let renderer = TauriRenderer::new(false);
        let container = make_container("frm1", LayoutControlType::Form);
        let diff = DiffTree {
            leaf_changes: vec![DiffChange {
                id: container.node_id(),
                kind: DiffKind::Same,
                child_diff: DiffTree::default(),
            }],
            unchanged_containers: vec![],
        };
        let result = renderer.render_node_with_diff(&LayoutNode::Container(container), Some(&diff));
        assert!(result.is_empty());
    }

    #[test]
    fn incremental_render_container_value_changed_returns_full_html() {
        let renderer = TauriRenderer::new(false);
        let mut container = make_container("frm1", LayoutControlType::Form);
        container.children.push(LayoutNode::Leaf(make_leaf(
            "lbl1",
            LayoutControlType::Label,
            Some("Hello".into()),
        )));
        container.visible = false;
        let diff = DiffTree {
            leaf_changes: vec![DiffChange {
                id: container.node_id(),
                kind: DiffKind::VisibilityChanged {
                    old_visible: true,
                    new_visible: false,
                },
                child_diff: DiffTree::default(),
            }],
            unchanged_containers: vec![],
        };
        let result = renderer.render_node_with_diff(&LayoutNode::Container(container), Some(&diff));
        assert!(!result.is_empty());
        assert!(result.contains("vb6-label"));
        assert!(result.contains("Hello"));
    }
}
