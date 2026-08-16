//! # `Oct$` Function
//!
//! The `Oct$` function in Visual Basic 6 returns a string representing the octal (base-8) value
//! of a number. The function name stands for "Octal String".
//!
//! ## Syntax
//!
//! ```vb6
//! Oct$(number)
//! ```
//!
//! ## Parameters
//!
//! - `number` - Required. Any valid numeric expression. If `number` is not a whole number, it is
//!   rounded to the nearest whole number before being evaluated.
//!
//! ## Return Value
//!
//! Returns a `String` representing the octal value of the number. The returned string contains
//! only the digits 0-7, without a leading "0" or "&O" prefix.
//!
//! ## Behavior and Characteristics
//!
//! ### Data Type Handling
//!
//! - Accepts any numeric type: `Byte`, `Integer`, `Long`, `Single`, `Double`, `Currency`
//! - Floating-point values are rounded to the nearest integer before conversion
//! - Negative numbers are represented using two's complement notation
//! - Returns unsigned octal representation for the underlying bit pattern
//!
//! ### Range Considerations
//!
//! - `Integer` values: Returns 1-6 octal digits (range: 0 to 177777 for positive, 100000-177777 for negative)
//! - `Long` values: Returns 1-11 octal digits (range: 0 to 17777777777 for positive)
//! - `Byte` values: Returns 1-3 octal digits (range: 0 to 377)
//!
//! ## Common Usage Patterns
//!
//! ### 1. Basic Octal Conversion
//!
//! ```vb6
//! Dim octStr As String
//! octStr = Oct$(64)  ' Returns "100"
//! octStr = Oct$(8)   ' Returns "10"
//! octStr = Oct$(511) ' Returns "777"
//! ```
//!
//! ### 2. Converting Negative Numbers
//!
//! ```vb6
//! Dim octStr As String
//! octStr = Oct$(-1)  ' Returns "177777" (Integer range, two's complement)
//! ```
//!
//! ### 3. File Permission Representation
//!
//! ```vb6
//! Function FormatPermissions(permissions As Integer) As String
//!     ' Unix-style file permissions (e.g., 755, 644)
//!     FormatPermissions = Oct$(permissions)
//! End Function
//!
//! Dim perms As String
//! perms = FormatPermissions(&H1ED)  ' Returns "755"
//! ```
//!
//! ### 4. Bit Mask Display
//!
//! ```vb6
//! Dim flags As Integer
//! Dim octDisplay As String
//! flags = &H1FF
//! octDisplay = "Flags: " & Oct$(flags)  ' "Flags: 777"
//! ```
//!
//! ### 5. Color Component Extraction (Octal)
//!
//! ```vb6
//! Dim colorValue As Long
//! Dim component As Integer
//! colorValue = &HFF8040
//! component = (colorValue And &HFF)
//! Debug.Print Oct$(component)  ' Shows octal representation
//! ```
//!
//! ### 6. Data Structure Field Values
//!
//! ```vb6
//! Type SystemFlags
//!     ReadWrite As Integer
//!     Execute As Integer
//! End Type
//!
//! Dim sysFlags As SystemFlags
//! sysFlags.ReadWrite = &O644  ' Octal literal
//! Debug.Print "RW: " & Oct$(sysFlags.ReadWrite)
//! ```
//!
//! ### 7. Debugging Bit Patterns
//!
//! ```vb6
//! Sub ShowBitPattern(value As Integer)
//!     Debug.Print "Decimal: " & value
//!     Debug.Print "Octal: " & Oct$(value)
//!     Debug.Print "Hex: " & Hex$(value)
//! End Sub
//! ```
//!
//! ### 8. Network Protocol Values
//!
//! ```vb6
//! Dim socketMode As Integer
//! socketMode = &O666  ' Read/write for all
//! Debug.Print "Mode: " & Oct$(socketMode)
//! ```
//!
//! ### 9. Conversion Table Generation
//!
//! ```vb6
//! Sub GenerateOctalTable()
//!     Dim i As Integer
//!     For i = 0 To 64
//!         Debug.Print i & " = " & Oct$(i)
//!     Next i
//! End Sub
//! ```
//!
//! ### 10. Configuration Value Formatting
//!
//! ```vb6
//! Function SaveConfigValue(value As Integer) As String
//!     ' Store configuration as octal string
//!     SaveConfigValue = "CONFIG=" & Oct$(value)
//! End Function
//! ```
//!
//! ## Related Functions
//!
//! - `Hex$()` - Converts a number to hexadecimal (base-16) string representation
//! - `Str$()` - Converts a number to decimal string representation
//! - `Val()` - Converts a string to a numeric value (doesn't parse octal)
//! - `CLng()` - Converts an expression to a `Long` integer
//! - `CInt()` - Converts an expression to an `Integer`
//! - `Format$()` - Provides custom number formatting options
//!
//! ## Best Practices
//!
//! ### When to Use `Oct$`
//!
//! 1. **Unix-style Permissions**: Representing file or directory permissions (e.g., 755, 644)
//! 2. **Bit Pattern Analysis**: When examining data in groups of 3 bits
//! 3. **Legacy System Integration**: Working with systems that use octal notation
//! 4. **Debugging**: Displaying bit patterns in a more compact form than binary
//! 5. **Configuration Files**: Storing numeric values in octal format
//!
//! ### Formatting Output
//!
//! ```vb6
//! ' Add prefix for clarity
//! Debug.Print "Octal: &O" & Oct$(value)
//!
//! ' Pad with leading zeros
//! Debug.Print Right$("000" & Oct$(value), 3)
//! ```
//!
//! ### Type Safety
//!
//! ```vb6
//! ' Explicitly convert to ensure correct range
//! Dim longValue As Long
//! longValue = 1000000
//! Debug.Print Oct$(longValue)  ' Uses Long range
//! ```
//!
//! ## Performance Considerations
//!
//! - `Oct$` is a lightweight function with minimal overhead
//! - String concatenation in loops should use a `String` buffer or array for better performance
//! - For frequent conversions, consider caching results if the same values are converted repeatedly
//!
//! ## Octal Literals in VB6
//!
//! VB6 supports octal literals using the `&O` prefix:
//!
//! ```vb6
//! Dim octValue As Integer
//! octValue = &O777  ' Octal literal (equals 511 decimal)
//! Debug.Print Oct$(octValue)  ' Returns "777"
//! ```
//!
//! ## Common Pitfalls
//!
//! ### 1. No Direct Reverse Function
//!
//! VB6's `Val()` function does not parse octal strings. You need a custom function:
//!
//! ```vb6
//! Function OctVal(octStr As String) As Long
//!     Dim i As Integer
//!     Dim result As Long
//!     For i = 1 To Len(octStr)
//!         result = result * 8 + Val(Mid$(octStr, i, 1))
//!     Next i
//!     OctVal = result
//! End Function
//! ```
//!
//! ### 2. Two's Complement Representation
//!
//! Negative numbers produce two's complement octal strings:
//!
//! ```vb6
//! Debug.Print Oct$(-1)   ' "177777" (for Integer)
//! Debug.Print Oct$(-100) ' Not intuitive without understanding two's complement
//! ```
//!
//! ### 3. Floating-Point Rounding
//!
//! ```vb6
//! Debug.Print Oct$(8.5)  ' "10" (rounds to 8)
//! Debug.Print Oct$(8.6)  ' "11" (rounds to 9)
//! ```
//!
//! ### 4. Leading Zeros Not Included
//!
//! ```vb6
//! Debug.Print Oct$(8)  ' "10", not "010"
//! ' Pad manually if needed
//! Debug.Print Right$("000" & Oct$(8), 3)  ' "010"
//! ```
//!
//! ### 5. No Prefix in Output
//!
//! Unlike some languages, VB6's `Oct$` doesn't include the `&O` prefix:
//!
//! ```vb6
//! Debug.Print Oct$(64)  ' "100", not "&O100"
//! ```
//!
//! ## Limitations
//!
//! - No built-in function to convert octal strings back to numbers (must implement manually)
//! - Cannot specify minimum width or padding (must format manually)
//! - Limited usefulness in modern applications (hexadecimal is more common)
//! - No validation that a string contains valid octal digits
//! - Returns unsigned representation for negative numbers (two's complement)

