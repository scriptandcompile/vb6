//! VB6 conversion function registry.
//!
//! One registry entry per conversion function, declared with the declarative
//! [`typed_builtin!`](crate::typed_builtin) spec. Every function here takes a
//! genuinely-Variant argument (`variant` kind): `hex`/`oct` format with
//! type-aware bit widths (Integer → u16, Long → u32), `vartype` observes the
//! raw variant, and `cverr`/`hex$`/`oct$` own their Null rejection in the
//! runtime body.

use super::{Builtin, Registry};
use crate::typed_builtin;
use vb6runtime::VBVariant;
use vb6runtime::library::conversion as convfn;

/// Register the conversion functions in `registry`.
pub(super) fn register(registry: &mut Registry) {
    registry.insert(typed_builtin!("cverr", 1, 1, (errornumber: variant),
        convfn::cverr::cverr(errornumber)));
    registry.insert(typed_builtin!("hex", 1, 1, (number: variant),
        convfn::hex::hex(number)));
    registry.insert(typed_builtin!("hex$", 1, 1, (number: variant),
        convfn::hex_dollar::hex_dollar(number).map(VBVariant::from)));
    registry.insert(typed_builtin!("oct", 1, 1, (number: variant),
        convfn::oct::oct(number)));
    registry.insert(typed_builtin!("oct$", 1, 1, (number: variant),
        convfn::oct_dollar::oct_dollar(number).map(VBVariant::from)));
    registry.insert(typed_builtin!("vartype", 1, 1, (value: variant),
        convfn::vartype::var_type(value)));
}
