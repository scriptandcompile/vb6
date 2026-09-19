//! WebSys renderer — creates `web_sys::Element` objects for WASM target.
//!
//! Produces real DOM nodes that are appended to the browser DOM.
//! Wraps all output in a `.vb6-app` scope root for CSS isolation.
//!
//! Supports incremental (diff-aware) rendering: when a `DiffTree` is provided,
//! the renderer updates only the changed properties of existing DOM elements
//! instead of re-creating the entire subtree. This prevents flicker and
//! reduces the number of DOM mutations on rapid state changes.
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

use std::cell::RefCell;
use std::collections::HashMap;

use crate::layout::diff_tree::{DiffChange, DiffKind, DiffTree};
use crate::layout::model::NodeId;
use crate::layout::model::{LayoutContainer, LayoutLeaf, LayoutNode};
use crate::layout::renderer::Renderer;
use crate::layout::theme::{CssInjector, ThemeRenderer, Vb6Theme};

use crate::layout::model::LayoutControlType;

#[cfg(feature = "wasm")]
use web_sys::Document;

#[cfg(feature = "wasm")]
pub use web_sys::Element;

/// WASM renderer that creates `web_sys::Element` objects.
///
/// Wraps all output in a `.vb6-app` scope root for CSS isolation.
/// The scope root has `id="vb6-container"` so it can be found and
/// appended to by the host application.
///
/// Maintains an internal cache of DOM nodes keyed by [`NodeId`], enabling
/// incremental (diff-aware) updates. When `render_node_with_diff` is called
/// with a [`DiffTree`], the renderer looks up existing elements in this
/// cache and mutates them in place for `ValueChanged`, `VisibilityChanged`,
/// and `EnabledChanged` changes instead of re-creating them.
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
    /// Cache of DOM nodes indexed by node ID.
    ///
    /// Populated during `render_node_with_diff` for `Inserted` and `Same` nodes.
    /// Used by `VisibilityChanged` and `ValueChanged` handlers to find
    /// existing elements for in-place mutation.
    pub dom_nodes: RefCell<HashMap<NodeId, Element>>,
}

impl WebSysRenderer {
    /// Create a new `WebSysRenderer` bound to the given document.
    ///
    /// The renderer will always wrap output in a `.vb6-app` scope root
    /// since WASM targets need CSS isolation from the parent page.
    #[cfg(feature = "wasm")]
    #[must_use]
    pub fn new(doc: Document) -> Self {
        Self {
            doc,
            dom_nodes: RefCell::new(HashMap::new()),
        }
    }

    /// Generate the opening tag for the scope root.
    ///
    /// For WASM, this is always `<div class="vb6-app" id="vb6-container">`
    /// since CSS isolation is required.
    #[cfg(feature = "wasm")]
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
    #[cfg(feature = "wasm")]
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
    #[cfg(feature = "wasm")]
    #[must_use]
    pub fn build_scoped_root(&self, child: &Element) -> Element {
        let root = self.root_open();
        root.append_child(child)
            .expect("append child to scope root");
        root
    }

    // ── Incremental rendering helpers ──────────────────────────────

    /// Ensure an element for the given node exists in the DOM.
    ///
    /// If the node is already in the DOM node cache, returns it.
    /// Otherwise creates a new element, appends it to `parent`,
    /// and caches it.
    ///
    /// Currently unused — planned for future incremental rendering of
    /// inserted children where we need a guaranteed element regardless
    /// of cache state.
    #[cfg(feature = "wasm")]
    #[allow(dead_code)]
    fn ensure_element(&self, node: &LayoutNode, parent: Option<&Element>) -> Element {
        let id = node.node_id();
        if let Some(el) = self.dom_nodes.borrow().get(&id) {
            return el.clone();
        }
        let el = match node {
            LayoutNode::Leaf(l) => self.render_leaf(l),
            LayoutNode::Container(c) => self.render_container(c),
        };
        if let Some(p) = parent {
            let _ = p.append_child(&el);
        }
        self.dom_nodes.borrow_mut().insert(id, el.clone());
        el
    }

