//! # `Left$` Function
//!
//! Returns a `String` containing a specified number of characters from the left side of a string.
//!
//! ## Syntax
//!
//! ```vb6
//! Left$(string, length)
//! ```
//!
//! ## Parameters
//!
//! - `string`: Required. String expression from which the leftmost characters are returned. If `string` contains `Null`, `Null` is returned.
//! - `length`: Required. Numeric expression indicating how many characters to return. If 0, a zero-length string ("") is returned. If greater than or equal to the number of characters in `string`, the entire string is returned.
//!
//! ## Return Value
//!
//! Returns a `String` containing the leftmost `length` characters from `string`. If `length` is 0, returns an empty string. If `length` is greater than or equal to the length of `string`, returns the entire string.
//!
//! ## Remarks
//!
//! The `Left$` function returns the specified number of characters from the left (beginning) of a string. It's commonly used for string parsing, extracting prefixes, or taking substrings from the start of a string.
//!
//! To determine the number of characters in `string`, use the `Len` function.
//!
//! `Left$` is the string-specific version that always returns a `String`. The `Left` function returns a `Variant`.
//!
//! ## Typical Uses
//!
//! ### Example 1: Extracting File Extension Prefix
//! ```vb6
//! Dim filename As String
//! filename = "document.txt"
//! prefix = Left$(filename, 3)  ' "doc"
//! ```
//!
//! ### Example 2: Getting First Characters
//! ```vb6
//! Dim text As String
//! text = "Hello, World!"
//! greeting = Left$(text, 5)  ' "Hello"
//! ```
//!
//! ### Example 3: Extracting Area Code
//! ```vb6
//! Dim phone As String
//! phone = "5551234567"
//! areaCode = Left$(phone, 3)  ' "555"
//! ```
//!
//! ### Example 4: Getting Date Components
//! ```vb6
//! Dim dateStr As String
//! dateStr = "2024-01-15"
//! year = Left$(dateStr, 4)  ' "2024"
//! ```
//!
//! ## Common Usage Patterns
//!
//! ### Checking String Prefix
//! ```vb6
//! If Left$(filename, 4) = "tmp_" Then
//!     Debug.Print "Temporary file"
//! End If
//! ```
//!
//! ### Extracting Initials
//! ```vb6
//! Dim name As String
//! name = "John Doe"
//! initial = Left$(name, 1)  ' "J"
//! ```
//!
//! ### Parsing Fixed-Width Data
//! ```vb6
//! Dim record As String
//! record = "12345John     Smith    "
//! id = Left$(record, 5)  ' "12345"
//! ```
//!
//! ### Truncating Long Strings
//! ```vb6
//! Dim description As String
//! description = "Very long description text..."
//! If Len(description) > 50 Then
//!     description = Left$(description, 47) & "..."
//! End If
//! ```
//!
//! ### Extracting Drive Letter
//! ```vb6
//! Dim path As String
//! path = "C:\Windows\System32"
//! drive = Left$(path, 1)  ' "C"
//! ```
//!
//! ### Getting Protocol from URL
//! ```vb6
//! Dim url As String
//! url = "https://example.com"
//! protocol = Left$(url, 5)  ' "https"
//! ```
//!
//! ### Validating File Type
//! ```vb6
//! Dim fileName As String
//! fileName = "IMG_1234.JPG"
//! If Left$(fileName, 4) = "IMG_" Then
//!     processImage fileName
//! End If
//! ```
//!
//! ### Extracting Country Code
//! ```vb6
//! Dim phoneNumber As String
//! phoneNumber = "+1-555-1234"
//! If Left$(phoneNumber, 1) = "+" Then
//!     countryCode = Left$(phoneNumber, 2)  ' "+1"
//! End If
//! ```
//!
//! ### Creating Abbreviations
//! ```vb6
//! Dim state As String
//! state = "California"
//! abbr = UCase$(Left$(state, 2))  ' "CA"
//! ```
//!
//! ### Parsing CSV First Field
//! ```vb6
//! Dim csvLine As String
//! csvLine = "John,Doe,555-1234"
//! Dim pos As Integer
//! pos = InStr(csvLine, ",")
//! If pos > 0 Then
//!     firstName = Left$(csvLine, pos - 1)  ' "John"
//! End If
//! ```
//!
//! ## Related Functions
//!
//! - `Left`: Variant version that returns a `Variant`
//! - `Right$`: Returns characters from the right side of a string
//! - `Mid$`: Returns characters from the middle of a string
//! - `Len`: Returns the length of a string
//! - `InStr`: Finds the position of a substring
//! - `LTrim$`: Removes leading spaces from a string
//! - `Trim$`: Removes leading and trailing spaces
//!
//! ## Best Practices
//!
//! 1. Always validate that `length` is not negative before calling
//! 2. Use `Len` to check string length before extracting
//! 3. Handle empty strings appropriately in your logic
//! 4. Consider using `InStr` with `Left$` for dynamic parsing
//! 5. Remember that `Left$(str, 0)` returns an empty string
//! 6. Use `Left$` instead of `Left` when you need a `String` type explicitly
//! 7. Combine with `Trim$` when dealing with user input
//! 8. Be aware that requesting more characters than exist returns the full string
//! 9. Use comparison with `Left$` for prefix checking (faster than `InStr`)
//! 10. Cache the result if using the same `Left$` call multiple times
//!
//! ## Performance Considerations
//!
//! - `Left$` is a very fast operation in VB6
//! - More efficient than using `Mid$` for extracting from the beginning
//! - Faster than string concatenation for prefix operations
//! - No performance penalty for requesting more characters than available
//! - Using `Left$` for prefix comparison is faster than regular expressions
//!
//! ## String Indexing
//!
//! | Length Value | Result |
//! |--------------|--------|
//! | 0 | Returns empty string ("") |
//! | 1 to Len(string) | Returns that many characters from left |
//! | > Len(string) | Returns entire string |
//! | Negative | Runtime error (Invalid procedure call or argument) |
//!
//! ## Common Pitfalls
//!
//! - Passing negative length values (causes runtime error)
//! - Assuming `Left$` will throw an error if length exceeds string length (it doesn't)
//! - Not handling `Null` strings (causes runtime error)
//! - Confusing zero-based vs one-based indexing (VB6 strings are 1-based)
//! - Using `Left$` on binary data (use `LeftB$` instead)
//! - Forgetting that the length parameter is character count, not position
//! - Not trimming strings before extraction (may get unwanted spaces)
//!
//! ## Limitations
//!
//! - Cannot extract from right side (use `Right$` instead)
//! - Cannot specify starting position (use `Mid$` instead)
//! - Does not work with byte arrays directly
//! - No built-in support for Unicode surrogate pairs
//! - Length parameter cannot be an expression that evaluates to `Null`
//! - Returns `Null` if the string argument is `Null`

