//! VB6 statement implementations.
//!
//! Each VB6 runtime statement lives under its own submodule and is implemented
//! against the [`VBVariant`](crate::value::VBVariant) system. Documentation-only
//! stubs stay as plain modules until they are implemented.

pub mod file_operations;
pub mod filesystem;
pub mod runtime_state;
pub mod string_manipulation;
pub mod system_interaction;
