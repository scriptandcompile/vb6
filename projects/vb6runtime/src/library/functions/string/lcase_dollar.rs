//! # `LCase$` Function
//!
//! Returns a `String` that has been converted to lowercase.
//! The "$" suffix indicates this function returns a `String` type.
//!
//! ## Syntax
//!
//! ```vb
//! LCase$(string)
//! ```
//!
//! ## Parameters
//!
//! - **string**: Required. Any valid string expression. If `string` contains `Null`, `Null` is returned.
//!
//! ## Returns
//!
//! Returns a `String` with all uppercase letters converted to lowercase. Numbers and punctuation
//! are unchanged.
//!
//! ## Remarks
//!
//! - `LCase$` converts all uppercase letters in a string to lowercase.
//! - The "$" suffix explicitly indicates the function returns a `String` type rather than a `Variant`.
//! - Only uppercase letters (A-Z) are affected; lowercase letters and non-alphabetic characters remain unchanged.
//! - `LCase$` is functionally equivalent to `LCase`, but `LCase$` returns a `String` while `LCase` can return a `Variant`.
//! - For better performance when you know the result is a string, use `LCase$`.
//! - If the argument is `Null`, the function returns `Null`.
//! - The conversion is based on the system locale settings.
//! - For international characters, the behavior depends on the current code page.
//! - The inverse function is `UCase$`, which converts strings to uppercase.
//! - Common use cases include case-insensitive comparisons and normalizing user input.
//!
//! ## Typical Uses
//!
//! 1. **Case-insensitive comparisons** - Compare strings without regard to case
//! 2. **User input normalization** - Convert user input to a consistent case
//! 3. **Email address handling** - Normalize email addresses to lowercase
//! 4. **File path comparisons** - Compare file paths on case-insensitive file systems
//! 5. **Username validation** - Normalize usernames to lowercase
//! 6. **Search operations** - Perform case-insensitive searches
//! 7. **Data standardization** - Ensure consistent casing in databases
//!
//! ## Basic Examples
//!
//! ```vb
//! ' Example 1: Simple conversion
//! Dim result As String
//! result = LCase$("HELLO")  ' Returns "hello"
//! ```
//!
//! ```vb
//! ' Example 2: Mixed case
//! Dim text As String
//! text = LCase$("Hello World")  ' Returns "hello world"
//! ```
//!
//! ```vb
//! ' Example 3: With numbers and punctuation
//! Dim mixed As String
//! mixed = LCase$("ABC123!@#")  ' Returns "abc123!@#"
//! ```
//!
//! ```vb
//! ' Example 4: Already lowercase
//! Dim lower As String
//! lower = LCase$("already lowercase")  ' Returns "already lowercase"
//! ```
//!
//! ## Common Patterns
//!
//! ### Case-Insensitive Comparison
//! ```vb
//! Function CompareIgnoreCase(str1 As String, str2 As String) As Boolean
//!     CompareIgnoreCase = (LCase$(str1) = LCase$(str2))
//! End Function
//! ```
//!
//! ### Normalize User Input
//! ```vb
//! Function NormalizeInput(userInput As String) As String
//!     NormalizeInput = Trim$(LCase$(userInput))
//! End Function
//! ```
//!
//! ### Email Address Normalization
//! ```vb
//! Function NormalizeEmail(email As String) As String
//!     NormalizeEmail = LCase$(Trim$(email))
//! End Function
//! ```
//!
//! ### Case-Insensitive Search
//! ```vb
//! Function ContainsIgnoreCase(text As String, searchFor As String) As Boolean
//!     ContainsIgnoreCase = (InStr(LCase$(text), LCase$(searchFor)) > 0)
//! End Function
//! ```
//!
//! ### Username Validation
//! ```vb
//! Function ValidateUsername(username As String) As String
//!     ' Normalize to lowercase
//!     ValidateUsername = LCase$(Trim$(username))
//! End Function
//! ```
//!
//! ### File Extension Check
//! ```vb
//! Function HasExtension(filename As String, ext As String) As Boolean
//!     Dim fileExt As String
//!     fileExt = LCase$(Right$(filename, Len(ext)))
//!     HasExtension = (fileExt = LCase$(ext))
//! End Function
//! ```
//!
//! ### Dictionary Key Normalization
//! ```vb
//! Sub AddToDictionary(dict As Object, key As String, value As Variant)
//!     dict.Add LCase$(key), value
//! End Sub
//! ```
//!
//! ### Case-Insensitive Replace
//! ```vb
//! Function ReplaceIgnoreCase(text As String, findText As String, replaceWith As String) As String
//!     Dim lowerText As String
//!     Dim lowerFind As String
//!     Dim pos As Long
//!     
//!     lowerText = LCase$(text)
//!     lowerFind = LCase$(findText)
//!     pos = InStr(lowerText, lowerFind)
//!     
//!     If pos > 0 Then
//!         ReplaceIgnoreCase = Left$(text, pos - 1) & replaceWith & Mid$(text, pos + Len(findText))
//!     Else
//!         ReplaceIgnoreCase = text
//!     End If
//! End Function
//! ```
//!
//! ### Sort Key Generation
//! ```vb
//! Function GenerateSortKey(text As String) As String
//!     GenerateSortKey = LCase$(Trim$(text))
//! End Function
//! ```
//!
//! ### Command Parser
//! ```vb
//! Function ParseCommand(input As String) As String
//!     Dim cmd As String
//!     cmd = LCase$(Trim$(input))
//!     
//!     Select Case cmd
//!         Case "help"
//!             ParseCommand = "ShowHelp"
//!         Case "exit", "quit"
//!             ParseCommand = "Exit"
//!         Case Else
//!             ParseCommand = "Unknown"
//!     End Select
//! End Function
//! ```
//!
//! ## Advanced Examples
//!
//! ### Case-Insensitive Collection Lookup
//! ```vb
//! Function FindInCollection(col As Collection, key As String) As Variant
//!     On Error Resume Next
//!     Dim lowerKey As String
//!     Dim item As Variant
//!     Dim i As Long
//!     
//!     lowerKey = LCase$(key)
//!     
//!     ' Try direct lookup first
//!     FindInCollection = col(lowerKey)
//!     
//!     If Err.Number <> 0 Then
//!         ' Not found, search manually
//!         Err.Clear
//!         For i = 1 To col.Count
//!             If LCase$(col(i)) = lowerKey Then
//!                 FindInCollection = col(i)
//!                 Exit Function
//!             End If
//!         Next i
//!     End If
//! End Function
//! ```
//!
//! ### SQL Query Builder
//! ```vb
//! Function BuildWhereClause(fieldName As String, operator As String, value As String) As String
//!     Dim op As String
//!     op = LCase$(Trim$(operator))
//!     
//!     Select Case op
//!         Case "equals", "="
//!             BuildWhereClause = fieldName & " = '" & value & "'"
//!         Case "contains", "like"
//!             BuildWhereClause = fieldName & " LIKE '%" & value & "%'"
//!         Case "startswith"
//!             BuildWhereClause = fieldName & " LIKE '" & value & "%'"
//!         Case Else
//!             BuildWhereClause = ""
//!     End Select
//! End Function
//! ```
//!
//! ### Configuration File Parser
//! ```vb
//! Function ParseConfigLine(line As String, ByRef key As String, ByRef value As String) As Boolean
//!     Dim pos As Long
//!     Dim trimmedLine As String
//!     
//!     trimmedLine = Trim$(line)
//!     
//!     ' Skip comments and empty lines
//!     If Len(trimmedLine) = 0 Or Left$(trimmedLine, 1) = "#" Then
//!         ParseConfigLine = False
//!         Exit Function
//!     End If
//!     
//!     pos = InStr(trimmedLine, "=")
//!     If pos > 0 Then
//!         key = LCase$(Trim$(Left$(trimmedLine, pos - 1)))
//!         value = Trim$(Mid$(trimmedLine, pos + 1))
//!         ParseConfigLine = True
//!     Else
//!         ParseConfigLine = False
//!     End If
//! End Function
//! ```
//!
//! ### Smart String Comparison
//! ```vb
//! Function SmartCompare(str1 As String, str2 As String, caseSensitive As Boolean) As Boolean
//!     If caseSensitive Then
//!         SmartCompare = (str1 = str2)
//!     Else
//!         SmartCompare = (LCase$(str1) = LCase$(str2))
//!     End If
//! End Function
//! ```
//!
//! ## Error Handling
//!
//! ```vb
//! Function SafeLCase(text As String) As String
//!     On Error GoTo ErrorHandler
//!     
//!     If IsNull(text) Then
//!         SafeLCase = ""
//!         Exit Function
//!     End If
//!     
//!     SafeLCase = LCase$(text)
//!     Exit Function
//!     
//! ErrorHandler:
//!     SafeLCase = ""
//! End Function
//! ```
//!
//! ## Performance Notes
//!
//! - `LCase$` is a fast operation with minimal overhead
//! - For large strings, the performance is linear with string length
//! - `LCase$` (returns `String`) is slightly faster than `LCase` (returns `Variant`)
//! - When comparing strings, convert both once rather than repeatedly
//! - Consider caching lowercase versions of frequently compared strings
//! - For very large datasets, consider using database-level case-insensitive comparisons
//!
//! ## Best Practices
//!
//! 1. **Use for comparisons** - Always normalize case when doing case-insensitive comparisons
//! 2. **Prefer `LCase$` over `LCase`** - Use `LCase$` when you know the result is a string
//! 3. **Cache results** - Store lowercase versions rather than converting repeatedly
//! 4. **Handle Null** - Check for `Null` values before calling `LCase$`
//! 5. **Combine with Trim** - Often useful to combine `LCase$` with `Trim$` for user input
//! 6. **Document intent** - Make it clear when lowercase conversion is for comparison vs. display
//! 7. **Consider locale** - Be aware that conversion may vary by system locale
//!
//! ## Comparison with Related Functions
//!
//! | Function | Return Type | Conversion | Use Case |
//! |----------|-------------|------------|----------|
//! | `LCase` | `Variant` | To lowercase | When working with `Variant` types |
//! | `LCase$` | `String` | To lowercase | When result is definitely a string |
//! | `UCase` | `Variant` | To uppercase | Convert to uppercase (`Variant`) |
//! | `UCase$` | `String` | To uppercase | Convert to uppercase (`String`) |
//! | `StrConv` | `String` | Various conversions | Complex case conversions |
//!
//! ## Common Use Cases
//!
//! ### Case-Insensitive String Arrays
//!
//! ```vb
//! Function ArrayContains(arr() As String, value As String) As Boolean
//!     Dim i As Long
//!     Dim lowerValue As String
//!     
//!     lowerValue = LCase$(value)
//!     
//!     For i = LBound(arr) To UBound(arr)
//!         If LCase$(arr(i)) = lowerValue Then
//!             ArrayContains = True
//!             Exit Function
//!         End If
//!     Next i
//!     
//!     ArrayContains = False
//! End Function
//! ```
//!
//! ### URL Parameter Parsing
//!
//! ```vb
//! Function GetURLParameter(url As String, paramName As String) As String
//!     Dim lowerURL As String
//!     Dim lowerParam As String
//!     Dim startPos As Long
//!     Dim endPos As Long
//!     
//!     lowerURL = LCase$(url)
//!     lowerParam = LCase$(paramName) & "="
//!     
//!     startPos = InStr(lowerURL, lowerParam)
//!     If startPos > 0 Then
//!         startPos = startPos + Len(lowerParam)
//!         endPos = InStr(startPos, url, "&")
//!         If endPos = 0 Then endPos = Len(url) + 1
//!         GetURLParameter = Mid$(url, startPos, endPos - startPos)
//!     End If
//! End Function
//! ```
//!
//! ## Platform Notes
//!
//! - On Windows, `LCase$` respects the system locale for character conversion
//! - Behavior may vary for extended ASCII and international characters
//! - For ASCII characters (A-Z), behavior is consistent across all platforms
//! - Some characters may convert differently depending on the active code page
//! - Modern Windows systems handle Unicode characters in `LCase$` operations
//!
//! ## Limitations
//!
//! - Conversion is based on system locale; may not work as expected for all Unicode characters
//! - Returns `Null` if the input is `Null` (unlike some other string functions that error)
//! - Does not handle advanced Unicode normalization or case folding
//! - For true Unicode case folding, more sophisticated methods may be needed
//! - Some special characters (like German ß) may not convert in all locales

