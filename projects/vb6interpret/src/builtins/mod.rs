//! Builtin function dispatch.
//!
//! Calls the implemented `vb6runtime` functions directly. A builtin that
//! `vb6runtime` does not implement yet raises an error instead of being
//! handled inline here.
//!
//! Dispatch is registry-based instead of one large `match`: each standard
//! library category registers its functions in its own submodule (`string`,
//! `math`, ...), and adding a function is a single [`Builtin`] entry. New
//! categories only need a new submodule plus one `register` call in
//! [`registry`].

mod arrays;
mod conversion;
mod datetime;
mod financial;
mod logic;
mod math;
mod string;
mod type_checking;

/// Build a [`Builtin`] registry entry from an adapter closure.
///
/// The closure receives the evaluated argument slice and must return a
/// `VBVariant`. Argument-count validation is performed by [`Registry::dispatch`]
/// using `min_args`/`max_args`.
#[macro_export]
macro_rules! builtin {
    ($name:literal, $min:expr, $max:expr, |$args:ident| $body:block) => {
        Builtin {
            name: $name,
            min_args: $min,
            max_args: $max,
            call: |$args: &[VBVariant]| -> VBResult<VBVariant> { $body },
        }
    };
}

use std::collections::HashMap;
use std::sync::OnceLock;

use vb6core::error::{VBError, VBResult};
use vb6runtime::value::{VBLong, VBString};
use vb6runtime::VBVariant;

/// A callable that adapts a slice of evaluated arguments into a runtime call.
type BuiltinFn = fn(&[VBVariant]) -> VBResult<VBVariant>;

/// One standard-library function: its name, arity, and the adapter that calls
/// the corresponding `vb6runtime` implementation.
struct Builtin {
    name: &'static str,
    min_args: usize,
    max_args: usize,
    call: BuiltinFn,
}

/// The collection of registered standard-library functions.
struct Registry {
    by_name: HashMap<&'static str, Builtin>,
}

impl Registry {
    fn new() -> Self {
        Self {
            by_name: HashMap::new(),
        }
    }

    fn insert(&mut self, builtin: Builtin) {
        self.by_name.insert(builtin.name, builtin);
    }

    /// Look up and invoke `name`, validating its argument count.
    ///
    /// Returns `None` when the function is not registered.
    fn dispatch(&self, name: &str, args: &[VBVariant]) -> Option<VBResult<VBVariant>> {
        self.by_name.get(name).map(|builtin| {
            if args.len() < builtin.min_args || args.len() > builtin.max_args {
                return Err(VBError::new(450));
            }
            (builtin.call)(args)
        })
    }
}

/// The lazily built registry of all standard-library functions.
fn registry() -> &'static Registry {
    static REGISTRY: OnceLock<Registry> = OnceLock::new();
    REGISTRY.get_or_init(|| {
        let mut registry = Registry::new();
        arrays::register(&mut registry);
        conversion::register(&mut registry);
        datetime::register(&mut registry);
        financial::register(&mut registry);
        logic::register(&mut registry);
        string::register(&mut registry);
        math::register(&mut registry);
        type_checking::register(&mut registry);
        registry
    })
}

/// Dispatch a builtin function call by name.
///
/// Returns error 35 with a descriptive message when the function is not
/// implemented by `vb6runtime` yet.
pub(crate) fn call_builtin(name: &str, args: &[VBVariant]) -> VBResult<VBVariant> {
    let normalized_name = builtin_name(name);
    registry()
        .dispatch(&normalized_name, args)
        .unwrap_or_else(|| {
            Err(VBError::with_description(
                35,
                format!("Function '{name}' is not implemented yet"),
            ))
        })
}

// ---- Argument helpers ----

/// Extract the argument at `index` as a string, erroring when the argument is
/// absent (450) or does not convert to a string.
fn arg_string(args: &[VBVariant], index: usize) -> VBResult<VBString> {
    args.get(index)
        .ok_or_else(|| VBError::new(450))
        .and_then(VBString::try_from)
}

/// Extract the argument at `index` as a `Long`, erroring when the argument is
/// absent (450) or does not convert to a `Long`.
fn arg_long(args: &[VBVariant], index: usize) -> VBResult<VBLong> {
    args.get(index)
        .ok_or_else(|| VBError::new(450))
        .and_then(VBLong::try_from)
}

/// Normalize a builtin name for case-insensitive lookup: lowercase, and strip
/// a trailing type-declaration suffix (`%&!#@`). The `$` string suffix is
/// preserved because `Left` and `Left$` are distinct functions.
fn builtin_name(name: &str) -> String {
    let trimmed = name.trim();
    trimmed
        .strip_suffix('%')
        .or_else(|| trimmed.strip_suffix('&'))
        .or_else(|| trimmed.strip_suffix('!'))
        .or_else(|| trimmed.strip_suffix('#'))
        .or_else(|| trimmed.strip_suffix('@'))
        .unwrap_or(trimmed)
        .to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;
    use vb6runtime::ArrayValue;

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
            call_builtin("IsError", &[VBVariant::from_error(vb6core::error::VBError::new(13))])
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

        let result = call_builtin(
            "DateValue",
            &[VBVariant::from_string("2/14/2025 10:30 AM")],
        )
        .unwrap();
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
            &[
                VBVariant::from_string("a,b,c"),
                VBVariant::from_string(","),
            ],
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
            VBVariant::from_error(vb6core::error::VBError::new(13))
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
    }
}
