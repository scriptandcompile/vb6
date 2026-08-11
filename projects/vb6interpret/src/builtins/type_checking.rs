//! VB6 type-checking function registry.
//!
//! One [`Builtin`](super::Builtin) entry per type-checking function, each
//! wrapping the typed `vb6runtime::library::functions::type_checking`
//! implementation.

use super::{Builtin, Registry};
use crate::builtin;
use vb6core::error::VBResult;
use vb6runtime::library::functions::type_checking as typefn;
use vb6runtime::VBVariant;

/// Register the type-checking functions in `registry`.
pub(super) fn register(registry: &mut Registry) {
    registry.insert(builtin!("isarray", 1, 1, |args| {
        typefn::isarray::is_array(&args[0])
    }));
    registry.insert(builtin!("isdate", 1, 1, |args| {
        typefn::isdate::is_date(&args[0])
    }));
    registry.insert(builtin!("isempty", 1, 1, |args| {
        typefn::isempty::is_empty(&args[0])
    }));
    registry.insert(builtin!("iserror", 1, 1, |args| {
        typefn::iserror::is_error(&args[0])
    }));
    registry.insert(builtin!("ismissing", 0, 1, |args| {
        typefn::ismissing::is_missing(args.first())
    }));
    registry.insert(builtin!("isnull", 1, 1, |args| {
        typefn::isnull::is_null(&args[0])
    }));
    registry.insert(builtin!("isnumeric", 1, 1, |args| {
        typefn::isnumeric::is_numeric(&args[0])
    }));
    registry.insert(builtin!("isobject", 1, 1, |args| {
        typefn::isobject::is_object(&args[0])
    }));
}
