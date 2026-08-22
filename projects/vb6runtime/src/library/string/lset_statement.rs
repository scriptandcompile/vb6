//! VB6 LSet statement syntax:
//! - LSet stringvar = string
//! - LSet varname1 = varname2 (for user-defined types)
//!
//! Left-aligns a string within a string variable, or copies a variable of one user-defined type
//! to another variable of a different user-defined type.
//!
//! The LSet statement syntax has these parts:
//!
//! | Part          | Description |
//! |---------------|-------------|
//! | stringvar     | Required. Name of string variable. |
//! | string        | Required. String expression to be left-aligned within stringvar. |
//! | varname1      | Required. Variable name of the user-defined type being copied to. |
//! | varname2      | Required. Variable name of the user-defined type being copied from. |
//!
//! ## Remarks
//!
//! - LSet left-aligns strings within string variables.
//! - If string is shorter than stringvar, LSet left-aligns the string in stringvar and pads
//!   remaining characters with spaces.
//! - If string is longer than stringvar, LSet places only the leftmost characters that fit into
//!   stringvar.
//! - Warning: Using LSet to copy variables of different user-defined types is not recommended.
//!   Copying variables of one user-defined type into variables of a different user-defined type
//!   can produce unpredictable results.
//! - When copying between variables of user-defined types, the memory assigned to one variable is
//!   copied byte-for-byte to the memory assigned to the other variable.
//! - LSet is commonly used with fixed-length strings.
//! - LSet can be used with variant variables that contain strings.
//!
//! ## Examples
//!
//! ```vb
//! LSet MyString = "Left"
//! LSet FixedString = userName
//! LSet myRecord = sourceRecord
//! ```
//!
//! ## Reference
//!
//! [LSet Statement - Microsoft Docs](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/lset-statement)

use vb6core::error::VBResult;

use crate::value::VBString;

/// Left-aligns `string` within `stringvar`, returning the aligned result.
///
/// If `string` is shorter than `stringvar`, the remainder is padded with
/// spaces on the right; if it is longer, only the leftmost characters that
/// fit are kept. The width used is the current length of `stringvar`, which
/// mirrors a fixed-length string's declared size when callers track it.
///
/// # Errors
///
/// Never fails for string operands; returns `VBResult` for consistency with
/// the statement surface (user-defined-type copies may fail in the future).
pub fn lset_statement(stringvar: &VBString, string: &VBString) -> VBResult<VBString> {
    let width = stringvar.as_str().chars().count();
    let mut result: String = string.as_str().chars().take(width).collect();
    let padding = width - result.chars().count();
    result.push_str(&" ".repeat(padding));
    Ok(VBString::from(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pads_shorter_source_on_the_right() {
        assert_eq!(
            lset_statement(&VBString::from("XXXXX"), &VBString::from("Left")).unwrap(),
            VBString::from("Left ")
        );
        assert_eq!(
            lset_statement(&VBString::from("XXXXX"), &VBString::from("A")).unwrap(),
            VBString::from("A    ")
        );
    }

    #[test]
    fn truncates_longer_source_from_the_right() {
        assert_eq!(
            lset_statement(&VBString::from("XXX"), &VBString::from("Left")).unwrap(),
            VBString::from("Lef")
        );
    }

    #[test]
    fn exact_fit_is_unchanged() {
        assert_eq!(
            lset_statement(&VBString::from("abc"), &VBString::from("abc")).unwrap(),
            VBString::from("abc")
        );
    }

    #[test]
    fn empty_target_yields_empty_result() {
        assert_eq!(
            lset_statement(&VBString::from(""), &VBString::from("Left")).unwrap(),
            VBString::from("")
        );
    }

    #[test]
    fn counts_characters_not_bytes() {
        assert_eq!(
            lset_statement(&VBString::from("ééé"), &VBString::from("é")).unwrap(),
            VBString::from("é  ")
        );
    }
}
