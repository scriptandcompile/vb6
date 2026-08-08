//! # `Str$` Function
//!
//! The `Str$` function in Visual Basic 6 converts a numeric value to a string representation.
//! The dollar sign (`$`) suffix indicates that this function always returns a `String` type,
//! never a `Variant`.
//!
//! ## Syntax
//!
//! ```vb6
//! Str$(number)
//! ```
//!
//! ## Parameters
//!
//! - `number` - Required. Any valid numeric expression. Can be of type `Byte`, `Integer`, `Long`,
//!   `Single`, `Double`, or `Currency`.
//!
//! ## Return Value
//!
//! Returns a `String` representation of the number. Positive numbers include a leading space
//! for the sign position. Negative numbers include a leading minus sign (-).
//!
//! ## Behavior and Characteristics
//!
//! ### Sign Handling
//!
//! - Positive numbers: Include a leading space (e.g., " 123")
//! - Negative numbers: Include a leading minus sign (e.g., "-123")
//! - Zero: Returns " 0" (with leading space)
//! - The leading space reserves position for the sign
//!
//! ### Numeric Formatting
//!
//! - No thousands separators (e.g., "1000" not "1,000")
//! - Scientific notation for very large or very small numbers
//! - Floating-point numbers may show precision artifacts
//! - No control over decimal places
//!
//! ### Type Differences: `Str$` vs `Str`
//!
//! - `Str$`: Always returns `String` type (never `Variant`)
//! - `Str`: Returns `Variant` containing a string
//! - Use `Str$` when you need guaranteed `String` return type
//! - Use `Str` when working with `Variant` variables
//!
//! ## Common Usage Patterns
//!
//! ### 1. Basic Number to String Conversion
//!
//! ```vb6
//! Dim numStr As String
//! numStr = Str$(123)  ' Returns " 123" (note leading space)
//! numStr = Str$(-45)  ' Returns "-45"
//! ```
//!
//! ### 2. Concatenating Numbers with Text
//!
//! ```vb6
//! Function FormatMessage(count As Integer) As String
//!     FormatMessage = "Found" & Str$(count) & " items"
//! End Function
//!
//! Debug.Print FormatMessage(5)  ' "Found 5 items"
//! ```
//!
//! ### 3. Trimming the Leading Space
//!
//! ```vb6
//! Function NumberToString(value As Long) As String
//!     NumberToString = LTrim$(Str$(value))
//! End Function
//!
//! Dim result As String
//! result = NumberToString(100)  ' Returns "100" (no leading space)
//! ```
//!
//! ### 4. Building Comma-Separated Values
//!
//! ```vb6
//! Function BuildCSV(values() As Integer) As String
//!     Dim i As Integer
//!     Dim result As String
//!     For i = LBound(values) To UBound(values)
//!         If i > LBound(values) Then result = result & ","
//!         result = result & LTrim$(Str$(values(i)))
//!     Next i
//!     BuildCSV = result
//! End Function
//! ```
//!
//! ### 5. Logging and Debug Output
//!
//! ```vb6
//! Sub LogValue(name As String, value As Double)
//!     Debug.Print name & " =" & Str$(value)
//! End Sub
//! ```
//!
//! ### 6. Creating Numeric Labels
//!
//! ```vb6
//! Function CreateLabel(index As Integer) As String
//!     CreateLabel = "Item" & LTrim$(Str$(index))
//! End Function
//!
//! Dim label As String
//! label = CreateLabel(42)  ' Returns "Item42"
//! ```
//!
//! ### 7. File Output Formatting
//!
//! ```vb6
//! Sub WriteDataLine(fileNum As Integer, id As Long, amount As Currency)
//!     Print #fileNum, LTrim$(Str$(id)) & "," & LTrim$(Str$(amount))
//! End Sub
//! ```
//!
//! ### 8. Array Index Display
//!
//! ```vb6
//! Sub ShowArrayContents(arr() As Integer)
//!     Dim i As Integer
//!     For i = LBound(arr) To UBound(arr)
//!         Debug.Print "[" & LTrim$(Str$(i)) & "] = " & LTrim$(Str$(arr(i)))
//!     Next i
//! End Sub
//! ```
//!
//! ### 9. Simple Calculator Display
//!
//! ```vb6
//! Function UpdateDisplay(value As Double) As String
//!     UpdateDisplay = LTrim$(Str$(value))
//! End Function
//! ```
//!
//! ### 10. Building SQL Statements
//!
//! ```vb6
//! Function BuildQuery(userId As Long) As String
//!     BuildQuery = "SELECT * FROM Users WHERE ID = " & LTrim$(Str$(userId))
//! End Function
//! ```
//!
//! ## Related Functions
//!
//! - `Str()` - Returns a `Variant` containing the string representation of a number
//! - `CStr()` - Converts an expression to a `String` (no leading space for positive numbers)
//! - `Format$()` - Provides extensive formatting control for numeric values
//! - `Val()` - Converts a string to a numeric value (inverse operation)
//! - `LTrim$()` - Removes leading spaces (often used with `Str$`)
//! - `Hex$()` - Converts a number to hexadecimal string
//! - `Oct$()` - Converts a number to octal string
//!
//! ## Best Practices
//!
//! ### When to Use `Str$` vs `CStr` vs `Format$`
//!
//! ```vb6
//! Dim value As Integer
//! value = 42
//!
//! ' Str$ includes leading space for positive numbers
//! Debug.Print Str$(value)  ' " 42"
//!
//! ' CStr has no leading space
//! Debug.Print CStr(value)  ' "42"
//!
//! ' Format$ provides control over formatting
//! Debug.Print Format$(value, "000")  ' "042"
//! ```
//!
//! ### Always Trim for Display
//!
//! ```vb6
//! ' Without trim (has leading space for positive numbers)
//! Label1.Caption = Str$(count)  ' " 5"
//!
//! ' With trim (clean output)
//! Label1.Caption = LTrim$(Str$(count))  ' "5"
//!
//! ' Or use CStr instead
//! Label1.Caption = CStr(count)  ' "5"
//! ```
//!
//! ### Use `Format$` for Formatted Output
//!
//! ```vb6
//! ' Str$ has no formatting control
//! Debug.Print Str$(1234.5678)  ' " 1234.5678"
//!
//! ' Format$ provides control
//! Debug.Print Format$(1234.5678, "#,##0.00")  ' "1,234.57"
//! ```
//!
//! ### Handle Negative Numbers
//!
//! ```vb6
//! Function SafeConvert(value As Long) As String
//!     ' Str$ handles negative numbers correctly
//!     SafeConvert = LTrim$(Str$(value))
//!     ' For negative: "-123", for positive: "123"
//! End Function
//! ```
//!
//! ## Performance Considerations
//!
//! - `Str$` is very fast for simple conversions
//! - Faster than `Format$` when formatting is not needed
//! - Similar performance to `CStr`
//! - No significant overhead for any numeric type
//!
//! ```vb6
//! ' Fast: simple conversion
//! For i = 1 To 10000
//!     text = LTrim$(Str$(i))
//! Next i
//!
//! ' Slower: formatted conversion (but more control)
//! For i = 1 To 10000
//!     text = Format$(i, "0000")
//! Next i
//! ```
//!
//! ## Common Pitfalls
//!
//! ### 1. Leading Space for Positive Numbers
//!
//! ```vb6
//! Dim result As String
//! result = Str$(100)  ' " 100" (note the leading space!)
//!
//! ' This can cause problems in comparisons
//! If Str$(100) = "100" Then  ' FALSE! (" 100" <> "100")
//!     Debug.Print "Match"
//! End If
//!
//! ' Use LTrim$ or CStr instead
//! If LTrim$(Str$(100)) = "100" Then  ' TRUE
//!     Debug.Print "Match"
//! End If
//! ```
//!
//! ### 2. Confusion with `CStr`
//!
//! ```vb6
//! Dim value As Integer
//! value = 42
//!
//! Debug.Print Str$(value)   ' " 42" (with space)
//! Debug.Print CStr(value)   ' "42" (no space)
//!
//! ' Know which one you need
//! ```
//!
//! ### 3. No Formatting Control
//!
//! ```vb6
//! Dim amount As Currency
//! amount = 1234.56
//!
//! ' Str$ gives no control
//! Debug.Print Str$(amount)  ' " 1234.56"
//!
//! ' Use Format$ for currency
//! Debug.Print Format$(amount, "$#,##0.00")  ' "$1,234.56"
//! ```
//!
//! ### 4. Floating-Point Precision Issues
//!
//! ```vb6
//! Dim value As Double
//! value = 0.1 + 0.2
//!
//! Debug.Print Str$(value)  ' May show " 0.30000000000000004"
//!
//! ' Use Format$ to control precision
//! Debug.Print Format$(value, "0.00")  ' "0.30"
//! ```
//!
//! ### 5. Not Handling Very Large or Small Numbers
//!
//! ```vb6
//! Dim bigNum As Double
//! bigNum = 1E+20
//!
//! Debug.Print Str$(bigNum)  ' " 1E+20" (scientific notation)
//!
//! ' Be aware of scientific notation in output
//! ```
//!
//! ### 6. Null Values
//!
//! ```vb6
//! ' Str$ cannot handle Null
//! Dim result As String
//! result = Str$(nullValue)  ' Runtime error if nullValue is Null
//!
//! ' Check first
//! If Not IsNull(value) Then
//!     result = Str$(value)
//! Else
//!     result = ""
//! End If
//! ```
//!
//! ## Practical Examples
//!
//! ### Building a Progress Message
//!
//! ```vb6
//! Function ProgressMessage(current As Long, total As Long) As String
//!     ProgressMessage = "Processing item" & Str$(current) & _
//!                      " of" & Str$(total)
//! End Function
//!
//! Debug.Print ProgressMessage(5, 10)  ' "Processing item 5 of 10"
//! ```
//!
//! ### Creating Sequential Filenames
//!
//! ```vb6
//! Function GenerateFileName(baseNameStr As String, index As Integer) As String
//!     GenerateFileName = baseNameStr & LTrim$(Str$(index)) & ".dat"
//! End Function
//!
//! Dim fileName As String
//! fileName = GenerateFileName("data", 1)  ' "data1.dat"
//! ```
//!
//! ### Simple Data Export
//!
//! ```vb6
//! Sub ExportToCSV(data() As Double, fileName As String)
//!     Dim i As Integer
//!     Dim lineData As String
//!     
//!     Open fileName For Output As #1
//!     For i = LBound(data) To UBound(data)
//!         Print #1, LTrim$(Str$(data(i)))
//!     Next i
//!     Close #1
//! End Sub
//! ```
//!
//! ## Limitations
//!
//! - Always includes leading space for positive numbers (use `LTrim$` or `CStr` to remove)
//! - No formatting control (no thousands separators, decimal places, etc.)
//! - Cannot handle `Null` values (use `CStr` with error handling instead)
//! - May produce scientific notation for very large or small numbers
//! - Floating-point precision artifacts may appear in output
//! - No locale-specific formatting (always uses invariant format)

