//! VB6 datetime function registry.
//!
//! One [`Builtin`](super::Builtin) entry per datetime function, each wrapping
//! the typed `vb6runtime::library::datetime` implementation.

use super::{Builtin, Registry};
use crate::builtin;
use vb6core::error::VBResult;
use vb6runtime::library::datetime as datetimefn;
use vb6runtime::VBVariant;

/// Register the datetime functions in `registry`.
pub(super) fn register(registry: &mut Registry) {
    registry.insert(builtin!("date", 0, 0, |_args| { datetimefn::date::date() }));
    registry.insert(builtin!("date$", 0, 0, |_args| {
        datetimefn::date_dollar::date_dollar().map(VBVariant::from)
    }));
    registry.insert(builtin!("dateadd", 3, 3, |args| {
        datetimefn::dateadd::date_add(&args[0], &args[1], &args[2])
    }));
    registry.insert(builtin!("datediff", 3, 5, |args| {
        datetimefn::datediff::date_diff(&args[0], &args[1], &args[2], args.get(3), args.get(4))
    }));
    registry.insert(builtin!("datepart", 2, 4, |args| {
        datetimefn::datepart::date_part(&args[0], &args[1], args.get(2), args.get(3))
    }));
    registry.insert(builtin!("dateserial", 3, 3, |args| {
        datetimefn::dateserial::date_serial(&args[0], &args[1], &args[2])
    }));
    registry.insert(builtin!("datevalue", 1, 1, |args| {
        datetimefn::datevalue::date_value(&args[0])
    }));
    registry.insert(builtin!("day", 1, 1, |args| {
        datetimefn::day::day(&args[0])
    }));
    registry.insert(builtin!("hour", 1, 1, |args| {
        datetimefn::hour::hour(&args[0])
    }));
    registry.insert(builtin!("minute", 1, 1, |args| {
        datetimefn::minute::minute(&args[0])
    }));
    registry.insert(builtin!("month", 1, 1, |args| {
        datetimefn::month::month(&args[0])
    }));
    registry.insert(builtin!("monthname", 1, 2, |args| {
        datetimefn::monthname::month_name(&args[0], args.get(1)).map(VBVariant::from)
    }));
    registry.insert(builtin!("now", 0, 0, |_args| { datetimefn::now::now() }));
    registry.insert(builtin!("second", 1, 1, |args| {
        datetimefn::second::second(&args[0])
    }));
    registry.insert(builtin!("time", 0, 0, |_args| { datetimefn::time::time() }));
    registry.insert(builtin!("time$", 0, 0, |_args| {
        datetimefn::time_dollar::time_dollar().map(VBVariant::from)
    }));
    registry.insert(builtin!("timer", 0, 0, |_args| {
        datetimefn::timer::timer()
    }));
    registry.insert(builtin!("timeserial", 3, 3, |args| {
        datetimefn::timeserial::time_serial(&args[0], &args[1], &args[2])
    }));
    registry.insert(builtin!("timevalue", 1, 1, |args| {
        datetimefn::timevalue::time_value(&args[0])
    }));
    registry.insert(builtin!("weekday", 1, 2, |args| {
        datetimefn::weekday::weekday(&args[0], args.get(1))
    }));
    registry.insert(builtin!("weekdayname", 1, 3, |args| {
        datetimefn::weekdayname::weekday_name(&args[0], args.get(1), args.get(2))
            .map(VBVariant::from)
    }));
    registry.insert(builtin!("year", 1, 1, |args| {
        datetimefn::year::year(&args[0])
    }));
}
