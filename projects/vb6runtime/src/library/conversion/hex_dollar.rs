//! # `Hex$` Function
//!
//! Returns a string representing the hexadecimal (base-16) value of a number.
//!
//! ## Syntax
//!
//! ```vb6
//! Hex$(number)
//! ```
//!
//! ## Parameters
//!
//! - `number` - Required. Any valid numeric expression or string expression. If `number` is not a
//!   whole number, it is rounded to the nearest whole number before being evaluated.
//!
//! ## Return Value
//!
//! Returns a `String` representing the hexadecimal value of `number`. The string contains only
//! hexadecimal digits (0-9, A-F) without any prefix (no "0x" or "&H").
//!
//! ## Behavior and Characteristics
//!
//! ### Number Range and Representation
//!
//! - Positive numbers: Returns hexadecimal representation without leading zeros
//! - Negative numbers: Returns two's complement representation
//! - `Byte` values: Up to 2 hex digits (00-FF)
//! - `Integer` values: Up to 4 hex digits (0000-FFFF)
//! - `Long` values: Up to 8 hex digits (00000000-FFFFFFFF)
//! - Zero: Returns "0" (single character)
//! - If `number` contains `Null`, raises error 94 (Invalid use of Null)
//!
//! ### Type Differences: `Hex$` vs `Hex`
//!
//! - `Hex$`: Always returns `String` type (never `Variant`)
//! - `Hex`: Returns `Variant` (can propagate `Null` values)
//! - Use `Hex$` when you need guaranteed `String` return type
//! - Use `Hex` when working with potentially `Null` values
//!
//! ### Formatting Characteristics
//!
//! - No "0x" or "&H" prefix in output
//! - Uses uppercase letters (A-F, not a-f)
//! - No leading zeros for positive numbers (except zero itself)
//! - Negative numbers use two's complement representation
//! - Maximum 8 characters for `Long` type
//!
//! ## Common Usage Patterns
//!
//! ### 1. Convert Numbers to Hex Strings
//!
//! ```vb6
//! Function NumberToHex(value As Long) As String
//!     NumberToHex = Hex$(value)
//! End Function
//!
//! Debug.Print NumberToHex(255)      ' "FF"
//! Debug.Print NumberToHex(4096)     ' "1000"
//! Debug.Print NumberToHex(65535)    ' "FFFF"
//! ```
//!
//! ### 2. Display RGB Color Values
//!
//! ```vb6
//! Function ColorToHex(colorValue As Long) As String
//!     Dim hexStr As String
//!     hexStr = Hex$(colorValue)
//!     ' Pad to 6 characters for web colors
//!     ColorToHex = String$(6 - Len(hexStr), "0") & hexStr
//! End Function
//!
//! Dim webColor As String
//! webColor = "#" & ColorToHex(RGB(255, 128, 64))
//! ```
//!
//! ### 3. Debug Memory Addresses
//!
//! ```vb6
//! Function FormatAddress(address As Long) As String
//!     Dim hexAddr As String
//!     hexAddr = Hex$(address)
//!     ' Pad to 8 characters
//!     FormatAddress = "0x" & String$(8 - Len(hexAddr), "0") & hexAddr
//! End Function
//! ```
//!
//! ### 4. Generate Unique Identifiers
//!
//! ```vb6
//! Function GenerateHexID() As String
//!     Randomize
//!     Dim part1 As Long, part2 As Long
//!     part1 = Int(Rnd * &H7FFFFFFF)
//!     part2 = Int(Rnd * &H7FFFFFFF)
//!     GenerateHexID = Hex$(part1) & Hex$(part2)
//! End Function
//! ```
//!
//! ### 5. Format Byte Arrays as Hex Strings
//!
//! ```vb6
//! Function BytesToHex(bytes() As Byte) As String
//!     Dim result As String
//!     Dim i As Integer
//!     Dim hexByte As String
//!     
//!     For i = LBound(bytes) To UBound(bytes)
//!         hexByte = Hex$(bytes(i))
//!         If Len(hexByte) = 1 Then hexByte = "0" & hexByte
//!         result = result & hexByte
//!     Next i
//!     
//!     BytesToHex = result
//! End Function
//! ```
//!
//! ### 6. Log Error Codes in Hex
//!
//! ```vb6
//! Sub LogError(errNum As Long, errDesc As String)
//!     Dim logFile As Integer
//!     logFile = FreeFile
//!     Open "errors.log" For Append As #logFile
//!     Print #logFile, "Error 0x" & Hex$(errNum) & ": " & errDesc
//!     Close #logFile
//! End Sub
//! ```
//!
//! ### 7. Convert Character Codes
//!
//! ```vb6
//! Function CharToHex(ch As String) As String
//!     If Len(ch) > 0 Then
//!         CharToHex = Hex$(Asc(ch))
//!     Else
//!         CharToHex = ""
//!     End If
//! End Function
//!
//! Debug.Print CharToHex("A")  ' "41"
//! Debug.Print CharToHex("Z")  ' "5A"
//! ```
//!
//! ### 8. Create Hexadecimal Dump
//!
//! ```vb6
//! Function HexDump(data As String, Optional bytesPerLine As Integer = 16) As String
//!     Dim result As String
//!     Dim i As Long
//!     Dim hexVal As String
//!     
//!     For i = 1 To Len(data)
//!         hexVal = Hex$(Asc(Mid$(data, i, 1)))
//!         If Len(hexVal) = 1 Then hexVal = "0" & hexVal
//!         result = result & hexVal & " "
//!         
//!         If (i Mod bytesPerLine) = 0 Then
//!             result = result & vbCrLf
//!         End If
//!     Next i
//!     
//!     HexDump = result
//! End Function
//! ```
//!
//! ### 9. Parse and Format Checksums
//!
//! ```vb6
//! Function FormatChecksum(checksum As Long) As String
//!     Dim hexStr As String
//!     hexStr = Hex$(checksum)
//!     ' Pad to 8 characters
//!     FormatChecksum = String$(8 - Len(hexStr), "0") & hexStr
//! End Function
//!
//! Dim crc32 As Long
//! crc32 = CalculateCRC32(fileData)
//! Debug.Print "CRC32: " & FormatChecksum(crc32)
//! ```
//!
//! ### 10. Network Protocol Debugging
//!
//! ```vb6
//! Function FormatPacketHeader(packetType As Byte, packetLen As Integer) As String
//!     Dim typeHex As String, lenHex As String
//!     
//!     typeHex = Hex$(packetType)
//!     If Len(typeHex) = 1 Then typeHex = "0" & typeHex
//!     
//!     lenHex = Hex$(packetLen)
//!     While Len(lenHex) < 4
//!         lenHex = "0" & lenHex
//!     Wend
//!     
//!     FormatPacketHeader = "Type: 0x" & typeHex & " Len: 0x" & lenHex
//! End Function
//! ```
//!
//! ## Related Functions
//!
//! - `Hex()` - Returns a `Variant` containing the hexadecimal value (can handle `Null`)
//! - `Oct$()` - Returns the octal (base-8) representation of a number
//! - `Str$()` - Converts a number to its decimal string representation
//! - `Val()` - Converts a string to a numeric value
//! - `CLng()` - Converts an expression to a `Long` integer
//! - `Asc()` - Returns the character code of the first character in a string
//! - `Chr$()` - Returns the character associated with a character code
//! - `Format$()` - Formats expressions with more control over output
//!
//! ## Best Practices
//!
//! ### Padding Hex Values
//!
//! ```vb6
//! ' Pad to specific width for consistent formatting
//! Function PadHex(value As Long, width As Integer) As String
//!     Dim hexStr As String
//!     hexStr = Hex$(value)
//!     
//!     If Len(hexStr) < width Then
//!         PadHex = String$(width - Len(hexStr), "0") & hexStr
//!     Else
//!         PadHex = hexStr
//!     End If
//! End Function
//!
//! Debug.Print PadHex(255, 4)   ' "00FF"
//! Debug.Print PadHex(4096, 8)  ' "00001000"
//! ```
//!
//! ### Adding Hex Prefix
//!
//! ```vb6
//! Function HexWithPrefix(value As Long) As String
//!     HexWithPrefix = "&H" & Hex$(value)  ' VB6 style
//!     ' Or: HexWithPrefix = "0x" & Hex$(value)  ' C style
//! End Function
//! ```
//!
//! ### Converting Back from Hex String
//!
//! ```vb6
//! Function HexToLong(hexStr As String) As Long
//!     ' Remove any prefix
//!     If Left$(hexStr, 2) = "&H" Or Left$(hexStr, 2) = "0x" Then
//!         hexStr = Mid$(hexStr, 3)
//!     End If
//!     
//!     ' Convert using Val with &H prefix
//!     HexToLong = Val("&H" & hexStr)
//! End Function
//! ```
//!
//! ### Handling Byte Order (Endianness)
//!
//! ```vb6
//! Function LongToHexBytes(value As Long) As String
//!     Dim b1 As Byte, b2 As Byte, b3 As Byte, b4 As Byte
//!     
//!     b1 = value And &HFF
//!     b2 = (value \ &H100) And &HFF
//!     b3 = (value \ &H10000) And &HFF
//!     b4 = (value \ &H1000000) And &HFF
//!     
//!     ' Little-endian format
//!     LongToHexBytes = Right$("0" & Hex$(b1), 2) & " " & _
//!                      Right$("0" & Hex$(b2), 2) & " " & _
//!                      Right$("0" & Hex$(b3), 2) & " " & _
//!                      Right$("0" & Hex$(b4), 2)
//! End Function
//! ```
//!
//! ## Performance Considerations
//!
//! - `Hex$` is very fast for converting numbers to hexadecimal strings
//! - No significant performance difference between `Hex` and `Hex$` for non-Null values
//! - String concatenation in loops can be slow; consider building arrays and using `Join`
//! - For large byte arrays, consider buffering output
//!
//! ```vb6
//! ' Efficient for large arrays
//! Function BytesToHexEfficient(bytes() As Byte) As String
//!     Dim chunks() As String
//!     ReDim chunks(UBound(bytes) - LBound(bytes))
//!     
//!     Dim i As Long, idx As Long
//!     For i = LBound(bytes) To UBound(bytes)
//!         chunks(idx) = Right$("0" & Hex$(bytes(i)), 2)
//!         idx = idx + 1
//!     Next i
//!     
//!     BytesToHexEfficient = Join(chunks, "")
//! End Function
//! ```
//!
//! ## Common Pitfalls
//!
//! ### 1. No Automatic Padding
//!
//! ```vb6
//! ' Hex$ does NOT add leading zeros
//! Debug.Print Hex$(15)    ' "F" (not "0F")
//! Debug.Print Hex$(255)   ' "FF" (correct)
//! Debug.Print Hex$(16)    ' "10" (not "0010")
//!
//! ' Must pad manually for consistent width
//! Function PadHex(val As Integer) As String
//!     Dim h As String
//!     h = Hex$(val)
//!     PadHex = String$(4 - Len(h), "0") & h
//! End Function
//! ```
//!
//! ### 2. Negative Numbers Use Two's Complement
//!
//! ```vb6
//! ' Negative numbers are represented in two's complement
//! Debug.Print Hex$(-1)     ' "FFFFFFFF" (Long)
//! Debug.Print Hex$(-256)   ' "FFFFFF00"
//!
//! ' For signed interpretation, check range
//! Function SignedHex(value As Long) As String
//!     If value < 0 Then
//!         SignedHex = "-&H" & Hex$(Abs(value))
//!     Else
//!         SignedHex = "&H" & Hex$(value)
//!     End If
//! End Function
//! ```
//!
//! ### 3. No Prefix in Output
//!
//! ```vb6
//! ' Hex$ does NOT include "&H" or "0x" prefix
//! Dim hexValue As String
//! hexValue = Hex$(255)      ' "FF" (not "&HFF" or "0xFF")
//!
//! ' Add prefix manually if needed
//! hexValue = "&H" & Hex$(255)  ' "&HFF"
//! hexValue = "0x" & Hex$(255)  ' "0xFF"
//! ```
//!
//! ### 4. Uppercase Output Only
//!
//! ```vb6
//! ' Hex$ always returns uppercase A-F
//! Debug.Print Hex$(255)  ' "FF" (not "ff")
//!
//! ' Convert to lowercase if needed
//! hexValue = LCase$(Hex$(255))  ' "ff"
//! ```
//!
//! ### 5. Rounding of Non-Integer Values
//!
//! ```vb6
//! ' Non-integers are rounded before conversion
//! Debug.Print Hex$(15.3)   ' "F" (15 rounded)
//! Debug.Print Hex$(15.7)   ' "10" (16 rounded)
//! Debug.Print Hex$(15.5)   ' "10" (banker's rounding to even)
//!
//! ' Use Fix or Int if you need specific rounding
//! Debug.Print Hex$(Int(15.7))   ' "F" (truncated to 15)
//! Debug.Print Hex$(Fix(15.7))   ' "F" (truncated to 15)
//! ```
//!
//! ### 6. Type Range Limitations
//!
//! ```vb6
//! ' Different types have different ranges
//! Dim b As Byte
//! Dim i As Integer
//! Dim l As Long
//!
//! b = 255
//! Debug.Print Hex$(b)  ' "FF"
//!
//! i = -1
//! Debug.Print Hex$(i)  ' "FFFF" (16-bit two's complement)
//!
//! l = -1
//! Debug.Print Hex$(l)  ' "FFFFFFFF" (32-bit two's complement)
//! ```
//!
//! ## Practical Examples
//!
//! ### Memory Dump Utility
//!
//! ```vb6
//! Sub DumpMemory(startAddr As Long, length As Integer)
//!     Dim i As Integer
//!     Dim addr As Long
//!     Dim byteVal As Byte
//!     Dim line As String
//!     Dim ascii As String
//!     
//!     For i = 0 To length - 1
//!         If (i Mod 16) = 0 Then
//!             If i > 0 Then
//!                 Debug.Print line & "  " & ascii
//!             End If
//!             addr = startAddr + i
//!             line = Right$("00000000" & Hex$(addr), 8) & ": "
//!             ascii = ""
//!         End If
//!         
//!         ' Get byte value (pseudo-code)
//!         byteVal = GetMemoryByte(startAddr + i)
//!         line = line & Right$("0" & Hex$(byteVal), 2) & " "
//!         
//!         If byteVal >= 32 And byteVal <= 126 Then
//!             ascii = ascii & Chr$(byteVal)
//!         Else
//!             ascii = ascii & "."
//!         End If
//!     Next i
//!     
//!     ' Print last line
//!     If ascii <> "" Then
//!         Debug.Print line & String$(3 * (16 - Len(ascii)), " ") & "  " & ascii
//!     End If
//! End Sub
//! ```
//!
//! ### UUID/GUID Formatter
//!
//! ```vb6
//! Function FormatGUID(data1 As Long, data2 As Integer, data3 As Integer, _
//!                     data4() As Byte) As String
//!     Dim result As String
//!     Dim i As Integer
//!     
//!     result = Right$("00000000" & Hex$(data1), 8) & "-"
//!     result = result & Right$("0000" & Hex$(data2), 4) & "-"
//!     result = result & Right$("0000" & Hex$(data3), 4) & "-"
//!     
//!     For i = 0 To 1
//!         result = result & Right$("0" & Hex$(data4(i)), 2)
//!     Next i
//!     result = result & "-"
//!     
//!     For i = 2 To 7
//!         result = result & Right$("0" & Hex$(data4(i)), 2)
//!     Next i
//!     
//!     FormatGUID = result
//! End Function
//! ```
//!
//! ### Color Manipulation
//!
//! ```vb6
//! Function RGBToWebColor(r As Byte, g As Byte, b As Byte) As String
//!     RGBToWebColor = "#" & _
//!                     Right$("0" & Hex$(r), 2) & _
//!                     Right$("0" & Hex$(g), 2) & _
//!                     Right$("0" & Hex$(b), 2)
//! End Function
//!
//! Function WebColorToRGB(webColor As String, r As Byte, g As Byte, b As Byte)
//!     ' Remove # if present
//!     If Left$(webColor, 1) = "#" Then webColor = Mid$(webColor, 2)
//!     
//!     r = Val("&H" & Mid$(webColor, 1, 2))
//!     g = Val("&H" & Mid$(webColor, 3, 2))
//!     b = Val("&H" & Mid$(webColor, 5, 2))
//! End Function
//! ```
//!
//! ## Limitations
//!
//! - Returns only uppercase hexadecimal letters (A-F), not lowercase
//! - Does not include "&H" or "0x" prefix (must add manually)
//! - Does not pad with leading zeros (must pad manually)
//! - Cannot handle `Null` values (use `Hex` variant function instead)
//! - Limited to 32-bit `Long` integer range (no 64-bit support in VB6)
//! - Negative numbers return two's complement representation
//! - Fractional values are rounded before conversion
//! - No direct support for byte-order conversion (endianness)

