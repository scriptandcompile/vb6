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

use model::style::LayoutStyle;

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
