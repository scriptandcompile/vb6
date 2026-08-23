//! Tests for builtin dispatch, argument coercion, and category helpers.
//!
//! Lives in its own file to keep `mod.rs` dispatch-only; declared as
//! `#[cfg(test)] mod tests;` so every item in the parent module stays
//! visible through `use super::*` exactly as when the suite was inline.

use super::*;
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
fn non_dollar_variants_propagate_null() {
    assert_eq!(
        call_builtin("Left", &[VBVariant::Null, VBVariant::Long(3)]).unwrap(),
        VBVariant::Null
    );
    assert_eq!(
        call_builtin("LCase", &[VBVariant::Null]).unwrap(),
        VBVariant::Null
    );
    assert_eq!(
        call_builtin("Trim", &[VBVariant::Null]).unwrap(),
        VBVariant::Null
    );
    assert_eq!(
        call_builtin("Mid", &[VBVariant::Null, VBVariant::Long(1)]).unwrap(),
        VBVariant::Null
    );
    assert_eq!(
        call_builtin("Chr", &[VBVariant::Null]).unwrap(),
        VBVariant::Null
    );
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
