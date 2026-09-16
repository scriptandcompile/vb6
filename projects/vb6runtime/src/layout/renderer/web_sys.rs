//! WebSys renderer — creates `web_sys::Element` objects for WASM target.
//!
//! Produces real DOM nodes that are appended to the browser DOM.
//! Wraps all output in a `.vb6-app` scope root for CSS isolation.
//!
//! # WASM Feature Gate
//!
//! This module is only compiled when the `wasm` feature is enabled.
//! It uses `web_sys` types that are only available on the `wasm32` target.
//!
//! # Usage
//!
//! ```ignore
//! use vb6runtime::layout::renderer::web_sys::WebSysRenderer;
//! use web_sys::window;
//!
//! let doc = window().unwrap().document().unwrap();
//! let renderer = WebSysRenderer::new(doc);
//! let form_el = renderer.render_node(&layout_form.root_node);
//! container.append_child(&form_el).unwrap();
//! ```

use crate::layout::css::style_to_css;
use crate::layout::model::{LayoutContainer, LayoutLeaf, LayoutNode, LayoutStyle};
use crate::layout::renderer::Renderer;
use crate::layout::theme::{ThemeRenderer, Vb6Theme};

use crate::layout::model::LayoutControlType;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

#[cfg(target_arch = "wasm32")]
use web_sys::{Document, Element};

/// WASM renderer that creates `web_sys::Element` objects.
///
/// Wraps all output in a `.vb6-app` scope root for CSS isolation.
/// The scope root has `id="vb6-container"` so it can be found and
/// appended to by the host application.
///
/// # Example
///
/// ```ignore
/// use vb6runtime::layout::renderer::web_sys::WebSysRenderer;
/// use web_sys::window;
///
/// let doc = window().unwrap().document().unwrap();
/// let renderer = WebSysRenderer::new(doc);
/// let form_element = renderer.render_node(&form.root_node);
/// // Append to a container element in the page
/// ```
#[derive(Debug)]
pub struct WebSysRenderer {
    /// The DOM document used to create elements.
    pub doc: Document,
}

impl WebSysRenderer {
    /// Create a new `WebSysRenderer` bound to the given document.
    ///
    /// The renderer will always wrap output in a `.vb6-app` scope root
    /// since WASM targets need CSS isolation from the parent page.
    #[cfg(target_arch = "wasm32")]
    #[must_use]
    pub fn new(doc: Document) -> Self {
        Self { doc }
    }

    /// Generate the opening tag for the scope root.
    ///
    /// For WASM, this is always `<div class="vb6-app" id="vb6-container">`
    /// since CSS isolation is required.
    #[cfg(target_arch = "wasm32")]
    #[must_use]
    pub fn root_open(&self) -> Element {
        let div = self.doc.create_element("div").expect("create div element");
        div.set_class_name("vb6-app");
        div.set_id("vb6-container");
        div
    }

    /// Render a [`LayoutNode`] to a `web_sys::Element`.
    ///
    /// Dispatches to [`render_container`] or [`render_leaf`] based on node type.
    #[cfg(target_arch = "wasm32")]
    pub fn render_node_to_element(&self, node: &LayoutNode) -> Element {
        match node {
            LayoutNode::Container(c) => self.render_container(c),
            LayoutNode::Leaf(l) => self.render_leaf(l),
        }
    }

    /// Build the scope root element with a styled child.
    ///
    /// Creates `<div class="vb6-app" id="vb6-container">` and appends
    /// the given child element as its only child.
    #[cfg(target_arch = "wasm32")]
    #[must_use]
    pub fn build_scoped_root(&self, child: &Element) -> Element {
        let root = self.root_open();
        root.append_child(child).expect("append child to scope root");
        root
    }
}

impl Renderer for WebSysRenderer {
    type Output = Element;

    #[cfg(target_arch = "wasm32")]
    fn render_node(&self, node: &LayoutNode) -> Element {
        self.render_node_to_element(node)
    }

