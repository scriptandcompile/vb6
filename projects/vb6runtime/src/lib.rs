//! vb6runtime: VB6 runtime library - value system, type conversions, and standard library
//!
//! This library provides the runtime execution infrastructure for VB6 programs:
//!
//! - [`VBType`] - the single source of truth for VB6 static types
//! - [`VBVariant`] - runtime values with VB6-exact conversion semantics
//! - [`ArrayValue`] - VB6 arrays with arbitrary bounds
//! - [`VBError`] - runtime errors mirroring the `Err` object
//!
//! The standard library function implementations live under `library::` and are
//! added incrementally, using [`VBVariant`] and [`VBType`] as their foundations.
//! Process-global runtime state (the environment snapshot, RNG seed) lives under
//! `state::` and is shared across functions and statements.

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

pub mod array;
pub mod library;
pub mod state;
pub mod value;

/// `jiff` civil date/time types re-exported for consumers that need to pass
/// them across crate boundaries (e.g. interpreter configuration).
pub use jiff::civil;

/// Runtime error types, re-exported from [`vb6core`].
pub use vb6core::error;

/// Type system types, re-exported from [`vb6core`].
pub use vb6core::types;

pub use array::{ArrayDimension, ArrayValue};
pub use error::{VBError, VBResult};
pub use types::VBType;
pub use value::{VBObject, VBVariant, CURRENCY_SCALE};

/// Library version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
