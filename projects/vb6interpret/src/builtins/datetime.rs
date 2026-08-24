//! VB6 datetime function registry.
//!
//! One [`typed_builtin!`](crate::typed_builtin) entry per datetime function.
//! Date arguments use the `propdate` kind (documented Null propagation,
//! `CDate` coercion at the boundary); `datediff` keeps plain kinds because it
//! has never propagated Null (error 94 via coercion); `datevalue`/`timevalue`
//! keep Variant passthrough because their string-parsing branches are
//! load-bearing.

use super::{Builtin, Registry};
use crate::typed_builtin;
use vb6runtime::library::datetime as datetimefn;

/// Register the datetime functions in `registry`.
pub(super) fn register(registry: &mut Registry) {
    registry.insert(typed_builtin!("date", 0, 0, (), datetimefn::date::date()));
    registry.insert(typed_builtin!(
        "date$",
        0,
        0,
        (),
        datetimefn::date_dollar::date_dollar().map(vb6runtime::VBVariant::from)
    ));
    registry.insert(
        typed_builtin!("dateadd", 3, 3, (interval: propstring, number: propdouble, date: propdate),
        datetimefn::dateadd::date_add(&interval, &number, &date)),
    );
    registry.insert(typed_builtin!("datediff", 3, 5,
            (interval: string, date1: date, date2: date,
             firstdayofweek: opt_long, firstweekofyear: opt_long),
        datetimefn::datediff::date_diff(&interval, &date1, &date2,
            firstdayofweek.as_ref(), firstweekofyear.as_ref())));
    registry.insert(typed_builtin!("datepart", 2, 4,
            (interval: string, date: propdate,
             firstdayofweek: opt_long, firstweekofyear: opt_long),
        datetimefn::datepart::date_part(&interval, &date,
            firstdayofweek.as_ref(), firstweekofyear.as_ref())));
    registry.insert(
        typed_builtin!("dateserial", 3, 3, (year: long, month: long, day: long),
        datetimefn::dateserial::date_serial(&year, &month, &day)),
    );
    registry.insert(typed_builtin!("datevalue", 1, 1, (string: variant),
        datetimefn::datevalue::date_value(string)));
    registry.insert(typed_builtin!("day", 1, 1, (date: propdate),
        datetimefn::day::day(&date)));
    registry.insert(typed_builtin!("hour", 1, 1, (time: propdate),
        datetimefn::hour::hour(&time)));
    registry.insert(typed_builtin!("minute", 1, 1, (time: propdate),
        datetimefn::minute::minute(&time)));
    registry.insert(typed_builtin!("month", 1, 1, (date: propdate),
        datetimefn::month::month(&date)));
    registry.insert(
        typed_builtin!("monthname", 1, 2, (month: long, abbreviate: opt_boolean),
        datetimefn::monthname::month_name(&month,
            abbreviate.as_ref()).map(vb6runtime::VBVariant::from)),
    );
    registry.insert(typed_builtin!("now", 0, 0, (), datetimefn::now::now()));
    registry.insert(typed_builtin!("second", 1, 1, (time: propdate),
        datetimefn::second::second(&time)));
    registry.insert(typed_builtin!("time", 0, 0, (), datetimefn::time::time()));
    registry.insert(typed_builtin!(
        "time$",
        0,
        0,
        (),
        datetimefn::time_dollar::time_dollar().map(vb6runtime::VBVariant::from)
    ));
    registry.insert(typed_builtin!(
        "timer",
        0,
        0,
        (),
        datetimefn::timer::timer()
    ));
    registry.insert(
        typed_builtin!("timeserial", 3, 3, (hour: long, minute: long, second: long),
        datetimefn::timeserial::time_serial(&hour, &minute, &second)),
    );
    registry.insert(typed_builtin!("timevalue", 1, 1, (time: variant),
        datetimefn::timevalue::time_value(time)));
    registry.insert(
        typed_builtin!("weekday", 1, 2, (date: propdate, firstdayofweek: opt_long),
        datetimefn::weekday::weekday(&date, firstdayofweek.as_ref())),
    );
    registry.insert(typed_builtin!("weekdayname", 1, 3,
            (weekday: long, abbreviate: opt_boolean, firstdayofweek: opt_long),
        datetimefn::weekdayname::weekday_name(&weekday, abbreviate.as_ref(),
            firstdayofweek.as_ref()).map(vb6runtime::VBVariant::from)));
    registry.insert(typed_builtin!("year", 1, 1, (date: propdate),
        datetimefn::year::year(&date)));
}
