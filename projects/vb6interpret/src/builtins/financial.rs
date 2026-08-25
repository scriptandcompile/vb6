//! VB6 financial function registry.
//!
//! One [`typed_builtin!`](crate::typed_builtin) entry per financial function.
//! Scalar money/term parameters use `double` kinds (`CBool`/`CDbl` coercion,
//! present-Null → 94); payment-type flags use `integer`; optional scalars
//! use `opt_*` kinds so absent and unconvertible stay distinct. The
//! cash-flow arrays of `Irr`/`NPer`… stay Variant because they are observed
//! structurally.

use super::{Builtin, Registry};
use crate::typed_builtin;
use vb6runtime::library::financial as finfn;

/// Register the financial functions in `registry`.
pub(super) fn register(registry: &mut Registry) {
    registry.insert(typed_builtin!("ddb", 4, 5,
            (cost: double, salvage: double, life: double,
             period: double, factor: opt_double),
        finfn::ddb::ddb(&cost, &salvage, &life, &period, factor.as_ref())));
    registry.insert(typed_builtin!("fv", 3, 5,
        (rate: double, nper: double, pmt: double,
         pv: opt_double, type_: opt_integer),
        finfn::fv::fv(&rate, &nper, &pmt, pv.as_ref(), type_.as_ref())));
    registry.insert(typed_builtin!("ipmt", 4, 6,
            (rate: double, per: double, nper: double, pv: double,
             fv: opt_double, type_: opt_integer),
        finfn::ipmt::ipmt(&rate, &per, &nper, &pv, fv.as_ref(), type_.as_ref())));
    registry.insert(typed_builtin!("irr", 1, 2,
        (values: variant, guess: opt_double),
        finfn::irr::irr(values, guess.as_ref())));
    registry.insert(typed_builtin!("mirr", 2, 2,
        (values: variant, finance_rate: double, reinvest_rate: double),
        finfn::mirr::mirr(values, &finance_rate, &reinvest_rate)));
    registry.insert(typed_builtin!("nper", 3, 5,
            (rate: double, pmt: double, pv: double,
             fv: opt_double, type_: opt_integer),
        finfn::nper::nper(&rate, &pmt, &pv, fv.as_ref(), type_.as_ref())));
    registry.insert(typed_builtin!("npv", 2, 2,
        (rate: double, values: variant),
        finfn::npv::npv(&rate, values)));
    registry.insert(typed_builtin!("pmt", 3, 5,
            (rate: double, nper: double, pv: double,
             fv: opt_double, type_val: opt_double),
        finfn::pmt::pmt(&rate, &nper, &pv, fv.as_ref(), type_val.as_ref())));
    registry.insert(typed_builtin!("ppmt", 4, 6,
            (rate: double, per: double, nper: double, pv: double,
             fv: opt_double, type_: opt_integer),
        finfn::ppmt::ppmt(&rate, &per, &nper, &pv, fv.as_ref(), type_.as_ref())));
    registry.insert(typed_builtin!("pv", 3, 5,
        (rate: double, nper: double, pmt: double,
         fv: opt_double, type_: opt_integer),
        finfn::pv::pv(&rate, &nper, &pmt, fv.as_ref(), type_.as_ref())));
    registry.insert(typed_builtin!("rate", 3, 6,
            (nper: double, pmt: double, pv: double, fv: opt_double,
             type_: opt_integer, guess: opt_double),
        finfn::rate::rate(&nper, &pmt, &pv, fv.as_ref(), type_.as_ref(),
            guess.as_ref())));
    registry.insert(typed_builtin!("sln", 3, 3,
        (cost: double, salvage: double, life: double),
        finfn::sln::sln(&cost, &salvage, &life)));
    registry.insert(typed_builtin!("syd", 4, 4,
        (cost: double, salvage: double, life: double, period: double),
        finfn::syd::syd(&cost, &salvage, &life, &period)));
}
