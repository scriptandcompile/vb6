//! # `RightB$` Function
//!
//! The `RightB$` function in Visual Basic 6 returns a string containing a specified number of
//! bytes from the right side (end) of a string. Unlike `Right$` which works with characters,
//! `RightB$` operates at the byte level, making it essential for binary data processing and
//! working with DBCS (Double-Byte Character Set) strings.
//!
//! ## Syntax
//!
//! ```vb6
//! RightB$(string, length)
//! ```
//!
//! ## Parameters
//!
//! - `string` - Required. String expression from which the rightmost bytes are returned.
//!   If `string` contains `Null`, `Null` is returned.
//! - `length` - Required. Numeric expression indicating how many bytes to return. If 0,
//!   a zero-length string ("") is returned. If greater than or equal to the number of bytes
//!   in `string`, the entire string is returned.
//!
//! ## Return Value
//!
//! Returns a `String` containing the rightmost `length` bytes of `string`.
//!
//! ## Behavior and Characteristics
//!
//! ### Byte vs. Character Operation
//!
//! - `RightB$` counts bytes, not characters
//! - In VB6, each character is stored as 2 bytes (UCS-2/Unicode)
//! - Extracting an odd number of bytes from Unicode text may split a character
//! - DBCS characters (e.g., Japanese, Chinese) may require 2 bytes per character
//!
//! ### Length Handling
//!
//! - If `length` = 0: Returns an empty string ("")
//! - If `length` >= `LenB(string)`: Returns the entire string
//! - If `length` < 0: Generates a runtime error (Invalid procedure call or argument)
//! - If `string` is empty (""): Returns an empty string regardless of `length`
//!
//! ### Character Set Considerations
//!
//! - VB6 stores strings internally as Unicode (UCS-2)
//! - Each character typically occupies 2 bytes
//! - Extracting an odd number of bytes can result in incomplete characters
//! - Use `Right$` for character-based extraction in most cases
//!
//! ## Common Usage Patterns
//!
//! ### 1. Extract Binary Data Suffix
//!
//! ```vb6
//! Function GetBinarySuffix(data As String, numBytes As Integer) As String
//!     GetBinarySuffix = RightB$(data, numBytes)
//! End Function
//!
//! Dim suffix As String
//! suffix = GetBinarySuffix(binaryData, 4)  ' Get last 4 bytes
//! ```
//!
//! ### 2. Read File Trailer
//!
//! ```vb6
//! Function ReadFileTrailer(fileData As String) As String
//!     ' Get last 8 bytes of file (e.g., signature or checksum)
//!     ReadFileTrailer = RightB$(fileData, 8)
//! End Function
//! ```
//!
//! ### 3. Extract Protocol Footer
//!
//! ```vb6
//! Function GetProtocolFooter(packet As String) As String
//!     ' Get 2-byte footer from network packet
//!     GetProtocolFooter = RightB$(packet, 2)
//! End Function
//! ```
//!
//! ### 4. Process Binary Structures
//!
//! ```vb6
//! Type FileHeader
//!     signature As String * 4
//!     version As String * 2
//! End Type
//!
//! Function GetVersion(headerData As String) As String
//!     ' Extract last 2 bytes as version info
//!     GetVersion = RightB$(headerData, 2)
//! End Function
//! ```
//!
//! ### 5. Network Packet Checksum
//!
//! ```vb6
//! Function ExtractChecksum(packet As String) As String
//!     ' Network packets often have 2-4 byte checksums at end
//!     ExtractChecksum = RightB$(packet, 4)
//! End Function
//! ```
//!
//! ### 6. GUID/UUID Component Extraction
//!
//! ```vb6
//! Function GetGuidSuffix(guidData As String) As String
//!     ' Extract last 6 bytes of GUID (node portion)
//!     GetGuidSuffix = RightB$(guidData, 6)
//! End Function
//! ```
//!
//! ### 7. File Format Magic Bytes (Trailer)
//!
//! ```vb6
//! Function CheckFileTrailer(fileData As String, expectedTrailer As String) As Boolean
//!     Dim trailer As String
//!     trailer = RightB$(fileData, LenB(expectedTrailer))
//!     CheckFileTrailer = (trailer = expectedTrailer)
//! End Function
//! ```
//!
//! ### 8. Extract Record Suffix
//!
//! ```vb6
//! Function GetRecordSuffix(record As String) As String
//!     ' Fixed-length binary record with 4-byte suffix
//!     GetRecordSuffix = RightB$(record, 4)
//! End Function
//! ```
//!
//! ### 9. Image Data Footer
//!
//! ```vb6
//! Function GetImageFooter(imageData As String) As String
//!     ' Some image formats have trailers (e.g., JPEG end marker)
//!     GetImageFooter = RightB$(imageData, 2)
//! End Function
//! ```
//!
//! ### 10. Database Record Alignment
//!
//! ```vb6
//! Function AlignRecordEnd(record As String, alignment As Integer) As String
//!     ' Get aligned portion from end of record
//!     Dim alignedSize As Integer
//!     alignedSize = (LenB(record) \ alignment) * alignment
//!     If alignedSize > 0 Then
//!         AlignRecordEnd = RightB$(record, alignedSize)
//!     Else
//!         AlignRecordEnd = record
//!     End If
//! End Function
//! ```
//!
//! ## Related Functions
//!
//! - `Right$()` - Returns a specified number of characters from the right (character-based)
//! - `LeftB$()` - Returns a specified number of bytes from the left side of a string
//! - `MidB$()` - Returns a specified number of bytes from any position in a string
//! - `LenB()` - Returns the number of bytes used to represent a string in memory
//! - `RightB()` - Variant version that can return `Null`
//! - `InStrB()` - Finds the byte position of a substring
//! - `AscB()` - Returns the byte value of the first byte in a string
//! - `ChrB$()` - Returns a string containing a single-byte character
//!
//! ## Best Practices
//!
//! ### When to Use `RightB$` vs `Right$`
//!
//! ```vb6
//! ' Use Right$ for text/character operations
//! Dim fileExt As String
//! fileExt = Right$(fileName, 3)  ' Get last 3 characters
//!
//! ' Use RightB$ for binary data operations
//! Dim checksum As String
//! checksum = RightB$(binaryData, 4)  ' Get last 4 bytes
//! ```
//!
//! ### Validate Byte Count
//!
//! ```vb6
//! Function SafeRightB(data As String, numBytes As Integer) As String
//!     If numBytes < 0 Then
//!         SafeRightB = ""
//!     ElseIf numBytes >= LenB(data) Then
//!         SafeRightB = data
//!     Else
//!         SafeRightB = RightB$(data, numBytes)
//!     End If
//! End Function
//! ```
//!
//! ### Use with `LenB` for Byte Counting
//!
//! ```vb6
//! ' Always use LenB when working with RightB$
//! Dim totalBytes As Long
//! Dim footer As String
//! totalBytes = LenB(binaryData)
//! footer = RightB$(binaryData, 8)
//! ```
//!
//! ### Combine with `AscB` for Byte Values
//!
//! ```vb6
//! Function GetLastByte(data As String) As Byte
//!     Dim lastByte As String
//!     lastByte = RightB$(data, 1)
//!     GetLastByte = AscB(lastByte)
//! End Function
//! ```
//!
//! ### Binary Structure Parsing
//!
//! ```vb6
//! ' Extract multiple fields from end of structure
//! Dim checksum As String
//! Dim version As String
//! checksum = RightB$(structData, 4)
//! version = RightB$(LeftB$(structData, LenB(structData) - 4), 2)
//! ```
//!
//! ## Performance Considerations
//!
//! - `RightB$` is efficient for binary data operations
//! - Slightly faster than `Right$` for byte-aligned operations
//! - Avoid using in tight loops with string concatenation
//! - Consider byte arrays for large binary data processing
//!
//! ```vb6
//! ' Less efficient: multiple RightB$ calls
//! For i = 1 To 1000
//!     result = result & RightB$(data, 4)
//! Next i
//!
//! ' More efficient: single operation or byte array
//! Dim parts() As String
//! ReDim parts(1 To 1000)
//! For i = 1 To 1000
//!     parts(i) = RightB$(data, 4)
//! Next i
//! result = Join(parts, "")
//! ```
//!
//! ## Character Encoding and VB6
//!
//! ### Unicode String Storage
//!
//! VB6 stores strings internally as Unicode (UCS-2):
//!
//! ```vb6
//! Dim text As String
//! text = "AB"
//! Debug.Print Len(text)   ' Prints 2 (characters)
//! Debug.Print LenB(text)  ' Prints 4 (bytes: 2 bytes per character)
//!
//! Dim lastChar As String
//! lastChar = Right$(text, 1)   ' Returns "B" (1 character)
//! Dim lastTwoBytes As String
//! lastTwoBytes = RightB$(text, 2)  ' Returns "B" (last 2 bytes = 1 character)
//! ```
//!
//! ### DBCS Considerations
//!
//! When working with Double-Byte Character Sets:
//!
//! ```vb6
//! ' Be careful with DBCS text - extracting odd number of bytes
//! ' can split multi-byte characters
//! Dim japaneseText As String
//! japaneseText = "こんにちは"
//!
//! ' Right$ extracts by characters (safe)
//! Dim lastChar As String
//! lastChar = Right$(japaneseText, 1)  ' Gets last character properly
//!
//! ' RightB$ extracts by bytes (may split characters)
//! Dim lastByte As String
//! lastByte = RightB$(japaneseText, 1)  ' May get incomplete character!
//! ```
//!
//! ## Common Pitfalls
//!
//! ### 1. Confusing Bytes with Characters
//!
//! ```vb6
//! Dim text As String
//! text = "Hello"
//!
//! ' Wrong: thinking RightB$ works like Right$
//! result = RightB$(text, 3)  ' Gets last 3 BYTES (not characters)
//! ' With Unicode, 3 bytes = 1.5 characters (1 complete + half of another)
//!
//! ' Correct: use Right$ for character operations
//! result = Right$(text, 3)  ' Gets "llo"
//! ```
//!
//! ### 2. Odd Byte Counts with Unicode
//!
//! ```vb6
//! ' Problematic: odd number of bytes splits Unicode characters
//! Dim text As String
//! text = "Test"
//! Dim partial As String
//! partial = RightB$(text, 3)  ' 3 bytes = 1 character + half a character
//! ' Result may be unexpected or corrupted
//!
//! ' Better: use even byte counts for Unicode
//! Dim proper As String
//! proper = RightB$(text, 4)  ' 4 bytes = 2 complete characters
//! ```
//!
//! ### 3. Not Using `LenB` for Length
//!
//! ```vb6
//! ' Wrong: using Len instead of LenB
//! If RightB$(data, Len(data) - 4) Then  ' Incorrect!
//!
//! ' Correct: use LenB for byte operations
//! If RightB$(data, LenB(data) - 4) Then
//! ```
//!
//! ### 4. Negative Length Values
//!
//! ```vb6
//! ' Runtime error: Invalid procedure call or argument
//! result = RightB$("Hello", -1)  ' ERROR!
//!
//! ' Validate first
//! If numBytes >= 0 Then
//!     result = RightB$(data, numBytes)
//! End If
//! ```
//!
//! ### 5. Assuming ASCII/ANSI Encoding
//!
//! ```vb6
//! ' Wrong: assuming 1 byte = 1 character
//! Dim data As String
//! data = "ABCD"
//! Dim lastByte As String
//! lastByte = RightB$(data, 1)  ' Gets 1 byte, not last character
//! ' In Unicode, this is half of the last character
//!
//! ' Correct: use Right$ or account for 2 bytes per character
//! Dim lastChar As String
//! lastChar = Right$(data, 1)   ' Gets "D"
//! ' OR
//! lastChar = RightB$(data, 2)  ' Gets last 2 bytes = "D"
//! ```
//!
//! ## Limitations
//!
//! - Cannot handle `Null` values (use `RightB` variant function instead)
//! - Works with bytes, not characters (can split multi-byte characters)
//! - Negative `length` values cause runtime errors
//! - Limited usefulness for Unicode text (use `Right$` instead)
//! - No built-in validation for character boundaries
//! - Extracting odd byte counts from Unicode strings can produce invalid characters
//! - Less intuitive than `Right$` for general string processing

