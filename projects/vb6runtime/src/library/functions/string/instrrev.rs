//! # `InStrRev` Function
//!
//! Returns the position of an occurrence of one string within another, from the end of string.
//!
//! ## Syntax
//!
//! ```vb
//! InStrRev(stringcheck, stringmatch[, start[, compare]])
//! ```
//!
//! ## Parameters
//!
//! - `stringcheck` (Required): `String` expression being searched
//! - `stringmatch` (Required): `String` expression to search for
//! - `start` (Optional): Numeric expression that sets the starting position for each search. If omitted, -1 is used, which means the search begins at the last character position. If start contains Null, an error occurs
//! - `compare` (Optional): Numeric value indicating the kind of comparison to use when evaluating substrings. If omitted, a binary comparison is performed
//!
//! ### Compare Parameter Values
//!
//! - `vbUseCompareOption` (-1): Performs a comparison using the setting of the `Option Compare` statement
//! - `vbBinaryCompare` (0): Performs a binary comparison (case-sensitive)
//! - `vbTextCompare` (1): Performs a textual comparison (case-insensitive)
//! - `vbDatabaseCompare` (2): Microsoft Access only. Performs a comparison based on information in your database
//!
//! ## Return Value
//!
//! Returns a Long:
//! - If stringcheck is zero-length: Returns 0
//! - If stringcheck is `Null`: Returns `Null`
//! - If stringmatch is zero-length: Returns start
//! - If stringmatch is `Null`: Returns `Null`
//! - If stringmatch is not found: Returns 0
//! - If stringmatch is found within stringcheck: Returns position where match begins
//! - If start is greater than length of stringcheck: Returns 0
//!
//! ## Remarks
//!
//! The `InStrRev` function searches from the end of the string backward:
//!
//! - Searches from right to left (reverse direction)
//! - Returns the character position of the LAST occurrence (1-based indexing)
//! - Default start position is -1 (end of string) if omitted
//! - Search is case-sensitive by default (`vbBinaryCompare`)
//! - Use `vbTextCompare` for case-insensitive searching
//! - Unlike `InStr`, start parameter comes after the strings being compared
//! - The returned position is still counted from the beginning (1-based), not from the end
//! - Useful for finding file extensions, last delimiters, etc.
//! - More efficient than repeated `InStr` calls for finding last occurrence
//! - The compare parameter affects performance (binary is faster than text)
//! - Returns 0 if substring not found (test with > 0 for found)
//!
//! ## Typical Uses
//!
//! 1. **Find Last Occurrence**: Locate the last instance of a substring
//! 2. **File Extensions**: Extract file extension by finding last dot
//! 3. **Path Parsing**: Find last path separator in file paths
//! 4. **File Names**: Extract filename from full path
//! 5. **Domain Extraction**: Find last dot in domain names
//! 6. **String Truncation**: Find last delimiter for truncation
//! 7. **Reverse Parsing**: Parse from end of string
//! 8. **Last Word Extraction**: Find last space in sentences
//!
//! ## Basic Usage Examples
//!
//! ```vb
//! ' Example 1: Find last occurrence
//! Dim text As String
//! Dim pos As Long
//! text = "apple,banana,apple,orange"
//! pos = InStrRev(text, "apple")
//! Debug.Print pos  ' Prints: 14 (last apple)
//!
//! ' Example 2: Get file extension
//! Dim fileName As String
//! Dim pos As Long
//! Dim extension As String
//! fileName = "document.backup.txt"
//! pos = InStrRev(fileName, ".")
//! extension = Mid$(fileName, pos)
//! Debug.Print extension  ' Prints: .txt
//!
//! ' Example 3: Extract filename from path
//! Dim fullPath As String
//! Dim pos As Long
//! Dim fileName As String
//! fullPath = "C:\Projects\MyApp\MainForm.frm"
//! pos = InStrRev(fullPath, "\")
//! fileName = Mid$(fullPath, pos + 1)
//! Debug.Print fileName  ' Prints: MainForm.frm
//!
//! ' Example 4: Search with start position
//! Dim text As String
//! Dim pos As Long
//! text = "one,two,three,four"
//! pos = InStrRev(text, ",", 10)  ' Search up to position 10
//! Debug.Print pos  ' Prints: 8 (comma after "two")
//! ```
//!
//! ## Common Patterns
//!
//! ```vb
//! ' Pattern 1: Get file extension
//! Function GetFileExtension(fileName As String) As String
//!     Dim pos As Long
//!     
//!     pos = InStrRev(fileName, ".")
//!     If pos > 0 Then
//!         GetFileExtension = Mid$(fileName, pos + 1)
//!     Else
//!         GetFileExtension = ""
//!     End If
//! End Function
//!
//! ' Pattern 2: Get filename from path
//! Function GetFileNameFromPath(fullPath As String) As String
//!     Dim pos As Long
//!     
//!     pos = InStrRev(fullPath, "\")
//!     If pos = 0 Then pos = InStrRev(fullPath, "/")
//!     
//!     If pos > 0 Then
//!         GetFileNameFromPath = Mid$(fullPath, pos + 1)
//!     Else
//!         GetFileNameFromPath = fullPath
//!     End If
//! End Function
//!
//! ' Pattern 3: Get directory from path
//! Function GetDirectoryFromPath(fullPath As String) As String
//!     Dim pos As Long
//!     
//!     pos = InStrRev(fullPath, "\")
//!     If pos = 0 Then pos = InStrRev(fullPath, "/")
//!     
//!     If pos > 0 Then
//!         GetDirectoryFromPath = Left$(fullPath, pos - 1)
//!     Else
//!         GetDirectoryFromPath = ""
//!     End If
//! End Function
//!
//! ' Pattern 4: Remove file extension
//! Function RemoveFileExtension(fileName As String) As String
//!     Dim pos As Long
//!     
//!     pos = InStrRev(fileName, ".")
//!     If pos > 0 Then
//!         RemoveFileExtension = Left$(fileName, pos - 1)
//!     Else
//!         RemoveFileExtension = fileName
//!     End If
//! End Function
//!
//! ' Pattern 5: Get last word
//! Function GetLastWord(text As String) As String
//!     Dim pos As Long
//!     
//!     text = Trim$(text)
//!     pos = InStrRev(text, " ")
//!     
//!     If pos > 0 Then
//!         GetLastWord = Mid$(text, pos + 1)
//!     Else
//!         GetLastWord = text
//!     End If
//! End Function
//!
//! ' Pattern 6: Truncate at last delimiter
//! Function TruncateAtLastDelimiter(text As String, delimiter As String) As String
//!     Dim pos As Long
//!     
//!     pos = InStrRev(text, delimiter)
//!     If pos > 0 Then
//!         TruncateAtLastDelimiter = Left$(text, pos - 1)
//!     Else
//!         TruncateAtLastDelimiter = text
//!     End If
//! End Function
//!
//! ' Pattern 7: Get parent directory
//! Function GetParentDirectory(path As String) As String
//!     Dim pos As Long
//!     
//!     ' Remove trailing slash if present
//!     If Right$(path, 1) = "\" Then
//!         path = Left$(path, Len(path) - 1)
//!     End If
//!     
//!     pos = InStrRev(path, "\")
//!     If pos > 0 Then
//!         GetParentDirectory = Left$(path, pos - 1)
//!     Else
//!         GetParentDirectory = ""
//!     End If
//! End Function
//!
//! ' Pattern 8: Extract domain without subdomain
//! Function GetRootDomain(url As String) As String
//!     Dim domain As String
//!     Dim pos As Long
//!     Dim dotPos As Long
//!     
//!     ' Extract domain first (simplified)
//!     pos = InStr(url, "://")
//!     If pos > 0 Then
//!         domain = Mid$(url, pos + 3)
//!     Else
//!         domain = url
//!     End If
//!     
//!     ' Remove path
//!     pos = InStr(domain, "/")
//!     If pos > 0 Then
//!         domain = Left$(domain, pos - 1)
//!     End If
//!     
//!     ' Find last two parts (domain.tld)
//!     dotPos = InStrRev(domain, ".")
//!     If dotPos > 0 Then
//!         ' Find second-to-last dot
//!         pos = InStrRev(domain, ".", dotPos - 1)
//!         If pos > 0 Then
//!             GetRootDomain = Mid$(domain, pos + 1)
//!         Else
//!             GetRootDomain = domain
//!         End If
//!     Else
//!         GetRootDomain = domain
//!     End If
//! End Function
//!
//! ' Pattern 9: Replace last occurrence
//! Function ReplaceLastOccurrence(text As String, findText As String, replaceText As String) As String
//!     Dim pos As Long
//!     
//!     pos = InStrRev(text, findText)
//!     If pos > 0 Then
//!         ReplaceLastOccurrence = Left$(text, pos - 1) & replaceText & _
//!                                 Mid$(text, pos + Len(findText))
//!     Else
//!         ReplaceLastOccurrence = text
//!     End If
//! End Function
//!
//! ' Pattern 10: Get text before last occurrence
//! Function GetBeforeLastOccurrence(text As String, delimiter As String) As String
//!     Dim pos As Long
//!     
//!     pos = InStrRev(text, delimiter)
//!     If pos > 0 Then
//!         GetBeforeLastOccurrence = Left$(text, pos - 1)
//!     Else
//!         GetBeforeLastOccurrence = text
//!     End If
//! End Function
//! ```
//!
//! ## Advanced Usage Examples
//!
//! ```vb
//! ' Example 1: Path utilities class
//! Public Class PathUtils
//!     Public Function GetFileName(fullPath As String) As String
//!         Dim pos As Long
//!         pos = InStrRev(fullPath, "\")
//!         If pos = 0 Then pos = InStrRev(fullPath, "/")
//!         
//!         If pos > 0 Then
//!             GetFileName = Mid$(fullPath, pos + 1)
//!         Else
//!             GetFileName = fullPath
//!         End If
//!     End Function
//!     
//!     Public Function GetFileNameWithoutExtension(fullPath As String) As String
//!         Dim fileName As String
//!         Dim pos As Long
//!         
//!         fileName = GetFileName(fullPath)
//!         pos = InStrRev(fileName, ".")
//!         
//!         If pos > 0 Then
//!             GetFileNameWithoutExtension = Left$(fileName, pos - 1)
//!         Else
//!             GetFileNameWithoutExtension = fileName
//!         End If
//!     End Function
//!     
//!     Public Function GetExtension(fullPath As String) As String
//!         Dim pos As Long
//!         pos = InStrRev(fullPath, ".")
//!         
//!         If pos > 0 Then
//!             GetExtension = Mid$(fullPath, pos + 1)
//!         Else
//!             GetExtension = ""
//!         End If
//!     End Function
//!     
//!     Public Function GetDirectory(fullPath As String) As String
//!         Dim pos As Long
//!         pos = InStrRev(fullPath, "\")
//!         If pos = 0 Then pos = InStrRev(fullPath, "/")
//!         
//!         If pos > 0 Then
//!             GetDirectory = Left$(fullPath, pos - 1)
//!         Else
//!             GetDirectory = ""
//!         End If
//!     End Function
//!     
//!     Public Function ChangeExtension(fullPath As String, newExtension As String) As String
//!         Dim pos As Long
//!         pos = InStrRev(fullPath, ".")
//!         
//!         If pos > 0 Then
//!             ChangeExtension = Left$(fullPath, pos - 1) & "." & newExtension
//!         Else
//!             ChangeExtension = fullPath & "." & newExtension
//!         End If
//!     End Function
//! End Class
//!
//! ' Example 2: URL parser
//! Public Class URLParser
//!     Private m_url As String
//!     
//!     Public Sub Parse(url As String)
//!         m_url = url
//!     End Sub
//!     
//!     Public Function GetProtocol() As String
//!         Dim pos As Long
//!         pos = InStr(m_url, "://")
//!         
//!         If pos > 0 Then
//!             GetProtocol = Left$(m_url, pos - 1)
//!         Else
//!             GetProtocol = ""
//!         End If
//!     End Function
//!     
//!     Public Function GetDomain() As String
//!         Dim startPos As Long
//!         Dim endPos As Long
//!         
//!         startPos = InStr(m_url, "://")
//!         If startPos > 0 Then
//!             startPos = startPos + 3
//!         Else
//!             startPos = 1
//!         End If
//!         
//!         endPos = InStr(startPos, m_url, "/")
//!         If endPos > 0 Then
//!             GetDomain = Mid$(m_url, startPos, endPos - startPos)
//!         Else
//!             GetDomain = Mid$(m_url, startPos)
//!         End If
//!     End Function
//!     
//!     Public Function GetPath() As String
//!         Dim startPos As Long
//!         Dim endPos As Long
//!         
//!         startPos = InStr(m_url, "://")
//!         If startPos > 0 Then
//!             startPos = InStr(startPos + 3, m_url, "/")
//!         Else
//!             startPos = InStr(m_url, "/")
//!         End If
//!         
//!         If startPos > 0 Then
//!             endPos = InStr(startPos, m_url, "?")
//!             If endPos > 0 Then
//!                 GetPath = Mid$(m_url, startPos, endPos - startPos)
//!             Else
//!                 GetPath = Mid$(m_url, startPos)
//!             End If
//!         Else
//!             GetPath = ""
//!         End If
//!     End Function
//!     
//!     Public Function GetLastPathSegment() As String
//!         Dim path As String
//!         Dim pos As Long
//!         
//!         path = GetPath()
//!         
//!         ' Remove trailing slash
//!         If Right$(path, 1) = "/" Then
//!             path = Left$(path, Len(path) - 1)
//!         End If
//!         
//!         pos = InStrRev(path, "/")
//!         If pos > 0 Then
//!             GetLastPathSegment = Mid$(path, pos + 1)
//!         Else
//!             GetLastPathSegment = path
//!         End If
//!     End Function
//! End Class
//!
//! ' Example 3: String truncation utility
//! Function TruncateWithEllipsis(text As String, maxLength As Long, _
//!                               Optional breakAtWord As Boolean = True) As String
//!     If Len(text) <= maxLength Then
//!         TruncateWithEllipsis = text
//!         Exit Function
//!     End If
//!     
//!     If breakAtWord Then
//!         Dim truncated As String
//!         Dim pos As Long
//!         
//!         truncated = Left$(text, maxLength - 3)
//!         pos = InStrRev(truncated, " ")
//!         
//!         If pos > 0 Then
//!             TruncateWithEllipsis = Left$(truncated, pos - 1) & "..."
//!         Else
//!             TruncateWithEllipsis = truncated & "..."
//!         End If
//!     Else
//!         TruncateWithEllipsis = Left$(text, maxLength - 3) & "..."
//!     End If
//! End Function
//!
//! ' Example 4: Email domain extractor
//! Function GetEmailDomain(email As String) As String
//!     Dim atPos As Long
//!     
//!     atPos = InStrRev(email, "@")
//!     If atPos > 0 And atPos < Len(email) Then
//!         GetEmailDomain = Mid$(email, atPos + 1)
//!     Else
//!         GetEmailDomain = ""
//!     End If
//! End Function
//! ```
//!
//! ## Error Handling
//!
//! The `InStrRev` function can raise errors or return `Null`:
//!
//! - **Type Mismatch (Error 13)**: If arguments are not string-compatible or numeric where expected
//! - **Invalid use of Null (Error 94)**: If `stringcheck` or `stringmatch` is `Null` and result is assigned to non-Variant
//! - **Invalid procedure call (Error 5)**: If start is 0 or negative (except -1)
//!
//! ```vb
//! On Error GoTo ErrorHandler
//! Dim pos As Long
//! Dim fileName As String
//!
//! fileName = "document.txt"
//! pos = InStrRev(fileName, ".")
//!
//! If pos > 0 Then
//!     Debug.Print "Extension: " & Mid$(fileName, pos + 1)
//! Else
//!     Debug.Print "No extension found"
//! End If
//! Exit Sub
//!
//! ErrorHandler:
//!     MsgBox "Error in InStrRev: " & Err.Description, vbCritical
//! ```
//!
//! ## Performance Considerations
//!
//! - **Binary vs Text Compare**: Binary comparison (vbBinaryCompare) is faster than text comparison
//! - **String Length**: Performance degrades with very long strings
//! - **Start Position**: Using specific start position can improve performance for partial searches
//! - **Reverse Search**: More efficient than repeated `InStr` calls to find last occurrence
//! - **Alternative**: For complex pattern matching, consider regular expressions (VBScript.RegExp)
//!
//! ## Best Practices
//!
//! 1. **Test for Found**: Always check if result > 0 before using position
//! 2. **Default Start**: Use -1 or omit start parameter to search from end
//! 3. **Path Operations**: Prefer `InStrRev` for file path operations (finding last separator)
//! 4. **Extension Extraction**: `InStrRev` is ideal for getting file extensions
//! 5. **Null Handling**: Use `Variant` for result if strings might be `Null`
//! 6. **Parameter Order**: Remember `InStrRev` has different parameter order than `InStr`
//! 7. **Return Value**: Position is still counted from beginning (not from end)
//!
//! ## Comparison with Other Functions
//!
//! | Function | Purpose | Search Direction | Start Parameter |
//! |----------|---------|------------------|-----------------|
//! | `InStr` | Find substring | Left to right | Optional, before strings |
//! | `InStrRev` | Find substring | Right to left | Optional, after strings |
//! | `Like` | Pattern matching | N/A | N/A |
//! | `StrComp` | Compare strings | N/A | N/A |
//!
//! ## Platform and Version Notes
//!
//! - Available in VB6 and VBA (not in earlier VB versions)
//! - Returns `Long` (32-bit integer), not `Integer`
//! - 1-based indexing (first character is position 1)
//! - Position returned is from the start of string, not from the end
//! - Default start is -1 (search from end)
//! - Parameter order differs from `InStr` (strings first, then start)
//! - Maximum string length is approximately 2GB
//!
//! ## Limitations
//!
//! - Finds only one occurrence (the last one)
//! - No built-in regex or wildcard support
//! - Case-insensitive search (`vbTextCompare`) is slower
//! - Different parameter order than `InStr` can cause confusion
//! - Cannot search for multiple substrings in single call
//! - Performance can degrade with very long strings
//!
//! ## Related Functions
//!
//! - `InStr`: Search from beginning of string
//! - `Mid`, `Left`, `Right`: Extract substrings
//! - `Replace`: Find and replace text
//! - `Split`: Split string into array
//! - `StrReverse`: Reverse a string

