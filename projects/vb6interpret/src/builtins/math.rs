//! VB6 math function registry.
//!
//! One [`Builtin`](super::Builtin) entry per math function, each wrapping the
//! typed `vb6runtime::library::functions::math` implementation.

use super::{Builtin, Registry};
use crate::builtin;
use vb6core::error::VBResult;
use vb6runtime::library::functions::math as mathfn;
use vb6runtime::value::VBLong;
use vb6runtime::VBVariant;

/// Register the math functions in `registry`.
pub(super) fn register(registry: &mut Registry) {
    registry.insert(builtin!("abs", 1, 1, |args| {
        mathfn::abs::abs(args[0].clone())
    }));
    registry.insert(builtin!("atn", 1, 1, |args| {
        mathfn::atn::atn(args[0].clone())
    }));
    registry.insert(builtin!("cos", 1, 1, |args| {
        mathfn::cos::cos(args[0].clone())
    }));
    registry.insert(builtin!("exp", 1, 1, |args| {
        mathfn::exp::exp(args[0].clone())
    }));
    registry.insert(builtin!("fix", 1, 1, |args| {
        mathfn::fix::fix(args[0].clone())
    }));
    registry.insert(builtin!("int", 1, 1, |args| {
        mathfn::int::int(args[0].clone())
    }));
    registry.insert(builtin!("log", 1, 1, |args| {
        mathfn::log::log(args[0].clone())
    }));
    registry.insert(builtin!("rnd", 0, 1, |args| {
        mathfn::rnd::rnd(args.first().cloned().unwrap_or(VBVariant::Empty))
    }));
    registry.insert(builtin!("round", 1, 2, |args| {
        let places = args.get(1).map(VBLong::try_from).transpose()?;
        mathfn::round::round(&args[0], places.as_ref())
    }));
    registry.insert(builtin!("sgn", 1, 1, |args| {
        mathfn::sgn::sgn(args[0].clone())
    }));
    registry.insert(builtin!("sin", 1, 1, |args| {
        mathfn::sin::sin(args[0].clone())
    }));
    registry.insert(builtin!("sqr", 1, 1, |args| {
        mathfn::sqr::sqr(args[0].clone())
    }));
    registry.insert(builtin!("tan", 1, 1, |args| {
        mathfn::tan::tan(args[0].clone())
    }));
}
