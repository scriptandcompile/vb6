//! Tests for builtin dispatch, argument coercion, and category helpers.
//!
//! Lives in its own file to keep `mod.rs` dispatch-only; declared as
//! `#[cfg(test)] mod tests;` so every item in the parent module stays
//! visible through `use super::*` exactly as when the suite was inline.

use super::*;
use vb6runtime::value::VBString;
use vb6runtime::ArrayValue;

/// Serializes dispatch tests that read or write the shared environment
/// snapshot so parallel test execution cannot interfere with a test's
/// fixed environment.
static ENV_DISPATCH_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[test]
fn builtin_name_preserves_dollar_suffix() {
    assert_eq!(builtin_name("Format$"), "format$");
    assert_eq!(builtin_name("format"), "format");
    assert_eq!(builtin_name("Left$"), "left$");
    assert_eq!(builtin_name("Left"), "left");
    assert_eq!(builtin_name("ChrW%"), "chrw");
}

#[test]
fn format_and_format_dollar_dispatch() {
    let args = vec![
        VBVariant::Double(1234.5),
        VBVariant::from(VBString::from("#,##0.00")),
    ];
    let result = call_builtin("Format$", &args).unwrap();
    assert_eq!(result.as_string().unwrap(), "1,234.50");
    let result = call_builtin("Format", &args).unwrap();
    assert_eq!(result.as_string().unwrap(), "1,234.50");
}

#[test]
fn dollar_variants_share_string_implementations() {
    let result = call_builtin(
        "Left$",
        &[VBVariant::from_string("abcdef"), VBVariant::Long(3)],
    )
    .unwrap();
    assert_eq!(result.as_string().unwrap(), "abc");
    let result = call_builtin("LCase$", &[VBVariant::from_string("ABC")]).unwrap();
    assert_eq!(result.as_string().unwrap(), "abc");
    let result = call_builtin("Chr$", &[VBVariant::Long(65)]).unwrap();
    assert_eq!(result.as_string().unwrap(), "A");
}

#[test]
fn dollar_variants_reject_null() {
    let err = call_builtin("Left$", &[VBVariant::Null, VBVariant::Long(3)]).unwrap_err();
    assert_eq!(err.number, vb6core::error::err_number::INVALID_USE_OF_NULL);
    let err = call_builtin("LCase$", &[VBVariant::Null]).unwrap_err();
    assert_eq!(err.number, vb6core::error::err_number::INVALID_USE_OF_NULL);
    let err = call_builtin("Chr$", &[VBVariant::Null]).unwrap_err();
    assert_eq!(err.number, vb6core::error::err_number::INVALID_USE_OF_NULL);
}

#[test]
fn non_dollar_string_functions_propagate_null() {
    // The non-`$` string family documents "If string contains Null, Null is
    // returned" — propagation happens at the dispatch boundary before any
    // typed conversion runs (propstring kind).
    for name in ["LCase", "UCase", "Trim", "LTrim", "RTrim"] {
        let result = call_builtin(name, &[VBVariant::Null]);
        match result {
            Ok(VBVariant::Null) => {}
            other => panic!("{name}(Null) should propagate Null, got {other:?}"),
        }
    }
    for name in ["Left", "Right"] {
        let result = call_builtin(name, &[VBVariant::Null, VBVariant::Long(2)]);
        match result {
            Ok(VBVariant::Null) => {}
            other => panic!("{name}(Null, 2) should propagate Null, got {other:?}"),
        }
    }
    let result = call_builtin("Mid", &[VBVariant::Null, VBVariant::Long(1)]);
    assert_eq!(result.unwrap(), VBVariant::Null);
    // InStr's documented table: string1/string2 Null → Null.
    let result = call_builtin("InStr", &[VBVariant::from_string("a"), VBVariant::Null]);
    assert_eq!(result.unwrap(), VBVariant::Null);
}

#[test]
fn instr_dispatch_keeps_arity_dependent_positions() {
    // Two-argument form: no `start`, search begins at 1.
    let result = call_builtin(
        "InStr",
        &[VBVariant::from_string("Hello"), VBVariant::from_string("l")],
    )
    .unwrap();
    assert_eq!(result, VBVariant::Long(3));
    // Three-argument form: first argument is the strict `start`.
    let result = call_builtin(
        "InStr",
        &[
            VBVariant::Long(4),
            VBVariant::from_string("Hello"),
            VBVariant::from_string("l"),
        ],
    )
    .unwrap();
    assert_eq!(result, VBVariant::Long(4));
    // Four-argument form: trailing `compare` switches to text comparison.
    let result = call_builtin(
        "InStr",
        &[
            VBVariant::Long(1),
            VBVariant::from_string("abc"),
            VBVariant::from_string("B"),
            VBVariant::Long(1),
        ],
    )
    .unwrap();
    assert_eq!(result, VBVariant::Long(2));
    // `start` converts like any Long parameter (CLng): numeric strings coerce.
    let result = call_builtin(
        "InStr",
        &[
            VBVariant::from_string("4"),
            VBVariant::from_string("Hello"),
            VBVariant::from_string("o"),
        ],
    )
    .unwrap();
    assert_eq!(result, VBVariant::Long(5));
    // Non-numeric starts are type mismatches...
    let err = call_builtin(
        "InStr",
        &[
            VBVariant::from_string("abc"),
            VBVariant::from_string("Hello"),
            VBVariant::from_string("l"),
        ],
    )
    .unwrap_err();
    assert_eq!(err.number, vb6core::error::err_number::TYPE_MISMATCH);
    // ...and Null in the auxiliary position rejects (94) instead of
    // propagating: only string1/string2 follow the propagation table.
    let err = call_builtin(
        "InStr",
        &[
            VBVariant::Null,
            VBVariant::from_string("Hello"),
            VBVariant::from_string("l"),
        ],
    )
    .unwrap_err();
    assert_eq!(err.number, vb6core::error::err_number::INVALID_USE_OF_NULL);
}