use crate::{
    error::{err_number, VBError, VBResult},
    value::{VBLong, VBString},
};

/// Returns the 1-based position of the last occurrence of `stringmatch` within
/// `stringcheck`, searching backward from `start`.
///
/// `start` defaults to -1, meaning the search begins at the last character
/// position. An empty `stringcheck` yields 0; an empty `stringmatch` yields
/// `start`; a missing substring yields 0. When `start` is greater than the
/// length of `stringcheck`, the result is 0. When `compare` is `Some(1)`
/// (`vbTextCompare`) the search is case-insensitive; all other values
/// (including `None`) use binary (case-sensitive) comparison.
///
/// # Errors
///
/// Returns error 5 (`Invalid procedure call or argument`) when `start` is 0 or
/// less than -1.
pub fn instrrev(
    stringcheck: &VBString,
    stringmatch: &VBString,
    start: Option<&VBLong>,
    compare: Option<&VBLong>,
) -> VBResult<VBLong> {
    let len = stringcheck.as_str().chars().count() as i32;
    let mut start = start.map_or(-1, |value| value.as_i32());
    if start == 0 || start < -1 {
        return Err(VBError::with_description(
            err_number::INVALID_PROCEDURE_CALL,
            "Invalid start position",
        ));
    }
    if start == -1 {
        start = len;
    }
    if len == 0 || start > len {
        return Ok(VBLong::from(0));
    }
    if stringmatch.as_str().is_empty() {
        return Ok(VBLong::from(start));
    }

    let text_compare = compare.is_some_and(|value| value.as_i32() == 1);
    let char_eq = |a: char, b: char| {
        if text_compare {
            a.to_lowercase().eq(b.to_lowercase())
        } else {
            a == b
        }
    };

    let chars1: Vec<char> = stringcheck.as_str().chars().collect();
    let chars2: Vec<char> = stringmatch.as_str().chars().collect();
    let start0 = start as usize;

    if chars2.len() > start0 {
        return Ok(VBLong::from(0));
    }
    for i in (0..=start0 - chars2.len()).rev() {
        let matches = chars2
            .iter()
            .enumerate()
            .all(|(j, &c)| char_eq(chars1[i + j], c));
        if matches {
            return Ok(VBLong::from((i + 1) as i32));
        }
    }
    Ok(VBLong::from(0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{error::err_number, value::VBString};

    #[test]
    fn finds_last_occurrence() {
        assert_eq!(
            instrrev(
                &VBString::from("apple,banana,apple,orange"),
                &VBString::from("apple"),
                None,
                None
            )
            .unwrap(),
            VBLong::from(14)
        );
        assert_eq!(
            instrrev(
                &VBString::from("Hello World"),
                &VBString::from("o"),
                None,
                None
            )
            .unwrap(),
            VBLong::from(8)
        );
    }

    #[test]
    fn respects_start_position() {
        assert_eq!(
            instrrev(
                &VBString::from("one,two,three,four"),
                &VBString::from(","),
                Some(&VBLong::from(10)),
                None
            )
            .unwrap(),
            VBLong::from(8)
        );
        assert_eq!(
            instrrev(
                &VBString::from("the quick brown fox jumps over the lazy dog"),
                &VBString::from("the"),
                Some(&VBLong::from(16)),
                None
            )
            .unwrap(),
            VBLong::from(1)
        );
    }

    #[test]
    fn missing_substring_returns_zero() {
        assert_eq!(
            instrrev(&VBString::from("Hello"), &VBString::from("xyz"), None, None).unwrap(),
            VBLong::from(0)
        );
    }

    #[test]
    fn empty_stringcheck_returns_zero() {
        assert_eq!(
            instrrev(&VBString::from(""), &VBString::from("x"), None, None).unwrap(),
            VBLong::from(0)
        );
    }

    #[test]
    fn empty_stringmatch_returns_start() {
        assert_eq!(
            instrrev(&VBString::from("Hello"), &VBString::from(""), None, None).unwrap(),
            VBLong::from(5)
        );
        assert_eq!(
            instrrev(
                &VBString::from("Hello"),
                &VBString::from(""),
                Some(&VBLong::from(3)),
                None
            )
            .unwrap(),
            VBLong::from(3)
        );
    }

    #[test]
    fn start_beyond_string_returns_zero() {
        assert_eq!(
            instrrev(
                &VBString::from("Hello"),
                &VBString::from("o"),
                Some(&VBLong::from(10)),
                None
            )
            .unwrap(),
            VBLong::from(0)
        );
    }

    #[test]
    fn rejects_invalid_start() {
        for start in [0, -2, -100] {
            assert_eq!(
                instrrev(
                    &VBString::from("Hello"),
                    &VBString::from("o"),
                    Some(&VBLong::from(start)),
                    None
                )
                .unwrap_err()
                .number,
                err_number::INVALID_PROCEDURE_CALL
            );
        }
    }

    #[test]
    fn text_compare_ignores_case() {
        assert_eq!(
            instrrev(
                &VBString::from("Hello"),
                &VBString::from("HELLO"),
                None,
                Some(&VBLong::from(1))
            )
            .unwrap(),
            VBLong::from(1)
        );
        assert_eq!(
            instrrev(
                &VBString::from("Hello"),
                &VBString::from("hello"),
                None,
                None
            )
            .unwrap(),
            VBLong::from(0)
        );
    }
}
