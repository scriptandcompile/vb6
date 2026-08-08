//! Builtin function dispatch.
//!
//! Calls the implemented `vb6runtime` functions directly. A builtin that
//! `vb6runtime` does not implement yet raises an error instead of being
//! handled inline here.

use vb6core::error::{VBError, VBResult};
use vb6runtime::library::functions::string as strfn;
use vb6runtime::value::{VBLong, VBString};
use vb6runtime::VBVariant;

/// Dispatch a builtin function call by name.
///
/// Returns error 35 with a descriptive message when the function is not
/// implemented by `vb6runtime` yet.
pub(crate) fn call_builtin(name: &str, args: &[VBVariant]) -> VBResult<VBVariant> {
    let normalized_name = builtin_name(name);
    match normalized_name.as_str() {
        // ---- String functions (delegated to vb6runtime) ----
        "len" => {
            one_arg(name, args)?;
            let input = VBString::try_from(&args[0])?;
            let result = strfn::len(&input)?;
            Ok(VBVariant::from(result))
        }
        "left" => {
            two_args(name, args)?;
            let length = VBLong::try_from(&args[1])?;
            strfn::left(&args[0], &length)
        }
        "left$" => {
            two_args(name, args)?;
            let input = arg_string(args, 0)?;
            let length = VBLong::try_from(&args[1])?;
            strfn::left_dollar(&input, &length).map(VBVariant::from)
        }
        "right" => {
            two_args(name, args)?;
            let length = VBLong::try_from(&args[1])?;
            strfn::right(&args[0], &length)
        }
        "right$" => {
            two_args(name, args)?;
            let input = arg_string(args, 0)?;
            let length = VBLong::try_from(&args[1])?;
            strfn::right_dollar(&input, &length).map(VBVariant::from)
        }
        "mid" => {
            expect_args(name, args, 2, 3)?;
            let start = VBLong::try_from(&args[1])?;
            let length = args.get(2).map(VBLong::try_from).transpose()?;
            strfn::mid(&args[0], &start, length.as_ref())
        }
        "mid$" => {
            expect_args(name, args, 2, 3)?;
            let input = arg_string(args, 0)?;
            let start = VBLong::try_from(&args[1])?;
            let length = args.get(2).map(VBLong::try_from).transpose()?;
            strfn::mid_dollar(&input, &start, length.as_ref()).map(VBVariant::from)
        }
        "lcase" | "ucase" | "trim" | "ltrim" | "rtrim" => {
            one_arg(name, args)?;
            let result = match normalized_name.as_str() {
                "lcase" => strfn::lcase(&args[0])?,
                "ucase" => strfn::ucase(&args[0])?,
                "trim" => strfn::trim(&args[0])?,
                "ltrim" => strfn::ltrim(&args[0])?,
                _ => strfn::rtrim(&args[0])?,
            };
            Ok(result)
        }
        "lcase$" | "ucase$" | "trim$" | "ltrim$" | "rtrim$" => {
            one_arg(name, args)?;
            let input = arg_string(args, 0)?;
            let result = match normalized_name.as_str() {
                "lcase$" => strfn::lcase_dollar(&input)?,
                "ucase$" => strfn::ucase_dollar(&input)?,
                "trim$" => strfn::trim_dollar(&input)?,
                "ltrim$" => strfn::ltrim_dollar(&input)?,
                _ => strfn::rtrim_dollar(&input)?,
            };
            Ok(VBVariant::from(result))
        }
        "strreverse" => {
            one_arg(name, args)?;
            let arg0 = arg_string(args, 0)?;
            let result = strfn::strreverse(&arg0)?;
            Ok(VBVariant::from(result))
        }
        "asc" | "ascw" | "ascb" => {
            one_arg(name, args)?;
            let arg0 = arg_string(args, 0)?;
            let result = match normalized_name.as_str() {
                "asc" => strfn::asc(&arg0)?,
                "ascw" => strfn::ascw(&arg0)?,
                _ => strfn::ascb(&arg0)?,
            };
            Ok(VBVariant::from(result))
        }
        "chr" => {
            one_arg(name, args)?;
            strfn::chr(&args[0])
        }
        "chr$" => {
            one_arg(name, args)?;
            let arg0 = arg_long(args, 0)?;
            strfn::chr_dollar(&arg0).map(VBVariant::from)
        }
        "chrw" => {
            one_arg(name, args)?;
            strfn::chrw(&args[0])
        }
        "chrw$" => {
            one_arg(name, args)?;
            let arg0 = arg_long(args, 0)?;
            strfn::chrw_dollar(&arg0).map(VBVariant::from)
        }
        "chrb" => {
            one_arg(name, args)?;
            strfn::chrb(&args[0])
        }
        "chrb$" => {
            one_arg(name, args)?;
            let arg0 = arg_long(args, 0)?;
            strfn::chrb_dollar(&arg0).map(VBVariant::from)
        }
        "space" => {
            one_arg(name, args)?;
            let arg0 = arg_long(args, 0)?;
            strfn::space(&arg0)
        }
        "space$" => {
            one_arg(name, args)?;
            let arg0 = arg_long(args, 0)?;
            strfn::space_dollar(&arg0).map(VBVariant::from)
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
            Ok(VBVariant::from(result))
        }
        "instrrev" => {
            expect_args(name, args, 2, 4)?;
            let s1 = arg_string(args, 0)?;
            let s2 = arg_string(args, 1)?;
            let start = args.get(2).map(VBLong::try_from).transpose()?;
            let compare = args.get(3).map(VBLong::try_from).transpose()?;
            let result = strfn::instrrev(&s1, &s2, start.as_ref(), compare.as_ref())?;
            Ok(VBVariant::from(result))
        }
        "format" | "format$" => {
            expect_args(name, args, 1, 4)?;
            let format = args.get(1).map(VBString::try_from).transpose()?;
            let firstdayofweek = args.get(2).map(VBLong::try_from).transpose()?;
            let firstweekofyear = args.get(3).map(VBLong::try_from).transpose()?;
            let result = if normalized_name == "format$" {
                VBVariant::from(strfn::format_dollar(
                    &args[0],
                    format.as_ref(),
                    firstdayofweek.as_ref(),
                    firstweekofyear.as_ref(),
                )?)
            } else {
                strfn::format(
                    &args[0],
                    format.as_ref(),
                    firstdayofweek.as_ref(),
                    firstweekofyear.as_ref(),
                )?
            };
            Ok(result)
        }
        "lenb" => {
            one_arg(name, args)?;
            let input = VBString::try_from(&args[0])?;
            let result = strfn::lenb(&input)?;
            Ok(VBVariant::from(result))
        }
        "leftb" => {
            two_args(name, args)?;
            let length = VBLong::try_from(&args[1])?;
            strfn::leftb(&args[0], &length)
        }
        "leftb$" => {
            two_args(name, args)?;
            let input = arg_string(args, 0)?;
            let length = VBLong::try_from(&args[1])?;
            strfn::leftb_dollar(&input, &length).map(VBVariant::from)
        }
        "rightb" => {
            two_args(name, args)?;
            let length = VBLong::try_from(&args[1])?;
            strfn::rightb(&args[0], &length)
        }
        "rightb$" => {
            two_args(name, args)?;
            let input = arg_string(args, 0)?;
            let length = VBLong::try_from(&args[1])?;
            strfn::rightb_dollar(&input, &length).map(VBVariant::from)
        }
        "midb" => {
            expect_args(name, args, 2, 3)?;
            let start = VBLong::try_from(&args[1])?;
            let length = args.get(2).map(VBLong::try_from).transpose()?;
            strfn::midb(&args[0], &start, length.as_ref())
        }
        "midb$" => {
            expect_args(name, args, 2, 3)?;
            let input = arg_string(args, 0)?;
            let start = VBLong::try_from(&args[1])?;
            let length = args.get(2).map(VBLong::try_from).transpose()?;
            strfn::midb_dollar(&input, &start, length.as_ref()).map(VBVariant::from)
        }
        "str" | "str$" => {
            one_arg(name, args)?;
            if normalized_name == "str$" {
                strfn::str_dollar(&args[0]).map(VBVariant::from)
            } else {
                strfn::str(&args[0])
            }
        }
        "string" => {
            two_args(name, args)?;
            let number = VBLong::try_from(&args[0])?;
            strfn::string_function(&number, &args[1])
        }
        "string$" => {
            two_args(name, args)?;
            let number = VBLong::try_from(&args[0])?;
            strfn::string_dollar(&number, &args[1]).map(VBVariant::from)
        }
        "strcomp" => {
            expect_args(name, args, 2, 3)?;
            let s1 = arg_string(args, 0)?;
            let s2 = arg_string(args, 1)?;
            let compare = args.get(2).map(VBLong::try_from).transpose()?;
            let result = strfn::strcomp(&s1, &s2, compare.as_ref())?;
            Ok(VBVariant::from(result))
        }
        "strconv" => {
            expect_args(name, args, 2, 3)?;
            let conversion = VBLong::try_from(&args[1])?;
            let lcid = args.get(2).map(VBLong::try_from).transpose()?;
            strfn::strconv(&args[0], &conversion, lcid.as_ref())
        }
        "strconv$" => {
            expect_args(name, args, 2, 3)?;
            let input = arg_string(args, 0)?;
            let conversion = VBLong::try_from(&args[1])?;
            let lcid = args.get(2).map(VBLong::try_from).transpose()?;
            strfn::strconv_dollar(&input, &conversion, lcid.as_ref()).map(VBVariant::from)
        }
        "replace" => {
            expect_args(name, args, 3, 6)?;
            let start = args.get(3).map(VBLong::try_from).transpose()?;
            let count = args.get(4).map(VBLong::try_from).transpose()?;
            let compare = args.get(5).map(VBLong::try_from).transpose()?;
            strfn::replace(
                &args[0],
                &args[1],
                &args[2],
                start.as_ref(),
                count.as_ref(),
                compare.as_ref(),
            )
        }
        "formatnumber" => {
            expect_args(name, args, 1, 5)?;
            let digits = args.get(1).map(VBLong::try_from).transpose()?;
            let leading = args.get(2).map(VBLong::try_from).transpose()?;
            let parens = args.get(3).map(VBLong::try_from).transpose()?;
            let group = args.get(4).map(VBLong::try_from).transpose()?;
            strfn::formatnumber(&args[0], digits.as_ref(), leading.as_ref(), parens.as_ref(), group.as_ref())
        }
        "formatcurrency" => {
            expect_args(name, args, 1, 5)?;
            let digits = args.get(1).map(VBLong::try_from).transpose()?;
            let leading = args.get(2).map(VBLong::try_from).transpose()?;
            let parens = args.get(3).map(VBLong::try_from).transpose()?;
            let group = args.get(4).map(VBLong::try_from).transpose()?;
            strfn::formatcurrency(&args[0], digits.as_ref(), leading.as_ref(), parens.as_ref(), group.as_ref())
        }
        "formatpercent" => {
            expect_args(name, args, 1, 5)?;
            let digits = args.get(1).map(VBLong::try_from).transpose()?;
            let leading = args.get(2).map(VBLong::try_from).transpose()?;
            let parens = args.get(3).map(VBLong::try_from).transpose()?;
            let group = args.get(4).map(VBLong::try_from).transpose()?;
            strfn::formatpercent(&args[0], digits.as_ref(), leading.as_ref(), parens.as_ref(), group.as_ref())
        }
        "formatdatetime" => {
            expect_args(name, args, 1, 2)?;
            let namedformat = args.get(1).map(VBLong::try_from).transpose()?;
            strfn::formatdatetime(&args[0], namedformat.as_ref())
        }

        _ => Err(VBError::with_description(
            35,
            format!("Function '{name}' is not implemented yet"),
        )),
    }
}

// ---- Argument helpers ----

fn arg_string(args: &[VBVariant], index: usize) -> VBResult<VBString> {
    args.get(index)
        .ok_or_else(|| VBError::new(450))
        .and_then(VBString::try_from)
}

fn arg_long(args: &[VBVariant], index: usize) -> VBResult<VBLong> {
    args.get(index)
        .ok_or_else(|| VBError::new(450))
        .and_then(VBLong::try_from)
}

fn expect_args(name: &str, args: &[VBVariant], min: usize, max: usize) -> VBResult<()> {
    if args.len() < min || args.len() > max {
        let _ = name;
        return Err(VBError::new(450));
    }
    Ok(())
}

fn one_arg(name: &str, args: &[VBVariant]) -> VBResult<()> {
    expect_args(name, args, 1, 1)
}

fn two_args(name: &str, args: &[VBVariant]) -> VBResult<()> {
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
        let result =
            call_builtin("Left$", &[VBVariant::from_string("abcdef"), VBVariant::Long(3)]).unwrap();
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
        assert_eq!(call_builtin("Chr", &[VBVariant::Null]).unwrap(), VBVariant::Null);
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
}