#[test]
fn null_propagation_is_parameter_scoped() {
    // Only the parameters marked propagating short-circuit; auxiliary
    // numeric parameters and `$` forms still reject Null with error 94.
    let err = call_builtin("Left", &[VBVariant::from_string("abc"), VBVariant::Null]).unwrap_err();
    assert_eq!(err.number, vb6core::error::err_number::INVALID_USE_OF_NULL);
    let err = call_builtin("Mid", &[VBVariant::from_string("abc"), VBVariant::Null]).unwrap_err();
    assert_eq!(err.number, vb6core::error::err_number::INVALID_USE_OF_NULL);
    let err = call_builtin("Chrb", &[VBVariant::Null]).unwrap_err();
    assert_eq!(err.number, vb6core::error::err_number::INVALID_USE_OF_NULL);
}

#[test]
fn math_functions_propagate_null_at_the_boundary() {
    for name in ["Sqr", "Sin", "Fix", "Int", "Sgn", "Log"] {
        let result = call_builtin(name, &[VBVariant::Null]);
        match result {
            Ok(VBVariant::Null) => {}
            other => panic!("{name}(Null) should propagate Null, got {other:?}"),
        }
    }
    let result = call_builtin("Round", &[VBVariant::Null, VBVariant::Long(2)]);
    assert_eq!(result.unwrap(), VBVariant::Null);
}

#[test]
fn math_conversion_failures_report_at_the_boundary() {
    // Non-numeric strings fail with 13 during boundary coercion.
    let err = call_builtin("Sqr", &[VBVariant::from_string("abc")]).unwrap_err();
    assert_eq!(err.number, vb6core::error::err_number::TYPE_MISMATCH);
    // CVErr arguments re-raise their inner error through the wrapper.
    let err = call_builtin(
        "Sin",
        &[VBVariant::from_error(vb6core::error::VBError::new(31337))],
    )
    .unwrap_err();
    assert_eq!(err.number, 31337);
}

#[test]
fn abs_preserves_argument_type() {
    // `abs` is deliberately a Variant passthrough: Abs(Integer) stays
    // Integer, so the Integer-min overflow must be error 6.
    let err = call_builtin("Abs", &[VBVariant::Integer(i16::MIN)]).unwrap_err();
    assert_eq!(err.number, vb6core::error::err_number::OVERFLOW);
    assert_eq!(
        call_builtin("Abs", &[VBVariant::Integer(-5)]).unwrap(),
        VBVariant::Integer(5)
    );
}

#[test]
fn logic_functions_propagate_null_at_the_boundary() {
    // Logic functions propagate through their prop* kinds: IIf's condition
    // and Choose's index short-circuit to Null before coercion, while the
    // parts/choices stay untyped passthroughs.
    let result = call_builtin(
        "IIf",
        &[
            VBVariant::Null,
            VBVariant::from_long(1),
            VBVariant::from_long(2),
        ],
    );
    assert_eq!(result.unwrap(), VBVariant::Null);
    let result = call_builtin("Choose", &[VBVariant::Null, VBVariant::from_string("a")]);
    assert_eq!(result.unwrap(), VBVariant::Null);
    // The parts are returned unchanged even when they are Null themselves.
    let result = call_builtin(
        "IIf",
        &[
            VBVariant::Boolean(true),
            VBVariant::Null,
            VBVariant::from_long(2),
        ],
    );
    assert_eq!(result.unwrap(), VBVariant::Null);
    // Switch keeps its raw slice and stops at the first True condition, so
    // a Null condition in a later (unevaluated) pair never surfaces.
    let result = call_builtin(
        "Switch",
        &[
            VBVariant::Boolean(false),
            VBVariant::from_string("no"),
            VBVariant::Boolean(true),
            VBVariant::from_string("yes"),
            VBVariant::Null,
            VBVariant::from_string("skipped"),
        ],
    );
    assert_eq!(result.unwrap(), VBVariant::from_string("yes"));
    // An evaluated Null condition returns Null immediately.
    let result = call_builtin(
        "Switch",
        &[
            VBVariant::Boolean(false),
            VBVariant::from_string("no"),
            VBVariant::Null,
            VBVariant::from_string("skipped"),
        ],
    );
    assert_eq!(result.unwrap(), VBVariant::Null);
}

