//! Platform-abstract rendering trait + platform-specific backends.
//!
//! The `Renderer` trait defines the interface for producing platform-specific output
//! from the layout model tree. Implemented by `TauriRenderer` for the Tauri webview.
//!
//! # Renderer Trait
//!
//! The trait provides methods for rendering different node types:
//! - [`Renderer::render_node`] — dispatches to the correct render method based on node type
//! - [`Renderer::render_leaf`] — renders a leaf control (Label, Button, TextBox, etc.)
//! - [`Renderer::render_container`] — renders a container control (Form, Frame, PictureBox)
//! - [`Renderer::render_children`] — renders visible child nodes of a container
//! - [`Renderer::style_attr`] — builds a CSS style attribute string
//! - [`Renderer::render_node_with_diff`] — incremental render using a diff tree
//! - [`Renderer::apply_diff`] — apply a leaf-level diff change to the output
//!
//! # Diff-Aware Rendering
//!
//! For incremental updates, renderers can override:
//! - [`render_node_with_diff`](Renderer::render_node_with_diff) — render only changed nodes
//! - [`apply_diff`](Renderer::apply_diff) — apply individual diff changes (default: no-op)
//!
//! The default implementations fall back to full re-render (for `render_node_with_diff`)
//! or do nothing (for `apply_diff`). Renderers that support incremental updates
//! (`TauriRenderer`, `WebSysRenderer`) override these methods.
//!
//! # Renderers
//!
//! - [`TauriRenderer`] — produces HTML fragment strings for Tauri webview injection.
//!   Supports optional `.vb6-app` scope wrapping via the `with_scope` flag.
//! - [`WebSysRenderer`] (behind `wasm` feature) — creates `web_sys::Element` objects
//!   for direct DOM manipulation in the browser.

use super::diff_tree::{DiffChange, DiffTree};
use super::model::{LayoutContainer, LayoutLeaf, LayoutNode, LayoutStyle};

pub mod tauri;
pub use tauri::TauriRenderer;

#[cfg(feature = "wasm")]
pub mod web_sys;
#[cfg(feature = "wasm")]
pub use web_sys::WebSysRenderer;

#[cfg(feature = "wasm")]
use super::model::NodeId;
#[cfg(feature = "wasm")]
use web_sys::Element;

/// Trait for converting a [`LayoutNode`] tree into platform-specific output.
///
/// Implement this once per platform:
/// - `TauriRenderer` produces HTML fragment strings for Tauri webview injection.
/// - `WebSysRenderer` (behind `wasm` feature) creates `web_sys::Element` objects.
///
/// The default implementation of `style_attr` handles visibility and enabled overrides.
pub trait Renderer {
    /// The output type produced by this renderer.
    type Output;

    /// Render a complete [`LayoutNode`] (top-level form).
    fn render_node(&self, node: &LayoutNode) -> Self::Output;

    /// Render a leaf control.
    fn render_leaf(&self, leaf: &LayoutLeaf) -> Self::Output;

    /// Render a container control with children.
    fn render_container(&self, container: &LayoutContainer) -> Self::Output;

    /// Render the visible child nodes of a container.
    fn render_children(&self, children: &[LayoutNode]) -> Vec<Self::Output>;

    /// Build a CSS style attribute string for a node's style plus runtime state.
    ///
    /// Combines the computed [`LayoutStyle`] CSS with visibility (`visibility: hidden`)
    /// and enabled (`opacity: 0.5`) runtime overrides.
    fn style_attr(&self, style: &LayoutStyle, visible: bool, enabled: bool) -> String {
        let mut css = super::css::style_to_css(style);
        if !visible {
            if !css.is_empty() {
                css.push_str("; visibility: hidden");
            } else {
                css.push_str("visibility: hidden");
            }
        }
        if !enabled {
            if !css.is_empty() {
                css.push_str("; opacity: 0.5");
            } else {
                css.push_str("opacity: 0.5");
            }
        }
        css
    }

