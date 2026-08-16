//! # `Asc` Function
//!
//! Returns an `Integer` representing the character code corresponding to the first letter in a string.
//!
//! ## Syntax
//!
//! ```vb
//! Asc(string)
//! ```
//!
//! ## Parameters
//!
//! - `string` - Required. Any valid string expression. If the string contains no characters, a run-time error occurs.
//!
//! ## Return Value
//!
//! Returns an `Integer` representing the `ANSI` character code of the first character in the string.
//!
//! - For `ANSI` characters (0-127), returns standard `ASCII` values
//! - For extended `ANSI` characters (128-255), returns extended `ASCII` values
//! - Unicode characters are converted to `ANSI` before the code is returned
//!
//! ## Remarks
//!
//! The `Asc` function returns the numeric `ANSI` character code for the first character in a string.
//! This is useful for:
//! - Validating input characters
//! - Performing character-based operations
//! - Converting characters to their numeric representations
//! - Character range checking
//!
//! ### Important Notes
//!
//! 1. **Only First Character**: Only the first character of the string is examined
//! 2. **Empty String Error**: Passing an empty string results in a run-time error (Error 5: Invalid procedure call or argument)
//! 3. **ANSI vs. Unicode**: In VB6, `Asc` returns `ANSI` codes; `AscW` returns Unicode values
//! 4. **Return Type**: Returns `Integer` (16-bit signed), range -32,768 to 32,767, but character codes are 0-255
//! 5. **Case Sensitive**: Upper and lowercase letters have different codes (e.g., "A" = 65, "a" = 97)
//!
//! ### Character Code Ranges
//!
//! - **0-31**: Control characters (non-printable)
//! - **32**: Space
//! - **48-57**: Digits '0' through '9'
//! - **65-90**: Uppercase letters 'A' through 'Z'
//! - **97-122**: Lowercase letters 'a' through 'z'
//! - **128-255**: Extended ANSI characters
//!
//! ## Examples
//!
//! ### Basic Usage
//!
//! ```vb
//! Dim code As Integer
//! code = Asc("A")          ' Returns 65
//! code = Asc("Apple")      ' Returns 65 (first character only)
//! code = Asc("a")          ' Returns 97
//! code = Asc("0")          ' Returns 48
//! code = Asc(" ")          ' Returns 32 (space)
//! ```
//!
//! ### Character Validation
//!
//! ```vb
//! Function IsDigit(ch As String) As Boolean
//!     Dim code As Integer
//!     code = Asc(ch)
//!     IsDigit = (code >= 48 And code <= 57)
//! End Function
//!
//! Function IsUpperCase(ch As String) As Boolean
//!     Dim code As Integer
//!     code = Asc(ch)
//!     IsUpperCase = (code >= 65 And code <= 90)
//! End Function
//! ```
//!
//! ### Case Conversion Offset
//!
//! ```vb
//! ' Calculate offset between upper and lower case
//! Dim offset As Integer
//! offset = Asc("a") - Asc("A")  ' Returns 32
//! ```
//!
//! ### Character Range Checking
//!
//! ```vb
//! Function IsPrintable(ch As String) As Boolean
//!     Dim code As Integer
//!     code = Asc(ch)
//!     IsPrintable = (code >= 32 And code <= 126)
//! End Function
//! ```
//!
//! ### String Encoding
//!
//! ```vb
//! Function EncodeString(s As String) As String
//!     Dim i As Integer
//!     Dim result As String
//!     For i = 1 To Len(s)
//!         If result <> "" Then result = result & ","
//!         result = result & CStr(Asc(Mid(s, i, 1)))
//!     Next i
//!     EncodeString = result
//! End Function
//! ```
//!
//! ## Common Patterns
//!
//! ### 1. Input Validation
//!
//! ```vb
//! If Asc(userInput) >= 48 And Asc(userInput) <= 57 Then
//!     ' First character is a digit
//! End If
//! ```
//!
//! ### 2. Alphabetic Checking
//!
//! ```vb
//! Dim code As Integer
//! code = Asc(UCase(letter))
//! If code >= 65 And code <= 90 Then
//!     ' It's a letter
//! End If
//! ```
//!
//! ### 3. CSV Parsing Helper
//!
//! ```vb
//! If Asc(field) = 34 Then  ' 34 is double quote
//!     ' Handle quoted field
//! End If
//! ```
//!
//! ### 4. Character Comparison
//!
//! ```vb
//! If Asc(char1) < Asc(char2) Then
//!     ' char1 comes before char2 in ASCII order
//! End If
//! ```
//!
//! ### 5. Special Character Detection
//!
//! ```vb
//! Select Case Asc(ch)
//!     Case 9      ' Tab
//!     Case 10     ' Line feed
//!     Case 13     ' Carriage return
//!     Case 32     ' Space
//! End Select
//! ```
//!
//! ### 6. Keyboard Input Processing
//!
//! ```vb
//! Private Sub Text1_KeyPress(KeyAscii As Integer)
//!     If KeyAscii = 13 Then  ' Enter key
//!         ' Process input
//!     End If
//! End Sub
//! ```
//!
//! ### 7. Character Class Testing
//!
//! ```vb
//! Function IsControl(ch As String) As Boolean
//!     Dim code As Integer
//!     code = Asc(ch)
//!     IsControl = (code < 32 Or code = 127)
//! End Function
//! ```
//!
//! ### 8. Simple Encryption
//!
//! ```vb
//! Function ROT13Char(ch As String) As String
//!     Dim code As Integer
//!     code = Asc(UCase(ch))
//!     If code >= 65 And code <= 90 Then
//!         code = ((code - 65 + 13) Mod 26) + 65
//!         ROT13Char = Chr(code)
//!     Else
//!         ROT13Char = ch
//!     End If
//! End Function
//! ```
//!
//! ## Common Character Codes
//!
//! | Character | Code | Description |
//! |-----------|------|-------------|
//! | Null      | 0    | Null character |
//! | Tab       | 9    | Horizontal tab |
//! | LF        | 10   | Line feed |
//! | CR        | 13   | Carriage return |
//! | Space     | 32   | Space |
//! | !         | 33   | Exclamation mark |
//! | "         | 34   | Double quote |
//! | 0         | 48   | Digit zero |
//! | 9         | 57   | Digit nine |
//! | A         | 65   | Uppercase A |
//! | Z         | 90   | Uppercase Z |
//! | a         | 97   | Lowercase a |
//! | z         | 122  | Lowercase z |
//! | DEL       | 127  | Delete |
//!
//! ## Error Handling
//!
//! ```vb
//! On Error Resume Next
//! code = Asc(inputString)
//! If Err.Number = 5 Then
//!     ' Empty string error
//!     MsgBox "String cannot be empty"
//! End If
//! ```
//!
//! ## Related Functions
//!
//! - `AscB`: Returns the first byte of a string
//! - `AscW`: Returns the Unicode character code
//! - `Chr`: Returns the character for a given character code (inverse of Asc)
//! - `ChrB`: Returns a byte containing the character
//! - `ChrW`: Returns a Unicode character
//! - `InStr`: Finds the position of a character in a string
//! - `Mid`: Extracts a substring
//! - `Left`: Gets leftmost characters
//! - `Right`: Gets rightmost characters
//!
//! ## Performance Notes
//!
//! - Asc is a very fast operation (direct character code lookup)
//! - More efficient than string comparison for single character checks
//! - Use Asc for character-based validation instead of multiple string comparisons
//! - In tight loops, cache Asc results if checking the same character repeatedly
//!
//! ## Parsing Notes
//!
//! The `Asc` function is not a reserved keyword in VB6. It is parsed as a regular
//! function call (`CallExpression`).
//!
//! ## Encoding Notes
//!
//! VB6 stores strings as Unicode internally but `Asc` returns the code of the
//! first character converted to the system ANSI code page (Windows-1252). ASCII
//! characters map to themselves; characters representable in Windows-1252 map
//! to their ANSI byte (e.g. `é` -> 233, `€` -> 128); anything else raises
//! error 5. `AscW` is the Unicode code-point variant.