#[test]
fn datetime_functions_propagate_null_at_the_boundary() {
    // Extractors document "If date contains Null, Null is returned" —
    // `propdate` short-circuits before the typed runtime sees the value.
    for name in ["Year", "Month", "Day", "Hour", "Minute", "Second"] {
        let result = call_builtin(name, &[VBVariant::Null]);
        match result {
            Ok(VBVariant::Null) => {}
            other => panic!("{name}(Null) should propagate Null, got {other:?}"),
        }
    }
    // DateAdd propagates Null from any of its three parameters.
    let result = call_builtin(
        "DateAdd",
        &[
            VBVariant::from_string("d"),
            VBVariant::Long(1),
            VBVariant::Null,
        ],
    );
    assert_eq!(result.unwrap(), VBVariant::Null);
    let result = call_builtin(
        "DateAdd",
        &[
            VBVariant::Null,
            VBVariant::Long(1),
            VBVariant::from_date_serial(45_000.0),
        ],
    );
    assert_eq!(result.unwrap(), VBVariant::Null);
    // Weekday propagates its date but rejects a Null firstdayofweek.
    let result = call_builtin("Weekday", &[VBVariant::Null]);
    assert_eq!(result.unwrap(), VBVariant::Null);
    let err = call_builtin(
        "Weekday",
        &[VBVariant::from_date_serial(45_000.0), VBVariant::Null],
    )
    .unwrap_err();
    assert_eq!(err.number, vb6core::error::err_number::INVALID_USE_OF_NULL);
}

#[test]
fn datetime_null_policies_differ_per_function() {
    // DatePart propagates only the date: a Null interval is error 94,
    // a Null date still returns Null (VB6 documented table).
    let err = call_builtin(
        "DatePart",
        &[VBVariant::Null, VBVariant::from_date_serial(45_000.0)],
    )
    .unwrap_err();
    assert_eq!(err.number, vb6core::error::err_number::INVALID_USE_OF_NULL);
    let result = call_builtin(
        "DatePart",
        &[VBVariant::from_string("yyyy"), VBVariant::Null],
    );
    assert_eq!(result.unwrap(), VBVariant::Null);

    // DateDiff has no propagation at all: every Null reaches coercion and
    // fails with 94 instead of returning Null.
    let err = call_builtin(
        "DateDiff",
        &[
            VBVariant::from_string("d"),
            VBVariant::Null,
            VBVariant::from_date_serial(45_000.0),
        ],
    )
    .unwrap_err();
    assert_eq!(err.number, vb6core::error::err_number::INVALID_USE_OF_NULL);

    // DateSerial/TimeSerial never propagated either: Null year is 94.
    let err = call_builtin(
        "DateSerial",
        &[VBVariant::Null, VBVariant::Long(1), VBVariant::Long(1)],
    )
    .unwrap_err();
    assert_eq!(err.number, vb6core::error::err_number::INVALID_USE_OF_NULL);

    // MonthName keeps strict Long semantics: a Null month is 94, while
    // CVErr re-raises through propdate-style wrappers too.
    let err = call_builtin("MonthName", &[VBVariant::Null]).unwrap_err();
    assert_eq!(err.number, vb6core::error::err_number::INVALID_USE_OF_NULL);
    let err = call_builtin(
        "Year",
        &[VBVariant::from_error(vb6core::error::VBError::new(31337))],
    )
    .unwrap_err();
    assert_eq!(err.number, 31337);
}

#[test]
fn math_functions_dispatch() {
    assert_eq!(
        call_builtin("Abs", &[VBVariant::from_integer(-5)]).unwrap(),
        VBVariant::from_integer(5)
    );
    assert_eq!(
        call_builtin("Sqr", &[VBVariant::from_double(16.0)]).unwrap(),
        VBVariant::from_double(4.0)
    );
    assert_eq!(
        call_builtin("Round", &[VBVariant::from_double(2.5)]).unwrap(),
        VBVariant::from_double(2.0)
    );
    assert_eq!(
        call_builtin("Sgn", &[VBVariant::from_integer(-7)]).unwrap(),
        VBVariant::from_integer(-1)
    );
}

#[test]
fn rnd_accepts_zero_or_one_argument() {
    let value = call_builtin("Rnd", &[]).unwrap().as_f32().unwrap();
    assert!((0.0..1.0).contains(&value));
    let value = call_builtin("Rnd", &[VBVariant::from_long(1)])
        .unwrap()
        .as_f32()
        .unwrap();
    assert!((0.0..1.0).contains(&value));
}

#[test]
fn wrong_argument_count_is_450() {
    let err = call_builtin("Len", &[]).unwrap_err();
    assert_eq!(
        err.number,
        vb6core::error::err_number::WRONG_NUMBER_OF_ARGUMENTS
    );
    let err = call_builtin(
        "Len",
        &[VBVariant::from_string("a"), VBVariant::from_string("b")],
    )
    .unwrap_err();
    assert_eq!(
        err.number,
        vb6core::error::err_number::WRONG_NUMBER_OF_ARGUMENTS
    );
}

#[test]
fn unknown_function_is_error_35() {
    let err = call_builtin("DefinitelyNotAFunction", &[]).unwrap_err();
    assert_eq!(err.number, 35);
}

