//! VB6 conversion function registry.
//!
//! One [`Builtin`](super::Builtin) entry per conversion function, each wrapping
//! the typed `vb6runtime::library::conversion` implementation.

use super::{Builtin, Registry};
use crate::builtin;
use vb6core::error::VBResult;
use vb6runtime::library::conversion as convfn;
use vb6runtime::VBVariant;

/// Register the conversion functions in `registry`.
pub(super) fn register(registry: &mut Registry) {
    registry.insert(builtin!("cverr", 1, 1, |args| {
        convfn::cverr::cverr(&args[0])
    }));
    registry.insert(builtin!("hex", 1, 1, |args| { convfn::hex::hex(&args[0]) }));
    registry.insert(builtin!("hex$", 1, 1, |args| {
        convfn::hex_dollar::hex_dollar(&args[0]).map(VBVariant::from)
    }));
    registry.insert(builtin!("oct", 1, 1, |args| { convfn::oct::oct(&args[0]) }));
    registry.insert(builtin!("oct$", 1, 1, |args| {
        convfn::oct_dollar::oct_dollar(&args[0]).map(VBVariant::from)
    }));
    registry.insert(builtin!("vartype", 1, 1, |args| {
        convfn::vartype::var_type(&args[0])
    }));
}
