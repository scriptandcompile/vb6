//! VB6 string function registry.
//!
//! One registry entry per string function. Entries wrap the typed
//! `vb6runtime::library::string` implementation; those with already-typed
//! runtime parameters use the declarative [`typed_builtin!`](crate::typed_builtin)
//! spec, while entries still passing raw variants through (non-`$` forms whose
//! runtime side takes `&VBVariant`, and `instr`'s arity-dependent positions)
//! keep hand-written adapters until plan phase 1 migrates them.

use super::{arg_string, Builtin, Registry};
use crate::builtin;
use crate::typed_builtin;
use vb6core::error::VBResult;
use vb6runtime::library::string as strfn;
use vb6runtime::value::VBLong;
use vb6runtime::VBVariant;

/// Register the string functions in `registry`.
pub(super) fn register(registry: &mut Registry) {
    registry.insert(typed_builtin!("len", 1, 1, (input: string),
        strfn::len(&input).map(VBVariant::from)));
    registry.insert(typed_builtin!("left$", 2, 2, (input: string, length: long),
        strfn::left_dollar(&input, &length).map(VBVariant::from)));
    registry.insert(builtin!("left", 2, 2, |args| {
        let length = VBLong::try_from(&args[1])?;
        strfn::left(&args[0], &length)
    }));
    registry.insert(builtin!("right", 2, 2, |args| {
        let length = VBLong::try_from(&args[1])?;
        strfn::right(&args[0], &length)
    }));
    registry.insert(builtin!("mid", 2, 3, |args| {
        let start = VBLong::try_from(&args[1])?;
        let length = args.get(2).map(VBLong::try_from).transpose()?;
        strfn::mid(&args[0], &start, length.as_ref())
    }));
    registry.insert(
        typed_builtin!("right$", 2, 2, (input: string, length: long),
        strfn::right_dollar(&input, &length).map(VBVariant::from)),
    );
    registry.insert(
        typed_builtin!("mid$", 2, 3, (input: string, start: long, length: opt_long),
        strfn::mid_dollar(&input, &start, length.as_ref()).map(VBVariant::from)),
    );
    registry.insert(builtin!("lcase", 1, 1, |args| { strfn::lcase(&args[0]) }));
    registry.insert(builtin!("ucase", 1, 1, |args| { strfn::ucase(&args[0]) }));
    registry.insert(builtin!("trim", 1, 1, |args| { strfn::trim(&args[0]) }));
    registry.insert(builtin!("ltrim", 1, 1, |args| { strfn::ltrim(&args[0]) }));
    registry.insert(builtin!("rtrim", 1, 1, |args| { strfn::rtrim(&args[0]) }));
    // `LSet` is a statement; the registry entry exposes the alignment
    // primitive `(stringvar, string) -> aligned string` for dispatch.
    registry.insert(
        typed_builtin!("lset", 2, 2, (stringvar: string, value: string),
        strfn::lset_statement(&stringvar, &value).map(VBVariant::from)),
    );
    // `RSet` mirrors `LSet`: statement syntax wired in `exec`, with this
    // registry entry exposing the right-alignment primitive.
    registry.insert(
        typed_builtin!("rset", 2, 2, (stringvar: string, value: string),
        strfn::rset_statement(&stringvar, &value).map(VBVariant::from)),
    );
    registry.insert(typed_builtin!("lcase$", 1, 1, (input: string),
        strfn::lcase_dollar(&input).map(VBVariant::from)));
    registry.insert(typed_builtin!("ucase$", 1, 1, (input: string),
        strfn::ucase_dollar(&input).map(VBVariant::from)));
    registry.insert(typed_builtin!("trim$", 1, 1, (input: string),
        strfn::trim_dollar(&input).map(VBVariant::from)));
    registry.insert(typed_builtin!("ltrim$", 1, 1, (input: string),
        strfn::ltrim_dollar(&input).map(VBVariant::from)));
    registry.insert(typed_builtin!("rtrim$", 1, 1, (input: string),
        strfn::rtrim_dollar(&input).map(VBVariant::from)));
    registry.insert(typed_builtin!("strreverse", 1, 1, (input: string),
        strfn::strreverse(&input).map(VBVariant::from)));
    registry.insert(typed_builtin!("asc", 1, 1, (input: string),
        strfn::asc(&input).map(VBVariant::from)));
    registry.insert(typed_builtin!("ascw", 1, 1, (input: string),
        strfn::ascw(&input).map(VBVariant::from)));
    registry.insert(typed_builtin!("ascb", 1, 1, (input: string),
        strfn::ascb(&input).map(VBVariant::from)));
    registry.insert(builtin!("chr", 1, 1, |args| { strfn::chr(&args[0]) }));
    registry.insert(typed_builtin!("chr$", 1, 1, (charcode: long),
        strfn::chr_dollar(&charcode).map(VBVariant::from)));
    registry.insert(builtin!("chrw", 1, 1, |args| { strfn::chrw(&args[0]) }));
    registry.insert(typed_builtin!("chrw$", 1, 1, (charcode: long),
        strfn::chrw_dollar(&charcode).map(VBVariant::from)));
    registry.insert(builtin!("chrb", 1, 1, |args| { strfn::chrb(&args[0]) }));
    registry.insert(typed_builtin!("chrb$", 1, 1, (charcode: long),
        strfn::chrb_dollar(&charcode).map(VBVariant::from)));
    registry.insert(typed_builtin!("space", 1, 1, (number: long),
        strfn::space(&number)));
    registry.insert(typed_builtin!("space$", 1, 1, (number: long),
        strfn::space_dollar(&number).map(VBVariant::from)));
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
    registry.insert(typed_builtin!(
        "instrrev", 2, 4,
        (s1: string, s2: string, start: opt_long, compare: opt_long),
        strfn::instrrev(&s1, &s2, start.as_ref(), compare.as_ref()).map(VBVariant::from)
    ));
    registry.insert(typed_builtin!(
        "format", 1, 4,
        (expression: variant, format: opt_string, firstdayofweek: opt_long, firstweekofyear: opt_long),
        strfn::format(
            expression,
            format.as_ref(),
            firstdayofweek.as_ref(),
            firstweekofyear.as_ref(),
        )
    ));
    registry.insert(typed_builtin!(
        "format$", 1, 4,
        (expression: variant, format: opt_string, firstdayofweek: opt_long, firstweekofyear: opt_long),
        strfn::format_dollar(
            expression,
            format.as_ref(),
            firstdayofweek.as_ref(),
            firstweekofyear.as_ref(),
        )
        .map(VBVariant::from)
    ));
    registry.insert(typed_builtin!("lenb", 1, 1, (input: string),
        strfn::lenb(&input).map(VBVariant::from)));
    registry.insert(builtin!("leftb", 2, 2, |args| {
        let length = VBLong::try_from(&args[1])?;
        strfn::leftb(&args[0], &length)
    }));
    registry.insert(
        typed_builtin!("leftb$", 2, 2, (input: string, length: long),
        strfn::leftb_dollar(&input, &length).map(VBVariant::from)),
    );
    registry.insert(builtin!("rightb", 2, 2, |args| {
        let length = VBLong::try_from(&args[1])?;
        strfn::rightb(&args[0], &length)
    }));
    registry.insert(
        typed_builtin!("rightb$", 2, 2, (input: string, length: long),
        strfn::rightb_dollar(&input, &length).map(VBVariant::from)),
    );
    registry.insert(builtin!("midb", 2, 3, |args| {
        let start = VBLong::try_from(&args[1])?;
        let length = args.get(2).map(VBLong::try_from).transpose()?;
        strfn::midb(&args[0], &start, length.as_ref())
    }));
    registry.insert(
        typed_builtin!("midb$", 2, 3, (input: string, start: long, length: opt_long),
        strfn::midb_dollar(&input, &start, length.as_ref()).map(VBVariant::from)),
    );
    registry.insert(builtin!("str", 1, 1, |args| { strfn::str(&args[0]) }));
    registry.insert(builtin!("str$", 1, 1, |args| {
        strfn::str_dollar(&args[0]).map(VBVariant::from)
    }));
    registry.insert(
        typed_builtin!("string", 2, 2, (number: long, character: variant),
        strfn::string_function(&number, character)),
    );
    registry.insert(
        typed_builtin!("string$", 2, 2, (number: long, character: variant),
        strfn::string_dollar(&number, character).map(VBVariant::from)),
    );
    registry.insert(typed_builtin!("strcomp", 2, 3,
        (string1: string, string2: string, compare: opt_long),
        strfn::strcomp(&string1, &string2, compare.as_ref()).map(VBVariant::from)));
    registry.insert(builtin!("strconv", 2, 3, |args| {
        let conversion = VBLong::try_from(&args[1])?;
        let lcid = args.get(2).map(VBLong::try_from).transpose()?;
        strfn::strconv(&args[0], &conversion, lcid.as_ref())
    }));
    registry.insert(typed_builtin!("strconv$", 2, 3,
        (input: string, conversion: long, lcid: opt_long),
        strfn::strconv_dollar(&input, &conversion, lcid.as_ref()).map(VBVariant::from)));
    registry.insert(typed_builtin!(
        "replace", 3, 6,
        (expression: variant, find: variant, replacement: variant,
         start: opt_long, count: opt_long, compare: opt_long),
        strfn::replace(
            expression,
            find,
            replacement,
            start.as_ref(),
            count.as_ref(),
            compare.as_ref(),
        )
    ));
    registry.insert(typed_builtin!(
        "formatnumber", 1, 5,
        (expression: variant, digits: opt_long, leading: opt_long, parens: opt_long, group: opt_long),
        strfn::formatnumber(
            expression,
            digits.as_ref(),
            leading.as_ref(),
            parens.as_ref(),
            group.as_ref(),
        )
    ));
    registry.insert(typed_builtin!(
        "formatcurrency", 1, 5,
        (expression: variant, digits: opt_long, leading: opt_long, parens: opt_long, group: opt_long),
        strfn::formatcurrency(
            expression,
            digits.as_ref(),
            leading.as_ref(),
            parens.as_ref(),
            group.as_ref(),
        )
    ));
    registry.insert(typed_builtin!(
        "formatpercent", 1, 5,
        (expression: variant, digits: opt_long, leading: opt_long, parens: opt_long, group: opt_long),
        strfn::formatpercent(
            expression,
            digits.as_ref(),
            leading.as_ref(),
            parens.as_ref(),
            group.as_ref(),
        )
    ));
    registry.insert(typed_builtin!("formatdatetime", 1, 2,
        (expression: variant, namedformat: opt_long),
        strfn::formatdatetime(expression, namedformat.as_ref())));
}