use crate::{
    error::{err_number, VBError, VBResult},
    value::{VBLong, VBString},
};

/// Number of bytes used to encode one character in this runtime's UTF-16 model.
const BYTES_PER_CHAR: i32 = 2;

/// Returns the rightmost characters of `input` whose encoded size, in bytes,
/// does not exceed `length`.
///
/// Each character occupies 2 bytes, so `RightB` extracts `length / 2`
/// characters; a trailing partial byte is discarded. A `length` of 0 yields an
/// empty string and a `length` at least `LenB(input)` yields the whole string.
/// The `$` suffix indicates this function returns a `String` type (not `Variant`).
///
/// # Errors
///
/// Returns error 5 (`Invalid procedure call or argument`) when `length` is negative.
pub fn rightb_dollar(input: &VBString, length: &VBLong) -> VBResult<VBString> {
    let length = length.as_i32();
    if length < 0 {
        return Err(VBError::with_description(
            err_number::INVALID_PROCEDURE_CALL,
            "Invalid length",
        ));
    }
    let chars: Vec<char> = input.as_str().chars().collect();
    let n = (length / BYTES_PER_CHAR) as usize;
    let start = chars.len().saturating_sub(n);
    Ok(VBString::from(chars[start..].iter().collect::<String>()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::err_number;

    #[test]
    fn returns_rightmost_bytes() {
        assert_eq!(
            rightb_dollar(&VBString::from("ABC"), &VBLong::from(2)).unwrap(),
            VBString::from("C")
        );
        assert_eq!(
            rightb_dollar(&VBString::from("ABC"), &VBLong::from(4)).unwrap(),
            VBString::from("BC")
        );
        assert_eq!(
            rightb_dollar(&VBString::from("ABC"), &VBLong::from(0)).unwrap(),
            VBString::from("")
        );
    }

    #[test]
    fn length_beyond_byte_count_returns_whole() {
        assert_eq!(
            rightb_dollar(&VBString::from("ABC"), &VBLong::from(99)).unwrap(),
            VBString::from("ABC")
        );
    }

    #[test]
    fn negative_length_errors() {
        let err = rightb_dollar(&VBString::from("ABC"), &VBLong::from(-1)).unwrap_err();
        assert_eq!(err.number, err_number::INVALID_PROCEDURE_CALL);
    }
}
