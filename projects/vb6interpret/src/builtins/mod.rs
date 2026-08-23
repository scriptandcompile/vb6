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
mod environment;
mod file;
mod financial;
mod graphics;
mod interaction;
mod logic;
mod math;
mod objects;
mod resources;
mod string;
mod type_checking;

#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::sync::OnceLock;

use vb6core::error::{err_number, VBError, VBResult};
use vb6runtime::value::{VBString, VBVariant};

/// Build a [`Builtin`] registry entry from an adapter closure.
///
/// Transitional form: prefer [`typed_builtin!`] for functions whose runtime
/// implementation already declares typed parameters; this closure form remains
/// for entries with Variant passthrough or arity-dependent argument positions.
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

/// Declarative builtin signature spec (plan B2).
///
/// One line declares a function's name, arity bounds, and every parameter's
/// kind; the expansion emits the `&[VBVariant] -> typed-call` glue, converting
/// arguments left-to-right through [`vb6runtime::boundary`] so coercion logic
/// exists once (in the generator + boundary helpers), not per entry. The
/// legacy [`builtin!`] form remains only for entries whose runtime
/// implementation is not typed yet (plan phases 1..N migrate them over).
///
/// Parameter kinds map to VB6 declared types and to the wrapper converted at
/// the boundary:
///
/// | kind          | VB6 type      | value produced            |
/// |---------------|---------------|---------------------------|
/// | `variant`     | `As Variant`  | `&VBVariant` (no coerce)  |
/// | `opt_variant` | optional Var. | `Option<&VBVariant>`      |
/// | `string`      | `As String`   | `VBString`                |
/// | `long`        | `As Long`     | `VBLong`                  |
/// | `integer`     | `As Integer`  | `VBInteger`               |
/// | `byte`        | `As Byte`     | `VBByte`                  |
/// | `boolean`     | `As Boolean`  | `VBBoolean`               |
/// | `single`      | `As Single`   | `VBSingle`                |
/// | `double`      | `As Double`   | `VBDouble`                |
/// | `currency`    | `As Currency` | `VBCurrency`              |
/// | `date`        | `As Date`     | `VBDate`                  |
/// | `opt_<t>`     | optional `t`  | `None` when absent        |
///
/// The body receives one binding per declared parameter (in order) and must
/// produce `VBResult<VBVariant>` (map wrapper returns with `VBVariant::from`).
///
/// ```ignore
/// registry.insert(typed_builtin!("left$", 2, 2,
///     (input: string, length: long),
///     strfn::left_dollar(&input, &length).map(VBVariant::from)));
/// ```
#[macro_export]
macro_rules! typed_builtin {
    ($name:literal, $min:expr, $max:expr,
     ($($param:ident : $kind:ident),* $(,)?),
     $body:expr) => {
        Builtin {
            name: $name,
            min_args: $min,
            max_args: $max,
            call: |args: &[::vb6runtime::VBVariant]| -> ::vb6core::error::VBResult<::vb6runtime::VBVariant> {
                // Left-to-right conversion order (plan A9): one statement per
                // declared parameter, each converting the argument at the
                // running index, so the leftmost offending argument errors
                // first.
                let mut __arg_index = 0usize;
                $(
                    let $param =
                        $crate::__convert_arg!(args, __arg_index, $kind)?;
                    __arg_index += 1;
                )*
                $body
            },
        }
    };
}

/// Kind dispatch for [`typed_builtin!`]: converts the argument at `$index`
/// according to the declared parameter kind. Internal — do not invoke directly.
#[macro_export]
#[doc(hidden)]
macro_rules! __convert_arg {
    ($args:ident, $index:expr, variant) => {
        ::vb6runtime::boundary::variant_arg($args, $index)
    };
    ($args:ident, $index:expr, opt_variant) => {
        Ok::<_, ::vb6core::error::VBError>(::vb6runtime::boundary::opt_variant_arg($args, $index))
    };
    ($args:ident, $index:expr, string) => {
        ::vb6runtime::boundary::arg::<::vb6runtime::value::VBString>($args, $index)
    };
    ($args:ident, $index:expr, long) => {
        ::vb6runtime::boundary::arg::<::vb6runtime::value::VBLong>($args, $index)
    };
    ($args:ident, $index:expr, integer) => {
        ::vb6runtime::boundary::arg::<::vb6runtime::value::VBInteger>($args, $index)
    };
    ($args:ident, $index:expr, byte) => {
        ::vb6runtime::boundary::arg::<::vb6runtime::value::VBByte>($args, $index)
    };
    ($args:ident, $index:expr, boolean) => {
        ::vb6runtime::boundary::arg::<::vb6runtime::value::VBBoolean>($args, $index)
    };
    ($args:ident, $index:expr, single) => {
        ::vb6runtime::boundary::arg::<::vb6runtime::value::VBSingle>($args, $index)
    };
    ($args:ident, $index:expr, double) => {
        ::vb6runtime::boundary::arg::<::vb6runtime::value::VBDouble>($args, $index)
    };
    ($args:ident, $index:expr, currency) => {
        ::vb6runtime::boundary::arg::<::vb6runtime::value::VBCurrency>($args, $index)
    };
    ($args:ident, $index:expr, date) => {
        ::vb6runtime::boundary::arg::<::vb6runtime::value::VBDate>($args, $index)
    };
    ($args:ident, $index:expr, opt_string) => {
        ::vb6runtime::boundary::opt_arg::<::vb6runtime::value::VBString>($args, $index)
    };
    ($args:ident, $index:expr, opt_long) => {
        ::vb6runtime::boundary::opt_arg::<::vb6runtime::value::VBLong>($args, $index)
    };
    ($args:ident, $index:expr, opt_integer) => {
        ::vb6runtime::boundary::opt_arg::<::vb6runtime::value::VBInteger>($args, $index)
    };
    ($args:ident, $index:expr, opt_byte) => {
        ::vb6runtime::boundary::opt_arg::<::vb6runtime::value::VBByte>($args, $index)
    };
    ($args:ident, $index:expr, opt_boolean) => {
        ::vb6runtime::boundary::opt_arg::<::vb6runtime::value::VBBoolean>($args, $index)
    };
    ($args:ident, $index:expr, opt_single) => {
        ::vb6runtime::boundary::opt_arg::<::vb6runtime::value::VBSingle>($args, $index)
    };
    ($args:ident, $index:expr, opt_double) => {
        ::vb6runtime::boundary::opt_arg::<::vb6runtime::value::VBDouble>($args, $index)
    };
    ($args:ident, $index:expr, opt_currency) => {
        ::vb6runtime::boundary::opt_arg::<::vb6runtime::value::VBCurrency>($args, $index)
    };
    ($args:ident, $index:expr, opt_date) => {
        ::vb6runtime::boundary::opt_arg::<::vb6runtime::value::VBDate>($args, $index)
    };
}

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
                return Err(VBError::new(err_number::WRONG_NUMBER_OF_ARGUMENTS));
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
        environment::register(&mut registry);
        file::register(&mut registry);
        financial::register(&mut registry);
        graphics::register(&mut registry);
        logic::register(&mut registry);
        interaction::register(&mut registry);
        string::register(&mut registry);
        math::register(&mut registry);
        objects::register(&mut registry);
        resources::register(&mut registry);
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
        .ok_or_else(|| VBError::new(err_number::WRONG_NUMBER_OF_ARGUMENTS))
        .and_then(VBString::try_from)
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
