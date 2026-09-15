//! Tree walker: converts a parsed `FormRoot` into a `LayoutNode` tree.
//!
//! Recursively processes controls, rejects unsupported types, and builds
//! style and position data via the converter pipeline.
