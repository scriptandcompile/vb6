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

#[cfg(target_arch = "wasm32")]
pub mod web_sys;
#[cfg(target_arch = "wasm32")]
pub use web_sys::WebSysRenderer;

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

    /// Apply a leaf-level diff change to the rendered output.
    ///
    /// The default implementation is a no-op. Renderers that support
    /// fine-grained incremental DOM updates (e.g. `WebSysRenderer`)
    /// override this to mutate existing DOM nodes in place.
    ///
    /// This method is primarily used by diff-aware rendering pipelines
    /// to update individual control properties (text, visibility, enabled
    /// state) without re-rendering the entire tree.
    ///
    /// # Arguments
    /// * `target` — The rendered output (HTML string or DOM element) to apply changes to.
    /// * `change` — The diff change describing what was modified.
    fn apply_diff(&self, _target: &Self::Output, _change: &DiffChange) {
        // No-op by default. Incremental renderers override this.
    }
}
