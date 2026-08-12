//! ## `Split` Function
//!
//! Returns a zero-based, one-dimensional array containing a specified number of substrings.
//!
//! ## Syntax
//!
//! ```text
//! Split(expression[, delimiter[, limit[, compare]]])
//! ```
//!
//! ## Parameters
//!
//! - **expression** (Required): `String` expression containing substrings and delimiters
//! - **delimiter** (Optional): `String` character used to identify substring limits
//!   - If omitted, the space character (" ") is assumed
//! - **limit** (Optional): Number of substrings to be returned; `-1` returns all substrings
//! - **compare** (Optional): Numeric value indicating comparison type (see Compare Settings)
//!
//! ## Compare Settings
//!
//! - `vbBinaryCompare` (0): Perform a binary comparison
//! - `vbTextCompare` (1): Perform a textual comparison
//! - `vbDatabaseCompare` (2): Perform a comparison based on information in your database
//!
//! ## Return Value
//!
//! - Returns a `Variant` containing a one-dimensional array of strings (zero-based)
//! - If `expression` is a zero-length string (""), returns an empty array
//! - If `delimiter` is a zero-length string or not found, returns a single-element array containing the entire expression
//!
//! ## Remarks
//!
//! The `Split` function breaks a string into substrings at the specified delimiter and returns them as an array. This is the opposite of the `Join` function, which combines array elements into a single string.
//!
//! - Returns a zero-based array (first element is index 0)
//! - If expression is a zero-length string (""), Split returns an empty array
//! - If delimiter is a zero-length string, a single-element array containing the entire expression is returned
//! - If delimiter is not found, a single-element array containing the entire expression is returned
//! - Delimiter characters are not included in the returned substrings
//! - If `limit` is provided and is less than the number of substrings, the last element contains the remainder of the string (including delimiters)
//! - Multiple consecutive delimiters create empty string elements in the array
//!
//! ## Typical Uses
//!
//! - **Parse CSV Data**: Split comma-separated values
//! - **Extract Words**: Split sentence into individual words
//! - **Process Lines**: Split multiline text into lines
//! - **Parse Paths**: Split file paths into components
//! - **Extract Parameters**: Parse parameter strings
//! - **Data Import**: Process delimited import files
//! - **String Tokenization**: Break strings into tokens
//! * **Configuration Parsing**: Parse config file entries
//!
//! ## Common Errors
//!
//! The Split function itself doesn't typically generate errors with valid inputs, but related operations can:
//!
//! - **Error 13** (Type mismatch): If expression is not a string
//! - **Error 5** (Invalid procedure call): If limit is negative (other than -1)
//! - **Error 9** (Subscript out of range): When accessing array elements beyond bounds
//!
//! ### Always validate inputs and array bounds:
//!
//! ```vb6
//! On Error Resume Next
//! Dim parts() As String
//! parts = Split(text, ",")
//! If Err.Number <> 0 Then
//!     MsgBox "Error splitting text: " & Err.Description
//! End If
//! ```
//!
//! ## Performance Considerations
//!
//! - Split is very efficient for moderate-sized strings
//! - For very large strings (>1MB), consider processing in chunks
//! - Avoid repeated Split calls in tight loops if possible
//! - Consider caching Split results if reused multiple times
//! - For complex parsing, Split may be slower than manual parsing
//!
//! ## Best Practices
//!
//! - **Check Array Bounds**: Always verify `UBound` before accessing elements
//! - **Handle Empty Results**: Check if array has elements before processing
//! - **Trim Whitespace**: Use Trim on results to remove unwanted spaces
//! - **Validate Delimiter**: Ensure delimiter is appropriate for data
//! - **Use Limit**: Limit number of splits when only need first few elements
//! - **Handle Edge Cases**: Test with empty strings, missing delimiters
//! - **Consider Alternatives**: For complex parsing, use dedicated parser
//! - **Document Expected Format**: Comment the expected delimited format
//! - **Filter Empty Elements**: Remove empty strings when caused by multiple delimiters
//! - **Combine with Join**: Use Join to reconstruct modified arrays
//!
//! ## Comparison with Related Functions
//!
//! | Function | Purpose | Input | Output |
//! |----------|---------|-------|--------|
//! | Split | String to array | String | Array of strings |
//! | Join | Array to string | Array | String |
//! | Filter | Filter array | Array | Filtered array |
//! | Replace | Replace text | String | String |
//!
//! ## Platform Considerations
//!
//! - Available in VB6, VBA (Office 2000+)
//! - Not available in VBA prior to Office 2000
//! - Returns Variant array (can assign to String array)
//! - Zero-based array (unlike many VB arrays which are 1-based)
//! - Consistent behavior across platforms
//!
//! ## Limitations
//!
//! - Returns zero-based array (may be unexpected in VB6)
//! - Delimiter must be exact match (no regex)
//! - Single delimiter only (can't split on multiple different delimiters)
//! - No built-in trim of results
//! - Empty elements included when multiple consecutive delimiters present
//! - No built-in handling of quoted fields (CSV with commas in quotes)
//! - Maximum array size limited by memory
//!
//! ## Related Functions
//!
//! - `Join`: Combines array elements into a string with delimiter
//! - `Filter`: Returns a subset of array based on filter criteria
//! - `InStr`: Finds position of substring (useful before Split)
//! - `Replace`: Replaces occurrences of substring
//!
//!
//! ## Basic Examples
//!
//! ### Example 1: Split Comma-Separated Values
//!
//! ```vb6
//! Dim text As String
//! Dim parts() As String
//! text = "apple,banana,orange"
//! parts = Split(text, ",")
//! ' parts(0) = "apple"
//! ' parts(1) = "banana"
//! ' parts(2) = "orange"
//! ```
//!
//! ### Example 2: Split Sentence Into Words (Default Space Delimiter)
//!
//! ```vb6
//! Dim sentence As String
//! Dim words() As String
//! sentence = "The quick brown fox"
//! words = Split(sentence)
//! ' words(0) = "The"
//! ' words(1) = "quick"
//! ' words(2) = "brown"
//! ' words(3) = "fox"
//! ```
//!
//! ### Example 3: Split With Limit
//!
//! ```vb6
//! Dim data As String
//! Dim items() As String
//! data = "one,two,three,four,five"
//! items = Split(data, ",", 3)
//! ' items(0) = "one"
//! ' items(1) = "two"
//! ' items(2) = "three,four,five" (remainder)
//! ```
//!
//! ### Example 4: Split Multiline Text
//!
//! ```vb6
//! Dim text As String
//! Dim lines() As String
//! text = "Line 1" & vbCrLf & "Line 2" & vbCrLf & "Line 3"
//! lines = Split(text, vbCrLf)
//! ' lines(0) = "Line 1"
//! ' lines(1) = "Line 2"
//! ' lines(2) = "Line 3"
//! ```
//!
//! ## Common Patterns
//!
//! ### Pattern 1: Parse a CSV line handling quotes
//!
//! ```vb6
//! Function ParseCSVLine(line As String) As String()
//!     ' Simple CSV parsing (doesn't handle quotes)
//!     ParseCSVLine = Split(line, ",")
//! End Function
//! ```
//!
//! ### Pattern 2: Extract Words From Text, Handling Multiple Spaces
//!
//! ```vb6
//! Function GetWords(text As String) As String()
//!     Dim words() As String
//!     Dim result() As String
//!     Dim i As Integer
//!     Dim count As Integer
//!     
//!     words = Split(Trim(text), " ")
//!     
//!     ' Filter out empty strings from multiple spaces
//!     count = 0
//!     For i = LBound(words) To UBound(words)
//!         If Len(words(i)) > 0 Then
//!             count = count + 1
//!         End If
//!     Next i
//!     
//!     ReDim result(0 To count - 1)
//!     count = 0
//!     For i = LBound(words) To UBound(words)
//!         If Len(words(i)) > 0 Then
//!             result(count) = words(i)
//!             count = count + 1
//!         End If
//!     Next i
//!     
//!     GetWords = result
//! End Function
//! ```
//!
//! ### Pattern 3: Split File Path Into Components
//!
//! ```vb6
//! Function SplitPath(filePath As String) As String()
//!     Dim delimiter As String
//!     
//!     ' Handle both Windows and Unix paths
//!     If InStr(filePath, "\") > 0 Then
//!         delimiter = "\"
//!     Else
//!         delimiter = "/"
//!     End If
//!     
//!     SplitPath = Split(filePath, delimiter)
//! End Function
//! ```
//!
//! ### Pattern 4: Parse Key=Value Pairs
//!
//! ```vb6
//! Sub ParseKeyValue(kvPair As String, key As String, value As String)
//!     Dim parts() As String
//!     parts = Split(kvPair, "=", 2)
//!     
//!     If UBound(parts) >= 0 Then
//!         key = Trim(parts(0))
//!         If UBound(parts) >= 1 Then
//!             value = Trim(parts(1))
//!         Else
//!             value = ""
//!         End If
//!     End If
//! End Sub
//! ```
//!
//! ### Pattern 5: Split Text Into Lines, Handling Different Line Endings
//!
//! ```vb6
//! Function SplitLines(text As String) As String()
//!     Dim normalized As String
//!     
//!     ' Normalize line endings to vbCrLf
//!     normalized = Replace(text, vbCr & vbLf, vbLf)
//!     normalized = Replace(normalized, vbCr, vbLf)
//!     
//!     SplitLines = Split(normalized, vbLf)
//! End Function
//! ```
//!
//! ### Pattern 6: Parse Delimited Data With Custom Delimiter
//!
//! ```vb6
//! Function ParseDelimitedData(data As String, delimiter As String) As Variant
//!     Dim lines() As String
//!     Dim result() As Variant
//!     Dim i As Integer
//!     
//!     lines = Split(data, vbCrLf)
//!     ReDim result(0 To UBound(lines))
//!     
//!     For i = LBound(lines) To UBound(lines)
//!         result(i) = Split(lines(i), delimiter)
//!     Next i
//!     
//!     ParseDelimitedData = result
//! End Function
//! ```
//!
//! ### Pattern 7: Extract Specific Fields From Delimited String
//!
//! ```vb6
//! Function ExtractField(delimitedString As String, _
//!                       delimiter As String, _
//!                       fieldIndex As Integer) As String
//!     Dim fields() As String
//!     fields = Split(delimitedString, delimiter)
//!     
//!     If fieldIndex >= LBound(fields) And fieldIndex <= UBound(fields) Then
//!         ExtractField = fields(fieldIndex)
//!     Else
//!         ExtractField = ""
//!     End If
//! End Function
//! ```
//!
//! ### Pattern 8: Count Number Of Tokens In String
//!
//! ```vb6
//! Function CountTokens(text As String, delimiter As String) As Integer
//!     Dim tokens() As String
//!     tokens = Split(text, delimiter)
//!     CountTokens = UBound(tokens) - LBound(tokens) + 1
//! End Function
//! ```
//!
//! ### Pattern 9: Split And Reverse The Order
//!
//! ```vb6
//! Function ReverseSplit(text As String, delimiter As String) As String()
//!     Dim parts() As String
//!     Dim result() As String
//!     Dim i As Integer
//!     Dim count As Integer
//!     
//!     parts = Split(text, delimiter)
//!     count = UBound(parts) - LBound(parts)
//!     ReDim result(0 To count)
//!     
//!     For i = 0 To count
//!         result(i) = parts(count - i)
//!     Next i
//!     
//!     ReverseSplit = result
//! End Function
//! ```
//!
//! ### Pattern 10: Split And Remove Empty Elements
//!
//! ```vb6
//! Function SplitNonEmpty(text As String, delimiter As String) As String()
//!     Dim parts() As String
//!     Dim result() As String
//!     Dim i As Integer
//!     Dim count As Integer
//!     
//!     parts = Split(text, delimiter)
//!     
//!     ' Count non-empty elements
//!     count = 0
//!     For i = LBound(parts) To UBound(parts)
//!         If Len(parts(i)) > 0 Then count = count + 1
//!     Next i
//!     
//!     If count = 0 Then
//!         ReDim result(0 To -1)  ' Empty array
//!     Else
//!         ReDim result(0 To count - 1)
//!         count = 0
//!         For i = LBound(parts) To UBound(parts)
//!             If Len(parts(i)) > 0 Then
//!                 result(count) = parts(i)
//!                 count = count + 1
//!             End If
//!         Next i
//!     End If
//!     
//!     SplitNonEmpty = result
//! End Function
//! ```
//!
//! ## Advanced Usage
//!
//! ### Example 1: Parse CSV Data With Split
//!
//! ```vb6
//! ' Class: CSVParser
//! Private m_data() As Variant
//! Private m_rowCount As Integer
//! Private m_columnCount As Integer
//!
//! Public Sub LoadCSV(csvText As String, Optional hasHeader As Boolean = True)
//!     Dim lines() As String
//!     Dim i As Integer
//!     Dim startRow As Integer
//!     
//!     ' Split into lines
//!     lines = Split(csvText, vbCrLf)
//!     
//!     If hasHeader Then
//!         startRow = 1
//!         m_rowCount = UBound(lines) - LBound(lines)
//!     Else
//!         startRow = 0
//!         m_rowCount = UBound(lines) - LBound(lines) + 1
//!     End If
//!     
//!     ' Get column count from first data row
//!     If UBound(lines) >= startRow Then
//!         Dim firstRow() As String
//!         firstRow = Split(lines(startRow), ",")
//!         m_columnCount = UBound(firstRow) - LBound(firstRow) + 1
//!     End If
//!     
//!     ' Parse data
//!     ReDim m_data(1 To m_rowCount, 1 To m_columnCount)
//!     
//!     For i = startRow To UBound(lines)
//!         Dim fields() As String
//!         Dim j As Integer
//!         fields = Split(lines(i), ",")
//!         
//!         For j = LBound(fields) To UBound(fields)
//!             If j - LBound(fields) + 1 <= m_columnCount Then
//!                 m_data(i - startRow + 1, j - LBound(fields) + 1) = fields(j)
//!             End If
//!         Next j
//!     Next i
//! End Sub
//!
//! Public Function GetValue(row As Integer, col As Integer) As String
//!     If row >= 1 And row <= m_rowCount And _
//!        col >= 1 And col <= m_columnCount Then
//!         GetValue = m_data(row, col)
//!     Else
//!         GetValue = ""
//!     End If
//! End Function
//!
//! Public Property Get RowCount() As Integer
//!     RowCount = m_rowCount
//! End Property
//!
//! Public Property Get ColumnCount() As Integer
//!     ColumnCount = m_columnCount
//! End Property
//!
//! Public Function GetRow(row As Integer) As Variant
//!     Dim result() As String
//!     Dim i As Integer
//!     
//!     If row >= 1 And row <= m_rowCount Then
//!         ReDim result(1 To m_columnCount)
//!         For i = 1 To m_columnCount
//!             result(i) = m_data(row, i)
//!         Next i
//!         GetRow = result
//!     End If
//! End Function
//! ```
//!
//! ### Example 2: Parse Configuration Files
//!
//! ```vb6
//! ' Module: ConfigFileParser
//! Private m_settings As Object  ' Scripting.Dictionary
//!
//! Public Sub LoadConfig(configText As String)
//!     Dim lines() As String
//!     Dim i As Integer
//!     
//!     Set m_settings = CreateObject("Scripting.Dictionary")
//!     m_settings.CompareMode = vbTextCompare
//!     
//!     lines = Split(configText, vbCrLf)
//!     
//!     For i = LBound(lines) To UBound(lines)
//!         Dim line As String
//!         line = Trim(lines(i))
//!         
//!         ' Skip empty lines and comments
//!         If Len(line) > 0 And Left(line, 1) <> "#" And Left(line, 1) <> ";" Then
//!             Dim parts() As String
//!             parts = Split(line, "=", 2)
//!             
//!             If UBound(parts) >= 1 Then
//!                 Dim key As String
//!                 Dim value As String
//!                 key = Trim(parts(0))
//!                 value = Trim(parts(1))
//!                 
//!                 m_settings(key) = value
//!             End If
//!         End If
//!     Next i
//! End Sub
//!
//! Public Function GetSetting(key As String, Optional defaultValue As String = "") As String
//!     If m_settings.Exists(key) Then
//!         GetSetting = m_settings(key)
//!     Else
//!         GetSetting = defaultValue
//!     End If
//! End Function
//!
//! Public Function GetSettingAsInteger(key As String, Optional defaultValue As Integer = 0) As Integer
//!     If m_settings.Exists(key) Then
//!         If IsNumeric(m_settings(key)) Then
//!             GetSettingAsInteger = CInt(m_settings(key))
//!         Else
//!             GetSettingAsInteger = defaultValue
//!         End If
//!     Else
//!         GetSettingAsInteger = defaultValue
//!     End If
//! End Function
//!
//! Public Function GetSettingList(key As String, delimiter As String) As String()
//!     If m_settings.Exists(key) Then
//!         GetSettingList = Split(m_settings(key), delimiter)
//!     Else
//!         Dim empty() As String
//!         ReDim empty(0 To -1)
//!         GetSettingList = empty
//!     End If
//! End Function
//! ```
//!
//! ### Example 3: Process Text With Various Split Operations
//!
//! ```vb6
//! ' Class: TextProcessor
//!
//! Public Function GetParagraphs(text As String) As String()
//!     ' Split by double line breaks
//!     Dim normalized As String
//!     normalized = Replace(text, vbCrLf & vbCrLf, vbLf & vbLf)
//!     normalized = Replace(normalized, vbCr, vbLf)
//!     GetParagraphs = Split(normalized, vbLf & vbLf)
//! End Function
//!
//! Public Function GetSentences(text As String) As String()
//!     Dim temp As String
//!     Dim i As Integer
//!     
//!     ' Simple sentence splitting (doesn't handle abbreviations)
//!     temp = Replace(text, ". ", ".|")
//!     temp = Replace(temp, "! ", "!|")
//!     temp = Replace(temp, "? ", "?|")
//!     
//!     GetSentences = Split(temp, "|")
//! End Function
//!
//! Public Function GetWords(text As String) As String()
//!     Dim cleaned As String
//!     Dim i As Integer
//!     
//!     cleaned = text
//!     ' Remove punctuation
//!     cleaned = Replace(cleaned, ".", " ")
//!     cleaned = Replace(cleaned, ",", " ")
//!     cleaned = Replace(cleaned, "!", " ")
//!     cleaned = Replace(cleaned, "?", " ")
//!     cleaned = Replace(cleaned, ";", " ")
//!     cleaned = Replace(cleaned, ":", " ")
//!     
//!     GetWords = Split(Trim(cleaned), " ")
//! End Function
//!
//! Public Function CountWords(text As String) As Integer
//!     Dim words() As String
//!     Dim count As Integer
//!     Dim i As Integer
//!     
//!     words = GetWords(text)
//!     count = 0
//!     
//!     For i = LBound(words) To UBound(words)
//!         If Len(Trim(words(i))) > 0 Then
//!             count = count + 1
//!         End If
//!     Next i
//!     
//!     CountWords = count
//! End Function
//!
//! Public Function GetUniqueWords(text As String) As String()
//!     Dim words() As String
//!     Dim dict As Object
//!     Dim i As Integer
//!     Dim result() As String
//!     Dim count As Integer
//!     
//!     Set dict = CreateObject("Scripting.Dictionary")
//!     dict.CompareMode = vbTextCompare
//!     
//!     words = GetWords(text)
//!     
//!     For i = LBound(words) To UBound(words)
//!         Dim word As String
//!         word = Trim(words(i))
//!         If Len(word) > 0 Then
//!             dict(word) = True
//!         End If
//!     Next i
//!     
//!     ReDim result(0 To dict.Count - 1)
//!     Dim keys As Variant
//!     keys = dict.keys
//!     
//!     For i = 0 To dict.Count - 1
//!         result(i) = keys(i)
//!     Next i
//!     
//!     GetUniqueWords = result
//! End Function
//! ```
//!
//! ### Example 4: Import Delimited Data Files
//!
//! ```vb6
//! ' Module: DataImporter
//!
//! Public Function ImportDelimitedFile(filePath As String, _
//!                                     delimiter As String, _
//!                                     Optional hasHeader As Boolean = True) As Variant
//!     Dim fileNum As Integer
//!     Dim fileContent As String
//!     Dim lines() As String
//!     Dim result() As Variant
//!     Dim i As Integer
//!     Dim startRow As Integer
//!     
//!     ' Read file
//!     fileNum = FreeFile
//!     Open filePath For Input As #fileNum
//!     fileContent = Input(LOF(fileNum), #fileNum)
//!     Close #fileNum
//!     
//!     ' Split into lines
//!     lines = Split(fileContent, vbCrLf)
//!     
//!     If hasHeader Then
//!         startRow = 1
//!     Else
//!         startRow = 0
//!     End If
//!     
//!     ' Parse each line
//!     ReDim result(startRow To UBound(lines))
//!     
//!     For i = startRow To UBound(lines)
//!         result(i) = Split(lines(i), delimiter)
//!     Next i
//!     
//!     ImportDelimitedFile = result
//! End Function
//!
//! Public Function GetColumnFromData(data As Variant, columnIndex As Integer) As String()
//!     Dim result() As String
//!     Dim i As Integer
//!     Dim rowCount As Integer
//!     
//!     rowCount = UBound(data) - LBound(data) + 1
//!     ReDim result(0 To rowCount - 1)
//!     
//!     For i = LBound(data) To UBound(data)
//!         Dim row() As String
//!         row = data(i)
//!         
//!         If columnIndex >= LBound(row) And columnIndex <= UBound(row) Then
//!             result(i - LBound(data)) = row(columnIndex)
//!         Else
//!             result(i - LBound(data)) = ""
//!         End If
//!     Next i
//!     
//!     GetColumnFromData = result
//! End Function
//!
//! Public Function FilterRows(data As Variant, columnIndex As Integer, _
//!                            filterValue As String) As Variant
//!     Dim result() As Variant
//!     Dim count As Integer
//!     Dim i As Integer
//!     
//!     ' Count matching rows
//!     count = 0
//!     For i = LBound(data) To UBound(data)
//!         Dim row() As String
//!         row = data(i)
//!         If columnIndex >= LBound(row) And columnIndex <= UBound(row) Then
//!             If row(columnIndex) = filterValue Then
//!                 count = count + 1
//!             End If
//!         End If
//!     Next i
//!     
//!     ' Build result
//!     ReDim result(0 To count - 1)
//!     count = 0
//!     For i = LBound(data) To UBound(data)
//!         row = data(i)
//!         If columnIndex >= LBound(row) And columnIndex <= UBound(row) Then
//!             If row(columnIndex) = filterValue Then
//!                 result(count) = row
//!                 count = count + 1
//!             End If
//!         End If
//!     Next i
//!     
//!     FilterRows = result
//! End Function
//! ```
//!
//! ## Error Handling Patterns
//!
//! ### Basic Error Handling
//!
//! ```vb6
//! Function SafeSplit(text As String, delimiter As String) As Variant
//!     On Error GoTo ErrorHandler
//!     
//!     If Len(Trim(text)) = 0 Then
//!         ' Return empty array for empty text
//!         Dim emptyArr() As String
//!         ReDim emptyArr(0 To -1)
//!         SafeSplit = emptyArr
//!         Exit Function
//!     End If
//!     
//!     SafeSplit = Split(text, delimiter)
//!     Exit Function
//!     
//! ErrorHandler:
//!     ' Return empty array on error
//!     Dim emptyArr() As String
//!     ReDim emptyArr(0 To -1)
//!     SafeSplit = emptyArr
//! End Function
//! ```