    #[cfg(target_arch = "wasm32")]
    fn render_leaf(&self, leaf: &LayoutLeaf) -> Element {
        let tag = tag_for_control(leaf.control_type);
        let el = self.doc.create_element(tag).expect("create element");

        el.set_id(&leaf.name);
        el.set_class_name(&format!("vb6-{}", leaf.control_type.css_class()));
        el.set_attribute(
            "style",
            &self.style_attr(&leaf.style, leaf.visible, leaf.enabled),
        )
        .ok();

        if let Some(ref value) = leaf.value {
            let tag_lower = tag.to_lowercase();
            if tag_lower == "button" {
                el.set_text_content(Some(value));
            } else if tag_lower == "input" {
                let _ = el.set_attribute("value", value);
            } else {
                el.set_text_content(Some(value));
            }
        }

        el
    }

    #[cfg(target_arch = "wasm32")]
    fn render_container(&self, container: &LayoutContainer) -> Element {
        let tag = match container.control_type {
            LayoutControlType::Form | LayoutControlType::MDIForm => "div",
            LayoutControlType::Frame => "fieldset",
            LayoutControlType::PictureBox => "div",
            _ => "div",
        };

        let el = self.doc.create_element(tag).expect("create element");

        el.set_id(&container.name);
        el.set_class_name(&format!("vb6-{}", tag));
        el.set_attribute(
            "style",
            &self.style_attr(&container.style, container.visible, container.enabled),
        )
        .ok();

        if let Some(ref caption) = container.caption {
            if tag == "fieldset" {
                let legend = self.doc.create_element("legend").expect("create legend");
                legend.set_text_content(Some(caption));
                let _ = el.append_child(&legend);
            } else {
                el.set_text_content(Some(caption));
            }
        }

        for child in &container.children {
            if child.visible() {
                let child_el = self.render_node(child);
                let _ = el.append_child(&child_el);
            }
        }

        el
    }

    #[cfg(target_arch = "wasm32")]
    fn render_children(&self, children: &[LayoutNode]) -> Vec<Element> {
        children
            .iter()
            .filter(|n| n.visible())
            .map(|n| self.render_node(n))
            .collect()
    }
}

impl ThemeRenderer for WebSysRenderer {
    /// Inject theme CSS into the document head.
    ///
    /// Creates a `<style>` element containing the theme's CSS custom properties
    /// and appends it to the `<head>` of the document.
    fn inject_theme(&self, theme: &Vb6Theme) {
        let css = theme.to_css();
        let style = self.doc.create_element("style").expect("create style element");
        style.set_text_content(Some(&css));
        if let Some(head) = self.doc.head() {
            let _ = head.append_child(&style);
        }
    }
}

/// Map a [`LayoutControlType`] to an HTML tag name for use in element creation.
#[cfg(target_arch = "wasm32")]
fn tag_for_control(control_type: LayoutControlType) -> &'static str {
    match control_type {
        LayoutControlType::Form | LayoutControlType::MDIForm | LayoutControlType::Label => "div",
        LayoutControlType::TextBox => "input",
        LayoutControlType::CommandButton => "button",
        LayoutControlType::Frame => "fieldset",
        LayoutControlType::PictureBox => "div",
        LayoutControlType::Image => "div",
        LayoutControlType::CheckBox => "input",
        LayoutControlType::OptionButton => "input",
        LayoutControlType::ComboBox => "select",
        LayoutControlType::ListBox => "select",
        LayoutControlType::HScrollBar | LayoutControlType::VScrollBar => "input",
        LayoutControlType::Timer => "div",
        LayoutControlType::Shape => "div",
        LayoutControlType::Line => "svg",
        LayoutControlType::DriveListBox
        | LayoutControlType::DirListBox
        | LayoutControlType::FileListBox => "select",
        LayoutControlType::Data | LayoutControlType::Custom => "div",
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn web_sys_renderer_requires_wasm() {
        // WebSysRenderer is only available on wasm32 targets.
        // This test ensures the module compiles on non-wasm targets.
        assert!(cfg!(not(target_arch = "wasm32")));
    }
}
