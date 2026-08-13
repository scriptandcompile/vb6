//! # `Hex` Function
//!
//! Returns a `String` representing the hexadecimal value of a number.
//!
//! ## Syntax
//!
//! ```vb
//! Hex(number)
//! ```
//!
//! ## Parameters
//!
//! - `number` (Required): Any valid numeric expression or string expression. If `number` is not already a whole number, it is rounded to the nearest whole number before being evaluated.
//!
//! ## Return Value
//!
//! Returns a `String` representing the hexadecimal value of the number. The return value contains hexadecimal digits (0-9, A-F) without the "&H" prefix.
//!
//! ## Remarks
//!
//! The `Hex` function converts a decimal number to its hexadecimal (base 16) string representation:
//!
//! - If `number` is `Null`, `Hex` returns `Null`
//! - If `number` is `Empty`, `Hex` returns "0"
//! - Negative numbers are represented in two's complement form
//! - For Byte values: Returns up to 2 hexadecimal digits
//! - For Integer values: Returns up to 4 hexadecimal digits
//! - For Long values: Returns up to 8 hexadecimal digits
//! - Fractional values are rounded to the nearest integer before conversion
//! - The result does not include the "&H" prefix (use "&H" & `Hex(n)` to include it)
//! - Leading zeros are not included in the result (e.g., 15 returns "F", not "0F")
//! - For hexadecimal to decimal conversion, use the `CLng` or `CInt` functions with "&H" prefix
//!
//! ## Typical Uses
//!
//! 1. **Color Values**: Convert RGB color values to hexadecimal for HTML/CSS
//! 2. **Memory Addresses**: Display memory addresses in hexadecimal format
//! 3. **Debugging**: Show binary data in readable hexadecimal format
//! 4. **Low-Level Programming**: Work with hardware registers and bit patterns
//! 5. **File I/O**: Display byte values when working with binary files
//! 6. **Cryptography**: Show hash values and checksums in hex format
//!
//! ## Basic Usage Examples
//!
//! ```vb
//! ' Example 1: Simple conversion
//! Debug.Print Hex(255)        ' Prints: FF
//! Debug.Print Hex(16)         ' Prints: 10
//! Debug.Print Hex(0)          ' Prints: 0
//!
//! ' Example 2: Color conversion
//! Dim red As Long
//! red = RGB(255, 0, 0)
//! Debug.Print Hex(red)        ' Prints: FF (on little-endian systems)
//!
//! ' Example 3: Negative numbers (two's complement)
//! Debug.Print Hex(-1)         ' Prints: FFFFFFFF (Long)
//! Debug.Print Hex(-256)       ' Prints: FFFFFF00
//!
//! ' Example 4: With "&H" prefix
//! Dim value As Long
//! value = 42
//! Debug.Print "&H" & Hex(value)  ' Prints: &H2A
//! ```
//!
//! ## Common Patterns
//!
//! ```vb
//! ' Pattern 1: RGB to hex color string
//! Function RGBToHex(r As Integer, g As Integer, b As Integer) As String
//!     RGBToHex = Right$("0" & Hex(r), 2) & _
//!                Right$("0" & Hex(g), 2) & _
//!                Right$("0" & Hex(b), 2)
//! End Function
//!
//! ' Pattern 2: Pad hex string to specific length
//! Function HexPad(value As Long, length As Integer) As String
//!     HexPad = Right$(String$(length, "0") & Hex(value), length)
//! End Function
//!
//! ' Pattern 3: Display memory dump
//! Sub ShowMemoryDump(bytes() As Byte)
//!     Dim i As Long
//!     Dim line As String
//!     For i = LBound(bytes) To UBound(bytes)
//!         line = line & Right$("0" & Hex(bytes(i)), 2) & " "
//!         If (i + 1) Mod 16 = 0 Then
//!             Debug.Print line
//!             line = ""
//!         End If
//!     Next i
//!     If Len(line) > 0 Then Debug.Print line
//! End Sub
//!
//! ' Pattern 4: Format address with hex
//! Function FormatAddress(addr As Long) As String
//!     FormatAddress = "0x" & Right$("00000000" & Hex(addr), 8)
//! End Function
//!
//! ' Pattern 5: Convert byte array to hex string
//! Function BytesToHex(bytes() As Byte) As String
//!     Dim result As String
//!     Dim i As Long
//!     For i = LBound(bytes) To UBound(bytes)
//!         result = result & Right$("0" & Hex(bytes(i)), 2)
//!     Next i
//!     BytesToHex = result
//! End Function
//!
//! ' Pattern 6: Parse hex string back to number
//! Function HexToLong(hexStr As String) As Long
//!     If Left$(hexStr, 2) = "&H" Then
//!         HexToLong = CLng(hexStr)
//!     Else
//!         HexToLong = CLng("&H" & hexStr)
//!     End If
//! End Function
//!
//! ' Pattern 7: Show bit pattern
//! Sub ShowBitPattern(value As Long)
//!     Debug.Print "Hex: " & Hex(value)
//! End Sub
//!
//! ' Pattern 8: Color component extraction
//! Function GetRedComponent(color As Integer) As Integer
//!     GetRedComponent = color And &HFF
//!     Debug.Print "Red: " & Hex(GetRedComponent)
//! End Function
//!
//! ' Pattern 9: Build lookup table
//! Dim hexLookup(0 To 255) As String
//! Sub InitHexLookup()
//!     Dim i As Long
//!     For i = 0 To 255
//!         hexLookup(i) = Right$("0" & Hex(i), 2)
//!     Next i
//! End Sub
//!
//! ' Pattern 10: Format hex with separator
//! Function HexWithSeparator(value As Long, separator As String) As String
//!     Dim hexStr As String
//!     Dim i As Integer
//!     hexStr = Right$("00000000" & Hex(value), 8)
//!     For i = 1 To 7 Step 2
//!         HexWithSeparator = HexWithSeparator & Mid$(hexStr, i, 2)
//!         If i < 7 Then HexWithSeparator = HexWithSeparator & separator
//!     Next i
//! End Function
//! ```
//!
//! ## Advanced Usage Examples
//!
//! ```vb
//! ' Example 1: Hex dump utility class
//! Public Class HexDumper
//!     Private Const BYTES_PER_LINE As Integer = 16
//!     
//!     Public Function DumpToString(data() As Byte) As String
//!         Dim result As String
//!         Dim i As Long
//!         Dim offset As Long
//!         Dim hexPart As String
//!         Dim asciiPart As String
//!         Dim b As Byte
//!         
//!         For i = LBound(data) To UBound(data)
//!             If i Mod BYTES_PER_LINE = 0 Then
//!                 If i > 0 Then
//!                     result = result & hexPart & "  " & asciiPart & vbCrLf
//!                 End If
//!                 hexPart = Right$("00000000" & Hex(i), 8) & ": "
//!                 asciiPart = ""
//!             End If
//!             
//!             b = data(i)
//!             hexPart = hexPart & Right$("0" & Hex(b), 2) & " "
//!             
//!             If b >= 32 And b <= 126 Then
//!                 asciiPart = asciiPart & Chr$(b)
//!             Else
//!                 asciiPart = asciiPart & "."
//!             End If
//!         Next i
//!         
//!         ' Pad last line
//!         If Len(hexPart) > 10 Then
//!             hexPart = hexPart & String$((BYTES_PER_LINE - Len(asciiPart)) * 3, " ")
//!             result = result & hexPart & "  " & asciiPart
//!         End If
//!         
//!         DumpToString = result
//!     End Function
//! End Class
//!
//! ' Example 2: HTML color converter
//! Public Class ColorConverter
//!     Public Function VBColorToHTML(vbColor As Long) As String
//!         Dim r As Integer, g As Integer, b As Integer
//!         r = vbColor And &HFF
//!         g = (vbColor \ &H100) And &HFF
//!         b = (vbColor \ &H10000) And &HFF
//!         
//!         VBColorToHTML = "#" & _
//!             Right$("0" & Hex(r), 2) & _
//!             Right$("0" & Hex(g), 2) & _
//!             Right$("0" & Hex(b), 2)
//!     End Function
//!     
//!     Public Function HTMLToVBColor(htmlColor As String) As Long
//!         Dim r As Long, g As Long, b As Long
//!         If Left$(htmlColor, 1) = "#" Then htmlColor = Mid$(htmlColor, 2)
//!         
//!         r = CLng("&H" & Mid$(htmlColor, 1, 2))
//!         g = CLng("&H" & Mid$(htmlColor, 3, 2))
//!         b = CLng("&H" & Mid$(htmlColor, 5, 2))
//!         
//!         HTMLToVBColor = RGB(r, g, b)
//!     End Function
//! End Class
//!
//! ' Example 3: Checksum calculator
//! Public Function CalculateChecksum(data() As Byte) As String
//!     Dim checksum As Long
//!     Dim i As Long
//!     
//!     For i = LBound(data) To UBound(data)
//!         checksum = checksum Xor data(i)
//!         checksum = ((checksum And &H7FFFFFFF) * 2) Or (checksum And &H80000000) \ &H80000000
//!     Next i
//!     
//!     CalculateChecksum = Right$("00000000" & Hex(checksum), 8)
//! End Function
//!
//! ' Example 4: Binary file viewer
//! Public Class BinaryFileViewer
//!     Public Function LoadAndDisplayFile(filePath As String) As String
//!         Dim fileNum As Integer
//!         Dim fileData() As Byte
//!         Dim fileSize As Long
//!         Dim result As String
//!         Dim i As Long
//!         
//!         fileNum = FreeFile
//!         Open filePath For Binary As #fileNum
//!         fileSize = LOF(fileNum)
//!         ReDim fileData(0 To fileSize - 1)
//!         Get #fileNum, , fileData
//!         Close #fileNum
//!         
//!         For i = 0 To UBound(fileData)
//!             If i Mod 16 = 0 Then
//!                 result = result & vbCrLf & Right$("00000000" & Hex(i), 8) & ": "
//!             End If
//!             result = result & Right$("0" & Hex(fileData(i)), 2) & " "
//!         Next i
//!         
//!         LoadAndDisplayFile = result
//!     End Function
//! End Class
//! ```
//!
//! ## Error Handling
//!
//! The Hex function generally doesn't raise errors, but type conversion can:
//!
//! - **Type Mismatch (Error 13)**: If the argument cannot be converted to a number
//! - **Overflow (Error 6)**: If the value exceeds Long range before conversion
//! - **Null Propagation**: If the argument is Null, Hex returns Null
//!
//! ```vb
//! On Error Resume Next
//! Dim result As String
//! result = Hex(someValue)
//! If Err.Number <> 0 Then
//!     Debug.Print "Error converting to hex: " & Err.Description
//!     Err.Clear
//! End If
//! On Error GoTo 0
//! ```
//!
//! ## Performance Considerations
//!
//! - **Fast Operation**: Hex conversion is a fast, native operation
//! - **String Building**: When converting many values, use string builder pattern to avoid repeated concatenation
//! - **Lookup Tables**: For repeated conversions of small numbers (0-255), consider pre-building a lookup table
//! - **Memory**: The returned string length depends on the magnitude of the number
//!
//! ## Best Practices
//!
//! 1. **Padding**: Use Right$() or Format$() to pad hex values to fixed width
//! 2. **Prefixes**: Add "&H" prefix when the value will be parsed back to numeric
//! 3. **Case**: VB6 Hex returns uppercase (A-F); convert to lowercase if needed
//! 4. **Validation**: Validate input before conversion to avoid type mismatch errors
//! 5. **Documentation**: Document whether hex strings include prefixes ("0x", "&H")
//! 6. **Null Handling**: Check for Null before calling Hex if working with Variants
//!
//! ## Comparison with Other Functions
//!
//! | Function | Purpose | Example |
//! |----------|---------|---------|
//! | Hex | Decimal to hexadecimal string | Hex(255) → "FF" |
//! | Oct | Decimal to octal string | Oct(8) → "10" |
//! | Str | Number to decimal string | Str(255) → " 255" |
//! | Format | Number to formatted string | Format(255, "00") → "255" |
//! | CLng("&H...") | Hexadecimal string to Long | CLng("&HFF") → 255 |
//!
//! ## Platform and Version Notes
//!
//! - Available in all VB6 versions
//! - Behavior consistent across Windows platforms
//! - Integer size (2 bytes) and Long size (4 bytes) are fixed
//! - Two's complement representation for negative numbers is standard
//!
//! ## Limitations
//!
//! - Cannot directly convert Currency or Decimal types (convert to Long first)
//! - No built-in support for padding with leading zeros (use Right$ or Format$)
//! - Returns uppercase letters only (A-F, not a-f)
//! - No control over prefix inclusion (always excludes "&H")
//! - Maximum value is Long range (-2,147,483,648 to 2,147,483,647)
//! - Fractional parts are rounded, not truncated
//!
//! ## Related Functions
//!
//! - `Oct`: Returns a String representing the octal value of a number
//! - `Str`: Returns a String representation of a number
//! - `Format`: Returns a Variant (String) formatted according to instructions
//! - `CLng`: Converts an expression to a Long
//! - `CInt`: Converts an expression to an Integer
//! - `Val`: Returns the numbers contained in a string as a numeric value

