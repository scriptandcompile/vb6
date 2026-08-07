//! # `LCase` Function
//!
//! Returns a `String` that has been converted to lowercase.
//!
//! ## Syntax
//!
//! ```vb
//! LCase(string)
//! ```
//!
//! ## Parameters
//!
//! - `string` (Required): Any valid string expression
//!   - If string contains `Null`, `Null` is returned
//!
//! ## Return Value
//!
//! Returns a `String`:
//! - Contains the same string with all uppercase letters converted to lowercase
//! - Lowercase letters and non-alphabetic characters are unchanged
//! - Returns `Null` if string argument is `Null`
//! - Empty string returns empty string
//! - Only affects A-Z characters (not accented characters in some locales)
//! - Numbers, punctuation, and symbols are unchanged
//! - Whitespace is preserved
//!
//! ## Remarks
//!
//! The `LCase` function converts uppercase letters to lowercase:
//!
//! - Only affects uppercase letters A-Z
//! - All other characters remain unchanged
//! - Counterpart to `UCase` function (converts to uppercase)
//! - `Null` propagates through the function (`Null` input returns `Null`)
//! - Does not modify the original string (strings are immutable in VB6)
//! - Locale-aware in some versions (may affect accented characters)
//! - Common for case-insensitive string comparisons
//! - Useful for normalizing user input
//! - Works with string variables, literals, and expressions
//! - Can be combined with other string functions (`Trim`, `Replace`, etc.)
//! - Performance is generally fast for typical strings
//! - For single character, consider using `LCase$` for slightly better performance
//! - `LCase$` variant returns `String` type (not `Variant`)
//!
//! ## Typical Uses
//!
//! 1. **Case-Insensitive Comparison**: Compare strings ignoring case
//! 2. **User Input Normalization**: Convert user input to consistent case
//! 3. **File Path Comparison**: Compare file paths case-insensitively (on Windows)
//! 4. **Search Operations**: Case-insensitive text searching
//! 5. **Data Validation**: Normalize data for validation
//! 6. **Database Queries**: Prepare strings for case-insensitive matching
//! 7. **Email Addresses**: Normalize email addresses to lowercase
//! 8. **Configuration Keys**: Standardize configuration key format
//!
//! ## Basic Usage Examples
//!
//! ```vb
//! ' Example 1: Basic lowercase conversion
//! Dim result As String
//!
//! result = LCase("HELLO")              ' "hello"
//! result = LCase("Hello World")        ' "hello world"
//! result = LCase("VB6 Programming")    ' "vb6 programming"
//!
//! ' Example 2: Case-insensitive comparison
//! Dim input As String
//! input = "Yes"
//!
//! If LCase(input) = "yes" Then
//!     MsgBox "User answered yes"
//! End If
//!
//! ' Example 3: Mixed case preservation
//! Dim text As String
//! text = "Hello123WORLD"
//!
//! Debug.Print LCase(text)              ' "hello123world" - numbers unchanged
//!
//! ' Example 4: Null handling
//! Dim value As Variant
//! value = Null
//!
//! Debug.Print IsNull(LCase(value))     ' True - Null propagates
//! ```
//!
//! ## Common Patterns
//!
//! ```vb
//! ' Pattern 1: Case-insensitive string comparison
//! Function EqualsIgnoreCase(str1 As String, str2 As String) As Boolean
//!     EqualsIgnoreCase = (LCase(str1) = LCase(str2))
//! End Function
//!
//! ' Pattern 2: Case-insensitive contains check
//! Function ContainsIgnoreCase(text As String, searchFor As String) As Boolean
//!     ContainsIgnoreCase = (InStr(1, LCase(text), LCase(searchFor)) > 0)
//! End Function
//!
//! ' Pattern 3: Normalize user input
//! Function NormalizeInput(userInput As String) As String
//!     NormalizeInput = LCase(Trim(userInput))
//! End Function
//!
//! ' Pattern 4: Validate yes/no input
//! Function IsYes(input As String) As Boolean
//!     Select Case LCase(Trim(input))
//!         Case "yes", "y", "true", "1"
//!             IsYes = True
//!         Case Else
//!             IsYes = False
//!     End Select
//! End Function
//!
//! ' Pattern 5: Case-insensitive array search
//! Function FindInArray(arr As Variant, searchValue As String) As Long
//!     Dim i As Long
//!     Dim searchLower As String
//!     
//!     FindInArray = -1
//!     If Not IsArray(arr) Then Exit Function
//!     
//!     searchLower = LCase(searchValue)
//!     For i = LBound(arr) To UBound(arr)
//!         If LCase(arr(i)) = searchLower Then
//!             FindInArray = i
//!             Exit Function
//!         End If
//!     Next i
//! End Function
//!
//! ' Pattern 6: Extract lowercase letters only
//! Function GetLowercaseLetters(text As String) As String
//!     Dim i As Long
//!     Dim char As String
//!     Dim result As String
//!     
//!     result = ""
//!     For i = 1 To Len(text)
//!         char = Mid(text, i, 1)
//!         If char = LCase(char) And char >= "a" And char <= "z" Then
//!             result = result & char
//!         End If
//!     Next i
//!     
//!     GetLowercaseLetters = result
//! End Function
//!
//! ' Pattern 7: Normalize email address
//! Function NormalizeEmail(email As String) As String
//!     NormalizeEmail = LCase(Trim(email))
//! End Function
//!
//! ' Pattern 8: Case-insensitive Replace
//! Function ReplaceIgnoreCase(text As String, findStr As String, _
//!                            replaceStr As String) As String
//!     Dim pos As Long
//!     Dim result As String
//!     Dim textLower As String
//!     Dim findLower As String
//!     
//!     result = text
//!     textLower = LCase(text)
//!     findLower = LCase(findStr)
//!     
//!     pos = InStr(1, textLower, findLower)
//!     Do While pos > 0
//!         result = Left(result, pos - 1) & replaceStr & _
//!                  Mid(result, pos + Len(findStr))
//!         textLower = LCase(result)
//!         pos = InStr(pos + Len(replaceStr), textLower, findLower)
//!     Loop
//!     
//!     ReplaceIgnoreCase = result
//! End Function
//!
//! ' Pattern 9: Check if string is all lowercase
//! Function IsAllLowercase(text As String) As Boolean
//!     IsAllLowercase = (text = LCase(text))
//! End Function
//!
//! ' Pattern 10: Toggle case
//! Function ToggleCase(text As String) As String
//!     Dim i As Long
//!     Dim char As String
//!     Dim result As String
//!     
//!     result = ""
//!     For i = 1 To Len(text)
//!         char = Mid(text, i, 1)
//!         If char = UCase(char) Then
//!             result = result & LCase(char)
//!         Else
//!             result = result & UCase(char)
//!         End If
//!     Next i
//!     
//!     ToggleCase = result
//! End Function
//! ```
//!
//! ## Advanced Usage Examples
//!
//! ```vb
//! ' Example 1: Case-insensitive dictionary/lookup
//! Public Class CaseInsensitiveDictionary
//!     Private m_dict As Object  ' Scripting.Dictionary
//!     
//!     Private Sub Class_Initialize()
//!         Set m_dict = CreateObject("Scripting.Dictionary")
//!         m_dict.CompareMode = vbTextCompare  ' Alternative to LCase
//!     End Sub
//!     
//!     Public Sub Add(key As String, value As Variant)
//!         Dim keyLower As String
//!         keyLower = LCase(key)
//!         
//!         If m_dict.Exists(keyLower) Then
//!             Err.Raise 457, "CaseInsensitiveDictionary", "Key already exists"
//!         End If
//!         
//!         If IsObject(value) Then
//!             Set m_dict(keyLower) = value
//!         Else
//!             m_dict(keyLower) = value
//!         End If
//!     End Sub
//!     
//!     Public Function Get(key As String) As Variant
//!         Dim keyLower As String
//!         keyLower = LCase(key)
//!         
//!         If Not m_dict.Exists(keyLower) Then
//!             Err.Raise 5, "CaseInsensitiveDictionary", "Key not found"
//!         End If
//!         
//!         If IsObject(m_dict(keyLower)) Then
//!             Set Get = m_dict(keyLower)
//!         Else
//!             Get = m_dict(keyLower)
//!         End If
//!     End Function
//!     
//!     Public Function Exists(key As String) As Boolean
//!         Exists = m_dict.Exists(LCase(key))
//!     End Function
//!     
//!     Public Sub Remove(key As String)
//!         m_dict.Remove LCase(key)
//!     End Sub
//! End Class
//!
//! ' Example 2: Text search with case-insensitive highlighting
//! Public Class TextHighlighter
//!     Public Function Highlight(text As String, searchTerm As String, _
//!                               highlightStart As String, _
//!                               highlightEnd As String) As String
//!         Dim result As String
//!         Dim pos As Long
//!         Dim lastPos As Long
//!         Dim textLower As String
//!         Dim searchLower As String
//!         
//!         If Len(searchTerm) = 0 Then
//!             Highlight = text
//!             Exit Function
//!         End If
//!         
//!         result = ""
//!         lastPos = 1
//!         textLower = LCase(text)
//!         searchLower = LCase(searchTerm)
//!         
//!         pos = InStr(lastPos, textLower, searchLower)
//!         Do While pos > 0
//!             ' Add text before match
//!             result = result & Mid(text, lastPos, pos - lastPos)
//!             ' Add highlighted match
//!             result = result & highlightStart & _
//!                      Mid(text, pos, Len(searchTerm)) & highlightEnd
//!             
//!             lastPos = pos + Len(searchTerm)
//!             pos = InStr(lastPos, textLower, searchLower)
//!         Loop
//!         
//!         ' Add remaining text
//!         result = result & Mid(text, lastPos)
//!         
//!         Highlight = result
//!     End Function
//! End Class
//!
//! ' Example 3: String matcher with wildcards
//! Public Class WildcardMatcher
//!     Public Function Matches(text As String, pattern As String, _
//!                             Optional caseSensitive As Boolean = False) As Boolean
//!         Dim textToMatch As String
//!         Dim patternToMatch As String
//!         
//!         If caseSensitive Then
//!             textToMatch = text
//!             patternToMatch = pattern
//!         Else
//!             textToMatch = LCase(text)
//!             patternToMatch = LCase(pattern)
//!         End If
//!         
//!         Matches = MatchesInternal(textToMatch, patternToMatch)
//!     End Function
//!     
//!     Private Function MatchesInternal(text As String, pattern As String) As Boolean
//!         ' Simple wildcard matching (* = any chars, ? = any single char)
//!         If pattern = "*" Then
//!             MatchesInternal = True
//!             Exit Function
//!         End If
//!         
//!         If Len(pattern) = 0 Then
//!             MatchesInternal = (Len(text) = 0)
//!             Exit Function
//!         End If
//!         
//!         If Left(pattern, 1) = "*" Then
//!             ' Try matching rest of pattern at various positions
//!             Dim i As Long
//!             For i = 0 To Len(text)
//!                 If MatchesInternal(Mid(text, i + 1), Mid(pattern, 2)) Then
//!                     MatchesInternal = True
//!                     Exit Function
//!                 End If
//!             Next i
//!             MatchesInternal = False
//!         ElseIf Left(pattern, 1) = "?" Then
//!             If Len(text) > 0 Then
//!                 MatchesInternal = MatchesInternal(Mid(text, 2), Mid(pattern, 2))
//!             Else
//!                 MatchesInternal = False
//!             End If
//!         Else
//!             If Len(text) > 0 And Left(text, 1) = Left(pattern, 1) Then
//!                 MatchesInternal = MatchesInternal(Mid(text, 2), Mid(pattern, 2))
//!             Else
//!                 MatchesInternal = False
//!             End If
//!         End If
//!     End Function
//! End Class
//!
//! ' Example 4: Command parser with case-insensitive commands
//! Public Class CommandParser
//!     Private m_commands As Collection
//!     
//!     Private Sub Class_Initialize()
//!         Set m_commands = New Collection
//!     End Sub
//!     
//!     Public Sub RegisterCommand(commandName As String, handler As Object)
//!         m_commands.Add handler, LCase(commandName)
//!     End Sub
//!     
//!     Public Function Parse(input As String) As Boolean
//!         Dim parts() As String
//!         Dim command As String
//!         Dim handler As Object
//!         
//!         Parse = False
//!         input = Trim(input)
//!         If Len(input) = 0 Then Exit Function
//!         
//!         parts = Split(input, " ")
//!         If UBound(parts) < 0 Then Exit Function
//!         
//!         command = LCase(parts(0))
//!         
//!         On Error Resume Next
//!         Set handler = m_commands(command)
//!         On Error GoTo 0
//!         
//!         If Not handler Is Nothing Then
//!             ' Execute command handler
//!             ' handler.Execute(parts)
//!             Parse = True
//!         End If
//!     End Function
//!     
//!     Public Function GetCommandList() As String
//!         Dim i As Long
//!         Dim result As String
//!         
//!         result = ""
//!         For i = 1 To m_commands.Count
//!             If i > 1 Then result = result & ", "
//!             ' Note: Can't easily get key from Collection
//!             ' This is simplified example
//!         Next i
//!         
//!         GetCommandList = result
//!     End Function
//! End Class
//! ```
//!
//! ## Error Handling
//!
//! `LCase` handles special cases gracefully:
//!
//! ```vb
//! ' Empty string returns empty string
//! Debug.Print LCase("")                ' ""
//!
//! ' Null propagates
//! Dim value As Variant
//! value = Null
//! Debug.Print IsNull(LCase(value))     ' True
//!
//! ' Non-alphabetic characters unchanged
//! Debug.Print LCase("123!@#")          ' "123!@#"
//!
//! ' Mixed content
//! Debug.Print LCase("ABC123xyz")       ' "abc123xyz"
//!
//! ' Safe pattern with Null check
//! Function SafeLCase(value As Variant) As String
//!     If IsNull(value) Then
//!         SafeLCase = ""
//!     Else
//!         SafeLCase = LCase(value)
//!     End If
//! End Function
//! ```
//!
//! ## Performance Considerations
//!
//! - **Fast Operation**: `LCase` is generally very fast
//! - **String Creation**: Creates new string (strings are immutable)
//! - **Repeated Calls**: Cache result if using same lowercase value multiple times
//! - **`LCase$` Variant**: Use `LCase$` for `String` return type (slightly faster)
//!
//! Performance tips:
//! ```vb
//! ' Less efficient - multiple conversions
//! If LCase(str1) = LCase(str2) And LCase(str1) = LCase(str3) Then
//!
//! ' More efficient - cache conversion
//! Dim str1Lower As String
//! str1Lower = LCase(str1)
//! If str1Lower = LCase(str2) And str1Lower = LCase(str3) Then
//! ```
//!
//! ## Best Practices
//!
//! 1. **Case-Insensitive Comparisons**: Always use `LCase` for both operands
//! 2. **Null Handling**: Check for `Null` before calling `LCase` if needed
//! 3. **Cache Results**: Store converted strings when used multiple times
//! 4. **Database Comparisons**: Use `LCase` to normalize before database queries
//! 5. **User Input**: Always normalize user input with `LCase` + `Trim`
//! 6. **Email Addresses**: Convert email addresses to lowercase for storage/comparison
//! 7. **File Extensions**: Use `LCase` when comparing file extensions
//! 8. **Configuration**: Use consistent casing for configuration keys
//!
//! ## Comparison with Related Functions
//!
//! | Function | Purpose | Returns | Use Case |
//! |----------|---------|---------|----------|
//! | `LCase` | Convert to lowercase | `String` | Lowercase conversion |
//! | `UCase` | Convert to uppercase | `String` | Uppercase conversion |
//! | `StrComp` | Compare strings | `Integer` | Case-sensitive or insensitive comparison |
//! | `Trim` | Remove whitespace | `String` | Cleanup whitespace |
//! | `Left`/`Right`/`Mid` | Extract substring | `String` | Substring extraction |
//!
//! ## `LCase` vs `StrComp`
//!
//! ```vb
//! Dim str1 As String, str2 As String
//! str1 = "Hello"
//! str2 = "HELLO"
//!
//! ' Using LCase for comparison
//! If LCase(str1) = LCase(str2) Then
//!     MsgBox "Equal (case-insensitive)"
//! End If
//!
//! ' Using StrComp for comparison
//! If StrComp(str1, str2, vbTextCompare) = 0 Then
//!     MsgBox "Equal (case-insensitive)"
//! End If
//!
//! ' LCase is more explicit and readable for simple comparisons
//! ' StrComp is better when you need the comparison result (-1, 0, 1)
//! ```
//!
//! ## `LCase$` Variant
//!
//! ```vb
//! ' LCase returns Variant
//! Dim result As Variant
//! result = LCase("HELLO")
//!
//! ' LCase$ returns String (slightly faster, cannot handle Null)
//! Dim resultStr As String
//! resultStr = LCase$("HELLO")
//!
//! ' LCase$ will error on Null
//! ' resultStr = LCase$(Null)  ' Error 94: Invalid use of Null
//! ```
//!
//! ## Platform and Version Notes
//!
//! - Available in all VB6 versions
//! - Part of VBA core functions
//! - Returns `Variant` containing `String` (`LCase$` returns `String` type)
//! - Locale-aware in some implementations
//! - Only converts A-Z in most locales
//! - Accented characters may or may not be converted depending on locale
//!
//! ## Limitations
//!
//! - Only converts standard ASCII uppercase letters (A-Z)
//! - Accented characters may not be converted consistently
//! - Does not handle Unicode case mapping comprehensively
//! - Cannot convert specific ranges of characters
//! - No option to preserve certain characters
//! - Creates new string (cannot modify in place)
//!
//! ## Related Functions
//!
//! - `UCase`: Convert string to uppercase
//! - `StrComp`: Compare strings with case options
//! - `Trim`/`LTrim`/`RTrim`: Remove whitespace
//! - `Replace`: Replace substrings (with case-sensitive option)
//! - `InStr`: Find substring (can be case-insensitive)

use crate::{error::VBResult, value::VBString};

/// Returns the string converted to lowercase.
///
/// Only uppercase letters are converted; lowercase letters and non-letter
/// characters are unchanged. Uses full Unicode case mapping.
pub fn lcase(input: &VBString) -> VBResult<VBString> {
    Ok(VBString::from(input.as_str().to_lowercase()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lowercases_letters() {
        assert_eq!(
            lcase(&VBString::from("Hello World")).unwrap(),
            VBString::from("hello world")
        );
        assert_eq!(
            lcase(&VBString::from("ABC")).unwrap(),
            VBString::from("abc")
        );
    }

    #[test]
    fn leaves_non_letters_unchanged() {
        assert_eq!(
            lcase(&VBString::from("123 !@#")).unwrap(),
            VBString::from("123 !@#")
        );
    }

    #[test]
    fn handles_unicode() {
        assert_eq!(
            lcase(&VBString::from("HÉLLO")).unwrap(),
            VBString::from("héllo")
        );
        assert_eq!(lcase(&VBString::from("")).unwrap(), VBString::from(""));
    }
}
