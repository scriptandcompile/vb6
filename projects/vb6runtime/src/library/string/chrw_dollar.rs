//! # `ChrW$` Function
//!
//! Returns a `String` containing the Unicode character associated with the specified character code.
//! The dollar sign suffix (`$`) explicitly indicates that this function returns a `String` type
//! (not a `Variant`), and the "W" suffix indicates this is the wide (Unicode) version.
//!
//! ## Syntax
//!
//! ```vb
//! ChrW$(charcode)
//! ```
//!
//! ## Parameters
//!
//! - **`charcode`**: Required. `Long` value that identifies a Unicode character. Valid values are
//!   -32768 to 65535. The range 0-65535 represents Unicode characters. Negative values are treated
//!   as unsigned values (e.g., -1 becomes 65535).
//!
//! ## Return Value
//!
//! Returns a `String` containing the single Unicode character corresponding to the specified character
//! code. The return value is always a `String` type (never `Variant`), and represents a Unicode
//! character (2 bytes).
//!
//! ## Remarks
//!
//! - The `ChrW$` function combines the behavior of `ChrW` (Unicode character) with the `$` suffix
//!   (explicit `String` return type).
//! - Valid range: -32768 to 65535 (values outside this range may cause errors).
//! - `ChrW$` returns Unicode characters, allowing access to the full Unicode Basic Multilingual Plane (BMP).
//! - For values 0-127, `ChrW$` and `Chr$` return the same ASCII characters.
//! - For values 128-255, `ChrW$` returns Unicode characters while `Chr$` returns ANSI characters.
//! - For values above 255, only `ChrW$` can be used (not `Chr$` or `ChrB$`).
//! - `ChrW$(0)` returns a null character (`vbNullChar`).
//! - `ChrW$(13)` returns carriage return (`vbCr`).
//! - `ChrW$(10)` returns line feed (`vbLf`).
//! - `ChrW$(9)` returns tab character (`vbTab`).
//! - The inverse function is `AscW`, which returns the Unicode character code of a character.
//! - For better performance when you know the result is a string, use `ChrW$` instead of `ChrW`.
//!
//! ## Common Character Codes
//!
//! | Code | Character | Constant | Description |
//! |------|-----------|----------|-------------|
//! | 0 | (null) | `vbNullChar` | Null character |
//! | 9 | \t | `vbTab` | Horizontal tab |
//! | 10 | \n | `vbLf` | Line feed |
//! | 13 | \r | `vbCr` | Carriage return |
//! | 32 | (space) | - | Space character |
//! | 34 | " | - | Double quote |
//! | 39 | ' | - | Single quote |
//! | 65 | A | - | Uppercase A |
//! | 97 | a | - | Lowercase a |
//! | 169 | © | - | Copyright symbol |
//! | 8364 | € | - | Euro sign |
//!
//! ## Typical Uses
//!
//! 1. **Unicode characters** - Access characters beyond the ANSI range
//! 2. **International text** - Work with non-English characters
//! 3. **Special symbols** - Insert mathematical, currency, and other Unicode symbols
//! 4. **Line breaks** - Insert carriage returns and line feeds
//! 5. **Emoji and symbols** - Access Unicode symbols (within BMP range)
//! 6. **Cross-platform text** - Generate Unicode text for better compatibility
//! 7. **Web content** - Create Unicode strings for web applications
//!
//! ## Basic Examples
//!
//! ```vb
//! ' Example 1: Get character from code
//! Dim ch As String
//! ch = ChrW$(65)  ' Returns "A"
//! ```
//!
//! ```vb
//! ' Example 2: Unicode character beyond ANSI
//! Dim euro As String
//! euro = ChrW$(8364)  ' Returns "€"
//! ```
//!
//! ```vb
//! ' Example 3: Copyright symbol
//! Dim copyright As String
//! copyright = ChrW$(169)  ' Returns "©"
//! ```
//!
//! ```vb
//! ' Example 4: Line break
//! Dim msg As String
//! msg = "Line 1" & ChrW$(13) & ChrW$(10) & "Line 2"
//! ```
//!
//! ## Common Patterns
//!
//! ### Multi-line Strings
//! ```vb
//! Function CreateMultiLine() As String
//!     Dim result As String
//!     result = "First Line" & ChrW$(13) & ChrW$(10)
//!     result = result & "Second Line" & ChrW$(13) & ChrW$(10)
//!     result = result & "Third Line"
//!     CreateMultiLine = result
//! End Function
//! ```
//!
//! ### Unicode Symbols
//! ```vb
//! Function CreateCopyrightNotice(year As String, company As String) As String
//!     CreateCopyrightNotice = "Copyright " & ChrW$(169) & " " & year & " " & company
//! End Function
//! ```
//!
//! ### Currency Symbols
//! ```vb
//! Function FormatPrice(amount As Double, currency As String) As String
//!     Dim symbol As String
//!     Select Case currency
//!         Case "EUR"
//!             symbol = ChrW$(8364)  ' €
//!         Case "GBP"
//!             symbol = ChrW$(163)   ' £
//!         Case "YEN"
//!             symbol = ChrW$(165)   ' ¥
//!         Case Else
//!             symbol = "$"
//!     End Select
//!     FormatPrice = symbol & Format(amount, "0.00")
//! End Function
//! ```
//!
//! ### International Characters
//! ```vb
//! Function GetGreeting(language As String) As String
//!     Select Case language
//!         Case "German"
//!             GetGreeting = "Gr" & ChrW$(252) & ChrW$(223) & " Gott"  ' Grüß Gott
//!         Case "French"
//!             GetGreeting = "Fran" & ChrW$(231) & "ais"  ' Français
//!         Case "Spanish"
//!             GetGreeting = "Espa" & ChrW$(241) & "ol"   ' Español
//!         Case Else
//!             GetGreeting = "Hello"
//!     End Select
//! End Function
//! ```
//!
//! ### Mathematical Symbols
//! ```vb
//! Function CreateMathExpression() As String
//!     Dim result As String
//!     result = "x " & ChrW$(8804) & " 10"  ' x ≤ 10
//!     result = result & " " & ChrW$(8743) & " "  ' ∧ (and)
//!     result = result & "x " & ChrW$(8805) & " 0"  ' x ≥ 0
//!     CreateMathExpression = result
//! End Function
//! ```
//!
//! ### Tab-Separated Values
//! ```vb
//! Function CreateTSV(col1 As String, col2 As String, col3 As String) As String
//!     CreateTSV = col1 & ChrW$(9) & col2 & ChrW$(9) & col3
//! End Function
//! ```
//!
//! ### Quote in String
//! ```vb
//! Function AddQuotes(text As String) As String
//!     AddQuotes = ChrW$(34) & text & ChrW$(34)
//! End Function
//! ```
//!
//! ### Bullet Points
//! ```vb
//! Function CreateBulletList() As String
//!     Dim result As String
//!     Dim bullet As String
//!     bullet = ChrW$(8226)  ' •
//!     result = bullet & " First item" & ChrW$(13) & ChrW$(10)
//!     result = result & bullet & " Second item" & ChrW$(13) & ChrW$(10)
//!     result = result & bullet & " Third item"
//!     CreateBulletList = result
//! End Function
//! ```
//!
//! ### Null-Terminated String
//! ```vb
//! Function CreateNullTerminated(text As String) As String
//!     CreateNullTerminated = text & ChrW$(0)
//! End Function
//! ```
//!
//! ### Generate Alphabet
//! ```vb
//! Function GenerateAlphabet() As String
//!     Dim i As Integer
//!     Dim result As String
//!     For i = 65 To 90
//!         result = result & ChrW$(i)
//!     Next i
//!     GenerateAlphabet = result  ' Returns "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
//! End Function
//! ```
//!
//! ## Related Functions
//!
//! - `ChrW`: Returns Unicode character as `Variant` instead of `String`
//! - `Chr$`: Returns ANSI/system character (limited to 0-255)
//! - `ChrB$`: Returns byte character (limited to 0-255)
//! - `AscW`: Returns Unicode character code (inverse of `ChrW$`)
//! - `Asc`: Returns ANSI character code
//! - `AscB`: Returns byte value
//!
//! ## Error Handling
//!
//! ```vb
//! Function SafeChrW(code As Long) As String
//!     On Error Resume Next
//!     SafeChrW = ChrW$(code)
//!     If Err.Number <> 0 Then
//!         SafeChrW = ""
//!         Err.Clear
//!     End If
//! End Function
//! ```
//!
//! ## Performance Considerations
//!
//! - `ChrW$` is slightly more efficient than `ChrW` because it avoids `Variant` overhead
//! - For building strings from many characters, consider using arrays and `Join`
//! - Concatenating many `ChrW$` calls can be slow; use buffers for better performance
//! - Unicode strings may take more memory than ANSI strings
//!
//! ## Best Practices
//!
//! 1. Use named constants for common characters instead of magic numbers
//! 2. Use `ChrW$` for Unicode characters, `Chr$` for ANSI-only characters
//! 3. Document character codes with comments showing the actual character
//! 4. Validate character codes are in valid range before calling `ChrW$`
//! 5. Use `vbCrLf` constant instead of `ChrW$(13) & ChrW$(10)` when possible
//! 6. Prefer `ChrW$` over `ChrW` when you need a `String` result
//! 7. Consider internationalization when working with Unicode characters
//!
//! ## Unicode Ranges
//!
//! - 0-127: ASCII characters (same as ANSI)
//! - 128-255: Latin-1 Supplement
//! - 256-383: Latin Extended-A
//! - 384-591: Latin Extended-B
//! - 8192-8303: General Punctuation
//! - 8352-8399: Currency Symbols
//! - 8448-8527: Letterlike Symbols
//! - 8592-8703: Arrows
//! - 8704-8959: Mathematical Operators
//!
//! ## Limitations
//!
//! - Limited to Unicode BMP (Basic Multilingual Plane) - codes 0-65535
//! - Cannot directly create characters from supplementary planes (codes > 65535)
//! - VB6 uses UCS-2 encoding, not full UTF-16
//! - Some Unicode characters may not display correctly depending on font support

