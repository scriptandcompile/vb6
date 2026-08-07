//! # `Chr` Function
//!
//! Returns a `String` containing the character associated with the specified character code.
//!
//! ## Syntax
//!
//! ```vb
//! Chr(charcode)
//! ```
//!
//! ## Parameters
//!
//! - **`charcode`**: Required. Long value that identifies a character. The valid range for
//!   `charcode` is 0-255. For values outside this range, an error occurs.
//!
//! ## Return Value
//!
//! Returns a `String` containing the single character corresponding to the specified character
//! code. For charcode values 0-127, this corresponds to the ASCII character set. For values
//! 128-255, this corresponds to the extended ASCII or ANSI character set based on the system's
//! code page.
//!
//! ## Remarks
//!
//! The `Chr` function is the inverse of the `Asc` function. While `Asc` returns the numeric
//! character code of a character, `Chr` returns the character for a given code.
//!
//! **Important Characteristics:**
//!
//! - Valid range: 0-255 (Error 5 "Invalid procedure call or argument" for values outside range)
//! - Chr(0) returns a null character (vbNullChar)
//! - Chr(13) returns carriage return (vbCr)
//! - Chr(10) returns line feed (vbLf)
//! - Chr(9) returns tab character (vbTab)
//! - Values 0-31 are non-printable control characters
//! - Values 32-126 are standard printable ASCII characters
//! - Values 127-255 depend on the system code page (often Windows-1252 in VB6)
//!
//! ## Common Character Codes
//!
//! | Code | Character | Constant | Description |
//! |------|-----------|----------|-------------|
//! | 0 | (null) | vbNullChar | Null character |
//! | 9 | \t | vbTab | Horizontal tab |
//! | 10 | \n | vbLf | Line feed |
//! | 13 | \r | vbCr | Carriage return |
//! | 32 | (space) | - | Space character |
//! | 34 | " | - | Double quote |
//! | 39 | ' | - | Single quote |
//! | 65 | A | - | Uppercase A |
//! | 97 | a | - | Lowercase a |
//!
//! ## Examples
//!
//! ### Basic Usage
//!
//! ```vb
//! ' Get character from code
//! Dim ch As String
//! ch = Chr(65)  ' Returns "A"
//! ch = Chr(97)  ' Returns "a"
//! ch = Chr(48)  ' Returns "0"
//!
//! ' Special characters
//! ch = Chr(32)  ' Returns space " "
//! ch = Chr(13)  ' Returns carriage return
//! ch = Chr(10)  ' Returns line feed
//! ```
//!
//! ### Line Breaks and Special Characters
//!
//! ```vb
//! ' Create multi-line string
//! Dim msg As String
//! msg = "Line 1" & Chr(13) & Chr(10) & "Line 2"
//! ' Or use the constant
//! msg = "Line 1" & vbCrLf & "Line 2"
//!
//! ' Tab-separated values
//! Dim data As String
//! data = "Name" & Chr(9) & "Age" & Chr(9) & "City"
//! ```
//!
//! ### Building Strings from Codes
//!
//! ```vb
//! ' Build alphabet
//! Dim i As Integer
//! Dim alphabet As String
//! For i = 65 To 90
//!     alphabet = alphabet & Chr(i)
//! Next i
//! ' alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
//! ```
//!
//! ## Common Patterns
//!
//! ### Generating Character Sequences
//!
//! ```vb
//! Function GetAlphabet(uppercase As Boolean) As String
//!     Dim result As String
//!     Dim i As Integer
//!     Dim startCode As Integer
//!     
//!     If uppercase Then
//!         startCode = 65  ' 'A'
//!     Else
//!         startCode = 97  ' 'a'
//!     End If
//!     
//!     For i = startCode To startCode + 25
//!         result = result & Chr(i)
//!     Next i
//!     
//!     GetAlphabet = result
//! End Function
//! ```
//!
//! ### Quote Handling
//!
//! ```vb
//! Function QuoteString(text As String) As String
//!     QuoteString = Chr(34) & text & Chr(34)
//! End Function
//!
//! ' Usage: result = QuoteString("Hello")  ' Returns: "Hello"
//! ```
//!
//! ### CSV Generation
//!
//! ```vb
//! Function CreateCSVRow(ParamArray fields() As Variant) As String
//!     Dim result As String
//!     Dim i As Integer
//!     Dim field As String
//!     
//!     For i = LBound(fields) To UBound(fields)
//!         field = CStr(fields(i))
//!         
//!         ' Quote fields containing commas or quotes
//!         If InStr(field, ",") > 0 Or InStr(field, Chr(34)) > 0 Then
//!             field = Chr(34) & Replace(field, Chr(34), Chr(34) & Chr(34)) & Chr(34)
//!         End If
//!         
//!         If i > LBound(fields) Then result = result & ","
//!         result = result & field
//!     Next i
//!     
//!     CreateCSVRow = result
//! End Function
//! ```
//!
//! ### Control Character Removal
//!
//! ```vb
//! Function RemoveControlChars(text As String) As String
//!     Dim result As String
//!     Dim i As Integer
//!     Dim ch As String
//!     Dim code As Integer
//!     
//!     For i = 1 To Len(text)
//!         ch = Mid(text, i, 1)
//!         code = Asc(ch)
//!         
//!         ' Keep only printable characters (32-126) and common whitespace
//!         If code >= 32 Or code = 9 Or code = 10 Or code = 13 Then
//!             result = result & ch
//!         End If
//!     Next i
//!     
//!     RemoveControlChars = result
//! End Function
//! ```
//!
//! ### String Encoding/Decoding
//!
//! ```vb
//! Function EncodeString(text As String) As String
//!     Dim result As String
//!     Dim i As Integer
//!     
//!     For i = 1 To Len(text)
//!         If i > 1 Then result = result & ","
//!         result = result & CStr(Asc(Mid(text, i, 1)))
//!     Next i
//!     
//!     EncodeString = result
//! End Function
//!
//! Function DecodeString(encoded As String) As String
//!     Dim result As String
//!     Dim codes() As String
//!     Dim i As Integer
//!     
//!     codes = Split(encoded, ",")
//!     For i = LBound(codes) To UBound(codes)
//!         result = result & Chr(CLng(codes(i)))
//!     Next i
//!     
//!     DecodeString = result
//! End Function
//! ```
//!
//! ### Random Character Generation
//!
//! ```vb
//! Function GeneratePassword(length As Integer) As String
//!     Dim result As String
//!     Dim i As Integer
//!     Dim charType As Integer
//!     
//!     Randomize
//!     
//!     For i = 1 To length
//!         charType = Int(Rnd * 3)  ' 0=uppercase, 1=lowercase, 2=digit
//!         
//!         Select Case charType
//!             Case 0  ' Uppercase A-Z
//!                 result = result & Chr(65 + Int(Rnd * 26))
//!             Case 1  ' Lowercase a-z
//!                 result = result & Chr(97 + Int(Rnd * 26))
//!             Case 2  ' Digit 0-9
//!                 result = result & Chr(48 + Int(Rnd * 10))
//!         End Select
//!     Next i
//!     
//!     GeneratePassword = result
//! End Function
//! ```
//!
//! ### Box Drawing Characters
//!
//! ```vb
//! Function DrawBox(width As Integer, height As Integer) As String
//!     Dim result As String
//!     Dim i As Integer
//!     
//!     ' Top border (using extended ASCII box characters)
//!     result = Chr(218)  ' Top-left corner
//!     For i = 1 To width - 2
//!         result = result & Chr(196)  ' Horizontal line
//!     Next i
//!     result = result & Chr(191) & vbCrLf  ' Top-right corner
//!     
//!     ' Middle rows
//!     For i = 1 To height - 2
//!         result = result & Chr(179)  ' Vertical line
//!         result = result & Space(width - 2)
//!         result = result & Chr(179) & vbCrLf  ' Vertical line
//!     Next i
//!     
//!     ' Bottom border
//!     result = result & Chr(192)  ' Bottom-left corner
//!     For i = 1 To width - 2
//!         result = result & Chr(196)  ' Horizontal line
//!     Next i
//!     result = result & Chr(217)  ' Bottom-right corner
//!     
//!     DrawBox = result
//! End Function
//! ```
//!
//! ### Character Case Conversion
//!
//! ```vb
//! Function ToggleCase(text As String) As String
//!     Dim result As String
//!     Dim i As Integer
//!     Dim ch As String
//!     Dim code As Integer
//!     
//!     For i = 1 To Len(text)
//!         ch = Mid(text, i, 1)
//!         code = Asc(ch)
//!         
//!         If code >= 65 And code <= 90 Then
//!             ' Uppercase -> lowercase
//!             result = result & Chr(code + 32)
//!         ElseIf code >= 97 And code <= 122 Then
//!             ' Lowercase -> uppercase
//!             result = result & Chr(code - 32)
//!         Else
//!             result = result & ch
//!         End If
//!     Next i
//!     
//!     ToggleCase = result
//! End Function
//! ```
//!
//! ## Advanced Usage
//!
//! ### Binary Data Handling
//!
//! ```vb
//! Function BytesToString(bytes() As Byte) As String
//!     Dim result As String
//!     Dim i As Long
//!     
//!     For i = LBound(bytes) To UBound(bytes)
//!         result = result & Chr(bytes(i))
//!     Next i
//!     
//!     BytesToString = result
//! End Function
//!
//! Function StringToBytes(text As String) As Byte()
//!     Dim bytes() As Byte
//!     Dim i As Long
//!     
//!     ReDim bytes(1 To Len(text))
//!     
//!     For i = 1 To Len(text)
//!         bytes(i) = Asc(Mid(text, i, 1))
//!     Next i
//!     
//!     StringToBytes = bytes
//! End Function
//! ```
//!
//! ### Unicode Support (`ChrW` variant)
//!
//! ```vb
//! ' Note: VB6 has ChrW for Unicode characters
//! Function GetUnicodeChar(code As Long) As String
//!     ' For codes 0-255, Chr and ChrW are equivalent
//!     If code <= 255 Then
//!         GetUnicodeChar = Chr(code)
//!     Else
//!         ' For codes > 255, use ChrW (not covered by Chr function)
//!         GetUnicodeChar = ChrW(code)
//!     End If
//! End Function
//! ```
//!
//! ### Escape Sequence Processing
//!
//! ```vb
//! Function ProcessEscapeSequences(text As String) As String
//!     Dim result As String
//!     result = text
//!     
//!     ' Replace common escape sequences
//!     result = Replace(result, "\n", Chr(10))   ' Line feed
//!     result = Replace(result, "\r", Chr(13))   ' Carriage return
//!     result = Replace(result, "\t", Chr(9))    ' Tab
//!     result = Replace(result, "\\", Chr(92))   ' Backslash
//!     result = Replace(result, "\""", Chr(34))  ' Double quote
//!     
//!     ProcessEscapeSequences = result
//! End Function
//! ```
//!
//! ## Error Handling
//!
//! ```vb
//! Function SafeChr(charcode As Long) As String
//!     On Error GoTo ErrorHandler
//!     
//!     If charcode < 0 Or charcode > 255 Then
//!         Err.Raise 5, , "Invalid character code: " & charcode
//!     End If
//!     
//!     SafeChr = Chr(charcode)
//!     Exit Function
//!     
//! ErrorHandler:
//!     MsgBox "Error in Chr: " & Err.Description
//!     SafeChr = ""
//! End Function
//! ```
//!
//! ### Common Errors
//!
//! - **Error 5** (Invalid procedure call or argument): Character code is outside the range 0-255
//! - **Error 13** (Type mismatch): Argument is not numeric
//!
//! ## Performance Considerations
//!
//! - `Chr` is a fast function with minimal overhead
//! - For building long strings with many `Chr` calls, consider using a `StringBuilder` pattern
//! - Avoid repeated `Chr` calls for the same character code (use a constant instead)
//! - For Unicode support beyond 255, use `ChrW` or `ChrB` functions
//!
//! ## VB6 String Constants vs `Chr`
//!
//! VB6 provides built-in constants for common characters:
//!
//! ```vb
//! ' Prefer constants over Chr for readability
//! vbCr        ' Chr(13) - Carriage return
//! vbLf        ' Chr(10) - Line feed
//! vbCrLf      ' Chr(13) & Chr(10) - Carriage return + line feed
//! vbTab       ' Chr(9) - Tab
//! vbNullChar  ' Chr(0) - Null character
//! vbNullString ' Empty string ""
//! ```
//!
//! ## Limitations
//!
//! - Limited to character codes 0-255 (single-byte characters)
//! - For Unicode beyond 255, use `ChrW` instead
//! - Character interpretation depends on system code page
//! - Control characters (0-31) may not display in UI controls
//! - Extended ASCII (128-255) may vary across systems
//!
//! ## Related Functions
//!
//! - `Asc`: Returns the character code of the first character in a string (inverse of Chr)
//! - `ChrW`: Returns Unicode character for character codes 0-65535
//! - `ChrB`: Returns a byte containing the character code
//! - `AscW`: Returns the Unicode character code
//! - `AscB`: Returns the byte value