use crate::error::{VBError, VBResult};
use crate::value::VBVariant;

/// Implementation of the `Hex` function.
///
/// Converts a number to its hexadecimal string representation. The result uses
/// uppercase letters (A-F) and does not include any prefix ("0x" or "&H").
///
/// Unlike `Hex$`, this variant propagates `Null` — if the argument is `Null`,
/// it returns `Null` rather than raising an error.
///
/// VB6 behavior:
/// - Fractional values are rounded to the nearest integer before conversion
/// - Negative numbers use two's complement representation
/// - `Integer` values produce up to 4 hex digits
/// - `Long` values produce up to 8 hex digits
/// - Returns `Null` if `number` is `Null`
/// - Raises error 13 if `number` cannot be converted to a number
pub fn hex(number: &VBVariant) -> VBResult<VBVariant> {
    if number.is_null() {
        return Ok(VBVariant::Null);
    }

    let result = match number {
        VBVariant::Integer(v) => format!("{:X}", *v as u16),
        VBVariant::Long(v) => format!("{:X}", *v as u32),
        VBVariant::Byte(v) => format!("{:X}", *v as u32),
        _ => {
            let v = number.as_i64()?;
            if let Ok(long_val) = i32::try_from(v) {
                format!("{:X}", long_val as u32)
            } else {
                return Err(VBError::overflow());
            }
        }
    };

    Ok(VBVariant::from_string(&result))
}