use crate::{
    error::{err_number, VBError, VBResult},
    value::{VBLong, VBString},
};

/// Returns the specified number of characters from the left side of the string.
/// The `$` suffix indicates this function returns a `String` type (not `Variant`).
///
/// A `length` of 0 returns an empty string; a `length` greater than the number
/// of characters returns the entire string. Characters are counted as Unicode
/// scalar values.
///
/// # Errors
///
/// Returns error 5 (`Invalid procedure call or argument`) when `length` is negative.
pub fn left_dollar(input: &VBString, length: &VBLong) -> VBResult<VBString> {
    let length = length.as_i32();
    if length < 0 {
        return Err(VBError::with_description(
            err_number::INVALID_PROCEDURE_CALL,
            "Invalid length",
        ));
    }
    Ok(VBString::from(
        input
            .as_str()
            .chars()
            .take(length as usize)
            .collect::<String>(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_leftmost_characters() {
        assert_eq!(
            left_dollar(&VBString::from("Hello"), &VBLong::from(3)).unwrap(),
            VBString::from("Hel")
        );
        assert_eq!(
            left_dollar(&VBString::from("Hello"), &VBLong::from(0)).unwrap(),
            VBString::from("")
        );
    }

    #[test]
    fn length_beyond_string_returns_whole() {
        assert_eq!(
            left_dollar(&VBString::from("Hello"), &VBLong::from(99)).unwrap(),
            VBString::from("Hello")
        );
    }

    #[test]
    fn handles_unicode() {
        assert_eq!(
            left_dollar(&VBString::from("héllo"), &VBLong::from(2)).unwrap(),
            VBString::from("hé")
        );
    }

    #[test]
    fn negative_length_errors() {
        let err = left_dollar(&VBString::from("Hello"), &VBLong::from(-1)).unwrap_err();
        assert_eq!(err.number, err_number::INVALID_PROCEDURE_CALL);
    }
}