use crate::{
    error::VBResult,
    value::{VBLong, VBString},
};

/// Returns the Windows-1252 (ANSI) character code of the first character.
///
/// # Arguments
/// * `input` - The input string from which to get the character code.
///
/// # Returns
///
/// The Windows-1252 (ANSI) character code of the first character.
///
/// # Errors
///
/// Returns error 5 (`Invalid procedure call or argument`) when `input` is empty
/// or its first character cannot be represented in Windows-1252.
pub fn asc(input: &VBString) -> VBResult<VBLong> {
    Ok(VBLong::from(
        super::ansi::encode_first_byte(input.as_str()).map(i32::from)?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        error::err_number,
        value::{VBLong, VBString},
    };

    #[test]
    fn returns_ascii_codes() {
        assert_eq!(asc(&VBString::from("A")).unwrap(), VBLong::from(65));
        assert_eq!(asc(&VBString::from("a")).unwrap(), VBLong::from(97));
        assert_eq!(asc(&VBString::from("0")).unwrap(), VBLong::from(48));
        assert_eq!(asc(&VBString::from(" ")).unwrap(), VBLong::from(32));
    }

    #[test]
    fn uses_first_character_only() {
        assert_eq!(asc(&VBString::from("Apple")).unwrap(), VBLong::from(65));
        assert_eq!(asc(&VBString::from("A1")).unwrap(), VBLong::from(65));
    }

    #[test]
    fn maps_latin_1_to_ansi() {
        assert_eq!(asc(&VBString::from("é")).unwrap(), VBLong::from(233));
        assert_eq!(asc(&VBString::from("ñ")).unwrap(), VBLong::from(241));
    }

    #[test]
    fn maps_beyond_latin_1_via_windows_1252() {
        assert_eq!(asc(&VBString::from("€")).unwrap(), VBLong::from(128));
        assert_eq!(asc(&VBString::from("œ")).unwrap(), VBLong::from(156));
    }

    #[test]
    fn rejects_unrepresentable_characters() {
        assert_eq!(
            asc(&VBString::from("😀")).unwrap_err().number,
            err_number::INVALID_PROCEDURE_CALL
        );
    }

    #[test]
    fn rejects_empty_string() {
        assert_eq!(
            asc(&VBString::from("")).unwrap_err().number,
            err_number::INVALID_PROCEDURE_CALL
        );
    }
}
