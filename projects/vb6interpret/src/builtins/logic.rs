//! VB6 logic function registry.
//!
//! One [`typed_builtin!`](crate::typed_builtin) entry per logic function.
//! `iif`'s condition uses the `propboolean` kind (documented Null
//! propagation, `CBool` coercion at the boundary); its parts stay Variant
//! passthrough because they are returned unchanged. `choose`'s index uses
//! `proplong`; its choices and Switch's argument list bind the raw rest
//! slice — Switch evaluates pairs in order and must not pre-convert.

use super::{Builtin, Registry};
use crate::typed_builtin;
use vb6runtime::library::logic as logicfn;

/// Register the logic functions in `registry`.
pub(super) fn register(registry: &mut Registry) {
    registry.insert(typed_builtin!("choose", 2, usize::MAX, (index: proplong),
            rest choices: variants,
        logicfn::choose::choose(&index, choices)));
    registry.insert(typed_builtin!("iif", 3, 3,
            (condition: propboolean, truepart: variant, falsepart: variant),
        logicfn::iif::iif(&condition, truepart, falsepart)));
    registry.insert(typed_builtin!("switch", 2, usize::MAX, (),
        rest arguments: variants,
        logicfn::switch::switch(arguments)));
}