use super::lcase;
use crate::{error::VBResult, value::VBString};

/// Converts all uppercase letters in a string to lowercase.
/// The `$` suffix indicates this function returns a `String` type (not `Variant`).
/// Only uppercase ASCII letters A-Z are converted; all other characters remain unchanged.
///
/// # Arguments
///
/// * `s: &str` — The input string. If empty, an empty string is returned.
#[inline]
pub fn lcase_dollar(input: &VBString) -> VBResult<VBString> {
    lcase(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_conversions() {
        assert_eq!(
            lcase_dollar(&VBString::from("HELLO")).unwrap(),
            VBString::from("hello")
        );
        assert_eq!(
            lcase_dollar(&VBString::from("Hello World")).unwrap(),
            VBString::from("hello world")
        );
        assert_eq!(
            lcase_dollar(&VBString::from("VB6 Programming")).unwrap(),
            VBString::from("vb6 programming")
        );
    }

    #[test]
    fn test_numbers_and_punctuation_unchanged() {
        assert_eq!(
            lcase_dollar(&VBString::from("ABC123!@#")).unwrap(),
            VBString::from("abc123!@#")
        );
        assert_eq!(
            lcase_dollar(&VBString::from("Hello 456 World")).unwrap(),
            VBString::from("hello 456 world")
        );
    }

    #[test]
    fn test_empty_string_returns_empty() {
        assert_eq!(
            lcase_dollar(&VBString::from("")).unwrap(),
            VBString::from("")
        );
    }

    #[test]
    fn test_already_lowercase_unchanged() {
        assert_eq!(
            lcase_dollar(&VBString::from("already lowercase")).unwrap(),
            VBString::from("already lowercase")
        );
    }

    #[test]
    fn test_mixed_case() {
        assert_eq!(
            lcase_dollar(&VBString::from("Hello123WORLD")).unwrap(),
            VBString::from("hello123world")
        );
    }

    #[test]
    fn test_single_character_uppercase() {
        assert_eq!(
            lcase_dollar(&VBString::from("A")).unwrap(),
            VBString::from("a")
        );
        assert_eq!(
            lcase_dollar(&VBString::from("Z")).unwrap(),
            VBString::from("z")
        );
    }

    #[test]
    fn test_single_character_non_alpha() {
        assert_eq!(
            lcase_dollar(&VBString::from("5")).unwrap(),
            VBString::from("5")
        );
        assert_eq!(
            lcase_dollar(&VBString::from("!")).unwrap(),
            VBString::from("!")
        );
        assert_eq!(
            lcase_dollar(&VBString::from(" ")).unwrap(),
            VBString::from(" ")
        );
    }

    #[test]
    fn test_special_characters_unchanged() {
        assert_eq!(
            lcase_dollar(&VBString::from("@#$%^&*()")).unwrap(),
            VBString::from("@#$%^&*()")
        );
    }

    #[test]
    fn test_whitespace_preserved() {
        assert_eq!(
            lcase_dollar(&VBString::from("  HELLO   WORLD  ")).unwrap(),
            VBString::from("  hello   world  ")
        );
        assert_eq!(
            lcase_dollar(&VBString::from("\tTAB\nNEWLINE")).unwrap(),
            VBString::from("\ttab\nnewline")
        );
    }

    #[test]
    fn test_unicode_non_ascii_unchanged() {
        // Extended Unicode characters are not converted (VB6 behavior)
        assert_eq!(
            lcase_dollar(&VBString::from("café")).unwrap(),
            VBString::from("café")
        );
        assert_eq!(
            lcase_dollar(&VBString::from("über")).unwrap(),
            VBString::from("über")
        );
    }

    #[test]
    fn test_alternate_case_string() {
        assert_eq!(
            lcase_dollar(&VBString::from("aBcDeFgHiJkLmNoPqRsTuVwXyZ")).unwrap(),
            VBString::from("abcdefghijklmnopqrstuvwxyz")
        );
    }
}
