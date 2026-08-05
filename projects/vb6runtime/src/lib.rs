//! vb6runtime: VB6 runtime library - value system, type conversions, and standard library
//!
//! This library provides the runtime execution infrastructure for VB6 programs:
//!
//! - [`VBType`] - the single source of truth for VB6 static types
//! - [`Value`] - runtime values with VB6-exact conversion semantics
//! - [`ArrayValue`] - VB6 arrays with arbitrary bounds
//! - [`VbError`] - runtime errors mirroring the `Err` object
//!
//! The standard library function implementations live under `library::` and are
//! added incrementally, using [`Value`] and [`VBType`] as their foundations.

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

pub mod array;
pub mod error;
pub mod types;
pub mod value;

pub use array::{ArrayDimension, ArrayValue};
pub use error::{VBError, VBResult};
pub use types::VBType;
pub use value::{Value, VbObject, CURRENCY_SCALE};

/// Library version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