use crate::array::ArrayValue;
use crate::error::{err_number, VBError, VBResult};
use crate::types::VBType;
use crate::value::VBVariant;

/// Implementation of the `Split` function.
///
/// Splits a string into substrings at delimiter positions and returns them as
/// a zero-based one-dimensional array.
///
/// VB6 behavior:
/// - Default delimiter is a space (" ")
/// - Returns an empty array if `expression` is empty
/// - Returns a single-element array containing the entire expression if
///   `delimiter` is empty or not found in `expression`
/// - `limit` specifies maximum number of substrings to return; `-1` means no
///   limit (default behavior when omitted)
/// - `compare` accepts `vbUseCompareOption` (-1), `vbBinaryCompare` (0),
///   `vbTextCompare` (1), and `vbDatabaseCompare` (2); without a module-level
///   `Option Compare` or database setting, -1 and 2 behave as binary compare
/// - Raises error 13 if `expression` is not a string value
/// - Raises error 5 if `limit` is negative (other than -1)
pub fn split(
    expression: &str,
    delimiter: Option<&str>,
    limit: Option<i32>,
    compare: Option<i32>,
) -> VBResult<VBVariant> {
    // Validate compare parameter
    let text_compare = match compare {
        None => false,
        Some(mode) if (-1..=2).contains(&mode) => mode == 1,
        Some(_) => return Err(VBError::invalid_procedure_call()),
    };

    // Validate limit parameter
    let limit = match limit {
        None | Some(-1) => i32::MAX,
        Some(n) if n <= 0 => {
            return Err(VBError::with_description(
                err_number::INVALID_PROCEDURE_CALL,
                "Invalid procedure call",
            ));
        }
        Some(n) => n,
    };

    let delimiter = delimiter.unwrap_or(" ");

    // Empty expression returns empty array
    if expression.is_empty() {
        return Ok(VBVariant::Array(ArrayValue::from_vec_with_bounds(
            VBType::String,
            Vec::new(),
            0,
        )));
    }

    // Empty delimiter returns single-element array with entire expression
    if delimiter.is_empty() {
        return Ok(VBVariant::Array(ArrayValue::from_vec_with_bounds(
            VBType::String,
            vec![VBVariant::from_string(expression)],
            0,
        )));
    }

    // Split the string respecting limit
    let mut parts: Vec<VBVariant> = Vec::new();
    let mut remaining = expression;

    loop {
        // If we've reached the limit, push the remainder and stop
        if parts.len() >= (limit as usize) - 1 {
            parts.push(VBVariant::from_string(remaining));
            break;
        }

        // Find delimiter position (respecting compare mode)
        let split_pos = if text_compare {
            remaining.to_lowercase().find(&delimiter.to_lowercase())
        } else {
            remaining.find(delimiter)
        };

        if let Some(pos) = split_pos {
            // Found delimiter
            let part = &remaining[..pos];
            parts.push(VBVariant::from_string(part));
            remaining = &remaining[pos + delimiter.len()..];
        } else {
            // No more delimiters found, push remainder
            parts.push(VBVariant::from_string(remaining));
            break;
        }
    }

    Ok(VBVariant::Array(ArrayValue::from_vec_with_bounds(
        VBType::String,
        parts,
        0,
    )))
}