#[cfg(test)]
mod tests {
    use super::hex;
    use crate::value::VBVariant;

    #[test]
    fn hex_zero() {
        let result = hex(&VBVariant::from_long(0)).unwrap();
        assert_eq!(result, VBVariant::from_string("0"));
    }

    #[test]
    fn hex_positive_long() {
        assert_eq!(hex(&VBVariant::from_long(255)).unwrap(), VBVariant::from_string("FF"));
        assert_eq!(hex(&VBVariant::from_long(4096)).unwrap(), VBVariant::from_string("1000"));
        assert_eq!(hex(&VBVariant::from_long(65535)).unwrap(), VBVariant::from_string("FFFF"));
    }

    #[test]
    fn hex_positive_integer() {
        assert_eq!(hex(&VBVariant::from_integer(255)).unwrap(), VBVariant::from_string("FF"));
        assert_eq!(hex(&VBVariant::from_integer(-1)).unwrap(), VBVariant::from_string("FFFF"));
    }

    #[test]
    fn hex_negative_long() {
        assert_eq!(hex(&VBVariant::from_long(-1)).unwrap(), VBVariant::from_string("FFFFFFFF"));
        assert_eq!(hex(&VBVariant::from_long(-256)).unwrap(), VBVariant::from_string("FFFFFF00"));
    }

