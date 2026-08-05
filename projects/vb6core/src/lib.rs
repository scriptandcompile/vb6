//! vb6core: shared types for the VB6 compiler and interpreter
//!
//! This library owns the foundational type and error model shared between
//! `vb6semantic` (static analysis) and `vb6runtime` (execution):
//!
//! - [`types::VBType`] - the single source of truth for VB6 static types
//! - [`types::TypeInfo`] - static metadata (ByRef, array bounds) around a type
//! - [`error::VBError`] - runtime errors mirroring the `Err` object
//!
//! Keeping these types here ensures semantic analysis, code generation, and the
//! interpreter all agree on the same type system.

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

pub mod error;
pub mod types;

pub use error::{VBError, VBResult};
pub use types::{ArrayBound, TypeInfo, VBType};

/// Library version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