use crate::{
    error::{VBError, VBResult},
    value::{VBString, VBVariant},
};

/// Converts a number to a string with a leading space for positive values.
/// The `$` suffix indicates this function returns a `String` type (not `Variant`).
///
/// Positive numbers (and zero) are prefixed with a single space for the sign
/// position; negative numbers are prefixed with a minus sign. Booleans convert
/// to `-1`/` 0` (not `True`/`False`). No thousands separators are added and the
/// decimal point is always `.`, regardless of locale.
///
///
/// # Errors
///
/// Returns error 13 (`Type mismatch`) when `number` is a non-numeric string or
/// an object.
///
/// `Str$` raises error 94 when `number` is `Null`; use `Str` for the
/// Null-propagating variant.
pub fn str_dollar(number: &VBVariant) -> VBResult<VBString> {
    let body = match number {
        VBVariant::Null => return Err(VBError::invalid_use_of_null()),
        VBVariant::Empty => "0".to_string(),
        VBVariant::Boolean(b) => {
            return if *b {
                Ok(VBString::from("-1"))
            } else {
                Ok(VBString::from(" 0"))
            };
        }
        VBVariant::String(_) => number.as_f64()?.to_string(),
        VBVariant::Nothing | VBVariant::Object(_) | VBVariant::Array(_) => {
            return Err(VBError::type_mismatch())
        }
        _ => number.as_string()?,
    };

    if body.starts_with('-') {
        Ok(VBString::from(body))
    } else {
        Ok(VBString::from(format!(" {body}")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_numbers_have_leading_space() {
        assert_eq!(
            str_dollar(&VBVariant::Long(123)).unwrap(),
            VBString::from(" 123")
        );
        assert_eq!(
            str_dollar(&VBVariant::Long(0)).unwrap(),
            VBString::from(" 0")
        );
        assert_eq!(
            str_dollar(&VBVariant::Double(3.14159)).unwrap(),
            VBString::from(" 3.14159")
        );
    }

    #[test]
    fn negative_numbers_have_leading_minus() {
        assert_eq!(
            str_dollar(&VBVariant::Long(-456)).unwrap(),
            VBString::from("-456")
        );
    }

    #[test]
    fn booleans_become_numbers() {
        assert_eq!(
            str_dollar(&VBVariant::from_bool(true)).unwrap(),
            VBString::from("-1")
        );
        assert_eq!(
            str_dollar(&VBVariant::from_bool(false)).unwrap(),
            VBString::from(" 0")
        );
    }

    #[test]
    fn empty_coerces_to_zero() {
        assert_eq!(str_dollar(&VBVariant::Empty).unwrap(), VBString::from(" 0"));
    }

    #[test]
    fn numeric_strings_are_normalized() {
        assert_eq!(
            str_dollar(&VBVariant::from_string("42")).unwrap(),
            VBString::from(" 42")
        );
        assert_eq!(
            str_dollar(&VBVariant::from_string("-1.5")).unwrap(),
            VBString::from("-1.5")
        );
    }

    #[test]
    fn non_numeric_string_errors() {
        let err = str_dollar(&VBVariant::from_string("abc")).unwrap_err();
        assert_eq!(err.number, crate::error::err_number::TYPE_MISMATCH);
    }

    #[test]
    fn null_errors() {
        let err = str_dollar(&VBVariant::Null).unwrap_err();
        assert_eq!(err.number, crate::error::err_number::INVALID_USE_OF_NULL);
    }
}