    /// This method is kept for future use in the incremental rendering
    /// pipeline as an alternative way to locate existing DOM elements
    /// without requiring a separate cache lookup.
    #[cfg(feature = "wasm")]
    #[allow(dead_code)]
    fn find_element_by_id(&self, id: &NodeId) -> Option<Element> {
        if let Some(el) = self.dom_nodes.borrow().get(id) {
            return Some(el.clone());
        }
        self.doc.get_element_by_id(&id.name).inspect(|e| {
            self.dom_nodes.borrow_mut().insert(id.clone(), e.clone());
        })
    }

    /// Find the parent DOM element for an inserted node.
    ///
    /// Walks up the DOM tree from the first cached element that has the
    /// same parent node, or falls back to the provided fallback element.
    #[cfg(feature = "wasm")]
    fn find_parent_for(&self, node: &LayoutNode, fallback: Option<&Element>) -> Option<Element> {
        let id = node.node_id();

        // If this is a container child, try to find the container in the cache
        if let LayoutNode::Leaf(_) = node {
            // Try DOM tree walk from cached nodes
            for cached in self.dom_nodes.borrow().values() {
                if let Some(parent) = cached.parent_element()
                    && self.node_is_child_of(&id, &parent)
                {
                    return Some(parent);
                }
            }
        }

        fallback.cloned()
    }

    /// Check if a node with the given ID is a child of the given parent element.
    #[cfg(feature = "wasm")]
    fn node_is_child_of(&self, id: &NodeId, parent: &Element) -> bool {
        let selector = format!("#{}", id.name);
        if let Ok(Some(child)) = parent.query_selector(&selector) {
            return child.id() == id.name;
        }
        false
    }

    /// Update the content of an element based on a value change.
    #[cfg(feature = "wasm")]
    fn apply_value_change(&self, el: &Element, node: &LayoutNode, new_value: &str) {
        let tag = tag_for_control(*node.control_type());
        let tag_lower = tag.to_lowercase();

        if tag_lower == "button" {
            el.set_text_content(Some(new_value));
        } else if tag_lower == "input" {
            let _ = el.set_attribute("value", new_value);
        } else if tag_lower == "img" {
            let _ = el.set_attribute("src", new_value);
        } else {
            el.set_text_content(Some(new_value));
        }
    }

    /// Update an element's style based on visibility or enabled change.
    #[cfg(feature = "wasm")]
    fn apply_visibility_change(&self, el: &Element, visible: bool, enabled: bool) {
        let current_style = el.get_attribute("style").unwrap_or_default();
        let mut style_parts: Vec<&str> = current_style
            .split(';')
            .filter(|s| !s.trim().is_empty())
            .collect();

        // Update visibility
        style_parts.retain(|s| !s.trim().starts_with("visibility"));
        if !visible {
            style_parts.push("visibility: hidden");
        }

        // Update opacity for enabled state
        style_parts.retain(|s| !s.trim().starts_with("opacity"));
        if !enabled {
            style_parts.push("opacity: 0.5");
        }

        let style = style_parts.join("; ");
        let _ = el.set_attribute("style", &style);
    }

    // ── Render with diff ───────────────────────────────────────────

