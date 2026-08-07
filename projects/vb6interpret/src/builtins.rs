//! Builtin function dispatch.
//!
//! Calls the implemented `vb6runtime` functions directly. A builtin that
//! `vb6runtime` does not implement yet raises an error instead of being
//! handled inline here.

use vb6core::error::{VBError, VBResult};
use vb6runtime::library::functions::string as strfn;
use vb6runtime::library::functions::string::chrb_dollar::chrb_dollar;
use vb6runtime::value::{VBLong, VBString};
use vb6runtime::Value;

/// Dispatch a builtin function call by name.
///
/// Returns error 35 with a descriptive message when the function is not
/// implemented by `vb6runtime` yet.
pub(crate) fn call_builtin(name: &str, args: &[Value]) -> VBResult<Value> {
    let normalized_name = builtin_name(name);
    match normalized_name.as_str() {
        // ---- String functions (delegated to vb6runtime) ----
        "len" => {
            one_arg(name, args)?;
            let input = VBString::try_from(&args[0])?;
            let result = strfn::len(&input)?;
            Ok(Value::from(result))
        }
        "left" | "left$" => {
            two_args(name, args)?;
            let input = VBString::try_from(&args[0])?;
            let length = VBLong::try_from(&args[1])?;
            let result = strfn::left(&input, &length)?;
            Ok(Value::from(result))
        }
        "right" | "right$" => {
            two_args(name, args)?;
            let input = VBString::try_from(&args[0])?;
            let length = VBLong::try_from(&args[1])?;
            let result = strfn::right(&input, &length)?;
            Ok(Value::from(result))
        }
        "mid" | "mid$" => {
            expect_args(name, args, 2, 3)?;
            let input = VBString::try_from(&args[0])?;
            let start = VBLong::try_from(&args[1])?;
            let length = args.get(2).map(VBLong::try_from).transpose()?;
            let result = strfn::mid(&input, &start, length.as_ref())?;
            Ok(Value::from(result))
        }
        "lcase" | "lcase$" | "ucase" | "ucase$" | "trim" | "trim$" | "ltrim" | "ltrim$"
        | "rtrim" | "rtrim$" | "strreverse" => {
            one_arg(name, args)?;
            let arg0 = VBString::try_from(&args[0])?;
            let key = normalized_name
                .strip_suffix('$')
                .unwrap_or(&normalized_name);
            let result = match key {
                "lcase" => strfn::lcase(&arg0)?,
                "ucase" => strfn::ucase(&arg0)?,
                "trim" => strfn::trim(&arg0)?,
                "ltrim" => strfn::ltrim(&arg0)?,
                "rtrim" => strfn::rtrim(&arg0)?,
                _ => strfn::strreverse(&arg0)?,
            };
            Ok(result.into())
        }
        "asc" | "ascw" | "ascb" => {
            one_arg(name, args)?;
            let arg0 = arg_string(args, 0)?;
            let result = match normalized_name.as_str() {
                "asc" => strfn::asc(&arg0)?,
                "ascw" => strfn::ascw(&arg0)?,
                _ => strfn::ascb(&arg0)?,
            };
            Ok(Value::from(result))
        }
        "chr" | "chr$" | "chrw" | "chrw$" | "chrb" | "chrb$" => {
            one_arg(name, args)?;
            let arg0 = arg_long(args, 0)?;
            let key = normalized_name
                .strip_suffix('$')
                .unwrap_or(&normalized_name);
            let result = match key {
                "chr" => strfn::chr(&arg0)?,
                "chrb" => chrb_dollar(&arg0)?,
                _ => strfn::chrw(&arg0)?,
            };
            Ok(Value::from(result))
        }
        "space" | "space$" => {
            one_arg(name, args)?;
            let arg0 = arg_long(args, 0)?;
            let result = strfn::space(&arg0)?;
            Ok(Value::from(result))
        }
        "instr" => {
            expect_args(name, args, 2, 4)?;
            let start: Option<VBLong>;
            let s1_idx;
            let s2_idx;
            let cmp_idx: Option<usize>;
            match args.len() {
                4 => {
                    start = Some(VBLong::try_from(&args[0])?);
                    s1_idx = 1;
                    s2_idx = 2;
                    cmp_idx = Some(3);
                }
                3 => {
                    start = Some(VBLong::try_from(&args[0])?);
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
                .and_then(|index| args.get(index))
                .map(VBLong::try_from)
                .transpose()?;
            let result = strfn::instr(start.as_ref(), &s1, &s2, compare.as_ref())?;
            Ok(Value::from(result))
        }
        "instrrev" => {
            expect_args(name, args, 2, 4)?;
            let s1 = arg_string(args, 0)?;
            let s2 = arg_string(args, 1)?;
            let start = args.get(2).map(VBLong::try_from).transpose()?;
            let compare = args.get(3).map(VBLong::try_from).transpose()?;
            let result = strfn::instrrev(&s1, &s2, start.as_ref(), compare.as_ref())?;
            Ok(Value::from(result))
        }
        "format" | "format$" => {
            expect_args(name, args, 1, 4)?;
            let format = args.get(1).map(VBString::try_from).transpose()?;
            let firstdayofweek = args.get(2).map(VBLong::try_from).transpose()?;
            let firstweekofyear = args.get(3).map(VBLong::try_from).transpose()?;
            let result = if normalized_name == "format$" {
                strfn::format_dollar(
                    &args[0],
                    format.as_ref(),
                    firstdayofweek.as_ref(),
                    firstweekofyear.as_ref(),
                )?
            } else {
                strfn::format(
                    &args[0],
                    format.as_ref(),
                    firstdayofweek.as_ref(),
                    firstweekofyear.as_ref(),
                )?
            };
            Ok(Value::from(result))
        }

        _ => Err(VBError::with_description(
            35,
            format!("Function '{name}' is not implemented yet"),
        )),
    }
}

// ---- Argument helpers ----

fn arg_string(args: &[Value], index: usize) -> VBResult<VBString> {
    args.get(index)
        .ok_or_else(|| VBError::new(450))
        .and_then(VBString::try_from)
}

fn arg_long(args: &[Value], index: usize) -> VBResult<VBLong> {
    args.get(index)
        .ok_or_else(|| VBError::new(450))
        .and_then(VBLong::try_from)
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
            Value::Double(1234.5),
            Value::from(VBString::from("#,##0.00")),
        ];
        let result = call_builtin("Format$", &args).unwrap();
        assert_eq!(result.as_string().unwrap(), "1,234.50");
        let result = call_builtin("Format", &args).unwrap();
        assert_eq!(result.as_string().unwrap(), "1,234.50");
    }

    #[test]
    fn dollar_variants_share_string_implementations() {
        let result =
            call_builtin("Left$", &[Value::from_string("abcdef"), Value::Long(3)]).unwrap();
        assert_eq!(result.as_string().unwrap(), "abc");
        let result = call_builtin("LCase$", &[Value::from_string("ABC")]).unwrap();
        assert_eq!(result.as_string().unwrap(), "abc");
        let result = call_builtin("Chr$", &[Value::Long(65)]).unwrap();
        assert_eq!(result.as_string().unwrap(), "A");
    }
}