#[test]
fn logic_functions_dispatch() {
    let result = call_builtin(
        "IIf",
        &[
            VBVariant::from_bool(true),
            VBVariant::from_string("yes"),
            VBVariant::from_string("no"),
        ],
    )
    .unwrap();
    assert_eq!(result.as_string().unwrap(), "yes");

    let result = call_builtin(
        "Choose",
        &[
            VBVariant::Long(2),
            VBVariant::from_string("a"),
            VBVariant::from_string("b"),
            VBVariant::from_string("c"),
        ],
    )
    .unwrap();
    assert_eq!(result.as_string().unwrap(), "b");

    let result = call_builtin(
        "Switch",
        &[
            VBVariant::from_bool(false),
            VBVariant::from_string("a"),
            VBVariant::from_bool(true),
            VBVariant::from_string("b"),
        ],
    )
    .unwrap();
    assert_eq!(result.as_string().unwrap(), "b");
}

#[test]
fn logic_variadic_functions_validate_arity() {
    let err = call_builtin("Switch", &[VBVariant::from_bool(true)]).unwrap_err();
    assert_eq!(
        err.number,
        vb6core::error::err_number::WRONG_NUMBER_OF_ARGUMENTS
    );
    let err = call_builtin("Choose", &[VBVariant::Long(1)]).unwrap_err();
    assert_eq!(
        err.number,
        vb6core::error::err_number::WRONG_NUMBER_OF_ARGUMENTS
    );
}

#[test]
fn type_checking_functions_dispatch() {
    assert_eq!(
        call_builtin("IsEmpty", &[VBVariant::Empty]).unwrap(),
        VBVariant::from_bool(true)
    );
    assert_eq!(
        call_builtin("IsNull", &[VBVariant::Null]).unwrap(),
        VBVariant::from_bool(true)
    );
    assert_eq!(
        call_builtin(
            "IsError",
            &[VBVariant::from_error(vb6core::error::VBError::new(
                err_number::TYPE_MISMATCH
            ))]
        )
        .unwrap(),
        VBVariant::from_bool(true)
    );
    assert_eq!(
        call_builtin("IsDate", &[VBVariant::from_string("12/25/2025")]).unwrap(),
        VBVariant::from_bool(true)
    );
    assert_eq!(
        call_builtin("IsNumeric", &[VBVariant::from_string("123")]).unwrap(),
        VBVariant::from_bool(true)
    );
    assert_eq!(
        call_builtin(
            "IsArray",
            &[VBVariant::array_dynamic(vb6runtime::VBType::Integer)]
        )
        .unwrap(),
        VBVariant::from_bool(true)
    );
}

#[test]
fn type_checking_returns_false_for_other_values() {
    assert_eq!(
        call_builtin("IsEmpty", &[VBVariant::Null]).unwrap(),
        VBVariant::from_bool(false)
    );
    assert_eq!(
        call_builtin("IsNull", &[VBVariant::Empty]).unwrap(),
        VBVariant::from_bool(false)
    );
    assert_eq!(
        call_builtin("IsDate", &[VBVariant::from_string("not a date")]).unwrap(),
        VBVariant::from_bool(false)
    );
    assert_eq!(
        call_builtin("IsNumeric", &[VBVariant::from_string("abc")]).unwrap(),
        VBVariant::from_bool(false)
    );
}

#[test]
fn is_missing_reports_omitted_argument() {
    assert_eq!(
        call_builtin("IsMissing", &[]).unwrap(),
        VBVariant::from_bool(true)
    );
    assert_eq!(
        call_builtin("IsMissing", &[VBVariant::Empty]).unwrap(),
        VBVariant::from_bool(false)
    );
    assert_eq!(
        call_builtin("IsMissing", &[VBVariant::Null]).unwrap(),
        VBVariant::from_bool(false)
    );
}

#[test]
fn datetime_part_functions_dispatch() {
    let date = VBVariant::from_string("2/14/2025");
    assert_eq!(
        call_builtin("Day", std::slice::from_ref(&date)).unwrap(),
        VBVariant::from_integer(14)
    );
    assert_eq!(
        call_builtin("Month", std::slice::from_ref(&date)).unwrap(),
        VBVariant::from_integer(2)
    );
    assert_eq!(
        call_builtin("Year", std::slice::from_ref(&date)).unwrap(),
        VBVariant::from_integer(2025)
    );
    assert_eq!(
        call_builtin("Weekday", &[date]).unwrap(),
        VBVariant::from_integer(6)
    );
    assert_eq!(
        call_builtin("MonthName", &[VBVariant::from_integer(2)]).unwrap(),
        VBVariant::from_string("February")
    );
    assert_eq!(
        call_builtin("WeekdayName", &[VBVariant::from_integer(6)]).unwrap(),
        VBVariant::from_string("Friday")
    );
}

#[test]
fn datetime_serial_functions_dispatch() {
    let result = call_builtin(
        "DateSerial",
        &[
            VBVariant::from_integer(2025),
            VBVariant::from_integer(2),
            VBVariant::from_integer(0),
        ],
    )
    .unwrap();
    assert_eq!(
        call_builtin("Day", std::slice::from_ref(&result)).unwrap(),
        VBVariant::from_integer(31)
    );
    assert_eq!(
        call_builtin("Month", &[result]).unwrap(),
        VBVariant::from_integer(1)
    );

    let result = call_builtin(
        "TimeSerial",
        &[
            VBVariant::from_integer(13),
            VBVariant::from_integer(30),
            VBVariant::from_integer(0),
        ],
    )
    .unwrap();
    assert_eq!(
        call_builtin("Hour", std::slice::from_ref(&result)).unwrap(),
        VBVariant::from_integer(13)
    );
    assert_eq!(
        call_builtin("Minute", std::slice::from_ref(&result)).unwrap(),
        VBVariant::from_integer(30)
    );
    assert_eq!(
        call_builtin("Second", &[result]).unwrap(),
        VBVariant::from_integer(0)
    );
}

