//! # `RSet` Statement
//!
//! Right-aligns a string within a string variable or copies one user-defined variable to another.
//!
//! ## Syntax
//!
//! ```vb
//! RSet stringvar = string
//! RSet varname1 = varname2  ' For user-defined types
//! ```
//!
//! ## Parts
//!
//! - **stringvar**: Required. String variable or property name to be right-aligned.
//! - **string**: Required. String expression to be right-aligned within stringvar.
//! - **varname1**: Required. Variable of a user-defined type.
//! - **varname2**: Required. Variable of a different user-defined type.
//!
//!
//! ## Remarks
//!
//! - **String Alignment**: When used with string variables, `RSet` right-aligns the string within
//!   the variable. If the string is shorter than the variable, spaces are added on the left to
//!   achieve right alignment.
//! - **Fixed-Length Strings**: `RSet` is particularly useful with fixed-length strings where you
//!   need to right-justify text within a specific width.
//! - **User-Defined Types**: When used with user-defined types (UDTs), `RSet` copies data from one
//!   variable to another on a byte-by-byte basis. This can be useful for converting between
//!   different UDT structures that have the same size.
//! - **Shorter Strings**: If the source string is shorter than the target variable, spaces are
//!   added on the left side to right-align the text.
//! - **Longer Strings**: If the source string is longer than the target variable, the string is
//!   truncated on the left side, keeping only the rightmost characters that fit.
//! - **Comparison to `LSet`**: `RSet` is the opposite of `LSet`. While `LSet` left-aligns strings,
//!   `RSet` right-aligns them.
//!
//! ## Example
//!
//! ```vb
//! Dim MyString As String * 10
//! MyString = String(10, "X")  ' Fill with X's
//! RSet MyString = "VB6"       ' Result: "       VB6"
//! ```
//!
//! ## Example with User-Defined Types
//!
//! ```vb
//! Type TypeA
//!     Name As String * 20
//!     Age As Integer
//! End Type
//!
//! Type TypeB
//!     Data As String * 22
//! End Type
//!
//! Dim VarA As TypeA
//! Dim VarB As TypeB
//!
//! VarA.Name = "John"
//! VarA.Age = 30
//! RSet VarB = VarA  ' Copy VarA to VarB byte-by-byte
//! ```
//!
//! ## See Also
//!
//! - `LSet` statement (left-align strings)
//! - `Mid` statement (replace characters in a string)
//! - Fixed-length string variables
//!
//! ## References
//!
//! - [RSet Statement (Visual Basic 6.0)](https://docs.microsoft.com/en-us/previous-versions/visualstudio/visual-basic-6/aa266258(v=vs.60))

use vb6core::error::VBResult;

use crate::value::VBString;

/// Right-aligns `string` within `stringvar`, returning the aligned result.
///
/// If `string` is shorter than `stringvar`, it is padded with spaces on the
/// left; if it is longer, only the rightmost characters that fit are kept.
/// The width used is the current length of `stringvar`, which mirrors a
/// fixed-length string's declared size when callers track it.
///
/// # Errors
///
/// Never fails for string operands; returns `VBResult` for consistency with
/// the statement surface (user-defined-type copies may fail in the future).
pub fn rset_statement(stringvar: &VBString, string: &VBString) -> VBResult<VBString> {
    let width = stringvar.as_str().chars().count();
    let source: Vec<char> = string.as_str().chars().collect();
    let result = if source.len() >= width {
        source[source.len() - width..].iter().collect::<String>()
    } else {
        let mut padded = " ".repeat(width - source.len());
        padded.push_str(string.as_str());
        padded
    };
    Ok(VBString::from(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pads_shorter_source_on_the_left() {
        assert_eq!(
            rset_statement(&VBString::from("XXXXX"), &VBString::from("Right")).unwrap(),
            VBString::from("Right")
        );
        assert_eq!(
            rset_statement(&VBString::from("XXXXX"), &VBString::from("A")).unwrap(),
            VBString::from("    A")
        );
    }

    #[test]
    fn truncates_longer_source_from_the_left() {
        assert_eq!(
            rset_statement(&VBString::from("XXX"), &VBString::from("Right")).unwrap(),
            VBString::from("ght")
        );
    }

    #[test]
    fn exact_fit_is_unchanged() {
        assert_eq!(
            rset_statement(&VBString::from("abc"), &VBString::from("abc")).unwrap(),
            VBString::from("abc")
        );
    }

    #[test]
    fn empty_target_yields_empty_result() {
        assert_eq!(
            rset_statement(&VBString::from(""), &VBString::from("Right")).unwrap(),
            VBString::from("")
        );
    }

    #[test]
    fn empty_source_pads_with_spaces() {
        assert_eq!(
            rset_statement(&VBString::from("XXX"), &VBString::from("")).unwrap(),
            VBString::from("   ")
        );
    }

    #[test]
    fn counts_characters_not_bytes() {
        assert_eq!(
            rset_statement(&VBString::from("ééé"), &VBString::from("é")).unwrap(),
            VBString::from("  é")
        );
    }
}