use crate::{
    error::{err_number, VBError, VBResult},
    value::{VBLong, VBString},
};

/// Returns the character associated with the specified Windows-1252 (ANSI) code.
///
/// `charcode` must be in the range 0-255. Code 0 returns the null character
/// (U+0000, `vbNullChar`); codes 128-255 are decoded as Windows-1252.
///
/// # Errors
///
/// Returns error 5 (`Invalid procedure call or argument`) when `charcode` is
/// outside the range 0-255.
pub fn chr(charcode: &VBLong) -> VBResult<VBString> {
    let charcode = charcode.as_i32();
    if !(0..=255).contains(&charcode) {
        return Err(VBError::with_description(
            err_number::INVALID_PROCEDURE_CALL,
            "Character code out of range",
        ));
    }
    Ok(VBString::from(super::ansi::decode_byte(charcode as u8)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::VBLong;

    #[test]
    fn returns_ascii_characters() {
        assert_eq!(chr(&VBLong::from(65)).unwrap(), VBString::from("A"));
        assert_eq!(chr(&VBLong::from(97)).unwrap(), VBString::from("a"));
        assert_eq!(chr(&VBLong::from(34)).unwrap(), VBString::from("\""));
    }

    #[test]
    fn returns_ansi_extended_characters() {
        assert_eq!(chr(&VBLong::from(128)).unwrap(), VBString::from("€"));
        assert_eq!(chr(&VBLong::from(233)).unwrap(), VBString::from("é"));
    }

    #[test]
    fn code_zero_returns_null_character() {
        assert_eq!(chr(&VBLong::from(0)).unwrap(), VBString::from("\u{0}"));
    }

    #[test]
    fn rejects_out_of_range() {
        assert_eq!(
            chr(&VBLong::from(-1)).unwrap_err().number,
            err_number::INVALID_PROCEDURE_CALL
        );
        assert_eq!(
            chr(&VBLong::from(256)).unwrap_err().number,
            err_number::INVALID_PROCEDURE_CALL
        );
    }
}