#[test]
fn datetime_add_diff_part_dispatch() {
    let result = call_builtin(
        "DateAdd",
        &[
            VBVariant::from_string("d"),
            VBVariant::from_integer(1),
            VBVariant::from_string("12/31/2024"),
        ],
    )
    .unwrap();
    assert_eq!(
        call_builtin("Year", &[result]).unwrap(),
        VBVariant::from_integer(2025)
    );

    assert_eq!(
        call_builtin(
            "DateDiff",
            &[
                VBVariant::from_string("d"),
                VBVariant::from_string("1/1/2025"),
                VBVariant::from_string("1/31/2025"),
            ],
        )
        .unwrap(),
        VBVariant::from_long(30)
    );

    assert_eq!(
        call_builtin(
            "DatePart",
            &[
                VBVariant::from_string("yyyy"),
                VBVariant::from_string("2/14/2025"),
            ],
        )
        .unwrap(),
        VBVariant::from_integer(2025)
    );

    let result =
        call_builtin("DateValue", &[VBVariant::from_string("2/14/2025 10:30 AM")]).unwrap();
    assert_eq!(
        call_builtin("Day", &[result]).unwrap(),
        VBVariant::from_integer(14)
    );
}

#[test]
fn dollar_date_and_time_variants_dispatch() {
    let result = call_builtin("Date$", &[]).unwrap();
    assert!(result.as_string().is_ok());
    let result = call_builtin("Time$", &[]).unwrap();
    assert!(result.as_string().is_ok());
}

#[test]
fn array_lbound_and_ubound_dispatch() {
    let arr = call_builtin(
        "Array",
        &[
            VBVariant::from_integer(10),
            VBVariant::from_integer(20),
            VBVariant::from_integer(30),
        ],
    )
    .unwrap();
    assert_eq!(
        call_builtin("LBound", std::slice::from_ref(&arr)).unwrap(),
        VBVariant::from_integer(0)
    );
    assert_eq!(
        call_builtin("UBound", &[arr]).unwrap(),
        VBVariant::from_integer(2)
    );
}

#[test]
fn split_and_join_dispatch() {
    let arr = call_builtin(
        "Split",
        &[VBVariant::from_string("a,b,c"), VBVariant::from_string(",")],
    )
    .unwrap();
    assert_eq!(
        call_builtin("UBound", std::slice::from_ref(&arr)).unwrap(),
        VBVariant::from_integer(2)
    );
    assert_eq!(
        call_builtin("Join", &[arr, VBVariant::from_string("-")]).unwrap(),
        VBVariant::from_string("a-b-c")
    );
}

#[test]
fn lset_dispatch_left_aligns_within_the_target_width() {
    assert_eq!(
        call_builtin(
            "LSet",
            &[
                VBVariant::from_string("XXXXX"),
                VBVariant::from_string("ab")
            ]
        )
        .unwrap(),
        VBVariant::from_string("ab   ")
    );
    assert_eq!(
        call_builtin(
            "LSet",
            &[
                VBVariant::from_string("XXX"),
                VBVariant::from_string("abcdef")
            ]
        )
        .unwrap(),
        VBVariant::from_string("abc")
    );
}

#[test]
fn rset_dispatch_right_aligns_within_the_target_width() {
    assert_eq!(
        call_builtin(
            "RSet",
            &[
                VBVariant::from_string("XXXXX"),
                VBVariant::from_string("ab")
            ]
        )
        .unwrap(),
        VBVariant::from_string("   ab")
    );
    assert_eq!(
        call_builtin(
            "RSet",
            &[
                VBVariant::from_string("XXX"),
                VBVariant::from_string("abcdef")
            ]
        )
        .unwrap(),
        VBVariant::from_string("def")
    );
}

#[test]
fn environ_dollar_dispatch_reads_the_snapshot() {
    use vb6runtime::state::environment as env_state;

    // Serialize against the shared snapshot and restore it afterwards so
    // the process environment baseline is left intact.
    let _guard = ENV_DISPATCH_LOCK.lock().unwrap();
    env_state::reset();
    env_state::set_env("VB6INTERPRET_TEST_VAR", "hello");
    assert_eq!(
        call_builtin(
            "Environ$",
            &[VBVariant::from_string("vb6interpret_test_var")]
        )
        .unwrap(),
        VBVariant::from_string("hello")
    );
    assert_eq!(
        call_builtin(
            "Environ$",
            &[VBVariant::from_string("VB6INTERPRET_MISSING")]
        )
        .unwrap(),
        VBVariant::from_string("")
    );
    env_state::reset();
}

