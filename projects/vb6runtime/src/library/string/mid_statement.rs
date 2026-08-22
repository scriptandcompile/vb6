//! # Mid Statement
//!
//! Replaces a specified number of characters in a Variant (String) variable with characters from another string.
//!
//! ## Syntax
//!
//! ```vb
//! Mid(stringvar, start[, length]) = string
//! ```
//!
//! - `stringvar`: Required. Name of string variable to modify
//! - `start`: Required. Character position where replacement begins (1-based)
//! - `length`: Optional. Number of characters to replace. If omitted, uses entire length of `string`
//! - `string`: Required. String expression used as replacement
//!
//! ## Remarks
//!
//! - The number of characters replaced is always less than or equal to the number of characters in `stringvar`
//! - If `start` is greater than the length of `stringvar`, `stringvar` is unchanged
//! - If `length` is omitted, all characters from `start` to the end of the string are replaced
//! - `Mid` statement replaces characters in-place; it does not change the length of the original string
//! - If replacement string is longer than `length`, only `length` characters are used
//! - If replacement string is shorter than `length`, only available characters are replaced
//!
//! ## Examples
//!
//! ```vb
//! Dim s As String
//! s = "Hello World"
//! Mid(s, 7, 5) = "VB6!!"     ' s becomes "Hello VB6!!"
//!
//! s = "ABCDEFGH"
//! Mid(s, 3) = "123"          ' s becomes "AB123FGH"
//!
//! s = "Test"
//! Mid(s, 2, 2) = "XX"        ' s becomes "TXXt"
//! ```
//!
//! ## Reference
//!
//! [Mid Statement - Microsoft Docs](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/mid-statement)

use vb6core::error::{err_number, VBError, VBResult};

use crate::value::{VBLong, VBString};

/// Replaces characters in `stringvar` with `string`, returning the result.
///
/// Overwriting begins at the 1-based character position `start` and covers
/// at most `length` characters (to the end of `stringvar` when omitted).
/// The variable's length never changes: excess replacement characters are
/// truncated, a shorter replacement leaves the tail untouched, and a
/// `start` beyond the variable's length leaves it unchanged.
///
/// # Errors
///
/// Returns error 5 (`Invalid procedure call or argument`) when `start` is
/// less than 1 or `length` is negative.
pub fn mid_statement(
    stringvar: &VBString,
    start: &VBLong,
    length: Option<&VBLong>,
    string: &VBString,
) -> VBResult<VBString> {
    let start = start.as_i32();
    if start < 1 {
        return Err(VBError::with_description(
            err_number::INVALID_PROCEDURE_CALL,
            "Invalid start position",
        ));
    }
    let max_replace = match length {
        Some(n) => {
            let n = n.as_i32();
            if n < 0 {
                return Err(VBError::with_description(
                    err_number::INVALID_PROCEDURE_CALL,
                    "Invalid length",
                ));
            }
            n as usize
        }
        None => usize::MAX,
    };

    let mut target: Vec<char> = stringvar.as_str().chars().collect();
    // A `start` beyond the variable's length is a no-op.
    let offset = (start - 1) as usize;
    if offset >= target.len() {
        return Ok(stringvar.clone());
    }

    let replacement: Vec<char> = string.as_str().chars().collect();
    let take = max_replace
        .min(replacement.len())
        .min(target.len() - offset);
    target.splice(offset..offset + take, replacement[..take].iter().copied());
    Ok(VBString::from(target.into_iter().collect::<String>()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replaces_with_explicit_length() {
        assert_eq!(
            mid_statement(
                &VBString::from("Hello World"),
                &VBLong::from(7),
                Some(&VBLong::from(5)),
                &VBString::from("VB6!!")
            )
            .unwrap(),
            VBString::from("Hello VB6!!")
        );
    }

    #[test]
    fn omitted_length_replaces_to_the_end() {
        assert_eq!(
            mid_statement(
                &VBString::from("ABCDEFGH"),
                &VBLong::from(3),
                None,
                &VBString::from("123")
            )
            .unwrap(),
            VBString::from("AB123FGH")
        );
        assert_eq!(
            mid_statement(
                &VBString::from("ABCDEFGH"),
                &VBLong::from(3),
                None,
                &VBString::from("1234567890")
            )
            .unwrap(),
            VBString::from("AB123456")
        );
    }

    #[test]
    fn shorter_replacement_keeps_the_tail() {
        assert_eq!(
            mid_statement(
                &VBString::from("Test"),
                &VBLong::from(2),
                Some(&VBLong::from(2)),
                &VBString::from("XX")
            )
            .unwrap(),
            VBString::from("TXXt")
        );
    }

    #[test]
    fn start_beyond_the_length_is_a_noop() {
        assert_eq!(
            mid_statement(
                &VBString::from("abc"),
                &VBLong::from(10),
                None,
                &VBString::from("xyz")
            )
            .unwrap(),
            VBString::from("abc")
        );
    }

    #[test]
    fn rejects_start_below_one() {
        assert_eq!(
            mid_statement(
                &VBString::from("abc"),
                &VBLong::from(0),
                None,
                &VBString::from("x")
            )
            .unwrap_err()
            .number,
            err_number::INVALID_PROCEDURE_CALL
        );
    }

    #[test]
    fn rejects_negative_length() {
        assert_eq!(
            mid_statement(
                &VBString::from("abc"),
                &VBLong::from(1),
                Some(&VBLong::from(-1)),
                &VBString::from("x")
            )
            .unwrap_err()
            .number,
            err_number::INVALID_PROCEDURE_CALL
        );
    }

    #[test]
    fn counts_characters_not_bytes() {
        assert_eq!(
            mid_statement(
                &VBString::from("ééééé"),
                &VBLong::from(2),
                Some(&VBLong::from(2)),
                &VBString::from("XY")
            )
            .unwrap(),
            VBString::from("éXYéé")
        );
    }
}
