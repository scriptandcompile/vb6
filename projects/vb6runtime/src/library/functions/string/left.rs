//! # Left Function
//!
//! Returns a String containing a specified number of characters from the left side of a string.
//!
//! ## Syntax
//!
//! ```vb
//! Left(string, length)
//! ```
//!
//! ## Parameters
//!
//! - `string` (Required): String expression from which leftmost characters are returned
//!   - If string contains Null, Null is returned
//! - `length` (Required): Long indicating how many characters to return
//!   - If 0, empty string ("") is returned
//!   - If greater than or equal to number of characters in string, entire string is returned
//!   - Must be non-negative (negative values cause error)
//!
//! ## Return Value
//!
//! Returns a String (or Variant containing String):
//! - Contains the specified number of characters from the left side of the string
//! - Returns empty string if length is 0
//! - Returns entire string if length >= Len(string)
//! - Returns Null if string argument is Null
//! - Always returns String type (Left$ variant returns String, not Variant)
//!
//! ## Remarks
//!
//! The Left function extracts characters from the beginning of a string:
//!
//! - Returns leftmost characters up to specified length
//! - Complements Right function (which returns rightmost characters)
//! - Works with Mid function for complete substring extraction
//! - Zero-based extraction: Left("ABC", 2) returns "AB"
//! - Safe with lengths exceeding string length (returns full string)
//! - Null propagates through the function
//! - Negative length raises Error 5 (Invalid procedure call or argument)
//! - Common for extracting prefixes, file names, codes, etc.
//! - More efficient than Mid(string, 1, length) for left extraction
//! - Left$ variant returns String type (not Variant) for slight performance gain
//! - Cannot extract from right side (use Right for that)
//! - Cannot skip characters (use Mid for that)
//! - Does not modify original string (strings are immutable)
//!
//! ## Typical Uses
//!
//! 1. **Extract Prefix**: Get first N characters of string
//! 2. **Parse Codes**: Extract code prefixes from identifiers
//! 3. **Truncate Text**: Limit string length for display
//! 4. **File Extensions**: Extract drive letter or path prefix
//! 5. **Validation**: Check string starts with specific pattern
//! 6. **Data Parsing**: Extract fixed-width field data
//! 7. **Formatting**: Create abbreviations or short forms
//! 8. **Pattern Matching**: Compare string prefixes
//!
//! ## Basic Usage Examples
//!
//! ```vb
//! ' Example 1: Basic left extraction
//! Dim text As String
//! text = "Hello World"
//!
//! Debug.Print Left(text, 5)            ' "Hello"
//! Debug.Print Left(text, 3)            ' "Hel"
//! Debug.Print Left(text, 1)            ' "H"
//!
//! ' Example 2: Length exceeds string length
//! Debug.Print Left("ABC", 10)          ' "ABC" - entire string
//! Debug.Print Left("Test", 4)          ' "Test" - exact length
//!
//! ' Example 3: Zero length
//! Debug.Print Left("Hello", 0)         ' "" - empty string
//!
//! ' Example 4: Extract file extension check
//! Dim fileName As String
//! fileName = "C:\Data\file.txt"
//!
//! If Left(fileName, 3) = "C:\" Then
//!     Debug.Print "File on C: drive"
//! End If
//! ```
//!
//! ## Common Patterns
//!
//! ```vb
//! ' Pattern 1: Extract first N characters
//! Function GetPrefix(text As String, length As Long) As String
//!     If length <= 0 Then
//!         GetPrefix = ""
//!     Else
//!         GetPrefix = Left(text, length)
//!     End If
//! End Function
//!
//! ' Pattern 2: Truncate with ellipsis
//! Function Truncate(text As String, maxLength As Long) As String
//!     If Len(text) <= maxLength Then
//!         Truncate = text
//!     Else
//!         Truncate = Left(text, maxLength - 3) & "..."
//!     End If
//! End Function
//!
//! ' Pattern 3: Check if string starts with prefix
//! Function StartsWith(text As String, prefix As String) As Boolean
//!     StartsWith = (Left(text, Len(prefix)) = prefix)
//! End Function
//!
//! ' Pattern 4: Extract first word
//! Function GetFirstWord(text As String) As String
//!     Dim spacePos As Long
//!     
//!     spacePos = InStr(text, " ")
//!     If spacePos > 0 Then
//!         GetFirstWord = Left(text, spacePos - 1)
//!     Else
//!         GetFirstWord = text
//!     End If
//! End Function
//!
//! ' Pattern 5: Extract initials
//! Function GetInitials(fullName As String) As String
//!     Dim parts() As String
//!     Dim i As Long
//!     Dim initials As String
//!     
//!     parts = Split(Trim(fullName), " ")
//!     initials = ""
//!     
//!     For i = LBound(parts) To UBound(parts)
//!         If Len(parts(i)) > 0 Then
//!             initials = initials & UCase(Left(parts(i), 1))
//!         End If
//!     Next i
//!     
//!     GetInitials = initials
//! End Function
//!
//! ' Pattern 6: Safe Left with Null check
//! Function SafeLeft(value As Variant, length As Long) As String
//!     If IsNull(value) Then
//!         SafeLeft = ""
//!     Else
//!         SafeLeft = Left(value, length)
//!     End If
//! End Function
//!
//! ' Pattern 7: Extract drive letter
//! Function GetDriveLetter(path As String) As String
//!     If Len(path) >= 2 And Mid(path, 2, 1) = ":" Then
//!         GetDriveLetter = Left(path, 2)
//!     Else
//!         GetDriveLetter = ""
//!     End If
//! End Function
//!
//! ' Pattern 8: Pad left to fixed width
//! Function PadLeft(text As String, width As Long, Optional padChar As String = " ") As String
//!     If Len(text) >= width Then
//!         PadLeft = Left(text, width)
//!     Else
//!         PadLeft = String(width - Len(text), padChar) & text
//!     End If
//! End Function
//!
//! ' Pattern 9: Extract area code from phone
//! Function GetAreaCode(phone As String) As String
//!     Dim digitsOnly As String
//!     Dim i As Long
//!     Dim char As String
//!     
//!     ' Extract digits only
//!     digitsOnly = ""
//!     For i = 1 To Len(phone)
//!         char = Mid(phone, i, 1)
//!         If char >= "0" And char <= "9" Then
//!             digitsOnly = digitsOnly & char
//!         End If
//!     Next i
//!     
//!     ' Get first 3 digits as area code
//!     If Len(digitsOnly) >= 3 Then
//!         GetAreaCode = Left(digitsOnly, 3)
//!     Else
//!         GetAreaCode = ""
//!     End If
//! End Function
//!
//! ' Pattern 10: Create abbreviation
//! Function Abbreviate(text As String, maxLength As Long) As String
//!     If Len(text) <= maxLength Then
//!         Abbreviate = text
//!     Else
//!         ' Take first letter of each word
//!         Dim words() As String
//!         Dim i As Long
//!         Dim abbr As String
//!         
//!         words = Split(text, " ")
//!         abbr = ""
//!         
//!         For i = LBound(words) To UBound(words)
//!             If Len(words(i)) > 0 Then
//!                 abbr = abbr & UCase(Left(words(i), 1))
//!                 If Len(abbr) >= maxLength Then Exit For
//!             End If
//!         Next i
//!         
//!         Abbreviate = abbr
//!     End If
//! End Function
//! ```
//!
//! ## Advanced Usage Examples
//!
//! ```vb
//! ' Example 1: Fixed-width file parser
//! Public Class FixedWidthParser
//!     Private m_fieldLengths() As Long
//!     
//!     Public Sub SetFieldLengths(ParamArray lengths() As Variant)
//!         Dim i As Long
//!         ReDim m_fieldLengths(LBound(lengths) To UBound(lengths))
//!         For i = LBound(lengths) To UBound(lengths)
//!             m_fieldLengths(i) = CLng(lengths(i))
//!         Next i
//!     End Sub
//!     
//!     Public Function ParseLine(line As String) As Variant
//!         Dim fields() As String
//!         Dim i As Long
//!         Dim pos As Long
//!         
//!         ReDim fields(LBound(m_fieldLengths) To UBound(m_fieldLengths))
//!         pos = 1
//!         
//!         For i = LBound(m_fieldLengths) To UBound(m_fieldLengths)
//!             If pos <= Len(line) Then
//!                 fields(i) = Trim(Mid(line, pos, m_fieldLengths(i)))
//!             Else
//!                 fields(i) = ""
//!             End If
//!             pos = pos + m_fieldLengths(i)
//!         Next i
//!         
//!         ParseLine = fields
//!     End Function
//!     
//!     Public Function GetField(line As String, fieldIndex As Long) As String
//!         Dim pos As Long
//!         Dim i As Long
//!         
//!         pos = 1
//!         For i = LBound(m_fieldLengths) To fieldIndex - 1
//!             pos = pos + m_fieldLengths(i)
//!         Next i
//!         
//!         If pos <= Len(line) Then
//!             GetField = Trim(Mid(line, pos, m_fieldLengths(fieldIndex)))
//!         Else
//!             GetField = ""
//!         End If
//!     End Function
//! End Class
//!
//! ' Example 2: Text preview/truncation utility
//! Public Class TextPreview
//!     Public Function CreatePreview(text As String, maxLength As Long, _
//!                                   Optional ellipsis As String = "...") As String
//!         If Len(text) <= maxLength Then
//!             CreatePreview = text
//!             Exit Function
//!         End If
//!         
//!         ' Try to break at word boundary
//!         Dim truncated As String
//!         Dim lastSpace As Long
//!         
//!         truncated = Left(text, maxLength - Len(ellipsis))
//!         lastSpace = InStrRev(truncated, " ")
//!         
//!         If lastSpace > maxLength \ 2 Then
//!             ' Break at word if space found in second half
//!             CreatePreview = Left(truncated, lastSpace - 1) & ellipsis
//!         Else
//!             ' Break at character
//!             CreatePreview = truncated & ellipsis
//!         End If
//!     End Function
//!     
//!     Public Function WordWrap(text As String, lineWidth As Long) As String
//!         Dim result As String
//!         Dim remaining As String
//!         Dim line As String
//!         Dim spacePos As Long
//!         
//!         result = ""
//!         remaining = text
//!         
//!         Do While Len(remaining) > lineWidth
//!             ' Try to break at word
//!             line = Left(remaining, lineWidth)
//!             spacePos = InStrRev(line, " ")
//!             
//!             If spacePos > 0 Then
//!                 result = result & Left(line, spacePos - 1) & vbCrLf
//!                 remaining = Mid(remaining, spacePos + 1)
//!             Else
//!                 result = result & line & vbCrLf
//!                 remaining = Mid(remaining, lineWidth + 1)
//!             End If
//!         Loop
//!         
//!         result = result & remaining
//!         WordWrap = result
//!     End Function
//! End Class
//!
//! ' Example 3: Code prefix analyzer
//! Public Class CodeAnalyzer
//!     Private m_prefixes As Collection
//!     
//!     Private Sub Class_Initialize()
//!         Set m_prefixes = New Collection
//!     End Sub
//!     
//!     Public Sub RegisterPrefix(prefix As String, description As String)
//!         m_prefixes.Add Array(prefix, description), prefix
//!     End Sub
//!     
//!     Public Function GetCodeType(code As String) As String
//!         Dim i As Long
//!         Dim prefix As Variant
//!         Dim info As Variant
//!         
//!         For i = 1 To m_prefixes.Count
//!             info = m_prefixes(i)
//!             prefix = info(0)
//!             
//!             If Left(code, Len(prefix)) = prefix Then
//!                 GetCodeType = info(1)
//!                 Exit Function
//!             End If
//!         Next i
//!         
//!         GetCodeType = "Unknown"
//!     End Function
//!     
//!     Public Function ExtractPrefix(code As String, prefixLength As Long) As String
//!         ExtractPrefix = Left(code, prefixLength)
//!     End Function
//!     
//!     Public Function StripPrefix(code As String, prefixLength As Long) As String
//!         If Len(code) > prefixLength Then
//!             StripPrefix = Mid(code, prefixLength + 1)
//!         Else
//!             StripPrefix = ""
//!         End If
//!     End Function
//! End Class
//!
//! ' Example 4: String comparison helper
//! Public Class StringComparer
//!     Public Function StartsWith(text As String, prefix As String, _
//!                                Optional ignoreCase As Boolean = False) As Boolean
//!         Dim textPrefix As String
//!         Dim comparePrefix As String
//!         
//!         If Len(prefix) = 0 Then
//!             StartsWith = True
//!             Exit Function
//!         End If
//!         
//!         If Len(text) < Len(prefix) Then
//!             StartsWith = False
//!             Exit Function
//!         End If
//!         
//!         textPrefix = Left(text, Len(prefix))
//!         
//!         If ignoreCase Then
//!             StartsWith = (LCase(textPrefix) = LCase(prefix))
//!         Else
//!             StartsWith = (textPrefix = prefix)
//!         End If
//!     End Function
//!     
//!     Public Function GetCommonPrefix(str1 As String, str2 As String) As String
//!         Dim i As Long
//!         Dim minLen As Long
//!         
//!         minLen = IIf(Len(str1) < Len(str2), Len(str1), Len(str2))
//!         
//!         For i = 1 To minLen
//!             If Left(str1, i) <> Left(str2, i) Then
//!                 If i = 1 Then
//!                     GetCommonPrefix = ""
//!                 Else
//!                     GetCommonPrefix = Left(str1, i - 1)
//!                 End If
//!                 Exit Function
//!             End If
//!         Next i
//!         
//!         GetCommonPrefix = Left(str1, minLen)
//!     End Function
//!     
//!     Public Function RemovePrefix(text As String, prefix As String, _
//!                                  Optional ignoreCase As Boolean = False) As String
//!         If StartsWith(text, prefix, ignoreCase) Then
//!             RemovePrefix = Mid(text, Len(prefix) + 1)
//!         Else
//!             RemovePrefix = text
//!         End If
//!     End Function
//! End Class
//! ```
//!
//! ## Error Handling
//!
//! Left handles several special cases:
//!
//! ```vb
//! ' Empty string
//! Debug.Print Left("", 5)              ' "" - empty string
//!
//! ' Zero length
//! Debug.Print Left("Hello", 0)         ' "" - empty string
//!
//! ' Length exceeds string
//! Debug.Print Left("Hi", 10)           ' "Hi" - entire string
//!
//! ' Null propagates
//! Dim value As Variant
//! value = Null
//! Debug.Print IsNull(Left(value, 3))   ' True
//!
//! ' Negative length causes error
//! ' Debug.Print Left("Test", -1)       ' Error 5: Invalid procedure call
//!
//! ' Safe pattern with error handling
//! Function SafeLeft(text As Variant, length As Long) As String
//!     On Error Resume Next
//!     If IsNull(text) Then
//!         SafeLeft = ""
//!     ElseIf length < 0 Then
//!         SafeLeft = ""
//!     Else
//!         SafeLeft = Left(text, length)
//!     End If
//!     On Error GoTo 0
//! End Function
//! ```
//!
//! ## Performance Considerations
//!
//! - **Fast Operation**: Left is a very fast intrinsic function
//! - **String Creation**: Creates new string (strings are immutable)
//! - **Left$ Variant**: Use Left$ for String return type (slightly faster)
//! - **Repeated Calls**: Cache result if using same substring multiple times
//!
//! Performance tips:
//! ```vb
//! ' Efficient for single use
//! If Left(fileName, 2) = "C:" Then
//!
//! ' Cache if used multiple times
//! Dim prefix As String
//! prefix = Left(code, 3)
//! If prefix = "ABC" Or prefix = "DEF" Then
//! ```
//!
//! ## Best Practices
//!
//! 1. **Validate Length**: Ensure length is non-negative
//! 2. **`Null` Safety**: Check for `Null` before calling `Left` if needed
//! 3. **`StartsWith` Pattern**: Use `Left` for prefix checking
//! 4. **Truncation**: Consider word boundaries when truncating display text
//! 5. **Use `Left$`**: For `String` variables, use `Left$` for type safety
//! 6. **Combine with `Len`**: Check string length before extracting
//! 7. **Fixed-Width Data**: Use `Left` for fixed-width field extraction
//! 8. **Path Manipulation**: Use `Left` for drive/path prefix extraction
//!
//! ## Comparison with Related Functions
//!
//! | Function | Purpose | Parameters | Use Case |
//! |----------|---------|------------|----------|
//! | `Left` | Extract from left | ```(string, length)``` | Get prefix/first N chars |
//! | `Right` | Extract from right | ```(string, length)``` | Get suffix/last N chars |
//! | `Mid` | Extract from middle | ```(string, start, [length])``` | Get substring from any position |
//! | `InStr` | Find substring | ```([start,] string1, string2)``` | Locate substring position |
//! | `Len` | Get string length | ```(string)``` | Measure string |
//! ## Left vs Mid
//!
//! ```vb
//! Dim text As String
//! text = "Hello World"
//!
//! ' Left - simpler for leftmost characters
//! Debug.Print Left(text, 5)            ' "Hello"
//!
//! ' Mid - equivalent but more verbose
//! Debug.Print Mid(text, 1, 5)          ' "Hello"
//!
//! ' Use Left for clarity when extracting from start
//! ' Use Mid when start position is not 1
//! ```
//!
//! ## Left, Right, and Mid Together
//!
//! ```vb
//! Dim text As String
//! text = "ABCDEFGH"
//!
//! ' Left - first 3 characters
//! Debug.Print Left(text, 3)            ' "ABC"
//!
//! ' Right - last 3 characters  
//! Debug.Print Right(text, 3)           ' "FGH"
//!
//! ' Mid - middle characters
//! Debug.Print Mid(text, 3, 4)          ' "CDEF"
//!
//! ' Combine for complex extraction
//! Dim part As String
//! part = Left(Right(text, 5), 2)       ' "DE"
//! ```
//!
//! ## Platform and Version Notes
//!
//! - Available in all VB6 versions
//! - Part of VBA core functions
//! - Returns Variant containing String (Left$ returns String type)
//! - Maximum string length is approximately 2GB (theoretical)
//! - Practical limit is much lower due to memory constraints
//!
//! ## Limitations
//!
//! - Cannot extract from right side (use Right function)
//! - Cannot extract from middle with offset (use Mid function)
//! - Negative length raises error (not treated as 0)
//! - Creates new string (cannot modify in place)
//! - No option for character vs byte extraction
//! - No built-in word boundary awareness
//!
//! ## Related Functions
//!
//! - `Right`: Extract characters from right side of string
//! - `Mid`: Extract substring from any position
//! - `Len`: Get length of string
//! - `InStr`: Find position of substring
//! - `Trim`/`LTrim`/`RTrim`: Remove whitespace