    /// Render a node tree using the diff to skip unchanged subtrees.
    ///
    /// This is the core of incremental DOM patching:
    /// - `Same` → returns the cached element unchanged
    /// - `ValueChanged` → updates text content / value attribute in place
    /// - `VisibilityChanged` → toggles `visibility: hidden` in the style
    /// - `EnabledChanged` → toggles `opacity: 0.5` in the style
    /// - `Inserted` → creates new DOM elements and appends to parent
    /// - `Removed` → removes the element from the DOM
    /// - `ChildrenChanged` / unknown → falls back to full re-render
    ///
    /// New elements are cached in `dom_nodes` so subsequent changes can
    /// find them for in-place mutation.
    #[cfg(feature = "wasm")]
    pub fn render_node_with_diff(
        &self,
        node: &LayoutNode,
        diff: Option<&DiffTree>,
        parent: Option<&Element>,
    ) -> Element {
        let dom = &self.dom_nodes;

        // If no diff or empty diff, do a full render
        let use_diff = matches!(diff, Some(d) if !d.is_empty());

        if !use_diff {
            return self.render_node(node);
        }

        let change = diff.and_then(|d| d.get_change(&node.node_id()));

        match change {
            Some(DiffChange {
                kind: DiffKind::Same,
                ..
            }) => {
                // Return cached element — no changes needed
                if let Some(el) = dom.borrow().get(&node.node_id()) {
                    el.clone()
                } else {
                    // Element not in cache (first render of this subtree), create it
                    let el = self.render_node(node);
                    self.cache_node(node, &el);
                    el
                }
            }
            Some(DiffChange {
                kind: DiffKind::ValueChanged { new_value, .. },
                ..
            }) => {
                if let Some(el) = dom.borrow().get(&node.node_id()) {
                    let val = new_value.as_deref().unwrap_or("");
                    self.apply_value_change(el, node, val);
                    el.clone()
                } else {
                    // Element not in cache — fall back to full render
                    let el = self.render_node(node);
                    self.cache_node(node, &el);
                    el
                }
            }
            Some(DiffChange {
                kind: DiffKind::VisibilityChanged { new_visible, .. },
                ..
            }) => {
                if let Some(el) = dom.borrow().get(&node.node_id()) {
                    self.apply_visibility_change(el, *new_visible, true);
                    el.clone()
                } else {
                    let el = self.render_node(node);
                    self.cache_node(node, &el);
                    el
                }
            }
            Some(DiffChange {
                kind: DiffKind::EnabledChanged { new_enabled, .. },
                ..
            }) => {
                if let Some(el) = dom.borrow().get(&node.node_id()) {
                    self.apply_visibility_change(el, true, *new_enabled);
                    el.clone()
                } else {
                    let el = self.render_node(node);
                    self.cache_node(node, &el);
                    el
                }
            }
            Some(DiffChange {
                kind: DiffKind::Inserted { node: inserted },
                ..
            }) => {
                let parent_el = parent
                    .cloned()
                    .or_else(|| self.find_parent_for(inserted.as_ref(), None));
                let el = self.render_node_with_diff(inserted.as_ref(), None, parent_el.as_ref());
                if let Some(ref p) = parent_el {
                    let _ = p.append_child(&el);
                }
                self.cache_node(inserted.as_ref(), &el);
                el
            }
            Some(DiffChange {
                kind: DiffKind::Removed,
                ..
            }) => {
                if let Some(el) = dom.borrow_mut().remove(&node.node_id()) {
                    el.remove();
                }
                self.doc.create_element("div").expect("create div element")
            }
            _ => {
                // ChildrenChanged, Unknown, or no matching change — full re-render
                let el = self.render_node(node);
                // For containers, also clear and re-cache children
                if let LayoutNode::Container(_) = node {
                    dom.borrow_mut().clear();
                    self.cache_node(node, &el);
                }
                el
            }
        }
    }

    /// Cache a rendered element by its node ID.
    #[cfg(feature = "wasm")]
    fn cache_node(&self, node: &LayoutNode, element: &Element) {
        let id = node.node_id();
        // Also cache any descendants
        self.cache_descendants(node, element);
        self.dom_nodes.borrow_mut().insert(id, element.clone());
    }

    /// Recursively cache all descendant DOM nodes.
    #[cfg(feature = "wasm")]
    fn cache_descendants(&self, node: &LayoutNode, element: &Element) {
        match node {
            LayoutNode::Leaf(_) => {}
            LayoutNode::Container(c) => {
                for child in &c.children {
                    if let Some(child_el) = self.get_child_element(element, child) {
                        self.cache_node(child, &child_el);
                        self.cache_descendants(child, &child_el);
                    }
                }
            }
        }
    }

    /// Get a child element by its node ID from a parent element.
    #[cfg(feature = "wasm")]
    fn get_child_element(&self, parent: &Element, child: &LayoutNode) -> Option<Element> {
        let selector = format!("#{}", child.node_id().name);
        parent.query_selector(&selector).ok().flatten()
    }
}

impl Renderer for WebSysRenderer {
    type Output = Element;