use crate::error::{err_number, VBError, VBResult};
use crate::value::{VBString, VBVariant};

/// Implementation of the `Hex$` function.
///
/// Converts a number to its hexadecimal string representation. The result uses
/// uppercase letters (A-F) and does not include any prefix ("0x" or "&H").
///
/// VB6 behavior:
/// - Fractional values are rounded to the nearest integer before conversion
/// - Negative numbers use two's complement representation
/// - `Integer` values produce up to 4 hex digits
/// - `Long` values produce up to 8 hex digits
/// - Raises error 94 if `number` is `Null`
/// - Raises error 13 if `number` cannot be converted to a number
pub fn hex_dollar(number: &VBVariant) -> VBResult<VBString> {
    if number.is_null() {
        return Err(VBError::with_description(
            err_number::INVALID_USE_OF_NULL,
            "Invalid use of Null",
        ));
    }

    // Determine the bit width based on the variant type
    let value = match number {
        VBVariant::Integer(v) => {
            // Integer is 16-bit; mask to u16 to get correct hex representation
            format!("{:X}", *v as u16)
        }
        VBVariant::Long(v) => {
            format!("{:X}", *v as u32)
        }
        VBVariant::Byte(v) => {
            format!("{:X}", *v as u32)
        }
        _ => {
            // For other types (Double, Single, Currency, String, etc.),
            // convert to Long first via as_i64 which handles rounding
            let v = number.as_i64()?;
            if let Ok(long_val) = i32::try_from(v) {
                format!("{:X}", long_val as u32)
            } else {
                return Err(VBError::overflow());
            }
        }
    };

    Ok(VBString::from(value))
}