    /// Render a node tree, using the diff to skip unchanged subtrees.
    ///
    /// The default implementation falls back to a full re-render via
    /// [`render_node`](Renderer::render_node). Renderers that support
    /// incremental updates (e.g. `TauriRenderer`) override this method
    /// to produce partial output for changed nodes only.
    ///
    /// If `diff` is `None` or empty, behaves identically to `render_node`.
    fn render_node_with_diff(&self, node: &LayoutNode, diff: Option<&DiffTree>) -> Self::Output {
        match diff {
            Some(d) if !d.is_empty() => self.render_node(node),
            _ => self.render_node(node),
        }
    }

    /// Apply a leaf-level diff change and produce an HTML fragment.
    ///
    /// The default implementation is a no-op returning an empty string.
    /// Renderers that support incremental updates override this to produce
    /// a minimal HTML fragment for the changed node instead of the full
    /// re-render output from [`render_node`](Renderer::render_node).
    ///
    /// This method is used by diff-aware rendering pipelines to update
    /// individual control properties (text, visibility, enabled state)
    /// with the smallest possible output.
    ///
    /// # Arguments
    /// * `node` — The current layout node that was changed.
    /// * `change` — The diff change describing what was modified.
    ///
    /// # Returns
    /// An HTML fragment string for the changed node, or an empty string
    /// if the renderer cannot produce an incremental update for this change type.
    fn apply_diff(&self, _node: &LayoutNode, _change: &DiffChange) -> String {
        // No-op by default. Incremental renderers override this.
        String::new()
    }

    /// Render a node tree using the diff to skip unchanged subtrees.
    ///
    /// The default implementation falls back to a full re-render via
    /// [`render_node`](Renderer::render_node). `WebSysRenderer` overrides this
    /// method to perform incremental DOM patching — updating only the
    /// properties that changed (text content, visibility, enabled state)
    /// and re-creating only inserted or removed elements.
    ///
    /// # DOM Node Cache
    ///
    /// When the diff contains `DiffKind::Inserted`, this method creates new
    /// DOM elements and caches them by node ID in `dom_nodes`. The cache
    /// allows subsequent `VisibilityChanged` and `ValueChanged` updates
    /// to find existing elements for in-place mutation.
    ///
    /// # Arguments
    /// * `node` — The layout node to render.
    /// * `diff` — Optional diff tree describing changes since last render.
    /// * `parent` — Optional parent DOM element for appending inserted children.
    /// * `dom_nodes` — Mutable cache of node ID → DOM element, maintained across renders.
    ///
    /// # Returns
    /// The rendered `Element`. For unchanged subtrees (`DiffKind::Same`),
    /// returns the cached element. For changed subtrees, returns the updated
    /// or newly created element.
    #[cfg(feature = "wasm")]
    fn render_node_with_diff_dom(
        &self,
        _node: &LayoutNode,
        _diff: Option<&DiffTree>,
        _parent: Option<&Element>,
        _dom_nodes: &mut std::cell::RefCell<std::collections::HashMap<NodeId, Element>>,
    ) -> Element {
        // Default: not implemented. Override in WebSysRenderer.
        unimplemented!("render_node_with_diff_dom is only implemented by WebSysRenderer")
    }

    /// Apply a diff change directly to the DOM.
    ///
    /// Called by the diffing pipeline to update individual DOM elements
    /// without re-rendering the entire subtree. Used for leaf-level changes
    /// like text updates, visibility toggles, and enabled state changes.
    ///
    /// # Arguments
    /// * `node` — The layout node that was changed.
    /// * `change` — The diff change describing what was modified.
    /// * `parent` — The parent DOM element for inserting new nodes.
    /// * `dom_nodes` — Mutable cache of node ID → DOM element, maintained across renders.
    #[cfg(feature = "wasm")]
    fn apply_diff_dom(
        &self,
        _node: &LayoutNode,
        _change: &DiffChange,
        _parent: Option<&Element>,
        _dom_nodes: &mut std::cell::RefCell<std::collections::HashMap<NodeId, Element>>,
    ) -> Option<Element> {
        // Default: no-op, returns None to signal full re-render.
        None
    }
}