use crate::error::{err_number, VBError, VBResult};
use crate::value::{VBString, VBVariant};

/// Implementation of the `Oct$` function.
///
/// Converts a number to its octal (base-8) string representation. The result
/// contains only digits 0-7 and does not include any prefix ("&O" or "0o").
///
/// VB6 behavior:
/// - Fractional values are rounded to the nearest integer before conversion
/// - Negative numbers use two's complement representation
/// - `Integer` values produce up to 6 octal digits
/// - `Long` values produce up to 11 octal digits
/// - Raises error 94 if `number` is `Null`
/// - Raises error 6 (overflow) if the value cannot fit in a `Long`
pub fn oct_dollar(number: &VBVariant) -> VBResult<VBString> {
    if number.is_null() {
        return Err(VBError::with_description(
            err_number::INVALID_USE_OF_NULL,
            "Invalid use of Null",
        ));
    }

    // Determine the bit width based on the variant type
    let value = match number {
        VBVariant::Integer(v) => {
            // Integer is 16-bit; mask to u16 to get correct octal representation
            format!("{:o}", *v as u16)
        }
        VBVariant::Long(v) => {
            format!("{:o}", *v as u32)
        }
        VBVariant::Byte(v) => {
            format!("{:o}", *v as u32)
        }
        _ => {
            // For other types (Double, Single, Currency, String, etc.),
            // convert to Long first via as_i64 which handles rounding
            let v = number.as_i64()?;
            if let Ok(long_val) = i32::try_from(v) {
                format!("{:o}", long_val as u32)
            } else {
                return Err(VBError::overflow());
            }
        }
    };

    Ok(VBString::from(value))
}

