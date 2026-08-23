//! VB6 object function registry.
//!
//! One [`Builtin`](super::Builtin) entry per object function, each wrapping
//! the typed `vb6runtime::library::objects` implementation.

use super::{Builtin, Registry};
use crate::builtin;
use vb6core::error::VBResult;
use vb6runtime::library::objects as objfn;
use vb6runtime::VBVariant;

/// Register the object functions in `registry`.
pub(super) fn register(registry: &mut Registry) {
    registry.insert(builtin!("typename", 1, 1, |args| {
        objfn::typename::type_name(&args[0])
    }));
}