use crate::{
    error::{err_number, VBError, VBResult},
    value::{VBLong, VBString},
};

/// Returns the Unicode character associated with the specified code.
/// The `$` suffix indicates this function returns a `String` type (not `Variant`).
///
/// `charcode` is in the range -32768 to 65535; negative values are treated as
/// `65536 + charcode`, matching VB6's 16-bit integer behavior. Code 0 returns
/// the null character (U+0000).
///
/// # Errors
///
/// Returns error 5 (`Invalid procedure call or argument`) when `charcode` is
/// outside the range -32768 to 65535, or names a UTF-16 surrogate (which cannot
/// be represented as a single Rust `char`).
pub fn chrw_dollar(charcode: &VBLong) -> VBResult<VBString> {
    let charcode = charcode.as_i32();
    if !(-32768..=65535).contains(&charcode) {
        return Err(VBError::with_description(
            err_number::INVALID_PROCEDURE_CALL,
            "Character code out of range",
        ));
    }

    let code = if charcode < 0 {
        charcode + 65536
    } else {
        charcode
    } as u32;

    char::from_u32(code)
        .map(|c| VBString::from(c.to_string()))
        .ok_or_else(|| {
            VBError::with_description(
                err_number::INVALID_PROCEDURE_CALL,
                "Invalid Unicode character code",
            )
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::VBLong;

    #[test]
    fn returns_ascii_characters() {
        assert_eq!(chrw_dollar(&VBLong::from(65)).unwrap(), VBString::from("A"));
        assert_eq!(chrw_dollar(&VBLong::from(97)).unwrap(), VBString::from("a"));
    }

    #[test]
    fn returns_unicode_characters() {
        assert_eq!(
            chrw_dollar(&VBLong::from(8364)).unwrap(),
            VBString::from("€")
        );
        assert_eq!(
            chrw_dollar(&VBLong::from(20013)).unwrap(),
            VBString::from("中")
        );
    }

    #[test]
    fn negative_values_are_wrapped() {
        assert_eq!(
            chrw_dollar(&VBLong::from(-1)).unwrap(),
            VBString::from("\u{FFFF}")
        );
        assert_eq!(
            chrw_dollar(&VBLong::from(-8192)).unwrap(),
            VBString::from("\u{E000}")
        );
    }

    #[test]
    fn code_zero_returns_null_character() {
        assert_eq!(
            chrw_dollar(&VBLong::from(0)).unwrap(),
            VBString::from("\u{0}")
        );
    }

    #[test]
    fn rejects_out_of_range() {
        assert_eq!(
            chrw_dollar(&VBLong::from(-32769)).unwrap_err().number,
            err_number::INVALID_PROCEDURE_CALL
        );
        assert_eq!(
            chrw_dollar(&VBLong::from(65536)).unwrap_err().number,
            err_number::INVALID_PROCEDURE_CALL
        );
    }
}
