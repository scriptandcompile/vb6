//! # Replace Function
//!
//! Returns a string in which a specified substring has been replaced with another substring a specified number of times.
//!
//! ## Syntax
//!
//! ```vb
//! Replace(expression, find, replace, [start], [count], [compare])
//! ```
//!
//! ## Parameters
//!
//! - `expression` - Required. String expression containing substring to replace.
//! - `find` - Required. Substring being searched for.
//! - `replace` - Required. Replacement substring.
//! - `start` - Optional. Position within expression where substring search is to begin. If omitted, 1 is assumed. Must be used in conjunction with count.
//! - `count` - Optional. Number of substring substitutions to perform. If omitted, default value is -1, which means make all possible substitutions.
//! - `compare` - Optional. Numeric value indicating the kind of comparison to use when evaluating substrings. If omitted, 0 is assumed (vbBinaryCompare).
//!
//! ## Compare Values
//!
//! | Constant | Value | Description |
//! |----------|-------|-------------|
//! | vbBinaryCompare | 0 | Perform a binary comparison (case-sensitive) |
//! | vbTextCompare | 1 | Perform a textual comparison (case-insensitive) |
//! | vbDatabaseCompare | 2 | Perform a comparison based on database settings (Microsoft Access only) |
//!
//! ## Return Value
//!
//! Returns a `String` with the replacements made. The return value depends on the parameters:
//!
//! | If | Replace Returns |
//! |-----|----------------|
//! | expression is zero-length | Zero-length string ("") |
//! | expression is Null | An error |
//! | find is zero-length | Copy of expression |
//! | replace is zero-length | Copy of expression with all find occurrences removed |
//! | start > Len(expression) | Zero-length string ("") |
//! | count is 0 | Copy of expression |
//!
//! ## Remarks
//!
//! The `Replace` function returns a string with substitutions made. Unlike the `Replace` method of regular expressions, this function performs simple string substitution without pattern matching.
//!
//! The return value of the `Replace` function is a string that begins at the position specified by `start`, with substitutions made, and concludes at the end of the `expression` string. It is not a copy of the original string from start to finish.
//!
//! **Important Notes**:
//! - If `start` is specified, the return value starts from that position, not from position 1
//! - The original string before `start` position is not included in the result
//! - Use `count` parameter to limit the number of replacements
//! - Binary comparison (default) is case-sensitive; textual comparison is case-insensitive
//! - Empty `find` string returns the original expression unchanged
//! - Empty `replace` string removes all occurrences of `find`
//!
//! ## Typical Uses
//!
//! 1. **Text Sanitization**: Remove or replace unwanted characters from user input
//! 2. **Data Formatting**: Replace delimiters or format characters in data
//! 3. **Template Processing**: Replace placeholders in template strings
//! 4. **Path Manipulation**: Replace path separators or modify file paths
//! 5. **Case Normalization**: Replace mixed-case text with standardized case
//! 6. **String Cleaning**: Remove multiple spaces, tabs, or other whitespace
//! 7. **Data Import/Export**: Convert between different data formats
//! 8. **SQL String Building**: Escape quotes and special characters
//!
//! ## Basic Examples
//!
//! ### Example 1: Simple Replacement
//! ```vb
//! Dim result As String
//! result = Replace("Hello World", "World", "VB6")
//! ' Returns: "Hello VB6"
//! ```
//!
//! ### Example 2: Case-Insensitive Replacement
//! ```vb
//! Dim result As String
//! result = Replace("Hello WORLD", "world", "VB6", 1, -1, vbTextCompare)
//! ' Returns: "Hello VB6"
//! ```
//!
//! ### Example 3: Remove Substring
//! ```vb
//! Dim cleaned As String
//! cleaned = Replace("Remove   extra   spaces", "   ", " ")
//! ' Returns: "Remove extra spaces"
//! ```
//!
//! ### Example 4: Limited Replacements
//! ```vb
//! Dim result As String
//! result = Replace("one, two, three, four", ", ", " | ", 1, 2)
//! ' Returns: "one | two | three, four" (only first 2 commas replaced)
//! ```
//!
//! ## Common Patterns
//!
//! ### Pattern 1: `RemoveAllSpaces`
//! ```vb
//! Function RemoveAllSpaces(text As String) As String
//!     RemoveAllSpaces = Replace(text, " ", "")
//! End Function
//! ```
//!
//! ### Pattern 2: `NormalizeWhitespace`
//! ```vb
//! Function NormalizeWhitespace(text As String) As String
//!     Dim result As String
//!     result = text
//!     
//!     ' Replace tabs with spaces
//!     result = Replace(result, vbTab, " ")
//!     
//!     ' Replace multiple spaces with single space
//!     Do While InStr(result, "  ") > 0
//!         result = Replace(result, "  ", " ")
//!     Loop
//!     
//!     NormalizeWhitespace = Trim(result)
//! End Function
//! ```
//!
//! ### Pattern 3: `EscapeSQLString`
//! ```vb
//! Function EscapeSQLString(text As String) As String
//!     ' Escape single quotes for SQL
//!     EscapeSQLString = Replace(text, "'", "''")
//! End Function
//! ```
//!
//! ### Pattern 4: `ReplaceMultiple`
//! ```vb
//! Function ReplaceMultiple(text As String, findList() As String, _
//!                          replaceList() As String) As String
//!     Dim i As Integer
//!     Dim result As String
//!     
//!     result = text
//!     
//!     For i = LBound(findList) To UBound(findList)
//!         result = Replace(result, findList(i), replaceList(i))
//!     Next i
//!     
//!     ReplaceMultiple = result
//! End Function
//! ```
//!
//! ### Pattern 5: `ReplaceCaseInsensitive`
//! ```vb
//! Function ReplaceCaseInsensitive(text As String, find As String, _
//!                                replaceWith As String) As String
//!     ReplaceCaseInsensitive = Replace(text, find, replaceWith, 1, -1, vbTextCompare)
//! End Function
//! ```
//!
//! ### Pattern 6: `ReplaceSpecialChars`
//! ```vb
//! Function ReplaceSpecialChars(text As String, replacement As String) As String
//!     Dim result As String
//!     Dim specialChars As String
//!     Dim i As Integer
//!     
//!     result = text
//!     specialChars = "!@#$%^&*()[]{}|;:,.<>?/"
//!     
//!     For i = 1 To Len(specialChars)
//!         result = Replace(result, Mid(specialChars, i, 1), replacement)
//!     Next i
//!     
//!     ReplaceSpecialChars = result
//! End Function
//! ```
//!
//! ### Pattern 7: `SanitizeFilename`
//! ```vb
//! Function SanitizeFilename(filename As String) As String
//!     Dim result As String
//!     Dim invalidChars As String
//!     Dim i As Integer
//!     
//!     result = filename
//!     invalidChars = "\/:*?""<>|"
//!     
//!     For i = 1 To Len(invalidChars)
//!         result = Replace(result, Mid(invalidChars, i, 1), "_")
//!     Next i
//!     
//!     SanitizeFilename = result
//! End Function
//! ```
//!
//! ### Pattern 8: `ConvertLineEndings`
//! ```vb
//! Function ConvertLineEndings(text As String, newEnding As String) As String
//!     Dim result As String
//!     
//!     result = text
//!     
//!     ' Normalize to LF first
//!     result = Replace(result, vbCrLf, vbLf)
//!     result = Replace(result, vbCr, vbLf)
//!     
//!     ' Convert to desired ending
//!     If newEnding <> vbLf Then
//!         result = Replace(result, vbLf, newEnding)
//!     End If
//!     
//!     ConvertLineEndings = result
//! End Function
//! ```
//!
//! ### Pattern 9: `ReplaceWithCounter`
//! ```vb
//! Function CountReplacements(text As String, find As String) As Long
//!     ' Count how many times find appears in text
//!     Dim original As String
//!     Dim replaced As String
//!     
//!     If Len(find) = 0 Then
//!         CountReplacements = 0
//!         Exit Function
//!     End If
//!     
//!     original = text
//!     replaced = Replace(original, find, "")
//!     
//!     CountReplacements = (Len(original) - Len(replaced)) / Len(find)
//! End Function
//! ```
//!
//! ### Pattern 10: `TemplateReplace`
//! ```vb
//! Function ProcessTemplate(template As String, replacements As Collection) As String
//!     ' Replace {key} placeholders with values from collection
//!     Dim result As String
//!     Dim key As Variant
//!     Dim placeholder As String
//!     
//!     result = template
//!     
//!     For Each key In replacements
//!         placeholder = "{" & key & "}"
//!         result = Replace(result, placeholder, CStr(replacements(key)))
//!     Next key
//!     
//!     ProcessTemplate = result
//! End Function
//! ```
//!
//! ## Advanced Usage
//!
//! ### Example 1: Text Sanitizer with Multiple Rules
//! ```vb
//! ' Advanced text sanitization with configurable rules
//! Class TextSanitizer
//!     Private Type ReplacementRule
//!         Find As String
//!         ReplaceWith As String
//!         CaseSensitive As Boolean
//!         MaxReplacements As Long
//!     End Type
//!     
//!     Private m_rules() As ReplacementRule
//!     Private m_ruleCount As Integer
//!     
//!     Public Sub Initialize()
//!         m_ruleCount = 0
//!         ReDim m_rules(0 To 9)
//!     End Sub
//!     
//!     Public Sub AddRule(find As String, replaceWith As String, _
//!                       Optional caseSensitive As Boolean = True, _
//!                       Optional maxReplacements As Long = -1)
//!         If m_ruleCount > UBound(m_rules) Then
//!             ReDim Preserve m_rules(0 To UBound(m_rules) + 10)
//!         End If
//!         
//!         With m_rules(m_ruleCount)
//!             .Find = find
//!             .ReplaceWith = replaceWith
//!             .CaseSensitive = caseSensitive
//!             .MaxReplacements = maxReplacements
//!         End With
//!         
//!         m_ruleCount = m_ruleCount + 1
//!     End Sub
//!     
//!     Public Function Sanitize(text As String) As String
//!         Dim result As String
//!         Dim i As Integer
//!         Dim compareMode As Integer
//!         
//!         result = text
//!         
//!         For i = 0 To m_ruleCount - 1
//!             With m_rules(i)
//!                 If .CaseSensitive Then
//!                     compareMode = vbBinaryCompare
//!                 Else
//!                     compareMode = vbTextCompare
//!                 End If
//!                 
//!                 result = Replace(result, .Find, .ReplaceWith, 1, .MaxReplacements, compareMode)
//!             End With
//!         Next i
//!         
//!         Sanitize = result
//!     End Function
//!     
//!     Public Sub ClearRules()
//!         m_ruleCount = 0
//!     End Sub
//!     
//!     Public Function GetRuleCount() As Integer
//!         GetRuleCount = m_ruleCount
//!     End Function
//! End Class
//! ```
//!
//! ### Example 2: String Template Engine
//! ```vb
//! ' Simple template engine with variable replacement
//! Module TemplateEngine
//!     Public Function ProcessTemplate(template As String, _
//!                                    variables As Scripting.Dictionary) As String
//!         Dim result As String
//!         Dim key As Variant
//!         Dim placeholder As String
//!         Dim value As String
//!         
//!         result = template
//!         
//!         ' Replace {variable} placeholders
//!         For Each key In variables.Keys
//!             placeholder = "{" & CStr(key) & "}"
//!             value = CStr(variables(key))
//!             result = Replace(result, placeholder, value)
//!         Next key
//!         
//!         ProcessTemplate = result
//!     End Function
//!     
//!     Public Function ProcessConditional(template As String, condition As Boolean, _
//!                                       trueValue As String, falseValue As String) As String
//!         Dim result As String
//!         
//!         result = template
//!         
//!         If condition Then
//!             result = Replace(result, "{if}", trueValue)
//!             result = Replace(result, "{else}", "")
//!         Else
//!             result = Replace(result, "{if}", "")
//!             result = Replace(result, "{else}", falseValue)
//!         End If
//!         
//!         ProcessConditional = result
//!     End Function
//!     
//!     Public Function ProcessLoop(template As String, items() As String) As String
//!         Dim result As String
//!         Dim itemText As String
//!         Dim i As Integer
//!         
//!         ' Extract the loop template
//!         Dim loopStart As Long
//!         Dim loopEnd As Long
//!         Dim loopTemplate As String
//!         
//!         loopStart = InStr(template, "{loop}")
//!         loopEnd = InStr(template, "{/loop}")
//!         
//!         If loopStart = 0 Or loopEnd = 0 Then
//!             ProcessLoop = template
//!             Exit Function
//!         End If
//!         
//!         loopTemplate = Mid(template, loopStart + 6, loopEnd - loopStart - 6)
//!         
//!         itemText = ""
//!         For i = LBound(items) To UBound(items)
//!             itemText = itemText & Replace(loopTemplate, "{item}", items(i))
//!         Next i
//!         
//!         result = Left(template, loopStart - 1) & itemText & Mid(template, loopEnd + 7)
//!         
//!         ProcessLoop = result
//!     End Function
//! End Module
//! ```
//!
//! ### Example 3: CSV/TSV Converter
//! ```vb
//! ' Convert between CSV and TSV formats
//! Class DelimiterConverter
//!     Private m_sourceDelimiter As String
//!     Private m_targetDelimiter As String
//!     Private m_textQualifier As String
//!     
//!     Public Sub Initialize(sourceDelim As String, targetDelim As String, _
//!                          Optional textQual As String = """")
//!         m_sourceDelimiter = sourceDelim
//!         m_targetDelimiter = targetDelim
//!         m_textQualifier = textQual
//!     End Sub
//!     
//!     Public Function Convert(data As String) As String
//!         Dim result As String
//!         Dim inQuotes As Boolean
//!         Dim i As Long
//!         Dim ch As String
//!         
//!         result = ""
//!         inQuotes = False
//!         
//!         For i = 1 To Len(data)
//!             ch = Mid(data, i, 1)
//!             
//!             If ch = m_textQualifier Then
//!                 inQuotes = Not inQuotes
//!                 result = result & ch
//!             ElseIf ch = m_sourceDelimiter And Not inQuotes Then
//!                 result = result & m_targetDelimiter
//!             Else
//!                 result = result & ch
//!             End If
//!         Next i
//!         
//!         Convert = result
//!     End Function
//!     
//!     Public Function ConvertSimple(data As String) As String
//!         ' Simple conversion without quote handling
//!         ConvertSimple = Replace(data, m_sourceDelimiter, m_targetDelimiter)
//!     End Function
//!     
//!     Public Function EscapeField(field As String) As String
//!         ' Escape field for CSV/TSV
//!         Dim needsQuotes As Boolean
//!         Dim result As String
//!         
//!         result = field
//!         
//!         ' Check if field needs quoting
//!         needsQuotes = (InStr(field, m_sourceDelimiter) > 0) Or _
//!                      (InStr(field, m_textQualifier) > 0) Or _
//!                      (InStr(field, vbCrLf) > 0)
//!         
//!         If needsQuotes Then
//!             ' Escape existing quotes
//!             result = Replace(result, m_textQualifier, m_textQualifier & m_textQualifier)
//!             result = m_textQualifier & result & m_textQualifier
//!         End If
//!         
//!         EscapeField = result
//!     End Function
//! End Class
//! ```
//!
//! ### Example 4: Smart String Replacer
//! ```vb
//! ' Advanced string replacement with history and undo
//! Class SmartReplacer
//!     Private m_originalText As String
//!     Private m_currentText As String
//!     Private m_history() As String
//!     Private m_historyCount As Integer
//!     
//!     Public Sub Initialize(text As String)
//!         m_originalText = text
//!         m_currentText = text
//!         m_historyCount = 0
//!         ReDim m_history(0 To 99)
//!         AddToHistory text
//!     End Sub
//!     
//!     Public Function ReplaceText(find As String, replaceWith As String, _
//!                                Optional caseSensitive As Boolean = True, _
//!                                Optional maxCount As Long = -1) As String
//!         Dim compareMode As Integer
//!         
//!         If caseSensitive Then
//!             compareMode = vbBinaryCompare
//!         Else
//!             compareMode = vbTextCompare
//!         End If
//!         
//!         m_currentText = Replace(m_currentText, find, replaceWith, 1, maxCount, compareMode)
//!         AddToHistory m_currentText
//!         
//!         ReplaceText = m_currentText
//!     End Function
//!     
//!     Public Function ReplaceFromStart(find As String, replaceWith As String, _
//!                                     startPos As Long) As String
//!         ' Replace starting from a specific position
//!         Dim beforeStart As String
//!         Dim afterStart As String
//!         
//!         If startPos < 1 Then startPos = 1
//!         If startPos > Len(m_currentText) Then
//!             ReplaceFromStart = m_currentText
//!             Exit Function
//!         End If
//!         
//!         beforeStart = Left(m_currentText, startPos - 1)
//!         afterStart = Replace(Mid(m_currentText, startPos), find, replaceWith)
//!         
//!         m_currentText = beforeStart & afterStart
//!         AddToHistory m_currentText
//!         
//!         ReplaceFromStart = m_currentText
//!     End Function
//!     
//!     Public Function GetCurrent() As String
//!         GetCurrent = m_currentText
//!     End Function
//!     
//!     Public Function Undo() As String
//!         If m_historyCount > 1 Then
//!             m_historyCount = m_historyCount - 1
//!             m_currentText = m_history(m_historyCount - 1)
//!         End If
//!         
//!         Undo = m_currentText
//!     End Function
//!     
//!     Public Function Reset() As String
//!         m_currentText = m_originalText
//!         m_historyCount = 0
//!         ReDim m_history(0 To 99)
//!         AddToHistory m_currentText
//!         
//!         Reset = m_currentText
//!     End Function
//!     
//!     Public Function GetReplacementCount(find As String) As Long
//!         Dim withFind As Long
//!         Dim withoutFind As Long
//!         
//!         If Len(find) = 0 Then
//!             GetReplacementCount = 0
//!             Exit Function
//!         End If
//!         
//!         withFind = Len(m_currentText)
//!         withoutFind = Len(Replace(m_currentText, find, ""))
//!         
//!         GetReplacementCount = (withFind - withoutFind) / Len(find)
//!     End Function
//!     
//!     Private Sub AddToHistory(text As String)
//!         If m_historyCount > UBound(m_history) Then
//!             ' Shift history
//!             Dim i As Integer
//!             For i = 0 To UBound(m_history) - 1
//!                 m_history(i) = m_history(i + 1)
//!             Next i
//!             m_history(UBound(m_history)) = text
//!         Else
//!             m_history(m_historyCount) = text
//!             m_historyCount = m_historyCount + 1
//!         End If
//!     End Sub
//! End Class
//! ```
//!
//! ## Error Handling
//!
//! The `Replace` function can raise errors in the following situations:
//!
//! - **Invalid Procedure Call (Error 5)**: When:
//!   - `start` parameter is less than 1
//!   - `count` parameter is less than -1
//! - **Type Mismatch (Error 13)**: When parameters cannot be converted to appropriate types
//! - **Invalid Use of Null (Error 94)**: When `expression` is Null
//!
//! Always validate inputs when necessary:
//!
//! ```vb
//! Function SafeReplace(text As String, find As String, replaceWith As String) As String
//!     On Error Resume Next
//!     SafeReplace = Replace(text, find, replaceWith)
//!     If Err.Number <> 0 Then
//!         SafeReplace = text  ' Return original on error
//!         Err.Clear
//!     End If
//!     On Error GoTo 0
//! End Function
//! ```
//!
//! ## Performance Considerations
//!
//! - The `Replace` function is optimized and very fast for most use cases
//! - Multiple sequential Replace calls can be slow for large strings
//! - Consider building a new string if making many replacements
//! - Using `count` parameter can improve performance by limiting replacements
//! - Binary comparison is faster than textual comparison
//! - Replacing with empty string is efficient for removing substrings
//!
//! ## Best Practices
//!
//! 1. **Use Meaningful Names**: Name variables clearly (find, replaceWith, not f, r)
//! 2. **Check Empty Strings**: Validate find parameter is not empty when expected
//! 3. **Consider Case Sensitivity**: Choose appropriate compare parameter
//! 4. **Limit Replacements**: Use count parameter when you know the limit
//! 5. **Chain Carefully**: Be aware that multiple Replace calls compound
//! 6. **Escape Special Characters**: Properly escape quotes and special chars
//! 7. **Validate Start Position**: Ensure start is within string bounds
//! 8. **Test Edge Cases**: Test with empty strings, no matches, all matches
//! 9. **Document Assumptions**: Comment why specific replacements are made
//! 10. **Use Constants**: Define commonly replaced strings as constants
//!
//! ## Comparison with Related Functions
//!
//! | Function | Purpose | Returns | Use Case |
//! |----------|---------|---------|----------|
//! | **Replace** | Replace substring | String (modified) | Simple string substitution |
//! | **`InStr`** | Find substring position | Long (position) | Locate substring, check existence |
//! | **Mid** | Extract substring | String (portion) | Get part of string |
//! | **Left/Right** | Extract from ends | String (portion) | Get start/end of string |
//! | **Trim/LTrim/RTrim** | Remove whitespace | String (trimmed) | Clean string edges |
//! | **UCase/LCase** | Change case | String (case changed) | Normalize case |
//!
//! ## Platform and Version Notes
//!
//! - Available in VB6 and VBA (Office 2000 and later)
//! - Not available in earlier VBA versions (use custom function)
//! - Behavior consistent across Windows platforms
//! - Case-insensitive comparison uses system locale settings
//! - vbDatabaseCompare only works in Microsoft Access
//!
//! ## Limitations
//!
//! - Cannot use regular expressions (use VBScript.RegExp for patterns)
//! - Cannot replace with different string based on match context
//! - Case-insensitive comparison depends on system locale
//! - No built-in way to replace with function result
//! - Cannot perform multiple different replacements in one call
//! - When start > 1, characters before start are not in result
//!
//! ## Related Functions
//!
//! - `InStr`: Returns position of substring within string
//! - `InStrRev`: Returns position of substring searching from end
//! - `Mid`: Returns specified portion of string
//! - `Left`: Returns specified number of characters from left
//! - `Right`: Returns specified number of characters from right
//! - `LCase`: Converts string to lowercase
//! - `UCase`: Converts string to uppercase

