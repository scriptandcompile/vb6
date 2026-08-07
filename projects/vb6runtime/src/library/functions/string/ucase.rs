//! VB6 `UCase` Function
//!
//! The `UCase` function returns a String that has been converted to uppercase.
//!
//! ## Syntax
//! ```vb6
//! UCase(string)
//! ```
//!
//! ## Parameters
//! - `string`: Required. Any valid string expression. If `string` contains Null, Null is returned.
//!
//! ## Returns
//! Returns a `Variant (String)` containing the specified string converted to uppercase. Only lowercase letters are converted to uppercase; all uppercase letters and non-letter characters remain unchanged.
//!
//! ## Remarks
//! The `UCase` function converts lowercase letters to uppercase:
//!
//! - **Case conversion**: Converts a-z to A-Z
//! - **Non-letters unchanged**: Numbers, punctuation, spaces, and symbols are not affected
//! - **Null handling**: Returns Null if the argument is Null
//! - **Empty string**: Returns empty string if argument is empty
//! - **Locale-aware**: Conversion respects current locale settings for international characters
//! - **Unicode support**: Handles Unicode characters according to locale
//! - **Already uppercase**: Characters already uppercase are unchanged
//! - **String variant**: `UCase`$ variant returns String instead of Variant
//!
//! ### `UCase` vs `UCase`$
//! - `UCase`: Returns Variant (String) - can handle and return Null
//! - `UCase$`: Returns String - generates error if argument is Null
//! - Best practice: Use `UCase$` when you know the string is not Null for slightly better performance
//!
//! ### Locale Considerations
//! - Conversion is locale-aware for international characters
//! - Turkish İ (dotted I) and ı (dotless i) handled per locale
//! - German ß (eszett) may convert to SS in some contexts
//! - Accented characters (é, ñ, ü, etc.) convert to uppercase equivalents
//!
//! ## Typical Uses
//! 1. **Case-Insensitive Comparison**: Normalize strings before comparison
//! 2. **User Input Normalization**: Convert user input to consistent case
//! 3. **Data Validation**: Standardize data before validation or storage
//! 4. **Display Formatting**: Format text for display purposes (headings, labels)
//! 5. **String Matching**: Prepare strings for case-insensitive searches
//! 6. **Database Queries**: Normalize search terms
//! 7. **File Processing**: Standardize file names or extensions
//! 8. **Code Generation**: Create uppercase identifiers or constants
//!
//! ## Basic Examples
//!
//! ### Example 1: Simple Conversion
//! ```vb6
//! Dim result As String
//!
//! result = UCase("Hello World")
//! ' result = "HELLO WORLD"
//!
//! result = UCase("abc123xyz")
//! ' result = "ABC123XYZ"
//!
//! result = UCase("ALREADY UPPERCASE")
//! ' result = "ALREADY UPPERCASE"
//! ```
//!
//! ### Example 2: Case-Insensitive Comparison
//! ```vb6
//! Function EqualsIgnoreCase(str1 As String, str2 As String) As Boolean
//!     EqualsIgnoreCase = (UCase$(str1) = UCase$(str2))
//! End Function
//!
//! ' Usage:
//! If EqualsIgnoreCase(userInput, "YES") Then
//!     ProcessConfirmation
//! End If
//! ```
//!
//! ### Example 3: Normalize User Input
//! ```vb6
//! Sub ProcessCommand(command As String)
//!     Select Case UCase$(Trim$(command))
//!         Case "SAVE"
//!             SaveData
//!         Case "LOAD"
//!             LoadData
//!         Case "EXIT"
//!             CloseApplication
//!         Case Else
//!             MsgBox "Unknown command"
//!     End Select
//! End Sub
//! ```
//!
//! ### Example 4: File Extension Check
//! ```vb6
//! Function IsImageFile(fileName As String) As Boolean
//!     Dim ext As String
//!     ext = UCase$(Right$(fileName, 4))
//!     
//!     IsImageFile = (ext = ".JPG" Or ext = ".PNG" Or ext = ".GIF" Or ext = ".BMP")
//! End Function
//! ```
//!
//! ## Common Patterns
//!
//! ### Pattern 1: Case-Insensitive String Comparison
//! ```vb6
//! Function CompareIgnoreCase(str1 As String, str2 As String) As Boolean
//!     CompareIgnoreCase = (UCase$(str1) = UCase$(str2))
//! End Function
//! ```
//!
//! ### Pattern 2: Normalize Database Input
//! ```vb6
//! Function NormalizeForDatabase(value As String) As String
//!     NormalizeForDatabase = UCase$(Trim$(value))
//! End Function
//! ```
//!
//! ### Pattern 3: Case-Insensitive Contains
//! ```vb6
//! Function ContainsIgnoreCase(source As String, searchTerm As String) As Boolean
//!     ContainsIgnoreCase = (InStr(UCase$(source), UCase$(searchTerm)) > 0)
//! End Function
//! ```
//!
//! ### Pattern 4: Validate Yes/No Response
//! ```vb6
//! Function IsYesResponse(response As String) As Boolean
//!     Dim normalized As String
//!     normalized = UCase$(Trim$(response))
//!     IsYesResponse = (normalized = "Y" Or normalized = "YES")
//! End Function
//! ```
//!
//! ### Pattern 5: Case-Insensitive `StartsWith`
//! ```vb6
//! Function StartsWithIgnoreCase(str As String, prefix As String) As Boolean
//!     StartsWithIgnoreCase = (UCase$(Left$(str, Len(prefix))) = UCase$(prefix))
//! End Function
//! ```
//!
//! ### Pattern 6: Format Heading Text
//! ```vb6
//! Function FormatHeading(text As String) As String
//!     FormatHeading = UCase$(text)
//! End Function
//! ```
//!
//! ### Pattern 7: Normalize Array Elements
//! ```vb6
//! Sub NormalizeArray(arr() As String)
//!     Dim i As Long
//!     For i = LBound(arr) To UBound(arr)
//!         arr(i) = UCase$(arr(i))
//!     Next i
//! End Sub
//! ```
//!
//! ### Pattern 8: Case-Insensitive Search in Array
//! ```vb6
//! Function FindInArray(arr() As String, searchValue As String) As Long
//!     Dim i As Long
//!     Dim searchUpper As String
//!     
//!     searchUpper = UCase$(searchValue)
//!     
//!     For i = LBound(arr) To UBound(arr)
//!         If UCase$(arr(i)) = searchUpper Then
//!             FindInArray = i
//!             Exit Function
//!         End If
//!     Next i
//!     
//!     FindInArray = -1 ' Not found
//! End Function
//! ```
//!
//! ### Pattern 9: Create Acronym
//! ```vb6
//! Function CreateAcronym(phrase As String) As String
//!     Dim words() As String
//!     Dim i As Integer
//!     Dim result As String
//!     
//!     words = Split(phrase, " ")
//!     result = ""
//!     
//!     For i = LBound(words) To UBound(words)
//!         If Len(words(i)) > 0 Then
//!             result = result & UCase$(Left$(words(i), 1))
//!         End If
//!     Next i
//!     
//!     CreateAcronym = result
//! End Function
//! ```
//!
//! ### Pattern 10: Safe Null-Handling Comparison
//! ```vb6
//! Function SafeCompareIgnoreCase(str1 As Variant, str2 As Variant) As Boolean
//!     If IsNull(str1) Or IsNull(str2) Then
//!         SafeCompareIgnoreCase = False
//!     Else
//!         SafeCompareIgnoreCase = (UCase(str1) = UCase(str2))
//!     End If
//! End Function
//! ```
//!
//! ## Advanced Usage
//!
//! ### Example 1: Case-Insensitive Dictionary Class
//! ```vb6
//! ' Class: CaseInsensitiveDictionary
//! ' Dictionary with case-insensitive key lookup
//! Option Explicit
//!
//! Private m_Items As Collection
//!
//! Private Sub Class_Initialize()
//!     Set m_Items = New Collection
//! End Sub
//!
//! Public Sub Add(key As String, value As Variant)
//!     On Error Resume Next
//!     
//!     If IsObject(value) Then
//!         m_Items.Add value, UCase$(key)
//!     Else
//!         m_Items.Add value, UCase$(key)
//!     End If
//!     
//!     If Err.Number <> 0 Then
//!         Err.Raise 457, , "Key already exists: " & key
//!     End If
//! End Sub
//!
//! Public Function Item(key As String) As Variant
//!     On Error Resume Next
//!     
//!     If IsObject(m_Items(UCase$(key))) Then
//!         Set Item = m_Items(UCase$(key))
//!     Else
//!         Item = m_Items(UCase$(key))
//!     End If
//!     
//!     If Err.Number <> 0 Then
//!         Err.Raise 5, , "Key not found: " & key
//!     End If
//! End Function
//!
//! Public Function Exists(key As String) As Boolean
//!     On Error Resume Next
//!     Dim test As Variant
//!     test = m_Items(UCase$(key))
//!     Exists = (Err.Number = 0)
//! End Function
//!
//! Public Sub Remove(key As String)
//!     On Error Resume Next
//!     m_Items.Remove UCase$(key)
//!     If Err.Number <> 0 Then
//!         Err.Raise 5, , "Key not found: " & key
//!     End If
//! End Sub
//!
//! Public Property Get Count() As Long
//!     Count = m_Items.Count
//! End Property
//! ```
//!
//! ### Example 2: String Comparison Utilities Module
//! ```vb6
//! ' Module: StringComparisonUtils
//! ' Case-insensitive string comparison utilities
//! Option Explicit
//!
//! Public Function EqualsIgnoreCase(str1 As String, str2 As String) As Boolean
//!     EqualsIgnoreCase = (UCase$(str1) = UCase$(str2))
//! End Function
//!
//! Public Function StartsWithIgnoreCase(str As String, prefix As String) As Boolean
//!     If Len(prefix) > Len(str) Then
//!         StartsWithIgnoreCase = False
//!     Else
//!         StartsWithIgnoreCase = (UCase$(Left$(str, Len(prefix))) = UCase$(prefix))
//!     End If
//! End Function
//!
//! Public Function EndsWithIgnoreCase(str As String, suffix As String) As Boolean
//!     If Len(suffix) > Len(str) Then
//!         EndsWithIgnoreCase = False
//!     Else
//!         EndsWithIgnoreCase = (UCase$(Right$(str, Len(suffix))) = UCase$(suffix))
//!     End If
//! End Function
//!
//! Public Function ContainsIgnoreCase(source As String, searchTerm As String) As Boolean
//!     ContainsIgnoreCase = (InStr(1, source, searchTerm, vbTextCompare) > 0)
//! End Function
//!
//! Public Function IndexOfIgnoreCase(source As String, searchTerm As String, _
//!                                  Optional startPos As Long = 1) As Long
//!     IndexOfIgnoreCase = InStr(startPos, source, searchTerm, vbTextCompare)
//! End Function
//!
//! Public Function ReplaceIgnoreCase(source As String, findText As String, _
//!                                  replaceText As String) As String
//!     ReplaceIgnoreCase = Replace(source, findText, replaceText, 1, -1, vbTextCompare)
//! End Function
//!
//! Public Function CompareIgnoreCase(str1 As String, str2 As String) As Integer
//!     If UCase$(str1) < UCase$(str2) Then
//!         CompareIgnoreCase = -1
//!     ElseIf UCase$(str1) > UCase$(str2) Then
//!         CompareIgnoreCase = 1
//!     Else
//!         CompareIgnoreCase = 0
//!     End If
//! End Function
//! ```
//!
//! ### Example 3: Text Normalizer Class
//! ```vb6
//! ' Class: TextNormalizer
//! ' Normalizes text for consistent processing
//! Option Explicit
//!
//! Public Enum NormalizationMode
//!     nmUpperCase = 0
//!     nmLowerCase = 1
//!     nmTrimmed = 2
//!     nmUpperCaseTrimmed = 3
//!     nmLowerCaseTrimmed = 4
//! End Enum
//!
//! Public Function Normalize(text As String, mode As NormalizationMode) As String
//!     Select Case mode
//!         Case nmUpperCase
//!             Normalize = UCase$(text)
//!         Case nmLowerCase
//!             Normalize = LCase$(text)
//!         Case nmTrimmed
//!             Normalize = Trim$(text)
//!         Case nmUpperCaseTrimmed
//!             Normalize = UCase$(Trim$(text))
//!         Case nmLowerCaseTrimmed
//!             Normalize = LCase$(Trim$(text))
//!         Case Else
//!             Normalize = text
//!     End Select
//! End Function
//!
//! Public Function NormalizeArray(arr() As String, mode As NormalizationMode) As String()
//!     Dim result() As String
//!     Dim i As Long
//!     
//!     ReDim result(LBound(arr) To UBound(arr))
//!     
//!     For i = LBound(arr) To UBound(arr)
//!         result(i) = Normalize(arr(i), mode)
//!     Next i
//!     
//!     NormalizeArray = result
//! End Function
//!
//! Public Function NormalizeForComparison(text As String) As String
//!     ' Remove extra spaces and convert to uppercase
//!     Dim temp As String
//!     temp = Trim$(text)
//!     
//!     ' Replace multiple spaces with single space
//!     Do While InStr(temp, "  ") > 0
//!         temp = Replace(temp, "  ", " ")
//!     Loop
//!     
//!     NormalizeForComparison = UCase$(temp)
//! End Function
//! ```
//!
//! ### Example 4: Command Parser Module
//! ```vb6
//! ' Module: CommandParser
//! ' Parses and processes user commands
//! Option Explicit
//!
//! Public Function ParseCommand(commandLine As String, command As String, _
//!                             arguments() As String) As Boolean
//!     Dim parts() As String
//!     Dim i As Integer
//!     
//!     commandLine = Trim$(commandLine)
//!     If Len(commandLine) = 0 Then
//!         ParseCommand = False
//!         Exit Function
//!     End If
//!     
//!     parts = Split(commandLine, " ")
//!     command = UCase$(parts(0))
//!     
//!     If UBound(parts) > 0 Then
//!         ReDim arguments(0 To UBound(parts) - 1)
//!         For i = 1 To UBound(parts)
//!             arguments(i - 1) = parts(i)
//!         Next i
//!     Else
//!         ReDim arguments(0 To -1) ' Empty array
//!     End If
//!     
//!     ParseCommand = True
//! End Function
//!
//! Public Function IsValidCommand(command As String, validCommands() As String) As Boolean
//!     Dim i As Long
//!     Dim cmdUpper As String
//!     
//!     cmdUpper = UCase$(command)
//!     
//!     For i = LBound(validCommands) To UBound(validCommands)
//!         If UCase$(validCommands(i)) = cmdUpper Then
//!             IsValidCommand = True
//!             Exit Function
//!         End If
//!     Next i
//!     
//!     IsValidCommand = False
//! End Function
//!
//! Public Function GetCommandHelp(command As String) As String
//!     Select Case UCase$(Trim$(command))
//!         Case "HELP"
//!             GetCommandHelp = "Displays help information"
//!         Case "SAVE"
//!             GetCommandHelp = "Saves the current document"
//!         Case "LOAD"
//!             GetCommandHelp = "Loads a document"
//!         Case "EXIT", "QUIT"
//!             GetCommandHelp = "Exits the application"
//!         Case Else
//!             GetCommandHelp = "Unknown command"
//!     End Select
//! End Function
//! ```
//!
//! ## Error Handling
//! The `UCase` function can raise the following errors:
//!
//! - **Error 13 (Type mismatch)**: If argument cannot be converted to a string
//! - **Error 94 (Invalid use of Null)**: When using `UCase$` (not `UCase`) with Null argument
//!
//! Note: `UCase` (without $) returns Null if passed Null, while `UCase$` raises an error.
//!
//! ## Performance Notes
//! - Very fast string operation
//! - Performance is linear O(n) with string length
//! - `UCase$` is slightly faster than `UCase` (returns String vs Variant)
//! - Consider caching result if used multiple times with same value
//! - No significant difference for short strings (< 100 characters)
//! - For large-scale comparisons, convert once and cache
//!
//! ## Best Practices
//! 1. **Use `UCase`$ when possible** for better performance and type safety
//! 2. **Cache conversions** when comparing the same string multiple times
//! 3. **Combine with Trim$** when normalizing user input
//! 4. **Handle Null explicitly** when using `UCase` (not `UCase`$)
//! 5. **Use `StrComp` for locale-aware comparison** instead of manual `UCase` conversion
//! 6. **Consider Option Compare Text** for case-insensitive operations in entire module
//! 7. **Document case-sensitivity** in function comments
//! 8. **Normalize early** in data processing pipeline
//! 9. **Use for display formatting** when uppercase is needed for UI
//! 10. **Avoid repeated conversion** in tight loops
//!
//! ## Comparison Table
//!
//! | Function | Conversion | Returns | Null Handling |
//! |----------|------------|---------|---------------|
//! | `UCase` | To uppercase | Variant (String) | Returns Null |
//! | `UCase$` | To uppercase | String | Error on Null |
//! | `LCase` | To lowercase | Variant (String) | Returns Null |
//! | `LCase$` | To lowercase | String | Error on Null |
//! | `StrConv` | Various | Variant | Configurable |
//!
//! ## Platform Notes
//! - Available in VB6, VBA, and `VBScript`
//! - Behavior consistent across platforms
//! - Locale-aware conversion for international characters
//! - Unicode support in VBA/VB6
//! - String length unchanged by conversion
//!
//! ## Limitations
//! - Cannot selectively convert parts of string (use Mid$ if needed)
//! - No way to specify locale explicitly (uses system locale)
//! - Cannot preserve original case information (one-way conversion)
//! - Does not handle special Unicode cases (e.g., title case)
//! - No built-in toggle case functionality
//! - Cannot convert specific character ranges

use crate::{error::VBResult, value::VBString};

/// Returns the string converted to uppercase.
///
/// Only lowercase letters are converted; uppercase letters and non-letter
/// characters are unchanged. Uses full Unicode case mapping.
pub fn ucase(input: &VBString) -> VBResult<VBString> {
    Ok(VBString::from(input.as_str().to_uppercase()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uppercases_letters() {
        assert_eq!(
            ucase(&VBString::from("Hello World")).unwrap(),
            VBString::from("HELLO WORLD")
        );
        assert_eq!(
            ucase(&VBString::from("abc")).unwrap(),
            VBString::from("ABC")
        );
    }

    #[test]
    fn leaves_non_letters_unchanged() {
        assert_eq!(
            ucase(&VBString::from("123 !@#")).unwrap(),
            VBString::from("123 !@#")
        );
    }

    #[test]
    fn handles_unicode() {
        assert_eq!(
            ucase(&VBString::from("héllo")).unwrap(),
            VBString::from("HÉLLO")
        );
        assert_eq!(ucase(&VBString::from("")).unwrap(), VBString::from(""));
    }
}