use crate::error::{err_number, VBError, VBResult};

/// Returns the specified number of characters from the left side of the string.
///
/// A `length` of 0 returns an empty string; a `length` greater than the number
/// of characters returns the entire string. Characters are counted as Unicode
/// scalar values.
///
/// # Errors
///
/// Returns error 5 (`Invalid procedure call or argument`) when `length` is negative.
pub fn left(input: &str, length: i32) -> VBResult<String> {
    if length < 0 {
        return Err(VBError::with_description(
            err_number::INVALID_PROCEDURE_CALL,
            "Invalid length",
        ));
    }
    Ok(input.chars().take(length as usize).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::err_number;

    #[test]
    fn returns_leftmost_characters() {
        assert_eq!(left("Hello", 3).unwrap(), "Hel");
        assert_eq!(left("Hello", 0).unwrap(), "");
    }

    #[test]
    fn length_greater_than_string_returns_all() {
        assert_eq!(left("Hello", 10).unwrap(), "Hello");
        assert_eq!(left("Hello", 5).unwrap(), "Hello");
    }

    #[test]
    fn handles_unicode() {
        assert_eq!(left("héllo", 2).unwrap(), "hé");
    }

    #[test]
    fn rejects_negative_length() {
        assert_eq!(
            left("Hello", -1).unwrap_err().number,
            err_number::INVALID_PROCEDURE_CALL
        );
    }
}