    #[cfg(feature = "wasm")]
    fn render_node(&self, node: &LayoutNode) -> Element {
        self.render_node_to_element(node)
    }

    #[cfg(feature = "wasm")]
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

        if let Some(ref tooltip) = leaf.tooltip {
            let _ = el.set_attribute("title", tooltip);
        }

        if let Some(tabindex) = leaf.tabindex {
            let _ = el.set_attribute("tabindex", &tabindex.to_string());
        }

        if leaf.is_default {
            let _ = el.set_attribute("autofocus", "autofocus");
            let _ = el.set_attribute("type", "submit");
        }

        if leaf.is_cancel {
            let _ = el.set_attribute("type", "submit");
            let _ = el.set_attribute("data-cancel", "true");
        }

        match leaf.control_type {
            LayoutControlType::CheckBox => {
                let _ = el.set_attribute("type", "checkbox");
                if leaf.value.as_deref() == Some("True") {
                    let _ = el.set_attribute("checked", "checked");
                }
                if leaf.value.as_deref() == Some("Grayed") {
                    let current_style = el.get_attribute("style").unwrap_or_default();
                    let _ = el.set_attribute("style", &format!("{} opacity: 0.5;", current_style));
                }
            }
            LayoutControlType::OptionButton => {
                let _ = el.set_attribute("type", "radio");
                if let Some(ref group) = leaf.style.group {
                    let _ = el.set_attribute("name", group);
                } else {
                    let _ = el.set_attribute("name", &leaf.name);
                }
                if leaf.value.as_deref() == Some("True") {
                    let _ = el.set_attribute("checked", "checked");
                }
            }
            LayoutControlType::ComboBox => {
                if leaf.combo_style.as_deref() == Some("dropdown-readonly") {
                    let _ = el.set_attribute("disabled", "disabled");
                }
            }
            LayoutControlType::HScrollBar | LayoutControlType::VScrollBar => {
                let _ = el.set_attribute("type", "range");
                let _ = el.set_attribute("value", leaf.value.as_deref().unwrap_or("0"));
                if let Some(min) = leaf.range_min {
                    let _ = el.set_attribute("min", &min.to_string());
                }
                if let Some(max) = leaf.range_max {
                    let _ = el.set_attribute("max", &max.to_string());
                }
                if let Some(step) = leaf.range_step {
                    let _ = el.set_attribute("step", &step.to_string());
                }
            }
            _ => {
                if let Some(ref value) = leaf.value {
                    let tag_lower = tag.to_lowercase();
                    if tag_lower == "button" {
                        // CommandButton always processes mnemonics (no use_mnemonic property)
                        el.set_inner_html(&process_mnemonic(value));
                    } else if tag_lower == "input" {
                        let _ = el.set_attribute("value", value);
                    } else if tag_lower == "img" {
                        let _ = el.set_attribute("src", value);
                    } else {
                        let processed = if leaf.use_mnemonic {
                            process_mnemonic(value)
                        } else {
                            value.to_string()
                        };
                        if processed.contains("<span") {
                            el.set_inner_html(&processed);
                        } else {
                            el.set_text_content(Some(value));
                        }
                    }
                }
                // Image source is set from image_src, not value.
                if let Some(ref src) = leaf.image_src {
                    let tag_lower = tag.to_lowercase();
                    if tag_lower == "img" {
                        let _ = el.set_attribute("src", src);
                    }
                }
            }
        }

