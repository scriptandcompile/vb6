//! # `RightB` Function
//!
//! Returns a Variant (String) containing a specified number of bytes from the right side of a string.
//!
//! ## Syntax
//!
//! ```vb
//! RightB(string, length)
//! ```
//!
//! ## Parameters
//!
//! - `string` (Required): String expression from which rightmost bytes are returned
//!   - If string contains Null, Null is returned
//! - `length` (Required): Numeric expression indicating how many bytes to return
//!   - If 0, empty string ("") is returned
//!   - If greater than or equal to number of bytes in string, entire string is returned
//!   - Must be non-negative (negative values cause error)
//!
//! ## Return Value
//!
//! Returns a Variant containing a String:
//! - Contains the specified number of bytes from the right side of the string
//! - Returns empty string if length is 0
//! - Returns entire string if length >= LenB(string)
//! - Returns Null if string argument is Null
//! - Returns Variant type (`RightB$` variant returns String type directly)
//!
//! ## Remarks
//!
//! The `RightB` function extracts bytes from the end of a string:
//!
//! - Returns rightmost bytes up to specified length
//! - Operates on byte level, not character level
//! - Particularly useful with double-byte character sets (DBCS)
//! - Complements `LeftB` function (which returns leftmost bytes)
//! - Works with `MidB` function for complete byte-level substring extraction
//! - Extraction from end: RightB("ABC", 2) returns last 2 bytes
//! - Safe with lengths exceeding string byte length (returns full string)
//! - Null propagates through the function
//! - Negative length raises Error 5 (Invalid procedure call or argument)
//! - Common for extracting binary data suffixes, checksums, trailers
//! - More efficient than `MidB` for right extraction
//! - `RightB$` variant returns String type (not Variant) for slight performance gain
//! - Cannot extract from left side (use `LeftB` for that)
//! - Cannot skip bytes (use `MidB` for that)
//! - Does not modify original string (strings are immutable)
//!
//! ## Differences from `Right` Function
//!
//! - `Right` operates on characters, `RightB` operates on bytes
//! - In single-byte character sets (SBCS), they are equivalent
//! - In double-byte character sets (DBCS), one character may be multiple bytes
//! - `RightB` is essential for binary data manipulation
//! - `RightB` is used with `LenB` (byte length) rather than `Len` (character length)
//!
//! ## Typical Uses
//!
//! 1. **Binary Data**: Extract trailing bytes from binary strings
//! 2. **Checksums**: Extract checksum bytes from data
//! 3. **File Trailers**: Extract trailing data from files
//! 4. **DBCS Strings**: Work with Japanese, Chinese, Korean text at byte level
//! 5. **Fixed Byte Records**: Parse trailing fields from binary records
//! 6. **Byte Validation**: Check byte suffixes in data
//! 7. **Binary Structures**: Extract trailing fields from binary structures
//! 8. **Network Data**: Process packet trailers
//!
//! ## Basic Usage Examples
//!
//! ```vb
//! ' Example 1: Basic byte extraction
//! Dim data As String
//! data = Chr$(65) & Chr$(66) & Chr$(67)  ' "ABC"
//!
//! Debug.Print RightB(data, 2)            ' Last 2 bytes
//! Debug.Print RightB(data, 1)            ' Last byte
//!
//! ' Example 2: Checksum extraction
//! Dim packet As String
//! packet = ReceiveNetworkData()
//!
//! Dim checksum As String
//! checksum = RightB(packet, 4)  ' 4-byte checksum at end
//!
//! ' Example 3: File trailer
//! Dim fileData As String
//! Open "data.bin" For Binary As #1
//! fileData = Input$(LOF(1), #1)
//! Close #1
//!
//! Dim trailer As String
//! trailer = RightB(fileData, 16)  ' 16-byte trailer
//!
//! ' Example 4: DBCS text handling
//! Dim japaneseText As String
//! japaneseText = LoadJapaneseText()  ' Load Japanese text
//!
//! Dim lastBytes As String
//! lastBytes = RightB(japaneseText, 4)  ' Last 4 bytes (may be 2 DBCS chars)
//! ```
//!
//! ## Common Patterns
//!
//! ```vb
//! ' Pattern 1: Extract checksum
//! Function GetChecksum(data As String) As String
//!     If LenB(data) < 4 Then
//!         GetChecksum = ""
//!     Else
//!         GetChecksum = RightB(data, 4)
//!     End If
//! End Function
//!
//! ' Pattern 2: Validate trailer
//! Function ValidateTrailer(data As String, trailer As String) As Boolean
//!     ValidateTrailer = (RightB(data, LenB(trailer)) = trailer)
//! End Function
//!
//! ' Pattern 3: Extract record suffix
//! Function GetRecordSuffix(record As String) As String
//!     ' Last 8 bytes contain record suffix
//!     GetRecordSuffix = RightB(record, 8)
//! End Function
//!
//! ' Pattern 4: Parse packet trailer
//! Sub ParsePacket(packet As String)
//!     Dim payload As String
//!     Dim trailer As String
//!     
//!     trailer = RightB(packet, 8)  ' 8-byte trailer
//!     payload = LeftB(packet, LenB(packet) - 8)  ' All but trailer
//!     
//!     ' Process payload and trailer
//! End Sub
//!
//! ' Pattern 5: Extract file extension bytes
//! Function GetExtensionBytes(fileName As String) As String
//!     ' Assumes extension is last 3 bytes after dot
//!     GetExtensionBytes = RightB(fileName, 3)
//! End Function
//! ```
//!
//! ## Advanced Examples
//!
//! ```vb
//! ' Example: CRC validation
//! Function ValidateCRC(data As String) As Boolean
//!     Dim crc As String
//!     Dim payload As String
//!     
//!     If LenB(data) < 4 Then
//!         ValidateCRC = False
//!         Exit Function
//!     End If
//!     
//!     crc = RightB(data, 4)
//!     payload = LeftB(data, LenB(data) - 4)
//!     
//!     ValidateCRC = (CalculateCRC(payload) = crc)
//! End Function
//!
//! ' Example: Extract GUID data
//! Sub ExtractGUID(guidData As String)
//!     Dim data4 As String
//!     data4 = RightB(guidData, 8)  ' Last 8 bytes of GUID
//!     Debug.Print "Data4: " & BytesToHex(data4)
//! End Sub
//!
//! ' Example: Binary record trailer
//! Function GetRecordTrailer(record As String) As String
//!     ' Records have 12-byte trailer
//!     Const TRAILER_SIZE As Long = 12
//!     
//!     If LenB(record) >= TRAILER_SIZE Then
//!         GetRecordTrailer = RightB(record, TRAILER_SIZE)
//!     Else
//!         GetRecordTrailer = record
//!     End If
//! End Function
//! ```
//!
//! ## See Also
//!
//! - `RightB$`: String-returning variant of `RightB`
//! - `LeftB`: Returns leftmost bytes from string
//! - `LeftB$`: String-returning variant of `LeftB`
//! - `MidB`: Returns bytes from middle of string
//! - `MidB$`: String-returning variant of `MidB`
//! - `LenB`: Returns byte length of string
//! - `Right`: Character-based right extraction
//! - `Left`: Character-based left extraction
//! - `Mid`: Character-based middle extraction

use crate::{
    error::VBResult,
    value::{VBLong, VBVariant},
};

use super::rightb_dollar::rightb_dollar;

/// `RightB` is the Variant-returning counterpart of `RightB$`; a `Null` input
/// propagates as `Null`.
///
/// # Errors
///
/// Returns error 5 (`Invalid procedure call or argument`) when `length` is negative.
pub fn rightb(input: &VBVariant, length: &VBLong) -> VBResult<VBVariant> {
    if input.is_null() {
        return Ok(VBVariant::Null);
    }
    rightb_dollar(&input.as_vbstring()?, length).map(VBVariant::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn propagates_null() {
        assert_eq!(
            rightb(&VBVariant::Null, &VBLong::from(2)).unwrap(),
            VBVariant::Null
        );
    }
}