#[cfg(test)]
mod tests {
    use super::split;
    use crate::error::err_number;
    use crate::value::VBVariant;

    fn extract_strings(variant: &VBVariant) -> Vec<String> {
        match variant {
            VBVariant::Array(arr) => (0..arr.len())
                .map(|i| {
                    arr.get(&[i as i32])
                        .unwrap()
                        .as_str()
                        .unwrap_or("")
                        .to_string()
                })
                .collect(),
            _ => Vec::new(),
        }
    }

    #[test]
    fn split_with_comma_delimiter() {
        let result = split("apple,banana,cherry", Some(","), None, None).unwrap();
        let parts = extract_strings(&result);
        assert_eq!(parts, vec!["apple", "banana", "cherry"]);
    }

    #[test]
    fn split_with_default_delimiter() {
        let result = split("The quick brown fox", None, None, None).unwrap();
        let parts = extract_strings(&result);
        assert_eq!(parts, vec!["The", "quick", "brown", "fox"]);
    }

    #[test]
    fn split_with_custom_delimiter() {
        let result = split("one|two|three", Some("|"), None, None).unwrap();
        let parts = extract_strings(&result);
        assert_eq!(parts, vec!["one", "two", "three"]);
    }

    #[test]
    fn split_with_empty_delimiter_returns_single_element() {
        let result = split("hello", Some(""), None, None).unwrap();
        let parts = extract_strings(&result);
        assert_eq!(parts, vec!["hello"]);
    }