#[cfg(test)]
mod tests {
    use super::hex_dollar;
    use crate::error::err_number;
    use crate::value::VBVariant;

    #[test]
    fn hex_dollar_zero() {
        let result = hex_dollar(&VBVariant::from_long(0)).unwrap();
        assert_eq!(result.as_str(), "0");
    }

    #[test]
    fn hex_dollar_positive_long() {
        let result = hex_dollar(&VBVariant::from_long(255)).unwrap();
        assert_eq!(result.as_str(), "FF");

        let result = hex_dollar(&VBVariant::from_long(4096)).unwrap();
        assert_eq!(result.as_str(), "1000");

        let result = hex_dollar(&VBVariant::from_long(65535)).unwrap();
        assert_eq!(result.as_str(), "FFFF");
    }

    #[test]
    fn hex_dollar_positive_integer() {
        let result = hex_dollar(&VBVariant::from_integer(255)).unwrap();
        assert_eq!(result.as_str(), "FF");

        let result = hex_dollar(&VBVariant::from_integer(-1)).unwrap();
        assert_eq!(result.as_str(), "FFFF");
    }

    #[test]
    fn hex_dollar_negative_long() {
        let result = hex_dollar(&VBVariant::from_long(-1)).unwrap();
        assert_eq!(result.as_str(), "FFFFFFFF");

        let result = hex_dollar(&VBVariant::from_long(-256)).unwrap();
        assert_eq!(result.as_str(), "FFFFFF00");
    }

