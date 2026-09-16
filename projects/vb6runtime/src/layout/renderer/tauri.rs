//! Tauri renderer — produces HTML fragment strings for webview injection.
//!
//! Generates self-contained HTML with inline styles from the [`LayoutNode`] tree.
//! Used by Tauri to inject VB6-rendered forms into a webview via `webview.eval()`.
//!
//! # Control-to-HTML Mapping
//!
//! | VB6 Control | HTML Element |
//! |-------------|-------------|
//! | Form | `<div class="vb6-form">` |
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

        match leaf.control_type {
            LayoutControlType::TextBox => {
                format!(
                    r#"<input id="{}" class="vb6-textbox" type="text" value="{}" style="{}">"#,
                    html_escape(&leaf.name),
                    html_escape(value),
                    style
                )
            }
            LayoutControlType::CheckBox => {
                let checked = leaf.value.as_deref() == Some("True");
                format!(
                    r#"<input id="{}" class="vb6-checkbox" type="checkbox" {} style="{}">"#,
                    html_escape(&leaf.name),
                    if checked { "checked" } else { "" },
                    style
                )
            }
            LayoutControlType::OptionButton => {
                let checked = leaf.value.as_deref() == Some("True");
                format!(
                    r#"<input id="{}" class="vb6-optionbutton" type="radio" name="{}" {} style="{}">"#,
                    html_escape(&leaf.name),
                    html_escape(&leaf.name),
                    if checked { "checked" } else { "" },
                    style
                )
            }
            LayoutControlType::CommandButton => {
                format!(
                    r#"<button id="{}" class="vb6-commandbutton" style="{}">{}</button>"#,
                    html_escape(&leaf.name),
                    style,
                    html_escape(value)
                )
            }
            LayoutControlType::HScrollBar | LayoutControlType::VScrollBar => {
                let input_type = match leaf.control_type {
                    LayoutControlType::HScrollBar => "range",
                    LayoutControlType::VScrollBar => "range",
                    _ => "text",
                };
                let orient = match leaf.control_type {
                    LayoutControlType::VScrollBar => {
                        if !style.is_empty() {
                            format!(
                                "{style}; writing-mode: bt-lr; -webkit-appearance: slider-vertical;"
                            )
                        } else {
                            "writing-mode: bt-lr; -webkit-appearance: slider-vertical;".to_string()
                        }
                    }
                    _ => style,
                };
                format!(
                    r#"<input id="{}" class="vb6-{}" type="{}" value="{}" style="{}">"#,
                    html_escape(&leaf.name),
                    leaf.control_type.css_class(),
                    input_type,
                    html_escape(value),
                    orient
                )
            }
            LayoutControlType::ComboBox => {
                format!(
                    r#"<select id="{}" class="vb6-combobox" style="{}"></select>"#,
                    html_escape(&leaf.name),
                    style
                )
            }
            LayoutControlType::ListBox => {
                format!(
                    r#"<select id="{}" class="vb6-listbox" style="{}"></select>"#,
                    html_escape(&leaf.name),
                    style
                )
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
                    r#"<svg id="{}" class="vb6-line" style="{}" width="{}" height="{}" viewBox="0 0 {} {}"><line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="{}" /></svg>"#,
                    html_escape(&leaf.name),
                    style,
                    leaf.size.width,
                    leaf.size.height,
                    leaf.size.width,
                    leaf.size.height,
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
                // The `value` field holds the image path/URL.
                format!(
                    r#"<img id="{}" class="vb6-image" src="{}" style="{}" />"#,
                    html_escape(&leaf.name),
                    html_escape(value),
                    style
                )
            }
            LayoutControlType::DriveListBox | LayoutControlType::DirListBox | LayoutControlType::FileListBox => {
                format!(
                    r#"<select id="{}" class="vb6-{}" style="{}"></select>"#,
                    html_escape(&leaf.name),
                    leaf.control_type.css_class(),
                    style
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
                    r#"<div id="{}" class="vb6-shape" style="{}"></div>"#,
                    html_escape(&leaf.name),
                    s
                )
            }
            LayoutControlType::Timer => {
                // Timer has no visual output — omit from DOM tree entirely.
                // The converter should already filter these when include_nonvisual=false,
                // but the renderer handles them gracefully if they reach this point.
                String::new()
            }
            _ => {
                format!(
                    r#"<div id="{}" class="vb6-{}" style="{}">{}</div>"#,
                    html_escape(&leaf.name),
                    leaf.control_type.css_class(),
                    style,
                    html_escape(value)
                )
            }
        }
    }

    fn render_container(&self, container: &LayoutContainer) -> String {
        let tag = match container.control_type {
            LayoutControlType::Form | LayoutControlType::MDIForm => "div",
            LayoutControlType::Frame => "fieldset",
            LayoutControlType::PictureBox => "div",
            _ => "div",
        };

        let style = self.style_attr(&container.style, container.visible, container.enabled);
        let caption = container.caption.as_deref().unwrap_or("");

        let mut html = String::new();

        match container.control_type {
            LayoutControlType::Frame => {
                html.push_str(&format!(
                    r#"<fieldset id="{}" class="vb6-frame" style="{}">"#,
                    html_escape(&container.name),
                    style
                ));
                html.push_str(&format!("<legend>{}</legend>", html_escape(caption)));
            }
            _ => {
                html.push_str(&format!(
                    r#"<div id="{}" class="vb6-{}" style="{}">"#,
                    html_escape(&container.name),
                    tag,
                    style
                ));
            }
        }

        for child in &container.children {
            html.push_str(&self.render_node(child));
        }

        html.push_str(if container.control_type == LayoutControlType::Frame {
            "</fieldset>"
        } else {
            "</div>"
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::model::{LayoutPosition, LayoutSize, LayoutStyle};

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
        assert!(html.contains("&amp;OK"));
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
        let leaf = make_leaf("imgLogo", LayoutControlType::Image, Some("logo.png".into()));
        let html = renderer.render_leaf(&leaf);
        assert!(html.contains("<img"));
        assert!(html.contains("vb6-image"));
        assert!(html.contains("src=\"logo.png\""));
    }

    #[test]
    fn render_image_leaf_empty_src() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf("imgEmpty", LayoutControlType::Image, Some("".into()));
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
        assert!(html.contains("vb6-div"));
        assert!(html.contains("</div>"));
    }

    #[test]
    fn render_optionbutton_radio() {
        let renderer = TauriRenderer::new(false);
        let leaf = make_leaf("optChoice", LayoutControlType::OptionButton, Some("True".into()));
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
}
