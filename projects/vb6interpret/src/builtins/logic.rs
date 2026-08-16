//! VB6 logic function registry.
//!
//! One [`Builtin`](super::Builtin) entry per logic function, each wrapping the
//! typed `vb6runtime::library::logic` implementation.

use super::{Builtin, Registry};
use crate::builtin;
use vb6core::error::VBResult;
use vb6runtime::library::logic as logicfn;
use vb6runtime::VBVariant;

/// Register the logic functions in `registry`.
pub(super) fn register(registry: &mut Registry) {
    registry.insert(builtin!("choose", 2, usize::MAX, |args| {
        logicfn::choose::choose(&args[0], &args[1..])
    }));
    registry.insert(builtin!("iif", 3, 3, |args| {
        logicfn::iif::iif(&args[0], &args[1], &args[2])
    }));
    registry.insert(builtin!("switch", 2, usize::MAX, |args| {
        logicfn::switch::switch(args)
    }));
}
