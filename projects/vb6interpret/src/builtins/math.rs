//! VB6 math function registry.
//!
//! One registry entry per math function, declared with the declarative
//! [`typed_builtin!`](crate::typed_builtin) spec. Numeric arguments use the
//! `propdouble` kind (documented Null propagation, `CDbl` coercion at the
//! boundary); `abs` stays a Variant passthrough because it preserves the
//! input's numeric type (Integer overflow must stay error 6).

use super::{Builtin, Registry};
use crate::typed_builtin;
use vb6runtime::library::math as mathfn;

/// Register the math functions in `registry`.
pub(super) fn register(registry: &mut Registry) {
    // `abs` preserves its argument's numeric type (Byte/Integer/Long ...),
    // so the whole Variant still reaches the runtime body.
    registry.insert(typed_builtin!("abs", 1, 1, (value: variant),
        mathfn::abs::abs(value)));
    registry.insert(typed_builtin!("atn", 1, 1, (number: propdouble),
        mathfn::atn::atn(&number)));
    registry.insert(typed_builtin!("cos", 1, 1, (number: propdouble),
        mathfn::cos::cos(&number)));
    registry.insert(typed_builtin!("exp", 1, 1, (number: propdouble),
        mathfn::exp::exp(&number)));
    registry.insert(typed_builtin!("fix", 1, 1, (number: propdouble),
        mathfn::fix::fix(&number)));
    registry.insert(typed_builtin!("int", 1, 1, (number: propdouble),
        mathfn::int::int(&number)));
    registry.insert(typed_builtin!("log", 1, 1, (number: propdouble),
        mathfn::log::log(&number)));
    // `rnd` keeps a hand-rolled body because the omitted argument (`None`)
    // is distinct from an explicit `0` (repeat last value). A present `Null`
    // argument rejects with error 94 at the boundary, matching VB6's typed
    // `Rnd(number)` declaration.
    registry.insert(typed_builtin!("rnd", 0, 1, (number: opt_single),
        mathfn::rnd::rnd(number.as_ref())));
    registry.insert(
        typed_builtin!("round", 1, 2, (expression: propdouble, numdecimalplaces: opt_long),
        mathfn::round::round(&expression, numdecimalplaces.as_ref())),
    );
    registry.insert(typed_builtin!("sgn", 1, 1, (number: propdouble),
        mathfn::sgn::sgn(&number)));
    registry.insert(typed_builtin!("sin", 1, 1, (number: propdouble),
        mathfn::sin::sin(&number)));
    registry.insert(typed_builtin!("sqr", 1, 1, (number: propdouble),
        mathfn::sqr::sqr(&number)));
    registry.insert(typed_builtin!("tan", 1, 1, (number: propdouble),
        mathfn::tan::tan(&number)));
}