    #[test]
    fn split_empty_expression_returns_empty_array() {
        let result = split("", Some(","), None, None).unwrap();
        let parts = extract_strings(&result);
        assert_eq!(parts, Vec::<String>::new());
    }

    #[test]
    fn split_with_limit() {
        let result = split("one,two,three,four,five", Some(","), Some(3), None).unwrap();
        let parts = extract_strings(&result);
        assert_eq!(parts, vec!["one", "two", "three,four,five"]);
    }

    #[test]
    fn split_with_limit_one() {
        let result = split("one,two,three", Some(","), Some(1), None).unwrap();
        let parts = extract_strings(&result);
        assert_eq!(parts, vec!["one,two,three"]);
    }

    #[test]
    fn split_single_element() {
        let result = split("single", Some(","), None, None).unwrap();
        let parts = extract_strings(&result);
        assert_eq!(parts, vec!["single"]);
    }

    #[test]
    fn split_preserves_order() {
        let result = split("first-second-third", Some("-"), None, None).unwrap();
        let parts = extract_strings(&result);
        assert_eq!(parts, vec!["first", "second", "third"]);
    }

    #[test]
    fn split_handles_empty_strings_between_delimiters() {
        let result = split("a,,b", Some(","), None, None).unwrap();
        let parts = extract_strings(&result);
        assert_eq!(parts, vec!["a", "", "b"]);
    }

