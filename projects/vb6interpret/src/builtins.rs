//! Builtin function dispatch.
//!
//! Calls the implemented `vb6runtime` functions directly. A builtin that
//! `vb6runtime` does not implement yet raises an error instead of being
//! handled inline here.

use vb6core::error::{VBError, VBResult};
use vb6runtime::library::functions::string as strfn;
use vb6runtime::Value;

/// Dispatch a builtin function call by name.
///
/// Returns error 35 with a descriptive message when the function is not
/// implemented by `vb6runtime` yet.
pub(crate) fn call_builtin(name: &str, args: &[Value]) -> VBResult<Value> {
    match name.to_lowercase().as_str() {
        // ---- String functions (delegated to vb6runtime) ----
        "len" => {
            one_arg(name, args)?;
            let arg0 = arg_string(args, 0)?;
            Ok(Value::from_long(strfn::len(&arg0)))
        }
        "left" => {
            two_args(name, args)?;
            let arg0 = arg_string(args, 0)?;
            let arg1 = arg_i32(args, 1)?;
            let s = strfn::left(&arg0, arg1)?;
            Ok(Value::from_string(s))
        }
        "right" => {
            two_args(name, args)?;
            let arg0 = arg_string(args, 0)?;
            let arg1 = arg_i32(args, 1)?;
            let s = strfn::right(&arg0, arg1)?;
            Ok(Value::from_string(s))
        }
        "mid" => {
            expect_args(name, args, 2, 3)?;
            let length = args.get(2).map(|a| a.as_i32()).transpose()?;
            let arg0 = arg_string(args, 0)?;
            let arg1 = arg_i32(args, 1)?;
            let s = strfn::mid(&arg0, arg1, length)?;
            Ok(Value::from_string(s))
        }
        "lcase" | "ucase" | "trim" | "ltrim" | "rtrim" | "strreverse" => {
            one_arg(name, args)?;
            let arg0 = arg_string(args, 0)?;
            let s = match name.to_lowercase().as_str() {
                "lcase" => strfn::lcase(&arg0),
                "ucase" => strfn::ucase(&arg0),
                "trim" => strfn::trim(&arg0),
                "ltrim" => strfn::ltrim(&arg0),
                "rtrim" => strfn::rtrim(&arg0),
                _ => strfn::strreverse(&arg0),
            };
            Ok(Value::from_string(s))
        }
        "asc" | "ascw" | "ascb" => {
            one_arg(name, args)?;
            let arg0 = arg_string(args, 0)?;
            let v = match name.to_lowercase().as_str() {
                "asc" => strfn::asc(&arg0)?,
                "ascw" => strfn::ascw(&arg0)?,
                _ => strfn::ascb(&arg0)?,
            };
            Ok(Value::from_long(v))
        }
        "chr" | "chrw" => {
            one_arg(name, args)?;
            let arg0 = arg_i32(args, 0)?;
            let s = match name.to_lowercase().as_str() {
                "chr" => strfn::chr(arg0)?,
                _ => strfn::chrw(arg0)?,
            };
            Ok(Value::from_string(s))
        }
        "space" => {
            one_arg(name, args)?;
            let arg0 = arg_i32(args, 0)?;
            let s = strfn::space(arg0)?;
            Ok(Value::from_string(s))
        }
        "instr" => {
            expect_args(name, args, 2, 4)?;
            let start: Option<i32>;
            let s1_idx;
            let s2_idx;
            let cmp_idx: Option<usize>;
            match args.len() {
                4 => {
                    start = Some(args[0].as_i32()?);
                    s1_idx = 1;
                    s2_idx = 2;
                    cmp_idx = Some(3);
                }
                3 => {
                    start = Some(args[0].as_i32()?);
                    s1_idx = 1;
                    s2_idx = 2;
                    cmp_idx = None;
                }
                _ => {
                    start = None;
                    s1_idx = 0;
                    s2_idx = 1;
                    cmp_idx = None;
                }
            }
            let s1 = arg_string(args, s1_idx)?;
            let s2 = arg_string(args, s2_idx)?;
            let compare = cmp_idx
                .and_then(|i| args.get(i))
                .map(|a| a.as_i32())
                .transpose()?;
            let v = strfn::instr(start, &s1, &s2, compare)?;
            Ok(Value::from_long(v))
        }

        _ => Err(VBError::with_description(
            35,
            format!("Function '{name}' is not implemented yet"),
        )),
    }
}

// ---- Argument helpers ----

fn arg_string(args: &[Value], index: usize) -> VBResult<String> {
    args.get(index)
        .ok_or_else(|| VBError::new(450))?
        .as_string()
}

fn arg_i32(args: &[Value], index: usize) -> VBResult<i32> {
    args.get(index).ok_or_else(|| VBError::new(450))?.as_i32()
}

fn expect_args(name: &str, args: &[Value], min: usize, max: usize) -> VBResult<()> {
    if args.len() < min || args.len() > max {
        let _ = name;
        return Err(VBError::new(450));
    }
    Ok(())
}

fn one_arg(name: &str, args: &[Value]) -> VBResult<()> {
    expect_args(name, args, 1, 1)
}

fn two_args(name: &str, args: &[Value]) -> VBResult<()> {
    expect_args(name, args, 2, 2)
}