    #[test]
    fn hex_dollar_byte() {
        let result = hex_dollar(&VBVariant::from_byte(15)).unwrap();
        assert_eq!(result.as_str(), "F");

        let result = hex_dollar(&VBVariant::from_byte(255)).unwrap();
        assert_eq!(result.as_str(), "FF");
    }

    #[test]
    fn hex_dollar_double_rounds() {
        // 15.3 rounds to 15 → "F"
        let result = hex_dollar(&VBVariant::Double(15.3)).unwrap();
        assert_eq!(result.as_str(), "F");

        // 15.7 rounds to 16 → "10"
        let result = hex_dollar(&VBVariant::Double(15.7)).unwrap();
        assert_eq!(result.as_str(), "10");
    }

    #[test]
    fn hex_dollar_from_string() {
        let result = hex_dollar(&VBVariant::from_string("255")).unwrap();
        assert_eq!(result.as_str(), "FF");
    }

    #[test]
    fn hex_dollar_null_is_error_94() {
        let err = hex_dollar(&VBVariant::Null).unwrap_err();
        assert_eq!(err.number, err_number::INVALID_USE_OF_NULL);
    }

    #[test]
    fn hex_dollar_max_long() {
        let result = hex_dollar(&VBVariant::from_long(i32::MAX)).unwrap();
        assert_eq!(result.as_str(), "7FFFFFFF");
    }

