//! Core types for the layout model.
//!
//! This module defines the fundamental types used throughout the layout engine:
//! - [`types`] — `LayoutControlType`, `LayoutPosition`, `LayoutSize`, `NodeId`
//! - [`container`] — `LayoutNode`, `LayoutContainer`, `LayoutLeaf`
//! - [`style`] — `LayoutStyle` (CSS-compatible style properties)

/// Container node hierarchy: `LayoutContainer`, `LayoutLeaf`, `LayoutNode`.
pub mod container;
/// `LayoutStyle` struct — CSS-compatible style properties.
pub mod style;
/// Shared enums and structs: `LayoutControlType`, `LayoutPosition`, `LayoutSize`, `NodeId`.
pub mod types;
