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

    registry.insert(builtin!("fv", 3, 5, |args| {
        let pv = if args.len() > 3 { Some(&args[3]) } else { None };
        let type_ = if args.len() > 4 { Some(&args[4]) } else { None };
        finfn::fv::fv(&args[0], &args[1], &args[2], pv, type_)
    }));

    registry.insert(builtin!("ipmt", 4, 6, |args| {
        let fv = if args.len() > 4 { Some(&args[4]) } else { None };
        let type_ = if args.len() > 5 { Some(&args[5]) } else { None };
        finfn::ipmt::ipmt(&args[0], &args[1], &args[2], &args[3], fv, type_)
    }));

    registry.insert(builtin!("irr", 1, 2, |args| {
        let guess = if args.len() > 1 { Some(&args[1]) } else { None };
        finfn::irr::irr(&args[0], guess)
    }));

    registry.insert(builtin!("mirr", 2, 2, |args| {
        finfn::mirr::mirr(
            &args[0],
            args[1].as_f64().unwrap(),
            args[2].as_f64().unwrap(),
        )
    }));

    registry.insert(builtin!("nper", 3, 5, |args| {
        let fv = if args.len() > 3 { Some(&args[3]) } else { None };
        let type_ = if args.len() > 4 { Some(&args[4]) } else { None };
        finfn::nper::nper(&args[0], &args[1], &args[2], fv, type_)
    }));

    registry.insert(builtin!("npv", 2, 2, |args| {
        finfn::npv::npv(&args[0], &args[1])
    }));
}