    #[test]
    fn hex_byte() {
        assert_eq!(hex(&VBVariant::from_byte(15)).unwrap(), VBVariant::from_string("F"));
        assert_eq!(hex(&VBVariant::from_byte(255)).unwrap(), VBVariant::from_string("FF"));
    }

    #[test]
    fn hex_double_rounds() {
        // 15.3 rounds to 15 → "F"
        assert_eq!(hex(&VBVariant::Double(15.3)).unwrap(), VBVariant::from_string("F"));
        // 15.7 rounds to 16 → "10"
        assert_eq!(hex(&VBVariant::Double(15.7)).unwrap(), VBVariant::from_string("10"));
    }

    #[test]
    fn hex_from_string() {
        assert_eq!(hex(&VBVariant::from_string("255")).unwrap(), VBVariant::from_string("FF"));
    }

    #[test]
    fn hex_null_returns_null() {
        let result = hex(&VBVariant::Null).unwrap();
        assert!(result.is_null());
    }

    #[test]
    fn hex_max_long() {
        assert_eq!(hex(&VBVariant::from_long(i32::MAX)).unwrap(), VBVariant::from_string("7FFFFFFF"));
    }

    #[test]
    fn hex_min_long() {
        assert_eq!(hex(&VBVariant::from_long(i32::MIN)).unwrap(), VBVariant::from_string("80000000"));
    }