#[cfg(test)]
mod tests {
    use super::oct_dollar;
    use crate::error::err_number;
    use crate::value::VBVariant;

    #[test]
    fn oct_dollar_zero() {
        let result = oct_dollar(&VBVariant::from_long(0)).unwrap();
        assert_eq!(result.as_str(), "0");
    }

    #[test]
    fn oct_dollar_positive_long() {
        let result = oct_dollar(&VBVariant::from_long(8)).unwrap();
        assert_eq!(result.as_str(), "10");

        let result = oct_dollar(&VBVariant::from_long(64)).unwrap();
        assert_eq!(result.as_str(), "100");

        let result = oct_dollar(&VBVariant::from_long(511)).unwrap();
        assert_eq!(result.as_str(), "777");
    }

    #[test]
    fn oct_dollar_positive_integer() {
        let result = oct_dollar(&VBVariant::from_integer(8)).unwrap();
        assert_eq!(result.as_str(), "10");

        let result = oct_dollar(&VBVariant::from_integer(-1)).unwrap();
        assert_eq!(result.as_str(), "177777");
    }

    #[test]
    fn oct_dollar_negative_long() {
        let result = oct_dollar(&VBVariant::from_long(-1)).unwrap();
        assert_eq!(result.as_str(), "37777777777");

        let result = oct_dollar(&VBVariant::from_long(-256)).unwrap();
        assert_eq!(result.as_str(), "37777777400");
    }

    #[test]
    fn oct_dollar_byte() {
        let result = oct_dollar(&VBVariant::from_byte(8)).unwrap();
        assert_eq!(result.as_str(), "10");

        let result = oct_dollar(&VBVariant::from_byte(64)).unwrap();
        assert_eq!(result.as_str(), "100");
    }

    #[test]
    fn oct_dollar_double_rounds() {
        // 8.5 rounds to 8 → "10"
        let result = oct_dollar(&VBVariant::Double(8.5)).unwrap();
        assert_eq!(result.as_str(), "10");

        // 8.6 rounds to 9 → "11"
        let result = oct_dollar(&VBVariant::Double(8.6)).unwrap();
        assert_eq!(result.as_str(), "11");
    }

    #[test]
    fn oct_dollar_from_string() {
        let result = oct_dollar(&VBVariant::from_string("255")).unwrap();
        assert_eq!(result.as_str(), "377");
    }

    #[test]
    fn oct_dollar_null_is_error_94() {
        let err = oct_dollar(&VBVariant::Null).unwrap_err();
        assert_eq!(err.number, err_number::INVALID_USE_OF_NULL);
    }

    #[test]
    fn oct_dollar_max_long() {
        let result = oct_dollar(&VBVariant::from_long(i32::MAX)).unwrap();
        assert_eq!(result.as_str(), "17777777777");
    }

    #[test]
    fn oct_dollar_min_long() {
        let result = oct_dollar(&VBVariant::from_long(i32::MIN)).unwrap();
        assert_eq!(result.as_str(), "20000000000");
    }

    #[test]
    fn oct_dollar_file_permissions() {
        // 493 decimal = 755 octal (rwxr-xr-x)
        let result = oct_dollar(&VBVariant::from_long(493)).unwrap();
        assert_eq!(result.as_str(), "755");

        // 420 decimal = 644 octal (rw-r--r--)
        let result = oct_dollar(&VBVariant::from_long(420)).unwrap();
        assert_eq!(result.as_str(), "644");
    }

    #[test]
    fn oct_dollar_powers_of_two() {
        let result = oct_dollar(&VBVariant::from_long(1)).unwrap();
        assert_eq!(result.as_str(), "1");

        let result = oct_dollar(&VBVariant::from_long(2)).unwrap();
        assert_eq!(result.as_str(), "2");

        let result = oct_dollar(&VBVariant::from_long(4)).unwrap();
        assert_eq!(result.as_str(), "4");

        let result = oct_dollar(&VBVariant::from_long(8)).unwrap();
        assert_eq!(result.as_str(), "10");

        let result = oct_dollar(&VBVariant::from_long(64)).unwrap();
        assert_eq!(result.as_str(), "100");
    }

    #[test]
    fn oct_dollar_empty_returns_zero() {
        let result = oct_dollar(&VBVariant::Empty).unwrap();
        assert_eq!(result.as_str(), "0");
    }

    #[test]
    fn oct_dollar_small_values() {
        for i in 0..8 {
            let result = oct_dollar(&VBVariant::from_long(i)).unwrap();
            let expected = format!("{:o}", i);
            assert_eq!(result.as_str(), expected, "Failed for value {}", i);
        }
    }
}