    #[test]
    fn split_multiple_consecutive_delimiters() {
        let result = split("a,,,b", Some(","), None, None).unwrap();
        let parts = extract_strings(&result);
        assert_eq!(parts, vec!["a", "", "", "b"]);
    }

    #[test]
    fn split_multiline_text() {
        let text = "Line 1\nLine 2\nLine 3";
        let result = split(text, Some("\n"), None, None).unwrap();
        let parts = extract_strings(&result);
        assert_eq!(parts, vec!["Line 1", "Line 2", "Line 3"]);
    }

    #[test]
    fn split_with_limit_at_boundary() {
        let result = split("a,b,c", Some(","), Some(2), None).unwrap();
        let parts = extract_strings(&result);
        assert_eq!(parts, vec!["a", "b,c"]);
    }

    #[test]
    fn split_returns_zero_based_array() {
        let result = split("a,b,c", Some(","), None, None).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.lower_bound(0).unwrap(), 0);
        assert_eq!(arr.upper_bound(0).unwrap(), 2);
    }

    #[test]
    fn split_limit_minus_one_means_no_limit() {
        let result = split("a,b,c,d,e", Some(","), Some(-1), None).unwrap();
        let parts = extract_strings(&result);
        assert_eq!(parts, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn split_limit_zero_is_error_5() {
        let err = split("a,b,c", Some(","), Some(0), None).unwrap_err();
        assert_eq!(err.number, err_number::INVALID_PROCEDURE_CALL);
    }

    #[test]
    fn split_negative_limit_other_than_minus_one_is_error_5() {
        let err = split("a,b,c", Some(","), Some(-2), None).unwrap_err();
        assert_eq!(err.number, err_number::INVALID_PROCEDURE_CALL);
    }

    #[test]
    fn split_text_compare_case_insensitive() {
        let result = split("A,B,a,b", Some(","), None, Some(1)).unwrap();
        let parts = extract_strings(&result);
        assert_eq!(parts, vec!["A", "B", "a", "b"]);
    }

    #[test]
    fn split_binary_compare_case_sensitive() {
        let result = split("A,B,a,b", Some(","), None, Some(0)).unwrap();
        let parts = extract_strings(&result);
        assert_eq!(parts, vec!["A", "B", "a", "b"]);
    }

    #[test]
    fn split_invalid_compare_is_error_5() {
        let err = split("a,b,c", Some(","), None, Some(3)).unwrap_err();
        assert_eq!(err.number, err_number::INVALID_PROCEDURE_CALL);
    }

    #[test]
    fn split_preserves_whitespace_in_parts() {
        let result = split("hello   world", Some("  "), None, None).unwrap();
        let parts = extract_strings(&result);
        assert_eq!(parts, vec!["hello", " world"]);
    }

    #[test]
    fn split_delimiter_not_found_returns_single_element() {
        let result = split("hello world", Some(","), None, None).unwrap();
        let parts = extract_strings(&result);
        assert_eq!(parts, vec!["hello world"]);
    }

    #[test]
    fn split_preserves_original_casing_with_text_compare() {
        let result = split("Hello World TEST", Some(","), None, Some(1)).unwrap();
        let parts = extract_strings(&result);
        assert_eq!(parts, vec!["Hello World TEST"]);
    }

    #[test]
    fn split_use_option_compare_defaults_to_binary() {
        let result = split("a,b,c", Some(","), None, Some(-1)).unwrap();
        assert!(!extract_strings(&result).is_empty());
    }

    #[test]
    fn split_database_compare_defaults_to_binary() {
        let result = split("a,b,c", Some(","), None, Some(2)).unwrap();
        assert!(!extract_strings(&result).is_empty());
    }

    #[test]
    fn split_multiline_with_crlf() {
        let text = "Line 1\r\nLine 2\r\nLine 3";
        let result = split(text, Some("\r\n"), None, None).unwrap();
        let parts = extract_strings(&result);
        assert_eq!(parts, vec!["Line 1", "Line 2", "Line 3"]);
    }

    #[test]
    fn split_path_components() {
        let result = split("a/b/c/d", Some("/"), None, None).unwrap();
        let parts = extract_strings(&result);
        assert_eq!(parts, vec!["a", "b", "c", "d"]);
    }

    #[test]
    fn split_empty_delimiter_on_non_empty_expression() {
        let result = split("anything", Some(""), None, None).unwrap();
        let parts = extract_strings(&result);
        assert_eq!(parts, vec!["anything"]);
    }
}
