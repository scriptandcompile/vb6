//! VB6 standard library implementations.
//!
//! Each VB6 runtime function/statement lives under `functions`/`statements` and
//! is implemented against the [`Value`](crate::value::Value) system. Modules are
//! added as they are implemented.

pub mod functions;
