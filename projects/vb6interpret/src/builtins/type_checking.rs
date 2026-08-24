//! VB6 type-checking function registry.
//!
//! One [`typed_builtin!`](crate::typed_builtin) entry per type-checking
//! function. Every parameter stays a `variant` kind on purpose: these are
//! *predicates* that observe the raw Variant (`IsNull` inspects Null without
//! raising 94, `IsEmpty` sees Empty, `IsError` sees CVErr values), so no
//! coercion may happen at the boundary.

use super::{Builtin, Registry};
use crate::typed_builtin;
use vb6runtime::library::type_checking as typefn;

/// Register the type-checking functions in `registry`.
pub(super) fn register(registry: &mut Registry) {
    registry.insert(typed_builtin!("isarray", 1, 1, (value: variant),
        typefn::isarray::is_array(value)));
    registry.insert(typed_builtin!("isdate", 1, 1, (value: variant),
        typefn::isdate::is_date(value)));
    registry.insert(typed_builtin!("isempty", 1, 1, (value: variant),
        typefn::isempty::is_empty(value)));
    registry.insert(typed_builtin!("iserror", 1, 1, (value: variant),
        typefn::iserror::is_error(value)));
    // IsMissing distinguishes an omitted argument from any present value
    // (including Null), so it keeps the raw `opt_variant` view.
    registry.insert(typed_builtin!("ismissing", 0, 1, (value: opt_variant),
        typefn::ismissing::is_missing(value)));
    registry.insert(typed_builtin!("isnull", 1, 1, (value: variant),
        typefn::isnull::is_null(value)));
    registry.insert(typed_builtin!("isnumeric", 1, 1, (value: variant),
        typefn::isnumeric::is_numeric(value)));
    registry.insert(typed_builtin!("isobject", 1, 1, (value: variant),
        typefn::isobject::is_object(value)));
}
