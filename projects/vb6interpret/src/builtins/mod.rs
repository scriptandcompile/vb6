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
use vb6runtime::value::VBVariant;

/// Declarative builtin signature spec.
///
/// One line declares a function's name, arity bounds, and every parameter's
/// kind; the expansion emits the `&[VBVariant] -> typed-call` glue, converting
/// arguments left-to-right through [`vb6runtime::boundary`] so coercion logic
/// exists once (in the generator + boundary helpers), not per entry.
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
/// | `propstring`  | propagating string | like `string`, but a `Null`
///   argument short-circuits the call to a `Null` result (VB6 documents Null
///   propagation for the non-`$` forms) |
/// | `propdouble`  | propagating number | like `string`→`VBDouble`: a `Null`
///   argument short-circuits to `Null`; anything else coerces with `CDbl`
///   semantics |
/// | `propdate`    | propagating date   | like `string`→`VBDate`: a `Null`
///   argument short-circuits to `Null`; anything else coerces with
///   `as_date_serial` semantics |
/// | `propboolean` | propagating bool   | like `string`→`VBBoolean`: a `Null`
///   argument short-circuits to `Null`; anything else coerces with `CBool`
///   semantics |
/// | `proplong`    | propagating long   | like `string`→`VBLong`: a `Null`
///   argument short-circuits to `Null`; anything else coerces with `CLng`
///   semantics |
///
/// A trailing `rest $name: variants` clause (after the parameter tuple)
/// binds the remaining arguments as `&[VBVariant]` for variadic functions.
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
             call: |_args: &[::vb6runtime::VBVariant]| -> ::vb6core::error::VBResult<::vb6runtime::VBVariant> {
                 let mut __arg_index = 0usize;
                 let __result: ::vb6core::error::VBResult<::vb6runtime::VBVariant> = {
                     $(
                         let __param_name = stringify!($param);
                         let $param =
                             $crate::__convert_arg!(_args, __arg_index, $kind)
                                 .map_err(|mut __e| {
                                     __e.param_index = Some(__arg_index);
                                     __e.param_name = Some(__param_name.to_string());
                                     __e
                                 })?;
                         __arg_index += 1;
                     )*
                     $body
                 };
                __result
             },
         }
     };
    // Variadic-tail form: after the fixed parameters, `rest $name: variants`
    // binds every remaining argument as an unconverted slice (for functions
    // like Switch whose evaluation semantics walk the raw argument list).
    ($name:literal, $min:expr, $max:expr,
      ($($param:ident : $kind:ident),* $(,)?),
      rest $rest:ident : variants,
      $body:expr) => {
         Builtin {
             name: $name,
             min_args: $min,
             max_args: $max,
             call: |_args: &[::vb6runtime::VBVariant]| -> ::vb6core::error::VBResult<::vb6runtime::VBVariant> {
                 let mut __arg_index = 0usize;
                 let __result: ::vb6core::error::VBResult<::vb6runtime::VBVariant> = {
                     $(
                         let __param_name = stringify!($param);
                         let $param =
                             $crate::__convert_arg!(_args, __arg_index, $kind)
                                 .map_err(|mut __e| {
                                     __e.param_index = Some(__arg_index);
                                     __e.param_name = Some(__param_name.to_string());
                                     __e
                                 })?;
                         __arg_index += 1;
                     )*
                     let $rest = &_args[__arg_index..];
                     $body
                 };
                __result
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
    ($args:ident, $index:expr, propstring) => {{
        if matches!($args.get($index), Some(::vb6runtime::VBVariant::Null)) {
            return Ok(::vb6runtime::VBVariant::Null);
        }
        ::vb6runtime::boundary::arg::<::vb6runtime::value::VBString>($args, $index)
    }};
    ($args:ident, $index:expr, propdouble) => {{
        if matches!($args.get($index), Some(::vb6runtime::VBVariant::Null)) {
            return Ok(::vb6runtime::VBVariant::Null);
        }
        ::vb6runtime::boundary::arg::<::vb6runtime::value::VBDouble>($args, $index)
    }};
    ($args:ident, $index:expr, propdate) => {{
        if matches!($args.get($index), Some(::vb6runtime::VBVariant::Null)) {
            return Ok(::vb6runtime::VBVariant::Null);
        }
        ::vb6runtime::boundary::arg::<::vb6runtime::value::VBDate>($args, $index)
    }};
    ($args:ident, $index:expr, propboolean) => {{
        if matches!($args.get($index), Some(::vb6runtime::VBVariant::Null)) {
            return Ok(::vb6runtime::VBVariant::Null);
        }
        ::vb6runtime::boundary::arg::<::vb6runtime::value::VBBoolean>($args, $index)
    }};
    ($args:ident, $index:expr, proplong) => {{
        if matches!($args.get($index), Some(::vb6runtime::VBVariant::Null)) {
            return Ok(::vb6runtime::VBVariant::Null);
        }
        ::vb6runtime::boundary::arg::<::vb6runtime::value::VBLong>($args, $index)
    }};
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
