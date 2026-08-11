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

mod math;
mod string;

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
        string::register(&mut registry);
        math::register(&mut registry);
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
}
