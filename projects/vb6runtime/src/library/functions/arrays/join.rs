//! ## `Join` Function
//!
//! Returns a string created by joining a number of substrings contained in an array.
//!
//! ## Syntax
//!
//! ```text
//! Join(sourcearray, [delimiter])
//! ```
//!
//! ## Parameters
//!
//! - **sourcearray** (Required): One-dimensional array containing substrings to be joined
//! - **delimiter** (Optional): `String` used to separate the substrings in the returned string
//!   - If omitted, space character (" ") is used
//!   - If empty string (""), items are concatenated with no separator
//!
//! ## Return Value
//!
//! Returns a `String`
//!
//! - `String` containing all elements of the array joined by the delimiter
//! - Empty string ("") if array has zero length or is undimensioned
//! - Each array element is converted to `String` before joining
//! - Non-string elements are automatically converted using `Str`/`CStr` semantics
//! - Empty array elements become empty strings in result
//! - Trailing / leading spaces in delimiter are preserved
//!
//! ## Remarks
//!
//! The `Join` function is the inverse of the `Split` function:
//!
//! - Combines array elements into a single string
//! - Only works with one-dimensional arrays
//! - Array elements are converted to strings automatically
//! - Default delimiter is a space (" ")
//! - Empty string delimiter concatenates without separators
//! - Empty/undimensioned array returns empty string
//! - Preserves empty array elements as empty strings
//! - Very efficient for building strings from multiple parts
//! - Much faster than repeated string concatenation in loops
//! - Available in VB6 and VBA (added in VB6/Office 2000)
//! - Common in text processing and file generation
//! - Works with `Variant` arrays containing mixed types
//! - Does not add delimiter after last element
//!
//! ### Common Errors
//!
//! - **Error 13** (Type Mismatch): `sourcearray` is not an array, or is a multi-dimensional array.
//! - **Error 94** (Invalid use of Null): `sourcearray` is `Null`.
//!
//! ## Performance Considerations
//!
//! - **Very Efficient**: Join is much faster than repeated concatenation
//! - **String Building**: Use Join instead of concatenation in loops
//! - **Memory Usage**: Creates single string allocation for result
//! - **Large Arrays**: Handles large arrays efficiently
//!
//! ### Performance comparison:
//!
//! ```vb6
//! ' SLOW: Repeated concatenation
//! Dim result As String
//! For i = 0 To 999
//!     result = result & arr(i) & ","
//! Next i
//!
//! ' FAST: Using Join
//! result = Join(arr, ",")
//! ```
//!
//! ## Typical Uses
//!
//! 1. **CSV Generation**: Create comma-separated value strings
//! 2. **Path Building**: Combine path components with backslashes
//! 3. **SQL Generation**: Build SQL queries from parts
//! 4. **Text Formatting**: Create formatted text from arrays
//! 5. **File Output**: Generate text file content
//! 6. **URL Building**: Construct URLs from components
//! 7. **String Building**: Efficient alternative to concatenation loops
//! 8. **Report Generation**: Format report lines from data arrays
//!
//! ## Limitations
//!
//! - Cannot join multi-dimensional arrays (use loops to flatten first)
//! - No built-in escaping for CSV (must implement manually)
//! - Cannot skip empty elements automatically
//! - No formatting options for numeric values
//! - Delimiter is applied between all elements (no custom logic)
//!
//! ### Platform and Version Notes
//!
//! - Added in VB6 and Office 2000 VBA
//! - Not available in VB5 or earlier
//! - Part of `VBA.Strings` module
//! - Returns `String` type
//! - Only works with one-dimensional arrays
//! - Automatically converts array elements to `String`
//!
//! ## Related Functions
//!
//! - `Split`: Split string into array (inverse of `Join`)
//! - `Filter`: Filter array elements based on criteria
//! - `UBound`/`LBound`: Get array bounds
//! - `Array`: Create array from values
//! - `Replace`: Replace substrings in `String`
//!
//! ## Examples
//!
//! ### Join With Default Delimiter (space)
//!
//! ```vb6
//! Dim words(2) As String
//! words(0) = "Hello"
//! words(1) = "Visual"
//! words(2) = "Basic"
//!
//! Debug.Print Join(words)              ' "Hello Visual Basic"
//! ```
//!
//! ### Join With Custom Delimiter
//!
//! ```vb6
//! Dim values(3) As String
//! values(0) = "apple"
//! values(1) = "banana"
//! values(2) = "cherry"
//! values(3) = "date"
//!
//! Debug.Print Join(values, ", ")       ' "apple, banana, cherry, date"
//! Debug.Print Join(values, " | ")      ' "apple | banana | cherry | date"
//! Debug.Print Join(values, "")         ' "applebananacherrydate"
//! ```
//!
//! ### CSV Generation
//!
//! ```vb6
//! Dim fields(2) As String
//! fields(0) = "John Doe"
//! fields(1) = "Engineer"
//! fields(2) = "50000"
//!
//! Dim csvLine As String
//! csvLine = Join(fields, ",")
//! Debug.Print csvLine                  ' "John Doe,Engineer,50000"
//! ```
//!
//! ### Working with Split and Join
//!
//! ```vb6
//! Dim original As String
//! Dim parts() As String
//! Dim rebuilt As String
//!
//! original = "one-two-three-four"
//! parts = Split(original, "-")
//! rebuilt = Join(parts, " ")
//! Debug.Print rebuilt                  ' "one two three four"
//! ```
//!
//! ## Common Patterns
//!
//! ### Pattern 1: Build CSV Row
//!
//! ```vb6
//! Function BuildCSVRow(fields As Variant) As String
//!     BuildCSVRow = Join(fields, ",")
//! End Function
//! ```
//!
//! ### Pattern 2: Join With Line Breaks
//!
//! ```vb6
//! Function JoinLines(lines As Variant) As String
//!     JoinLines = Join(lines, vbCrLf)
//! End Function
//! ```
//!
//! ### Pattern 3: Build Path From Components
//!
//! ```vb6
//! Function BuildPath(parts() As String) As String
//!     BuildPath = Join(parts, "\")
//! End Function
//! ```
//!
//! ### Pattern 4: Create Comma-Separated List
//!
//! ```vb6
//! Function ToCommaSeparated(items As Variant) As String
//!     If IsArray(items) Then
//!         ToCommaSeparated = Join(items, ", ")
//!     Else
//!         ToCommaSeparated = CStr(items)
//!     End If
//! End Function
//! ```
//!
//! ### Pattern 5: Build SQL IN Clause
//!
//! ```vb6
//! Function BuildInClause(values As Variant) As String
//!     Dim i As Long
//!     Dim quoted() As String
//!     
//!     If Not IsArray(values) Then Exit Function
//!     
//!     ReDim quoted(LBound(values) To UBound(values))
//!     For i = LBound(values) To UBound(values)
//!         quoted(i) = "'" & Replace(CStr(values(i)), "'", "''") & "'"
//!     Next i
//!     
//!     BuildInClause = Join(quoted, ", ")
//! End Function
//! ```
//!
//! ### Pattern 6: Join Non-Empty Values Only
//!
//! ```vb6
//! Function JoinNonEmpty(arr As Variant, delimiter As String) As String
//!     Dim result() As String
//!     Dim count As Long
//!     Dim i As Long
//!     
//!     If Not IsArray(arr) Then Exit Function
//!     
//!     ' Count non-empty elements
//!     count = 0
//!     For i = LBound(arr) To UBound(arr)
//!         If Len(arr(i)) > 0 Then count = count + 1
//!     Next i
//!     
//!     If count = 0 Then
//!         JoinNonEmpty = ""
//!         Exit Function
//!     End If
//!     
//!     ReDim result(0 To count - 1)
//!     count = 0
//!     For i = LBound(arr) To UBound(arr)
//!         If Len(arr(i)) > 0 Then
//!             result(count) = CStr(arr(i))
//!             count = count + 1
//!         End If
//!     Next i
//!     
//!     JoinNonEmpty = Join(result, delimiter)
//! End Function
//! ```
//!
//! ### Pattern 7: Format Array For Display
//!
//! ```vb6
//! Function FormatArray(arr As Variant) As String
//!     If Not IsArray(arr) Then
//!         FormatArray = CStr(arr)
//!     Else
//!         FormatArray = "[" & Join(arr, ", ") & "]"
//!     End If
//! End Function
//! ```
//!
//! ### Pattern 8: Build WHERE Clause
//!
//! ```vb6
//! Function BuildWhereClause(conditions As Variant) As String
//!     If Not IsArray(conditions) Then Exit Function
//!     
//!     If UBound(conditions) < LBound(conditions) Then
//!         BuildWhereClause = ""
//!     Else
//!         BuildWhereClause = Join(conditions, " AND ")
//!     End If
//! End Function
//! ```
//!
//! ## Advanced Usage Examples
//!
//! ### Example 1: CSV Builder with proper escaping
//!
//! ```vb6
//! Public Class CSVBuilder
//!     Private m_rows As Collection
//!     
//!     Private Sub Class_Initialize()
//!         Set m_rows = New Collection
//!     End Sub
//!     
//!     Public Sub AddRow(ParamArray values() As Variant)
//!         Dim i As Long
//!         Dim fields() As String
//!         
//!         ReDim fields(LBound(values) To UBound(values))
//!         For i = LBound(values) To UBound(values)
//!             fields(i) = EscapeCSV(CStr(values(i)))
//!         Next i
//!         
//!         m_rows.Add Join(fields, ",")
//!     End Sub
//!     
//!     Private Function EscapeCSV(value As String) As String
//!         If InStr(value, ",") > 0 Or InStr(value, Chr(34)) > 0 Or _
//!            InStr(value, vbCrLf) > 0 Then
//!             EscapeCSV = Chr(34) & Replace(value, Chr(34), Chr(34) & Chr(34)) & Chr(34)
//!         Else
//!             EscapeCSV = value
//!         End If
//!     End Function
//!     
//!     Public Function GetCSV() As String
//!         Dim i As Long
//!         Dim lines() As String
//!         
//!         If m_rows.Count = 0 Then
//!             GetCSV = ""
//!             Exit Function
//!         End If
//!         
//!         ReDim lines(0 To m_rows.Count - 1)
//!         For i = 1 To m_rows.Count
//!             lines(i - 1) = m_rows(i)
//!         Next i
//!         
//!         GetCSV = Join(lines, vbCrLf)
//!     End Function
//! End Class
//! ```
//!
//! ### Example 2: String Builder For Efficient Concatenation
//!
//! ```vb6
//! Public Class StringBuilder
//!     Private m_parts As Collection
//!     Private m_delimiter As String
//!     
//!     Private Sub Class_Initialize()
//!         Set m_parts = New Collection
//!         m_delimiter = ""
//!     End Sub
//!     
//!     Public Property Let Delimiter(value As String)
//!         m_delimiter = value
//!     End Property
//!     
//!     Public Sub Append(text As String)
//!         m_parts.Add text
//!     End Sub
//!     
//!     Public Function ToString() As String
//!         Dim i As Long
//!         Dim arr() As String
//!         
//!         If m_parts.Count = 0 Then
//!             ToString = ""
//!             Exit Function
//!         End If
//!         
//!         ReDim arr(0 To m_parts.Count - 1)
//!         For i = 1 To m_parts.Count
//!             arr(i - 1) = m_parts(i)
//!         Next i
//!         
//!         ToString = Join(arr, m_delimiter)
//!     End Function
//! End Class
//! ```
//!
//! ### Example 3: Query Builder Using Join
//!
//! ```vb6
//! Public Class QueryBuilder
//!     Private m_select As Collection
//!     Private m_from As String
//!     Private m_where As Collection
//!     
//!     Private Sub Class_Initialize()
//!         Set m_select = New Collection
//!         Set m_where = New Collection
//!     End Sub
//!     
//!     Public Sub AddField(fieldName As String)
//!         m_select.Add fieldName
//!     End Sub
//!     
//!     Public Sub SetTable(tableName As String)
//!         m_from = tableName
//!     End Sub
//!     
//!     Public Sub AddCondition(condition As String)
//!         m_where.Add condition
//!     End Sub
//!     
//!     Public Function BuildSQL() As String
//!         Dim fields() As String
//!         
//!         ' SELECT clause
//!         If m_select.Count > 0 Then
//!             ReDim fields(0 To m_select.Count - 1)
//!             For i = 1 To m_select.Count
//!                 fields(i - 1) = m_select(i)
//!             Next i
//!             BuildSQL = "SELECT " & Join(fields, ", ")
//!         Else
//!             BuildSQL = "SELECT *"
//!         End If
//!         
//!         ' FROM clause
//!         If m_from <> "" Then
//!             BuildSQL = BuildSQL & " FROM " & m_from
//!         End If
//!     End Function
//! End Class
//! ```
//!
//! ### Example 4: Report Formatter
//!
//! ```vb6
//! Public Class ReportFormatter
//!     Public Function FormatTable(data As Variant, headers As Variant, _
//!                                  Optional delimiter As String = " | ") As String
//!         Dim lines As Collection
//!         Set lines = New Collection
//!         
//!         ' Add header
//!         If IsArray(headers) Then
//!             lines.Add Join(headers, delimiter)
//!         End If
//!         
//!         ' Add data rows
//!         If IsArray(data) Then
//!             For i = LBound(data) To UBound(data)
//!                 If IsArray(data(i)) Then
//!                     lines.Add Join(data(i), delimiter)
//!                 Else
//!                     lines.Add CStr(data(i))
//!                 End If
//!             Next i
//!         End If
//!         
//!         ' Convert collection to array and join
//!         ReDim allLines(0 To lines.Count - 1)
//!         For i = 1 To lines.Count
//!             allLines(i - 1) = lines(i)
//!         Next i
//!         
//!         FormatTable = Join(allLines, vbCrLf)
//!     End Function
//! End Class
//! ```
//!
//! ## Error Handling
//!
//! Join handles several special cases:
//!
//! ### Empty Array Returns Empty String
//!
//! ```vb6
//! Dim emptyArr() As String
//! ReDim emptyArr(0 To -1)  ' Zero-length array
//! Debug.Print Join(emptyArr, ",")  ' Returns ""
//! ```
//!
//! ### Multi-Dimensional Arrays Cause Type Mismatch Error
//!
//! ```vb6
//! Dim multi(1, 1) As String
//! ' Join(multi, ",")  ' Error 13: Type Mismatch
//! ```
//!
//! ## Best Practices
//!
//! 1. **Use Join for String Building**: Much faster than repeated concatenation
//! 2. **CSV Generation**: Properly escape values containing delimiters
//! 3. **Empty Delimiter**: Use "" to concatenate without separators
//! 4. **Check Array**: Verify array exists before calling `Join`
//! 5. **Line Breaks**: Use `vbCrLf`, `vbLf`, or `vbCr` as delimiter for multi-line text
//! 6. **Type Conversion**: `Join` automatically converts non-string elements
//! 7. **Collection to String**: Convert `Collection` to array first, then `Join`
//!
//! ## Comparison with Related Functions
//!
//! | Function | Purpose | Input | Output |
//! |----------|---------|-------|--------|
//! | `Join` | Combine array to string | `Array` | `String` |
//! | `Split` | Split string to array | `String` | `Array` |
//! | `Filter` | Filter array elements | `Array` | `Array` |
//! | `UBound`/`LBound` | Get array bounds | `Array` | `Long` |
//!
//! ## Join vs String Concatenation
//!
//! ```vb6
//! Dim arr(2) As String
//! arr(0) = "A"
//! arr(1) = "B"
//! arr(2) = "C"
//!
//! ' Using Join (FAST)
//! result = Join(arr, ",")              ' "A,B,C"
//!
//! ' Using concatenation (SLOW)
//! result = arr(0) & "," & arr(1) & "," & arr(2)  ' "A,B,C"
//! ```
//!
//! ## Join and Split Round-Trip
//!
//! ```vb6
//! original = "apple,banana,cherry"
//! parts = Split(original, ",")         ' ["apple", "banana", "cherry"]
//! rebuilt = Join(parts, ",")           ' "apple,banana,cherry"
//! Debug.Print original = rebuilt       ' True - perfect round-trip
//! ```

