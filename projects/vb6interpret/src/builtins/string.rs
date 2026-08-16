//! VB6 string function registry.
//!
//! One [`Builtin`](super::Builtin) entry per string function, each wrapping the
//! typed `vb6runtime::library::string` implementation.

use super::{arg_long, arg_string, Builtin, Registry};
use crate::builtin;
use vb6core::error::VBResult;
use vb6runtime::library::string as strfn;
use vb6runtime::value::{VBLong, VBString};
use vb6runtime::VBVariant;

/// Register the string functions in `registry`.
pub(super) fn register(registry: &mut Registry) {
    registry.insert(builtin!("len", 1, 1, |args| {
        let input = VBString::try_from(&args[0])?;
        strfn::len(&input).map(VBVariant::from)
    }));
    registry.insert(builtin!("left", 2, 2, |args| {
        let length = VBLong::try_from(&args[1])?;
        strfn::left(&args[0], &length)
    }));
    registry.insert(builtin!("left$", 2, 2, |args| {
        let input = arg_string(args, 0)?;
        let length = VBLong::try_from(&args[1])?;
        strfn::left_dollar(&input, &length).map(VBVariant::from)
    }));
    registry.insert(builtin!("right", 2, 2, |args| {
        let length = VBLong::try_from(&args[1])?;
        strfn::right(&args[0], &length)
    }));
    registry.insert(builtin!("right$", 2, 2, |args| {
        let input = arg_string(args, 0)?;
        let length = VBLong::try_from(&args[1])?;
        strfn::right_dollar(&input, &length).map(VBVariant::from)
    }));
    registry.insert(builtin!("mid", 2, 3, |args| {
        let start = VBLong::try_from(&args[1])?;
        let length = args.get(2).map(VBLong::try_from).transpose()?;
        strfn::mid(&args[0], &start, length.as_ref())
    }));
    registry.insert(builtin!("mid$", 2, 3, |args| {
        let input = arg_string(args, 0)?;
        let start = VBLong::try_from(&args[1])?;
        let length = args.get(2).map(VBLong::try_from).transpose()?;
        strfn::mid_dollar(&input, &start, length.as_ref()).map(VBVariant::from)
    }));
    registry.insert(builtin!("lcase", 1, 1, |args| { strfn::lcase(&args[0]) }));
    registry.insert(builtin!("ucase", 1, 1, |args| { strfn::ucase(&args[0]) }));
    registry.insert(builtin!("trim", 1, 1, |args| { strfn::trim(&args[0]) }));
    registry.insert(builtin!("ltrim", 1, 1, |args| { strfn::ltrim(&args[0]) }));
    registry.insert(builtin!("rtrim", 1, 1, |args| { strfn::rtrim(&args[0]) }));
    registry.insert(builtin!("lcase$", 1, 1, |args| {
        strfn::lcase_dollar(&arg_string(args, 0)?).map(VBVariant::from)
    }));
    registry.insert(builtin!("ucase$", 1, 1, |args| {
        strfn::ucase_dollar(&arg_string(args, 0)?).map(VBVariant::from)
    }));
    registry.insert(builtin!("trim$", 1, 1, |args| {
        strfn::trim_dollar(&arg_string(args, 0)?).map(VBVariant::from)
    }));
    registry.insert(builtin!("ltrim$", 1, 1, |args| {
        strfn::ltrim_dollar(&arg_string(args, 0)?).map(VBVariant::from)
    }));
    registry.insert(builtin!("rtrim$", 1, 1, |args| {
        strfn::rtrim_dollar(&arg_string(args, 0)?).map(VBVariant::from)
    }));
    registry.insert(builtin!("strreverse", 1, 1, |args| {
        strfn::strreverse(&arg_string(args, 0)?).map(VBVariant::from)
    }));
    registry.insert(builtin!("asc", 1, 1, |args| {
        strfn::asc(&arg_string(args, 0)?).map(VBVariant::from)
    }));
    registry.insert(builtin!("ascw", 1, 1, |args| {
        strfn::ascw(&arg_string(args, 0)?).map(VBVariant::from)
    }));
    registry.insert(builtin!("ascb", 1, 1, |args| {
        strfn::ascb(&arg_string(args, 0)?).map(VBVariant::from)
    }));
    registry.insert(builtin!("chr", 1, 1, |args| { strfn::chr(&args[0]) }));
    registry.insert(builtin!("chr$", 1, 1, |args| {
        strfn::chr_dollar(&arg_long(args, 0)?).map(VBVariant::from)
    }));
    registry.insert(builtin!("chrw", 1, 1, |args| { strfn::chrw(&args[0]) }));
    registry.insert(builtin!("chrw$", 1, 1, |args| {
        strfn::chrw_dollar(&arg_long(args, 0)?).map(VBVariant::from)
    }));
    registry.insert(builtin!("chrb", 1, 1, |args| { strfn::chrb(&args[0]) }));
    registry.insert(builtin!("chrb$", 1, 1, |args| {
        strfn::chrb_dollar(&arg_long(args, 0)?).map(VBVariant::from)
    }));
    registry.insert(builtin!("space", 1, 1, |args| {
        strfn::space(&arg_long(args, 0)?)
    }));
    registry.insert(builtin!("space$", 1, 1, |args| {
        strfn::space_dollar(&arg_long(args, 0)?).map(VBVariant::from)
    }));
    registry.insert(builtin!("instr", 2, 4, |args| {
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
        strfn::instr(start.as_ref(), &s1, &s2, compare.as_ref()).map(VBVariant::from)
    }));
    registry.insert(builtin!("instrrev", 2, 4, |args| {
        let s1 = arg_string(args, 0)?;
        let s2 = arg_string(args, 1)?;
        let start = args.get(2).map(VBLong::try_from).transpose()?;
        let compare = args.get(3).map(VBLong::try_from).transpose()?;
        strfn::instrrev(&s1, &s2, start.as_ref(), compare.as_ref()).map(VBVariant::from)
    }));
    registry.insert(builtin!("format", 1, 4, |args| {
        let format = args.get(1).map(VBString::try_from).transpose()?;
        let firstdayofweek = args.get(2).map(VBLong::try_from).transpose()?;
        let firstweekofyear = args.get(3).map(VBLong::try_from).transpose()?;
        strfn::format(
            &args[0],
            format.as_ref(),
            firstdayofweek.as_ref(),
            firstweekofyear.as_ref(),
        )
    }));
    registry.insert(builtin!("format$", 1, 4, |args| {
        let format = args.get(1).map(VBString::try_from).transpose()?;
        let firstdayofweek = args.get(2).map(VBLong::try_from).transpose()?;
        let firstweekofyear = args.get(3).map(VBLong::try_from).transpose()?;
        strfn::format_dollar(
            &args[0],
            format.as_ref(),
            firstdayofweek.as_ref(),
            firstweekofyear.as_ref(),
        )
        .map(VBVariant::from)
    }));
    registry.insert(builtin!("lenb", 1, 1, |args| {
        let input = VBString::try_from(&args[0])?;
        strfn::lenb(&input).map(VBVariant::from)
    }));
    registry.insert(builtin!("leftb", 2, 2, |args| {
        let length = VBLong::try_from(&args[1])?;
        strfn::leftb(&args[0], &length)
    }));
    registry.insert(builtin!("leftb$", 2, 2, |args| {
        let input = arg_string(args, 0)?;
        let length = VBLong::try_from(&args[1])?;
        strfn::leftb_dollar(&input, &length).map(VBVariant::from)
    }));
    registry.insert(builtin!("rightb", 2, 2, |args| {
        let length = VBLong::try_from(&args[1])?;
        strfn::rightb(&args[0], &length)
    }));
    registry.insert(builtin!("rightb$", 2, 2, |args| {
        let input = arg_string(args, 0)?;
        let length = VBLong::try_from(&args[1])?;
        strfn::rightb_dollar(&input, &length).map(VBVariant::from)
    }));
    registry.insert(builtin!("midb", 2, 3, |args| {
        let start = VBLong::try_from(&args[1])?;
        let length = args.get(2).map(VBLong::try_from).transpose()?;
        strfn::midb(&args[0], &start, length.as_ref())
    }));
    registry.insert(builtin!("midb$", 2, 3, |args| {
        let input = arg_string(args, 0)?;
        let start = VBLong::try_from(&args[1])?;
        let length = args.get(2).map(VBLong::try_from).transpose()?;
        strfn::midb_dollar(&input, &start, length.as_ref()).map(VBVariant::from)
    }));
    registry.insert(builtin!("str", 1, 1, |args| { strfn::str(&args[0]) }));
    registry.insert(builtin!("str$", 1, 1, |args| {
        strfn::str_dollar(&args[0]).map(VBVariant::from)
    }));
    registry.insert(builtin!("string", 2, 2, |args| {
        let number = VBLong::try_from(&args[0])?;
        strfn::string_function(&number, &args[1])
    }));
    registry.insert(builtin!("string$", 2, 2, |args| {
        let number = VBLong::try_from(&args[0])?;
        strfn::string_dollar(&number, &args[1]).map(VBVariant::from)
    }));
    registry.insert(builtin!("strcomp", 2, 3, |args| {
        let s1 = arg_string(args, 0)?;
        let s2 = arg_string(args, 1)?;
        let compare = args.get(2).map(VBLong::try_from).transpose()?;
        strfn::strcomp(&s1, &s2, compare.as_ref()).map(VBVariant::from)
    }));
    registry.insert(builtin!("strconv", 2, 3, |args| {
        let conversion = VBLong::try_from(&args[1])?;
        let lcid = args.get(2).map(VBLong::try_from).transpose()?;
        strfn::strconv(&args[0], &conversion, lcid.as_ref())
    }));
    registry.insert(builtin!("strconv$", 2, 3, |args| {
        let input = arg_string(args, 0)?;
        let conversion = VBLong::try_from(&args[1])?;
        let lcid = args.get(2).map(VBLong::try_from).transpose()?;
        strfn::strconv_dollar(&input, &conversion, lcid.as_ref()).map(VBVariant::from)
    }));
    registry.insert(builtin!("replace", 3, 6, |args| {
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
    }));
    registry.insert(builtin!("formatnumber", 1, 5, |args| {
        let digits = args.get(1).map(VBLong::try_from).transpose()?;
        let leading = args.get(2).map(VBLong::try_from).transpose()?;
        let parens = args.get(3).map(VBLong::try_from).transpose()?;
        let group = args.get(4).map(VBLong::try_from).transpose()?;
        strfn::formatnumber(
            &args[0],
            digits.as_ref(),
            leading.as_ref(),
            parens.as_ref(),
            group.as_ref(),
        )
    }));
    registry.insert(builtin!("formatcurrency", 1, 5, |args| {
        let digits = args.get(1).map(VBLong::try_from).transpose()?;
        let leading = args.get(2).map(VBLong::try_from).transpose()?;
        let parens = args.get(3).map(VBLong::try_from).transpose()?;
        let group = args.get(4).map(VBLong::try_from).transpose()?;
        strfn::formatcurrency(
            &args[0],
            digits.as_ref(),
            leading.as_ref(),
            parens.as_ref(),
            group.as_ref(),
        )
    }));
    registry.insert(builtin!("formatpercent", 1, 5, |args| {
        let digits = args.get(1).map(VBLong::try_from).transpose()?;
        let leading = args.get(2).map(VBLong::try_from).transpose()?;
        let parens = args.get(3).map(VBLong::try_from).transpose()?;
        let group = args.get(4).map(VBLong::try_from).transpose()?;
        strfn::formatpercent(
            &args[0],
            digits.as_ref(),
            leading.as_ref(),
            parens.as_ref(),
            group.as_ref(),
        )
    }));
    registry.insert(builtin!("formatdatetime", 1, 2, |args| {
        let namedformat = args.get(1).map(VBLong::try_from).transpose()?;
        strfn::formatdatetime(&args[0], namedformat.as_ref())
    }));
}