#[test]
fn environ_dispatch_reads_the_snapshot() {
    use vb6runtime::state::environment as env_state;

    // Serialize against the shared snapshot and restore it afterwards so
    // the process environment baseline is left intact.
    let _guard = ENV_DISPATCH_LOCK.lock().unwrap();
    env_state::reset();
    env_state::set_env("VB6INTERPRET_TEST_VAR", "hello");
    assert_eq!(
        call_builtin(
            "Environ",
            &[VBVariant::from_string("vb6interpret_test_var")]
        )
        .unwrap(),
        VBVariant::from_string("hello")
    );
    assert_eq!(
        call_builtin("Environ", &[VBVariant::Null]).unwrap(),
        VBVariant::Null
    );
    env_state::reset();
}

#[test]
fn error_and_error_dollar_dispatch() {
    use vb6runtime::state::err as err_state;

    // Serialize against the shared current-error state and clear it
    // afterwards so other tests start from a no-error baseline.
    let _guard = ENV_DISPATCH_LOCK.lock().unwrap();
    err_state::clear();
    assert_eq!(
        call_builtin("Error$", &[]).unwrap(),
        VBVariant::from_string("")
    );
    assert_eq!(
        call_builtin("Error", &[VBVariant::from_integer(0)]).unwrap(),
        VBVariant::from_string("")
    );
    assert_eq!(
        call_builtin("Error", &[VBVariant::from_long(11)]).unwrap(),
        VBVariant::from_string("Division by zero")
    );
    assert_eq!(
        call_builtin("Error$", &[VBVariant::from_long(999)]).unwrap(),
        VBVariant::from_string("Application-defined or object-defined error")
    );
    err_state::set_number(53);
    assert_eq!(
        call_builtin("Error", &[]).unwrap(),
        VBVariant::from_string("File not found")
    );
    err_state::clear();
}

#[test]
fn settings_functions_dispatch_and_reject_null_at_the_boundary() {
    use vb6runtime::state::settings as settings_state;

    // Serialize against the shared settings store and reset it afterwards.
    static SETTINGS_DISPATCH_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = SETTINGS_DISPATCH_LOCK.lock().unwrap();
    settings_state::reset();

    // SaveSetting returns Empty; GetSetting round-trips the stored value.
    assert_eq!(
        call_builtin(
            "SaveSetting",
            &[
                VBVariant::from_string("DispatchApp"),
                VBVariant::from_string("Window"),
                VBVariant::from_string("Left"),
                VBVariant::from_string("150"),
            ],
        )
        .unwrap(),
        VBVariant::Empty
    );
    assert_eq!(
        call_builtin(
            "GetSetting",
            &[
                VBVariant::from_string("DispatchApp"),
                VBVariant::from_string("Window"),
                VBVariant::from_string("Left"),
            ],
        )
        .unwrap(),
        VBVariant::from_string("150")
    );

    // A present Null on a typed string parameter is error 94.
    let err = call_builtin(
        "GetSetting",
        &[
            VBVariant::Null,
            VBVariant::from_string("S"),
            VBVariant::from_string("K"),
        ],
    )
    .unwrap_err();
    assert_eq!(err.number, vb6core::error::err_number::INVALID_USE_OF_NULL);

    // Two-argument DeleteSetting (omitted key) deletes the whole section.
    call_builtin(
        "DeleteSetting",
        &[
            VBVariant::from_string("DispatchApp"),
            VBVariant::from_string("Window"),
        ],
    )
    .unwrap();
    assert_eq!(
        call_builtin(
            "GetAllSettings",
            &[
                VBVariant::from_string("DispatchApp"),
                VBVariant::from_string("Window")
            ],
        )
        .unwrap(),
        VBVariant::Empty
    );

    settings_state::reset();
}

#[test]
fn environment_boundary_pins_optional_null_and_error_number_policy() {
    // Optional parameters keep present-Null → 94: DeleteSetting with an
    // omitted (or empty) key removes the whole section, but an explicit Null
    // rejects at the boundary before any store access.
    let err = call_builtin(
        "DeleteSetting",
        &[
            VBVariant::from_string("App"),
            VBVariant::from_string("Section"),
            VBVariant::Null,
        ],
    )
    .unwrap_err();
    assert_eq!(err.number, vb6core::error::err_number::INVALID_USE_OF_NULL);

    // GetAutoServerSettings' parameters are declared As String: Null → 94.
    let err = call_builtin(
        "GetAutoServerSettings",
        &[
            VBVariant::from_string("progid"),
            VBVariant::from_string("clsid"),
            VBVariant::Null,
        ],
    )
    .unwrap_err();
    assert_eq!(err.number, vb6core::error::err_number::INVALID_USE_OF_NULL);

    // Error's optional number keeps a raw optional-Variant view: an explicit
    // Empty means "the current error number" while an explicit Null is
    // invalid use of Null.
    let _guard = ENV_DISPATCH_LOCK.lock().unwrap();
    vb6runtime::state::err::set_number(53);
    assert_eq!(
        call_builtin("Error", &[VBVariant::Empty]).unwrap(),
        VBVariant::from_string("File not found")
    );
    vb6runtime::state::err::clear();
    let err = call_builtin("Error$", &[VBVariant::Null]).unwrap_err();
    assert_eq!(err.number, vb6core::error::err_number::INVALID_USE_OF_NULL);

    // A CVErr argument re-raises its embedded error instead of being read as
    // a number or stringified.
    let err = call_builtin(
        "Error",
        &[VBVariant::from_error(vb6core::error::VBError::new(31337))],
    )
    .unwrap_err();
    assert_eq!(err.number, 31337);
}