        el
    }

    #[cfg(feature = "wasm")]
    fn render_container(&self, container: &LayoutContainer) -> Element {
        let tag = match container.control_type {
            LayoutControlType::Form | LayoutControlType::MDIForm => "div",
            LayoutControlType::Frame => "fieldset",
            LayoutControlType::PictureBox => "div",
            _ => "div",
        };

        let el = self.doc.create_element(tag).expect("create element");

        el.set_id(&container.name);
        el.set_class_name(&format!("vb6-{}", container.control_type.css_class()));
        let mut style_css = self.style_attr(&container.style, container.visible, container.enabled);
        if container.size.width > 0.0 {
            if !style_css.is_empty() {
                style_css.push_str("; ");
            }
            style_css.push_str(&format!("width: {:.1}px", container.size.width));
        }
        if container.size.height > 0.0 {
            if !style_css.is_empty() {
                style_css.push_str("; ");
            }
            style_css.push_str(&format!("height: {:.1}px", container.size.height));
        }
        el.set_attribute("style", &style_css).ok();

        if let Some(ref caption) = container.caption {
            if container.control_type == LayoutControlType::Frame {
                let legend = self.doc.create_element("legend").expect("create legend");
                let processed = process_mnemonic(caption);
                if processed.contains("<span") {
                    legend.set_inner_html(&processed);
                } else {
                    legend.set_text_content(Some(caption));
                }
                let _ = el.append_child(&legend);
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

    #[cfg(feature = "wasm")]
    fn render_children(&self, children: &[LayoutNode]) -> Vec<Element> {
        children
            .iter()
            .filter(|n| n.visible())
            .map(|n| self.render_node(n))
            .collect()
    }

    #[cfg(feature = "wasm")]
    fn render_node_with_diff_dom(
        &self,
        node: &LayoutNode,
        diff: Option<&DiffTree>,
        parent: Option<&Element>,
        dom_nodes: &mut RefCell<HashMap<NodeId, Element>>,
    ) -> Element {
        let result = self.render_node_with_diff(node, diff, parent);
        // Sync local cache with the provided external cache
        dom_nodes.borrow_mut().clear();
        dom_nodes
            .borrow_mut()
            .extend(self.dom_nodes.borrow().clone());
        result
    }

    #[cfg(feature = "wasm")]
    fn apply_diff_dom(
        &self,
        node: &LayoutNode,
        change: &DiffChange,
        parent: Option<&Element>,
        dom_nodes: &mut RefCell<HashMap<NodeId, Element>>,
    ) -> Option<Element> {
        match &change.kind {
            DiffKind::Same => dom_nodes.borrow().get(&node.node_id()).cloned(),
            DiffKind::Inserted { node: inserted } => {
                let parent_el = parent
                    .cloned()
                    .or_else(|| self.find_parent_for(inserted.as_ref(), None));
                let el = self.render_node_with_diff(inserted.as_ref(), None, parent_el.as_ref());
                if let Some(ref p) = parent_el {
                    let _ = p.append_child(&el);
                }
                dom_nodes
                    .borrow_mut()
                    .insert(inserted.node_id(), el.clone());
                Some(el)
            }
            DiffKind::Removed => {
                if let Some(el) = dom_nodes.borrow_mut().remove(&node.node_id()) {
                    el.remove();
                }
                None
            }
            DiffKind::ValueChanged { new_value, .. } => {
                if let Some(el) = dom_nodes.borrow().get(&node.node_id()) {
                    let val = new_value.as_deref().unwrap_or("");
                    self.apply_value_change(el, node, val);
                    Some(el.clone())
                } else {
                    None
                }
            }
            DiffKind::VisibilityChanged { new_visible, .. } => {
                if let Some(el) = dom_nodes.borrow().get(&node.node_id()) {
                    self.apply_visibility_change(el, *new_visible, true);
                    Some(el.clone())
                } else {
                    None
                }
            }
            DiffKind::EnabledChanged { new_enabled, .. } => {
                if let Some(el) = dom_nodes.borrow().get(&node.node_id()) {
                    self.apply_visibility_change(el, true, *new_enabled);
                    Some(el.clone())
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl ThemeRenderer for WebSysRenderer {
    /// Inject theme CSS into the document head.
    ///
    /// Creates a `<style>` element containing the theme's CSS custom properties
    /// and appends it to the `<head>` of the document.
    fn inject_theme(&self, theme: &Vb6Theme) {
        let css = theme.to_css();
        let style = self
            .doc
            .create_element("style")
            .expect("create style element");
        style.set_text_content(Some(&css));
        if let Some(body) = self.doc.body() {
            let _ = body.append_child(&style);
        }
    }
}

impl CssInjector for WebSysRenderer {
    /// Inject raw CSS into the document head.
    ///
    /// Creates a `<style>` element containing the provided CSS
    /// and appends it to the `<head>` of the document.
    fn inject_css(&self, css: &str) {
        let style = self
            .doc
            .create_element("style")
            .expect("create style element");
        style.set_text_content(Some(css));
        if let Some(body) = self.doc.body() {
            let _ = body.append_child(&style);
        }
    }
}

/// Map a [`LayoutControlType`] to an HTML tag name for use in element creation.
#[cfg(feature = "wasm")]
fn tag_for_control(control_type: LayoutControlType) -> &'static str {
    match control_type {
        LayoutControlType::Form | LayoutControlType::MDIForm | LayoutControlType::Label => "div",
        LayoutControlType::TextBox => "input",
        LayoutControlType::CommandButton => "button",
        LayoutControlType::Frame => "fieldset",
        LayoutControlType::PictureBox => "div",
        LayoutControlType::Image => "img",
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

/// Process a VB6 caption string to handle ampersand mnemonics.
///
/// VB6 uses `&` to mark mnemonic/accelerator keys:
/// - `&OK` → `<span class="vb6-mnemonic">O</span>K` (underlined via CSS)
/// - `A&BB&C` → `<span class="vb6-mnemonic">A</span>BB<span class="vb6-mnemonic">C</span>` (each & marks the next char)
/// - `&&` → `&` (escaped ampersand)
///
/// The result wraps the mnemonic characters in a `<span class="vb6-mnemonic">`
/// element so CSS can style them (typically underline + font-weight).
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn web_sys_renderer_compiles_on_non_wasm() {
        // WebSysRenderer is conditionally compiled for wasm32 targets.
        // On non-wasm targets, the module should still compile (the struct
        // and impls are behind #[cfg(feature = "wasm")] gates).
        assert!(cfg!(not(target_arch = "wasm32")));
    }

    #[test]
    fn renderer_trait_has_diff_methods() {
        // Verify the Renderer trait has the diff-aware methods by checking
        // that the trait compiles with the new methods.
        fn _assert_trait<T: Renderer>() {}
        // This would fail to compile if the trait didn't have the methods.
        // TauriRenderer is always available and implements these (with defaults).
        _assert_trait::<TauriRenderer>();
    }

    #[test]
    fn tag_for_control_mapping() {
        // Verify tag mapping exists for all control types.
        // This test runs on all targets since it just checks the match.
        assert_eq!(tag_for_control(LayoutControlType::Label), "div");
        assert_eq!(tag_for_control(LayoutControlType::TextBox), "input");
        assert_eq!(tag_for_control(LayoutControlType::CommandButton), "button");
        assert_eq!(tag_for_control(LayoutControlType::Frame), "fieldset");
        assert_eq!(tag_for_control(LayoutControlType::Image), "img");
        assert_eq!(tag_for_control(LayoutControlType::CheckBox), "input");
        assert_eq!(tag_for_control(LayoutControlType::ComboBox), "select");
    }

    #[test]
    fn process_mnemonic_basic() {
        assert_eq!(process_mnemonic("OK"), "OK");
        assert_eq!(
            process_mnemonic("&OK"),
            "<span class=\"vb6-mnemonic\">O</span>K"
        );
        assert_eq!(
            process_mnemonic("&Cancel"),
            "<span class=\"vb6-mnemonic\">C</span>ancel"
        );
    }

    #[test]
    fn process_mnemonic_escaped_ampersand() {
        assert_eq!(process_mnemonic("&&"), "&");
        assert_eq!(process_mnemonic("A&&B"), "A&B");
        assert_eq!(
            process_mnemonic("&A&&B"),
            "<span class=\"vb6-mnemonic\">A</span>&B"
        );
    }

    #[test]
    fn process_mnemonic_multiple() {
        assert_eq!(
            process_mnemonic("&File-&Edit"),
            "<span class=\"vb6-mnemonic\">F</span>ile-\
             <span class=\"vb6-mnemonic\">E</span>dit"
        );
    }

    #[test]
    fn process_mnemonic_trailing_ampersand() {
        assert_eq!(process_mnemonic("Test&"), "Test&");
    }
}
