//! VB6 standard library implementations.
//!
//! Each VB6 runtime function/statement lives under its category submodule
//! (string, math, file, datetime, environment, ...) and is implemented
//! against the [`VBVariant`](crate::value::VBVariant) system. Modules are
//! added as they are implemented.

pub mod arrays;
pub mod conversion;
pub mod datetime;
pub mod environment;
pub mod file;
pub mod financial;
pub mod graphics;
pub mod interaction;
pub mod logic;
pub mod math;
pub mod objects;
pub mod resources;
pub mod string;
pub mod type_checking;