#[test]
fn loadpicture_boundary_pins_optional_and_null_policy() {
    // The omitted argument means "an empty picture": a zero-sized StdPicture.
    let value = call_builtin("LoadPicture", &[]).unwrap();
    assert!(value.is_object());
    let pic = value.as_object().unwrap();
    assert_eq!(pic.type_name(), "StdPicture");

    // An explicit empty path unloads the picture: the result is Nothing.
    let value = call_builtin("LoadPicture", &[VBVariant::from_string("")]).unwrap();
    assert!(value.is_nothing());

    // A present Null rejects with 94 at the boundary (As String parameter).
    let err = call_builtin("LoadPicture", &[VBVariant::Null]).unwrap_err();
    assert_eq!(err.number, vb6core::error::err_number::INVALID_USE_OF_NULL);
}

#[test]
fn filter_dispatch() {
    let arr = call_builtin(
        "Filter",
        &[
            call_builtin(
                "Array",
                &[
                    VBVariant::from_string("apple"),
                    VBVariant::from_string("banana"),
                    VBVariant::from_string("cherry"),
                ],
            )
            .unwrap(),
            VBVariant::from_string("an"),
        ],
    )
    .unwrap();
    assert_eq!(
        call_builtin("UBound", std::slice::from_ref(&arr)).unwrap(),
        VBVariant::from_integer(0)
    );
    assert_eq!(
        call_builtin("Join", &[arr, VBVariant::from_string(" ")]).unwrap(),
        VBVariant::from_string("banana")
    );
}

#[test]
fn arrays_boundary_rejects_null_optionals_and_reports_structural_failures() {
    // Optional scalar parameters are typed: a present Null is error 94.
    let err = call_builtin("Split", &[VBVariant::from_string("a,b"), VBVariant::Null]).unwrap_err();
    assert_eq!(err.number, vb6core::error::err_number::INVALID_USE_OF_NULL);
    let err = call_builtin(
        "Filter",
        &[
            call_builtin("Array", &[VBVariant::from_string("x")]).unwrap(),
            VBVariant::from_string("x"),
            VBVariant::Null,
        ],
    )
    .unwrap_err();
    assert_eq!(err.number, vb6core::error::err_number::INVALID_USE_OF_NULL);

    // `sourcearray` stays structural: Null and non-arrays both fail the
    // Array-ness check with error 13 (no in-body propagation here).
    let err = call_builtin("LBound", &[VBVariant::Null]).unwrap_err();
    assert_eq!(err.number, vb6core::error::err_number::TYPE_MISMATCH);
    let err = call_builtin("LBound", &[VBVariant::from_long(7)]).unwrap_err();
    assert_eq!(err.number, vb6core::error::err_number::TYPE_MISMATCH);

    // Array() stores elements unconverted — even a Null element.
    let arr = call_builtin("Array", &[VBVariant::from_long(1), VBVariant::Null]).unwrap();
    assert!(matches!(arr, VBVariant::Array(_)));
}

#[test]
fn conversion_functions_dispatch() {
    assert_eq!(
        call_builtin("Hex", &[VBVariant::from_long(255)]).unwrap(),
        VBVariant::from_string("FF")
    );
    assert_eq!(
        call_builtin("Hex$", &[VBVariant::from_long(255)]).unwrap(),
        VBVariant::from_string("FF")
    );
    assert_eq!(
        call_builtin("Oct$", &[VBVariant::from_long(255)]).unwrap(),
        VBVariant::from_string("377")
    );
    assert_eq!(
        call_builtin("VarType", &[VBVariant::from_long(42)]).unwrap(),
        VBVariant::from_long(3)
    );
    assert_eq!(
        call_builtin("CVErr", &[VBVariant::from_integer(13)]).unwrap(),
        VBVariant::from_error(vb6core::error::VBError::new(err_number::TYPE_MISMATCH))
    );
}

#[test]
fn hex_oct_keep_variant_semantics() {
    // Type-aware bit widths require the raw variant (Integer -> u16,
    // Long -> u32); a typed Double parameter would erase the distinction.
    assert_eq!(
        call_builtin("Hex", &[VBVariant::Integer(-2)]).unwrap(),
        VBVariant::from_string("FFFE")
    );
    assert_eq!(
        call_builtin("Hex", &[VBVariant::Long(-2)]).unwrap(),
        VBVariant::from_string("FFFFFFFE")
    );
    // Fractional arguments round half-to-even before formatting.
    assert_eq!(
        call_builtin("Hex", &[VBVariant::from_double(2.5)]).unwrap(),
        VBVariant::from_string("2")
    );
    assert_eq!(
        call_builtin("Hex", &[VBVariant::from_double(3.5)]).unwrap(),
        VBVariant::from_string("4")
    );
    // Null propagation for the Variant-returning forms, 94 for `$` forms.
    assert_eq!(
        call_builtin("Hex", &[VBVariant::Null]).unwrap(),
        VBVariant::Null
    );
    let err = call_builtin("Hex$", &[VBVariant::Null]).unwrap_err();
    assert_eq!(err.number, err_number::INVALID_USE_OF_NULL);
    // CVErr re-raise survives the boundary passthrough.
    let err = call_builtin(
        "Hex",
        &[VBVariant::from_error(vb6core::error::VBError::new(7))],
    )
    .unwrap_err();
    assert_eq!(err.number, 7);
}

