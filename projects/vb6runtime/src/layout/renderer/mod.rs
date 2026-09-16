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
//!
//! # Renderers
//!
//! - [`TauriRenderer`] — produces HTML fragment strings for Tauri webview injection.
//!   Supports optional `.vb6-app` scope wrapping via the `with_scope` flag.
//! - [`WebSysRenderer`] (behind `wasm` feature) — creates `web_sys::Element` objects
//!   for direct DOM manipulation in the browser.

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
}
