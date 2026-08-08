//! # `Mid$` Function
//!
//! The `Mid$` function returns a `String` containing a specified number of characters from a string.
//! The dollar sign suffix (`$`) indicates that this function always returns a `String` type, never a `Variant`.
//!
//! ## Syntax
//!
//! ```vb
//! Mid$(string, start[, length])
//! ```
//!
//! ## Parameters
//!
//! - `string` - Required. `String` expression from which characters are returned.
//! - `start` - Required. `Long`. Character position in `string` at which the part to be taken begins (1-based).
//! - `length` - Optional. `Long`. Number of characters to return. If omitted or if there are fewer than `length` characters in the text (including the character at `start`), all characters from the start position to the end of the string are returned.
//!
//! ## Return Value
//!
//! Returns a `String` containing the specified portion of the input string.
//!
//! ## Behavior
//!
//! - If `start` is greater than the number of characters in `string`, `Mid$` returns a zero-length string ("").
//! - If `start` is less than 1, a runtime error occurs.
//! - If `length` is negative, a runtime error occurs.
//! - The first character in the string is at position 1.
//!
//! ## Difference from Mid
//!
//! The `Mid$` function always returns a `String`, while the `Mid` function (without the dollar sign) can return a `Variant`.
//! In practice, they behave identically in most scenarios, but the dollar sign version may be slightly more efficient
//! as it avoids the overhead of the `Variant` type.
//!
//! ## Examples
//!
//! ```vb
//! ' Extract 3 characters starting at position 2
//! Dim result As String
//! result = Mid$("Hello World", 2, 3)  ' Returns "ell"
//!
//! ' Extract from position 7 to the end
//! result = Mid$("Hello World", 7)  ' Returns "World"
//!
//! ' Start position beyond string length
//! result = Mid$("Hi", 10)  ' Returns ""
//! ```

use crate::{
    error::{err_number, VBError, VBResult},
    value::{VBLong, VBString},
};

/// Returns the specified number of characters from a string, starting at `start`.
/// The `$` suffix indicates this function returns a `String` type (not `Variant`).
///
/// Positions are 1-based. When `length` is `None` (or extends past the end of
/// the string), everything from `start` to the end is returned. A `start` past
/// the end of the string yields an empty string. Characters are counted as
/// Unicode scalar values.
///
/// # Errors
///
/// Returns error 5 (`Invalid procedure call or argument`) when `start` is less
/// than 1 or `length` is negative.
pub fn mid_dollar(input: &VBString, start: &VBLong, length: Option<&VBLong>) -> VBResult<VBString> {
    let start = start.as_i32();
    if start < 1 {
        return Err(VBError::with_description(
            err_number::INVALID_PROCEDURE_CALL,
            "Invalid start position",
        ));
    }
    if let Some(n) = length {
        let n = n.as_i32();
        if n < 0 {
            return Err(VBError::with_description(
                err_number::INVALID_PROCEDURE_CALL,
                "Invalid length",
            ));
        }
    }

    let chars: Vec<char> = input.as_str().chars().collect();
    if start as usize > chars.len() {
        return Ok(VBString::from(String::new()));
    }

    let skip = (start - 1) as usize;
    let result = match length {
        Some(n) => {
            let n = n.as_i32();
            chars
                .into_iter()
                .skip(skip)
                .take(n as usize)
                .collect::<String>()
        }
        None => chars.into_iter().skip(skip).collect::<String>(),
    };
    Ok(VBString::from(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::err_number;

    #[test]
    fn extracts_middle_characters() {
        assert_eq!(
            mid_dollar(
                &VBString::from("Hello World"),
                &VBLong::from(4),
                Some(&VBLong::from(5))
            )
            .unwrap(),
            VBString::from("lo Wo")
        );
    }

    #[test]
    fn omitting_length_returns_rest() {
        assert_eq!(
            mid_dollar(&VBString::from("Hello World"), &VBLong::from(7), None).unwrap(),
            VBString::from("World")
        );
    }

    #[test]
    fn length_beyond_end_is_clamped() {
        assert_eq!(
            mid_dollar(
                &VBString::from("Hello"),
                &VBLong::from(4),
                Some(&VBLong::from(10))
            )
            .unwrap(),
            VBString::from("lo")
        );
    }

    #[test]
    fn start_beyond_end_returns_empty() {
        assert_eq!(
            mid_dollar(&VBString::from("Hello"), &VBLong::from(6), None).unwrap(),
            VBString::from("")
        );
    }

    #[test]
    fn zero_length_returns_empty() {
        assert_eq!(
            mid_dollar(
                &VBString::from("Hello"),
                &VBLong::from(2),
                Some(&VBLong::from(0))
            )
            .unwrap(),
            VBString::from("")
        );
    }

    #[test]
    fn rejects_invalid_start() {
        assert_eq!(
            mid_dollar(&VBString::from("Hello"), &VBLong::from(0), None)
                .unwrap_err()
                .number,
            err_number::INVALID_PROCEDURE_CALL
        );
    }

    #[test]
    fn rejects_negative_length() {
        assert_eq!(
            mid_dollar(
                &VBString::from("Hello"),
                &VBLong::from(1),
                Some(&VBLong::from(-1))
            )
            .unwrap_err()
            .number,
            err_number::INVALID_PROCEDURE_CALL
        );
    }
}