#[test]
fn objects_functions_dispatch() {
    assert_eq!(
        call_builtin("TypeName", &[VBVariant::from_integer(-5)]).unwrap(),
        VBVariant::from_string("Integer")
    );
    assert_eq!(
        call_builtin("TypeName", &[VBVariant::Null]).unwrap(),
        VBVariant::from_string("Null")
    );
}

#[test]
fn file_functions_keep_strict_raw_variant_semantics() {
    // The file functions deliberately do not use boundary coercion: a Long
    // path is not CStr-coerced, it is a type mismatch naming the function.
    let err = call_builtin("ChDir", &[VBVariant::from_long(42)]).unwrap_err();
    assert_eq!(err.number, 13);
    assert!(err.description.contains("Type mismatch in ChDir"));

    // Likewise a numeric string is not CLng-coerced into a file number.
    let err = call_builtin("EOF", &[VBVariant::from_string("1")]).unwrap_err();
    assert_eq!(err.number, 13);
    assert!(err.description.contains("Type mismatch in EOF"));

    // FreeFile's Null acts as the omitted argument (default range 0), and
    // neither form touches the filesystem, so no backend fixture is needed.
    let omitted = call_builtin("FreeFile", &[]).unwrap();
    let explicit_null = call_builtin("FreeFile", &[VBVariant::Null]).unwrap();
    assert_eq!(omitted, explicit_null);
}

#[test]
fn financial_functions_dispatch() {
    // DDB with default factor (2.0)
    let result = call_builtin(
        "DDB",
        &[
            VBVariant::from_double(10000.0),
            VBVariant::from_double(1000.0),
            VBVariant::from_double(5.0),
            VBVariant::from_double(1.0),
        ],
    )
    .unwrap();
    assert_eq!(result.as_f64().unwrap(), 4000.0);

    // DDB with custom factor (1.5)
    let result = call_builtin(
        "DDB",
        &[
            VBVariant::from_double(10000.0),
            VBVariant::from_double(1000.0),
            VBVariant::from_double(5.0),
            VBVariant::from_double(1.0),
            VBVariant::from_double(1.5),
        ],
    )
    .unwrap();
    assert_eq!(result.as_f64().unwrap(), 3000.0);

    // NPV: first cash flow discounted for one period
    let result = call_builtin(
        "NPV",
        &[
            VBVariant::from_double(0.1),
            VBVariant::Array(ArrayValue::from_vec_with_bounds(
                vb6core::types::VBType::Double,
                vec![
                    VBVariant::from_double(1000.0),
                    VBVariant::from_double(2000.0),
                    VBVariant::from_double(3000.0),
                ],
                0,
            )),
        ],
    )
    .unwrap();
    assert!((result.as_f64().unwrap() - 4815.93).abs() < 0.1);

    // RATE: $10,000 loan, $200/month for 5 years
    let result = call_builtin(
        "RATE",
        &[
            VBVariant::from_double(60.0),
            VBVariant::from_double(-200.0),
            VBVariant::from_double(10000.0),
        ],
    )
    .unwrap();
    assert!((result.as_f64().unwrap() - 0.0061834).abs() < 1e-6);

    // SLN: straight-line depreciation
    let result = call_builtin(
        "SLN",
        &[
            VBVariant::from_double(50000.0),
            VBVariant::from_double(5000.0),
            VBVariant::from_double(5.0),
        ],
    )
    .unwrap();
    assert_eq!(result.as_f64().unwrap(), 9000.0);

    // SYD: sum-of-years digits depreciation
    let result = call_builtin(
        "SYD",
        &[
            VBVariant::from_double(10000.0),
            VBVariant::from_double(1000.0),
            VBVariant::from_double(5.0),
            VBVariant::from_double(2.0),
        ],
    )
    .unwrap();
    assert_eq!(result.as_f64().unwrap(), 2400.0);
}

#[test]
fn financial_functions_propagate_null_policy_at_the_boundary() {
    // Required scalars are typed: a present Null is error 94 — including
    // Pmt, whose pre-migration body collapsed every conversion failure
    // (Null included) to type mismatch.
    let err = call_builtin(
        "Pmt",
        &[
            VBVariant::Null,
            VBVariant::from_double(12.0),
            VBVariant::from_double(1000.0),
        ],
    )
    .unwrap_err();
    assert_eq!(err.number, vb6core::error::err_number::INVALID_USE_OF_NULL);

    // Optional scalars stay present-Null → 94 too.
    let err = call_builtin(
        "FV",
        &[
            VBVariant::from_double(0.1),
            VBVariant::from_double(12.0),
            VBVariant::from_double(-100.0),
            VBVariant::Null,
        ],
    )
    .unwrap_err();
    assert_eq!(err.number, vb6core::error::err_number::INVALID_USE_OF_NULL);

    // Cash-flow arrays are structural: a non-array `values` is error 13.
    let err = call_builtin("NPV", &[VBVariant::from_double(0.1), VBVariant::Null]).unwrap_err();
    assert_eq!(err.number, vb6core::error::err_number::TYPE_MISMATCH);
}