    #[test]
    fn hex_uppercase() {
        assert_eq!(hex(&VBVariant::from_long(10)).unwrap(), VBVariant::from_string("A"));
        assert_eq!(hex(&VBVariant::from_long(11)).unwrap(), VBVariant::from_string("B"));
        assert_eq!(hex(&VBVariant::from_long(15)).unwrap(), VBVariant::from_string("F"));
        assert_eq!(hex(&VBVariant::from_long(0xABCD)).unwrap(), VBVariant::from_string("ABCD"));
    }

    #[test]
    fn hex_power_of_two() {
        assert_eq!(hex(&VBVariant::from_long(16)).unwrap(), VBVariant::from_string("10"));
        assert_eq!(hex(&VBVariant::from_long(256)).unwrap(), VBVariant::from_string("100"));
        assert_eq!(hex(&VBVariant::from_long(4096)).unwrap(), VBVariant::from_string("1000"));
        assert_eq!(hex(&VBVariant::from_long(65536)).unwrap(), VBVariant::from_string("10000"));
    }

    #[test]
    fn hex_empty_returns_zero() {
        let result = hex(&VBVariant::Empty).unwrap();
        assert_eq!(result, VBVariant::from_string("0"));
    }

    #[test]
    fn hex_small_values() {
        for i in 0..16 {
            let result = hex(&VBVariant::from_long(i)).unwrap();
            let expected = format!("{:X}", i);
            assert_eq!(result, VBVariant::from_string(&expected), "Failed for value {}", i);
        }
    }

    #[test]
    fn hex_variants_propagate_null() {
        // Hex returns Null for Null input (different from Hex$ which raises error 94)
        let result = hex(&VBVariant::Null).unwrap();
        assert!(result.is_null());
    }
}
