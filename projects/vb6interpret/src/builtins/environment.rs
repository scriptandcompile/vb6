//! VB6 environment function registry.
//!
//! One [`Builtin`](super::Builtin) entry per environment function, each
//! wrapping the typed `vb6runtime::library::functions::environment`
//! implementation.

use super::{Builtin, Registry};
use crate::builtin;
use vb6core::error::VBResult;
use vb6runtime::library::functions::environment::environ::environ;
use vb6runtime::library::functions::environment::environ_dollar::environ_dollar;
use vb6runtime::VBVariant;

/// Register the environment functions in `registry`.
pub(super) fn register(registry: &mut Registry) {
    registry.insert(builtin!("environ", 1, 1, |args| { environ(&args[0]) }));
    registry.insert(builtin!("environ$", 1, 1, |args| {
        environ_dollar(&args[0])
    }));
}
