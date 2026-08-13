//! VB6 financial function registry.
//!
//! One [`Builtin`](super::Builtin) entry per financial function, each wrapping
//! the typed `vb6runtime::library::functions::financial` implementation.

use super::{Builtin, Registry};
use crate::builtin;
use vb6core::error::VBResult;
use vb6runtime::library::functions::financial as finfn;
use vb6runtime::VBVariant;

/// Register the financial functions in `registry`.
pub(super) fn register(registry: &mut Registry) {
    registry.insert(builtin!("ddb", 4, 5, |args| {
        let factor = if args.len() > 4 { Some(&args[4]) } else { None };
        finfn::ddb::ddb(&args[0], &args[1], &args[2], &args[3], factor)
    }));
}