use crate::error::{err_number, VBError, VBResult};
use crate::value::VBVariant;

/// Implementation of the `Join` function.
///
/// Joins the elements of a one-dimensional array into a single string with a
/// delimiter between each element.
///
/// VB6 behavior:
/// - Default delimiter is a space (" ")
/// - Works only with one-dimensional arrays
/// - Non-string elements are converted to string via `CStr`-like semantics
/// - Empty or undimensioned arrays return an empty string
/// - Raises error 94 if `sourcearray` is `Null`
/// - Raises error 13 if `sourcearray` is not an array or is multi-dimensional
pub fn join(
    sourcearray: &VBVariant,
    delimiter: Option<&str>,
) -> VBResult<String> {
    // Validate sourcearray is not Null
    if sourcearray.is_null() {
        return Err(VBError::with_description(
            err_number::INVALID_USE_OF_NULL,
            "Invalid use of Null",
        ));
    }

    // Validate sourcearray is an array
    let VBVariant::Array(arr) = sourcearray else {
        return Err(VBError::type_mismatch());
    };

    // Empty or undimensioned arrays return empty string
    if !arr.is_initialized() || arr.is_empty() {
        return Ok(String::new());
    }

    // Check for multi-dimensional arrays
    if arr.rank() != 1 {
        return Err(VBError::with_description(
            err_number::TYPE_MISMATCH,
            "Multi-dimensional array",
        ));
    }

    let delimiter = delimiter.unwrap_or(" ");

    let lower = arr.lower_bound(0).unwrap();
    let upper = arr.upper_bound(0).unwrap();

    let mut result = String::new();

    for i in lower..=upper {
        if i != lower {
            result.push_str(delimiter);
        }

        let element = arr.get(&[i]).unwrap();
        if element.is_null() {
            return Err(VBError::invalid_use_of_null());
        }
        let s = element.as_string().map_err(|_| VBError::type_mismatch())?;
        result.push_str(&s);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::join;
    use crate::array::{ArrayDimension, ArrayValue};
    use crate::error::err_number;
    use crate::types::VBType;
    use crate::value::VBVariant;

    fn make_string_array(strings: &[&str]) -> VBVariant {
        let data: Vec<VBVariant> = strings.iter().map(|s| VBVariant::from_string(*s)).collect();
        VBVariant::Array(ArrayValue::from_vec_with_bounds(VBType::String, data, 0))
    }

    #[test]
    fn join_with_default_delimiter() {
        let source = make_string_array(&["Hello", "Visual", "Basic"]);
        let result = join(&source, None).unwrap();
        assert_eq!(result, "Hello Visual Basic");
    }

    #[test]
    fn join_with_comma_space_delimiter() {
        let source = make_string_array(&["apple", "banana", "cherry"]);
        let result = join(&source, Some(", ")).unwrap();
        assert_eq!(result, "apple, banana, cherry");
    }

    #[test]
    fn join_with_custom_delimiter() {
        let source = make_string_array(&["one", "two", "three"]);
        let result = join(&source, Some(" | ")).unwrap();
        assert_eq!(result, "one | two | three");
    }

    #[test]
    fn join_with_empty_delimiter() {
        let source = make_string_array(&["a", "b", "c"]);
        let result = join(&source, Some("")).unwrap();
        assert_eq!(result, "abc");
    }

    #[test]
    fn join_single_element() {
        let source = make_string_array(&["single"]);
        let result = join(&source, Some(",")).unwrap();
        assert_eq!(result, "single");
    }

    #[test]
    fn join_empty_array_returns_empty_string() {
        let source = VBVariant::Array(ArrayValue::from_vec_with_bounds(
            VBType::String,
            Vec::new(),
            0,
        ));
        let result = join(&source, Some(",")).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn join_undimensioned_array_returns_empty_string() {
        let source = VBVariant::Array(ArrayValue::new_dynamic(VBType::String));
        let result = join(&source, Some(",")).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn join_preserves_order() {
        let source = make_string_array(&["first", "second", "third"]);
        let result = join(&source, Some("-")).unwrap();
        assert_eq!(result, "first-second-third");
    }

    #[test]
    fn join_handles_empty_strings_in_array() {
        let source = make_string_array(&["hello", "", "world"]);
        let result = join(&source, Some(" ")).unwrap();
        assert_eq!(result, "hello  world");
    }

    #[test]
    fn null_source_is_error_94() {
        let err = join(&VBVariant::Null, None).unwrap_err();
        assert_eq!(err.number, err_number::INVALID_USE_OF_NULL);
    }

    #[test]
    fn non_array_is_error_13() {
        let err = join(&VBVariant::from_string("not an array"), None).unwrap_err();
        assert_eq!(err.number, err_number::TYPE_MISMATCH);
    }

    #[test]
    fn multi_dimensional_array_is_error_13() {
        let source = VBVariant::Array(
            ArrayValue::new_fixed(
                VBType::String,
                &[ArrayDimension::new(0, 1), ArrayDimension::new(0, 1)],
            )
            .unwrap(),
        );
        let err = join(&source, None).unwrap_err();
        assert_eq!(err.number, err_number::TYPE_MISMATCH);
    }

    #[test]
    fn join_with_one_based_array() {
        let data: Vec<VBVariant> = vec![
            VBVariant::from_string("A"),
            VBVariant::from_string("B"),
            VBVariant::from_string("C"),
        ];
        let source = VBVariant::Array(ArrayValue::from_vec(VBType::String, data));
        let result = join(&source, Some(",")).unwrap();
        assert_eq!(result, "A,B,C");
    }

    #[test]
    fn join_with_arbitrary_lower_bound() {
        let data: Vec<VBVariant> = vec![
            VBVariant::from_string("x"),
            VBVariant::from_string("y"),
            VBVariant::from_string("z"),
        ];
        let source =
            VBVariant::Array(ArrayValue::from_vec_with_bounds(VBType::String, data, -1));
        let result = join(&source, Some("-")).unwrap();
        assert_eq!(result, "x-y-z");
    }

    #[test]
    fn join_preserves_whitespace_in_delimiter() {
        let source = make_string_array(&["a", "b", "c"]);
        let result = join(&source, Some("  ")).unwrap();
        assert_eq!(result, "a  b  c");
    }

    #[test]
    fn null_element_is_error_94() {
        let data: Vec<VBVariant> = vec![
            VBVariant::from_string("hello"),
            VBVariant::Null,
            VBVariant::from_string("world"),
        ];
        let source =
            VBVariant::Array(ArrayValue::from_vec_with_bounds(VBType::Variant, data, 0));
        let err = join(&source, None).unwrap_err();
        assert_eq!(err.number, err_number::INVALID_USE_OF_NULL);
    }

    #[test]
    fn join_converts_numeric_elements() {
        let data: Vec<VBVariant> = vec![
            VBVariant::from_integer(123),
            VBVariant::from_long(456),
            VBVariant::from_string("text"),
        ];
        let source =
            VBVariant::Array(ArrayValue::from_vec_with_bounds(VBType::Variant, data, 0));
        let result = join(&source, Some("-")).unwrap();
        assert_eq!(result, "123-456-text");
    }

    #[test]
    fn join_object_element_is_error_13() {
        use crate::value::VBObject;

        struct TestObj;
        impl VBObject for TestObj {
            fn type_name(&self) -> &str {
                "TestObj"
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
            fn clone_box(&self) -> Box<dyn VBObject> {
                Box::new(TestObj)
            }
        }
        impl std::fmt::Debug for TestObj {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("TestObj")
            }
        }

        let data: Vec<VBVariant> = vec![
            VBVariant::from_string("hello"),
            VBVariant::Object(Box::new(TestObj)),
            VBVariant::from_string("world"),
        ];
        let source =
            VBVariant::Array(ArrayValue::from_vec_with_bounds(VBType::Variant, data, 0));
        let err = join(&source, None).unwrap_err();
        assert_eq!(err.number, err_number::TYPE_MISMATCH);
    }
}
