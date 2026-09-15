//! Core types for the layout model.
//!
//! This module defines the fundamental types used throughout the layout engine:
//! - [`types`] — `LayoutControlType`, `LayoutPosition`, `LayoutSize`, `NodeId`
//! - [`container`] — `LayoutNode`, `LayoutContainer`, `LayoutLeaf`
//! - [`style`] — `LayoutStyle` (CSS-compatible style properties)

pub mod container;
pub mod style;
pub mod types;

pub use container::{LayoutContainer, LayoutLeaf, LayoutNode};
pub use style::{CssColor, LayoutStyle};
pub use types::{LayoutControlType, LayoutPosition, LayoutSize, NodeId};