    #[test]
    fn hex_dollar_min_long() {
        let result = hex_dollar(&VBVariant::from_long(i32::MIN)).unwrap();
        assert_eq!(result.as_str(), "80000000");
    }

    #[test]
    fn hex_dollar_uppercase() {
        // Values that produce A-F characters
        let result = hex_dollar(&VBVariant::from_long(10)).unwrap();
        assert_eq!(result.as_str(), "A");

        let result = hex_dollar(&VBVariant::from_long(11)).unwrap();
        assert_eq!(result.as_str(), "B");

        let result = hex_dollar(&VBVariant::from_long(15)).unwrap();
        assert_eq!(result.as_str(), "F");

        let result = hex_dollar(&VBVariant::from_long(0xABCD)).unwrap();
        assert_eq!(result.as_str(), "ABCD");
    }

    #[test]
    fn hex_dollar_power_of_two() {
        let result = hex_dollar(&VBVariant::from_long(16)).unwrap();
        assert_eq!(result.as_str(), "10");

        let result = hex_dollar(&VBVariant::from_long(256)).unwrap();
        assert_eq!(result.as_str(), "100");

        let result = hex_dollar(&VBVariant::from_long(4096)).unwrap();
        assert_eq!(result.as_str(), "1000");

        let result = hex_dollar(&VBVariant::from_long(65536)).unwrap();
        assert_eq!(result.as_str(), "10000");
    }

    #[test]
    fn hex_dollar_empty_returns_zero() {
        let result = hex_dollar(&VBVariant::Empty).unwrap();
        assert_eq!(result.as_str(), "0");
    }

    #[test]
    fn hex_dollar_small_values() {
        for i in 0..16 {
            let result = hex_dollar(&VBVariant::from_long(i)).unwrap();
            let expected = format!("{:X}", i);
            assert_eq!(result.as_str(), expected, "Failed for value {}", i);
        }
    }
}