use crate::{
    error::{err_number, VBError, VBResult},
    value::{VBLong, VBVariant},
};

/// Returns a string in which `find` has been replaced with `replace` within
/// `expression`, searching from `start` and performing at most `count`
/// substitutions.
///
/// Positions are 1-based characters. When `start` is omitted, 1 is assumed; the
/// portion of `expression` before `start` is not included in the result. `count`
/// of -1 (the default) replaces every occurrence, `count` of 0 returns the
/// substring from `start` unchanged. `compare` selects binary (0, default) or
/// case-insensitive text (1) matching.
///
/// Returns an empty string when `expression` is empty or when `start` is past
/// the end of `expression`. An empty `find` returns the substring from `start`
/// unchanged.
///
/// # Errors
///
/// Returns error 5 (`Invalid procedure call or argument`) when `start` is less
/// than 1 or `count` is less than -1, and error 94 (`Invalid use of Null`) when
/// any of the three required arguments is `Null`.
pub fn replace(
    expression: &VBVariant,
    find: &VBVariant,
    replace: &VBVariant,
    start: Option<&VBLong>,
    count: Option<&VBLong>,
    compare: Option<&VBLong>,
) -> VBResult<VBVariant> {
    let start = start.map(|s| s.as_i32()).unwrap_or(1);
    if start < 1 {
        return Err(VBError::with_description(
            err_number::INVALID_PROCEDURE_CALL,
            "Invalid start",
        ));
    }
    let count = count.map(|c| c.as_i32()).unwrap_or(-1);
    if count < -1 {
        return Err(VBError::with_description(
            err_number::INVALID_PROCEDURE_CALL,
            "Invalid count",
        ));
    }
    let compare = compare.map(|c| c.as_i32()).unwrap_or(0);

    let expression = expression.as_vbstring()?;
    let find = find.as_vbstring()?;
    let replace = replace.as_vbstring()?;

    let chars: Vec<char> = expression.as_str().chars().collect();
    if chars.is_empty() {
        return Ok(VBVariant::from_string(""));
    }
    let start_idx = (start - 1) as usize;
    if start_idx >= chars.len() {
        return Ok(VBVariant::from_string(""));
    }
    let tail: Vec<char> = chars[start_idx..].to_vec();
    let find_chars: Vec<char> = find.as_str().chars().collect();
    if find_chars.is_empty() {
        return Ok(VBVariant::from_string(tail.iter().collect::<String>()));
    }
    if count == 0 {
        return Ok(VBVariant::from_string(tail.iter().collect::<String>()));
    }

    let needle: Vec<char> = if compare == 1 {
        find_chars.iter().flat_map(|c| c.to_lowercase()).collect()
    } else {
        find_chars.clone()
    };

    let mut out = String::new();
    let mut i = 0;
    let mut made = 0;
    while i < tail.len() {
        let window: Vec<char> = if compare == 1 {
            tail[i..].iter().flat_map(|c| c.to_lowercase()).collect()
        } else {
            tail[i..].to_vec()
        };
        if (count < 0 || made < count) && window.starts_with(&needle) {
            out.push_str(replace.as_str());
            i += find_chars.len();
            made += 1;
        } else {
            out.push(tail[i]);
            i += 1;
        }
    }
    Ok(VBVariant::from_string(out))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vb(s: &str) -> VBVariant {
        VBVariant::from_string(s)
    }

    #[test]
    fn replaces_all_occurrences_by_default() {
        assert_eq!(
            replace(&vb("Hello World"), &vb("o"), &vb("0"), None, None, None).unwrap(),
            vb("Hell0 W0rld")
        );
    }

    #[test]
    fn removes_substring_when_replace_is_empty() {
        assert_eq!(
            replace(
                &vb("Remove   extra   spaces"),
                &vb("   "),
                &vb(" "),
                None,
                None,
                None
            )
            .unwrap(),
            vb("Remove extra spaces")
        );
    }

    #[test]
    fn limits_replacements_with_count() {
        assert_eq!(
            replace(
                &vb("one, two, three, four"),
                &vb(", "),
                &vb(" | "),
                None,
                Some(&VBLong::from(2)),
                None,
            )
            .unwrap(),
            vb("one | two | three, four")
        );
    }

    #[test]
    fn text_compare_is_case_insensitive() {
        assert_eq!(
            replace(
                &vb("Hello VB6"),
                &vb("hello"),
                &vb("Hi"),
                None,
                None,
                Some(&VBLong::from(1)),
            )
            .unwrap(),
            vb("Hi VB6")
        );
    }

    #[test]
    fn start_skips_prefix_and_trims_result() {
        assert_eq!(
            replace(
                &vb("abcabcabc"),
                &vb("b"),
                &vb("X"),
                Some(&VBLong::from(2)),
                None,
                None,
            )
            .unwrap(),
            vb("XcaXcaXc")
        );
    }

    #[test]
    fn count_zero_returns_tail_unchanged() {
        assert_eq!(
            replace(
                &vb("abc"),
                &vb("b"),
                &vb("X"),
                None,
                Some(&VBLong::from(0)),
                None,
            )
            .unwrap(),
            vb("abc")
        );
    }

    #[test]
    fn empty_find_returns_expression() {
        assert_eq!(
            replace(&vb("abc"), &vb(""), &vb("X"), None, None, None).unwrap(),
            vb("abc")
        );
    }

    #[test]
    fn start_beyond_end_returns_empty() {
        assert_eq!(
            replace(
                &vb("abc"),
                &vb("b"),
                &vb("X"),
                Some(&VBLong::from(99)),
                None,
                None,
            )
            .unwrap(),
            vb("")
        );
    }

    #[test]
    fn rejects_invalid_start_and_count() {
        assert_eq!(
            replace(
                &vb("abc"),
                &vb("b"),
                &vb("X"),
                Some(&VBLong::from(0)),
                None,
                None
            )
            .unwrap_err()
            .number,
            err_number::INVALID_PROCEDURE_CALL
        );
        assert_eq!(
            replace(
                &vb("abc"),
                &vb("b"),
                &vb("X"),
                None,
                Some(&VBLong::from(-2)),
                None,
            )
            .unwrap_err()
            .number,
            err_number::INVALID_PROCEDURE_CALL
        );
    }

    #[test]
    fn null_arguments_error() {
        assert_eq!(
            replace(&VBVariant::Null, &vb("b"), &vb("X"), None, None, None)
                .unwrap_err()
                .number,
            err_number::INVALID_USE_OF_NULL
        );
    }
}
