//! # `MidB` Statement
//!
//! Replaces a specified number of bytes in a Variant (String) variable with bytes from another string.
//!
//! ## Syntax
//!
//! ```vb
//! MidB(stringvar, start[, length]) = string
//! ```
//!
//! - `stringvar`: Required. Name of string variable to modify
//! - `start`: Required. Byte position where replacement begins (1-based)
//! - `length`: Optional. Number of bytes to replace. If omitted, uses entire length of `string`
//! - `string`: Required. String expression used as replacement
//!
//! ## Remarks
//!
//! - `MidB` is used with byte data contained in a string
//! - Works with byte positions rather than character positions (important for double-byte character sets)
//! - The number of bytes replaced is always less than or equal to the number of bytes in `stringvar`
//! - If `start` is greater than the number of bytes in `stringvar`, `stringvar` is unchanged
//! - If `length` is omitted, all bytes from `start` to the end of the string are replaced
//! - `MidB` statement replaces bytes in-place; it does not change the byte length of the original string
//! - If replacement string is longer than `length`, only `length` bytes are used
//! - If replacement string is shorter than `length`, only available bytes are replaced
//! - Primarily used when working with double-byte character sets (DBCS) like Japanese, Chinese, or Korean
//!
//! ## Examples
//!
//! ```vb
//! Dim s As String
//! s = "ABCDEFGH"
//! MidB(s, 3, 2) = "12"       ' Replaces 2 bytes starting at byte 3
//!
//! ' For DBCS strings:
//! Dim dbcsStr As String
//! dbcsStr = "日本語"          ' Japanese characters
//! MidB(dbcsStr, 1, 2) = "XX" ' Replaces first 2 bytes
//! ```
//!
//! ## Reference
//!
//! [MidB Statement - Microsoft Docs](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/midb-statement)

use vb6core::error::{err_number, VBError, VBResult};

use crate::value::{VBLong, VBString};

/// The runtime's byte model for VB6 strings: every character occupies two
/// bytes (UTF-16 units), mirroring `midb_dollar`.
const BYTES_PER_CHAR: i32 = 2;

/// Replaces bytes in `stringvar` with `string`, returning the result.
///
/// This is the byte-oriented form of [`mid_statement`]: `start` and
/// `length` are 1-based byte positions/counts, converted to characters at
/// two bytes per character (matching the `MidB$` function's model). The
/// variable's length never changes: excess replacement bytes are truncated,
/// a shorter replacement leaves the tail untouched, and a `start` beyond
/// the variable's byte length leaves it unchanged.
///
/// # Errors
///
/// Returns error 5 (`Invalid procedure call or argument`) when `start` is
/// less than 1 or `length` is negative.
pub fn midb_statement(
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
            (n / BYTES_PER_CHAR) as usize
        }
        None => usize::MAX,
    };

    let mut target: Vec<char> = stringvar.as_str().chars().collect();
    // A `start` beyond the variable's byte length is a no-op.
    let offset = ((start - 1) / BYTES_PER_CHAR) as usize;
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
    fn replaces_bytes_with_explicit_length() {
        assert_eq!(
            midb_statement(
                &VBString::from("ABCDEFGH"),
                &VBLong::from(3),
                Some(&VBLong::from(4)),
                &VBString::from("12")
            )
            .unwrap(),
            VBString::from("A12DEFGH")
        );
    }

    #[test]
    fn omitted_length_replaces_to_the_end() {
        assert_eq!(
            midb_statement(
                &VBString::from("ABCDEFGH"),
                &VBLong::from(3),
                None,
                &VBString::from("1234")
            )
            .unwrap(),
            VBString::from("A1234FGH")
        );
    }

    #[test]
    fn truncates_replacement_longer_than_the_byte_length() {
        assert_eq!(
            midb_statement(
                &VBString::from("ABCDEFGH"),
                &VBLong::from(3),
                Some(&VBLong::from(6)),
                &VBString::from("12345")
            )
            .unwrap(),
            VBString::from("A123EFGH")
        );
    }

    #[test]
    fn shorter_replacement_keeps_the_tail() {
        assert_eq!(
            midb_statement(
                &VBString::from("Test"),
                &VBLong::from(1),
                Some(&VBLong::from(2)),
                &VBString::from("X")
            )
            .unwrap(),
            VBString::from("Xest")
        );
    }

    #[test]
    fn start_beyond_the_length_is_a_noop() {
        assert_eq!(
            midb_statement(
                &VBString::from("abc"),
                &VBLong::from(99),
                None,
                &VBString::from("xyz")
            )
            .unwrap(),
            VBString::from("abc")
        );
    }

    #[test]
    fn rejects_invalid_start_and_length() {
        assert_eq!(
            midb_statement(
                &VBString::from("abc"),
                &VBLong::from(0),
                None,
                &VBString::from("x")
            )
            .unwrap_err()
            .number,
            err_number::INVALID_PROCEDURE_CALL
        );
        assert_eq!(
            midb_statement(
                &VBString::from("abc"),
                &VBLong::from(1),
                Some(&VBLong::from(-2)),
                &VBString::from("x")
            )
            .unwrap_err()
            .number,
            err_number::INVALID_PROCEDURE_CALL
        );
    }
}
