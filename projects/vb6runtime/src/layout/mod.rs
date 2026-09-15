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
