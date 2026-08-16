//! VB6 `UBound` Function
//!
//! The `UBound` function returns a Long containing the largest available subscript for the indicated dimension of an array.
//!
//! ## Syntax
//!
//! ```vb6
//! UBound(arrayname[, dimension])
//! ```
//!
//! ## Parameters
//!
//! - `arrayname`: Required. Name of the array variable. Follows standard Visual Basic naming conventions.
//! - `dimension`: Optional. Variant (Long). Specifies which dimension's upper bound is returned. Use 1 for the first dimension, 2 for the second, and so on. If `dimension` is omitted, 1 is assumed.
//!
//! ## Returns
//!
//! Returns a `Long` containing the largest available subscript for the specified dimension of the array.
//!
//! ## Remarks
//!
//! The `UBound` function is used to determine the upper limit of an array dimension:
//!
//! - **Dimension parameter**: If omitted, defaults to 1 (first dimension)
//! - **Multi-dimensional arrays**: Use `dimension` parameter to specify which dimension
//! - **Zero-based arrays**: `UBound` returns the upper index regardless of lower bound
//! - **Paired with `LBound`**: Use `LBound` to get the lower bound
//! - **Array size calculation**: Size = `UBound - LBound + 1`
//! - **Dynamic arrays**: Returns current upper bound (changes with `ReDim`)
//! - **Fixed arrays**: Returns the declared upper bound
//! - **Error on uninitialized**: Error 9 (Subscript out of range) if array not initialized
//! - **`ParamArray`**: Works with `ParamArray` arguments to find number of elements
//!
//! ### Common Array Declarations
//!
//! ```vb6
//! Dim arr(5)              ' LBound = 0, UBound = 5 (6 elements)
//! Dim arr(1 To 5)         ' LBound = 1, UBound = 5 (5 elements)
//! Dim arr(10 To 20)       ' LBound = 10, UBound = 20 (11 elements)
//! Dim arr(5, 3)           ' First: 0-5, Second: 0-3
//! Dim arr(1 To 5, 1 To 3) ' First: 1-5, Second: 1-3
//! ```
//!
//! ### Option Base Impact
//!
//! The `Option Base` statement affects default lower bounds:
//! - `Option Base 0`: Default lower bound is 0 (default)
//! - `Option Base 1`: Default lower bound is 1
//! - Explicit bounds (e.g., `1 To 5`) override Option Base
//!
//! ### Dynamic Arrays
//!
//! For dynamic arrays:
//! - Before `ReDim`: Error 9 if accessed
//! - After `ReDim`: Returns current upper bound
//! - `ReDim Preserve`: Can change upper bound while preserving data
//! - `Erase`: Makes array uninitialized again
//!
//! ## Typical Uses
//!
//! 1. **Loop Bounds**: Iterate through all array elements
//! 2. **Array Size**: Calculate the number of elements in an array
//! 3. **Validation**: Check if an index is within valid range
//! 4. **Dynamic Resizing**: Determine current size before `ReDim`
//! 5. **`ParamArray`**: Count variable number of arguments
//! 6. **Array Copying**: Determine target array size
//! 7. **Search Operations**: Set loop limits for array searches
//! 8. **Multi-dimensional**: Navigate complex array structures
//!
//! ## Basic Examples
//!
//! ### Example 1: Simple Array Iteration
//!
//! ```vb6
//! Dim values(10) As Integer
//! Dim i As Integer
//!
//! For i = LBound(values) To UBound(values)
//!     values(i) = i * 2
//! Next i
//! ```
//!
//! ### Example 2: Calculate Array Size
//!
//! ```vb6
//! Function GetArraySize(arr() As Variant) As Long
//!     GetArraySize = UBound(arr) - LBound(arr) + 1
//! End Function
//!
//! ' Usage:
//! Dim myArray(5 To 15) As String
//! Debug.Print GetArraySize(myArray) ' Prints: 11
//! ```
//!
//! ### Example 3: Multi-Dimensional Array
//!
//! ```vb6
//! Sub ProcessMatrix()
//!     Dim matrix(1 To 3, 1 To 4) As Double
//!     Dim row As Integer
//!     Dim col As Integer
//!     
//!     For row = LBound(matrix, 1) To UBound(matrix, 1)
//!         For col = LBound(matrix, 2) To UBound(matrix, 2)
//!             matrix(row, col) = row * col
//!         Next col
//!     Next row
//! End Sub
//! ```
//!
//! ### Example 4: `ParamArray` with `UBound`
//!
//! ```vb6
//! Function Sum(ParamArray values() As Variant) As Double
//!     Dim i As Integer
//!     Dim total As Double
//!     
//!     total = 0
//!     For i = LBound(values) To UBound(values)
//!         total = total + values(i)
//!     Next i
//!     
//!     Sum = total
//! End Function
//!
//! ' Usage: result = Sum(1, 2, 3, 4, 5)
//! ```
//!
//! ## Common Patterns
//!
//! ### Pattern 1: Safe Array Iteration
//!
//! ```vb6
//! Sub IterateArray(arr() As Variant)
//!     Dim i As Long
//!     
//!     For i = LBound(arr) To UBound(arr)
//!         Debug.Print arr(i)
//!     Next i
//! End Sub
//! ```
//!
//! ### Pattern 2: Check If Array Is Empty
//!
//! ```vb6
//! Function IsArrayEmpty(arr() As Variant) As Boolean
//!     On Error Resume Next
//!     IsArrayEmpty = (UBound(arr) < LBound(arr))
//!     If Err.Number <> 0 Then IsArrayEmpty = True
//! End Function
//! ```
//!
//! ### Pattern 3: Resize Array with Data Preservation
//!
//! ```vb6
//! Sub AddArrayElement(arr() As Variant, newValue As Variant)
//!     Dim newSize As Long
//!     
//!     On Error Resume Next
//!     newSize = UBound(arr) + 1
//!     If Err.Number <> 0 Then
//!         ' Array not initialized
//!         ReDim arr(0 To 0)
//!         newSize = 0
//!     Else
//!         ReDim Preserve arr(LBound(arr) To newSize)
//!     End If
//!     
//!     arr(newSize) = newValue
//! End Sub
//! ```
//!
//! ### Pattern 4: Count Elements in `ParamArray`
//!
//! ```vb6
//! Function CountArgs(ParamArray args() As Variant) As Long
//!     On Error Resume Next
//!     CountArgs = UBound(args) - LBound(args) + 1
//!     If Err.Number <> 0 Then CountArgs = 0
//! End Function
//! ```
//!
//! ### Pattern 5: Validate Array Index
//!
//! ```vb6
//! Function IsValidIndex(arr() As Variant, index As Long) As Boolean
//!     On Error Resume Next
//!     IsValidIndex = (index >= LBound(arr) And index <= UBound(arr))
//!     If Err.Number <> 0 Then IsValidIndex = False
//! End Function
//! ```
//!
//! ### Pattern 6: Copy Array
//!
//! ```vb6
//! Function CopyArray(source() As Variant) As Variant()
//!     Dim dest() As Variant
//!     Dim i As Long
//!     
//!     ReDim dest(LBound(source) To UBound(source))
//!     
//!     For i = LBound(source) To UBound(source)
//!         dest(i) = source(i)
//!     Next i
//!     
//!     CopyArray = dest
//! End Function
//! ```
//!
//! ### Pattern 7: Reverse Array
//!
//! ```vb6
//! Sub ReverseArray(arr() As Variant)
//!     Dim i As Long
//!     Dim j As Long
//!     Dim temp As Variant
//!     
//!     i = LBound(arr)
//!     j = UBound(arr)
//!     
//!     While i < j
//!         temp = arr(i)
//!         arr(i) = arr(j)
//!         arr(j) = temp
//!         i = i + 1
//!         j = j - 1
//!     Wend
//! End Sub
//! ```
//!
//! ### Pattern 8: Find Last Element
//!
//! ```vb6
//! Function GetLastElement(arr() As Variant) As Variant
//!     GetLastElement = arr(UBound(arr))
//! End Function
//! ```
//!
//! ### Pattern 9: Remove Last Element
//!
//! ```vb6
//! Sub RemoveLastElement(arr() As Variant)
//!     Dim newUpper As Long
//!     
//!     newUpper = UBound(arr) - 1
//!     If newUpper >= LBound(arr) Then
//!         ReDim Preserve arr(LBound(arr) To newUpper)
//!     End If
//! End Sub
//! ```
//!
//! ### Pattern 10: Multi-Dimensional Size
//!
//! ```vb6
//! Function GetArrayDimensions(arr As Variant) As Integer
//!     Dim dimension As Integer
//!     
//!     On Error Resume Next
//!     dimension = 1
//!     Do While Err.Number = 0
//!         Dim test As Long
//!         test = UBound(arr, dimension)
//!         dimension = dimension + 1
//!     Loop
//!     
//!     GetArrayDimensions = dimension - 1
//! End Function
//! ```
//!
//! ## Advanced Usage
//!
//! ### Example 1: Dynamic Array Manager Class
//!
//! ```vb6
//! ' Class: DynamicArrayManager
//! ' Manages a dynamic array with automatic resizing
//! Option Explicit
//!
//! Private m_Data() As Variant
//! Private m_Initialized As Boolean
//!
//! Public Sub Initialize(Optional initialSize As Long = 10)
//!     ReDim m_Data(0 To initialSize - 1)
//!     m_Initialized = True
//! End Sub
//!
//! Public Sub Add(value As Variant)
//!     Dim newIndex As Long
//!     
//!     If Not m_Initialized Then
//!         Initialize
//!         newIndex = 0
//!     Else
//!         newIndex = UBound(m_Data) + 1
//!         ReDim Preserve m_Data(0 To newIndex)
//!     End If
//!     
//!     m_Data(newIndex) = value
//! End Sub
//!
//! Public Function GetItem(index As Long) As Variant
//!     If index < LBound(m_Data) Or index > UBound(m_Data) Then
//!         Err.Raise 9, , "Index out of range"
//!     End If
//!     
//!     If IsObject(m_Data(index)) Then
//!         Set GetItem = m_Data(index)
//!     Else
//!         GetItem = m_Data(index)
//!     End If
//! End Function
//!
//! Public Sub SetItem(index As Long, value As Variant)
//!     If index < LBound(m_Data) Or index > UBound(m_Data) Then
//!         Err.Raise 9, , "Index out of range"
//!     End If
//!     
//!     m_Data(index) = value
//! End Sub
//!
//! Public Function Count() As Long
//!     If Not m_Initialized Then
//!         Count = 0
//!     Else
//!         Count = UBound(m_Data) - LBound(m_Data) + 1
//!     End If
//! End Function
//!
//! Public Sub Clear()
//!     If m_Initialized Then
//!         Erase m_Data
//!         m_Initialized = False
//!     End If
//! End Sub
//!
//! Public Function ToArray() As Variant()
//!     ToArray = m_Data
//! End Function
//! ```
//!
//! ### Example 2: Array Utilities Module
//!
//! ```vb6
//! ' Module: ArrayUtilities
//! ' Comprehensive array manipulation utilities
//! Option Explicit
//!
//! Public Function ArraySize(arr As Variant) As Long
//!     On Error Resume Next
//!     ArraySize = UBound(arr) - LBound(arr) + 1
//!     If Err.Number <> 0 Then ArraySize = 0
//! End Function
//!
//! Public Function ArrayContains(arr() As Variant, value As Variant) As Boolean
//!     Dim i As Long
//!     
//!     ArrayContains = False
//!     For i = LBound(arr) To UBound(arr)
//!         If arr(i) = value Then
//!             ArrayContains = True
//!             Exit Function
//!         End If
//!     Next i
//! End Function
//!
//! Public Function ArrayIndexOf(arr() As Variant, value As Variant) As Long
//!     Dim i As Long
//!     
//!     ArrayIndexOf = -1
//!     For i = LBound(arr) To UBound(arr)
//!         If arr(i) = value Then
//!             ArrayIndexOf = i
//!             Exit Function
//!         End If
//!     Next i
//! End Function
//!
//! Public Sub ArraySort(arr() As Variant)
//!     Dim i As Long
//!     Dim j As Long
//!     Dim temp As Variant
//!     
//!     For i = LBound(arr) To UBound(arr) - 1
//!         For j = i + 1 To UBound(arr)
//!             If arr(i) > arr(j) Then
//!                 temp = arr(i)
//!                 arr(i) = arr(j)
//!                 arr(j) = temp
//!             End If
//!         Next j
//!     Next i
//! End Sub
//!
//! Public Function ArrayFilter(arr() As Variant, filterValue As Variant) As Variant()
//!     Dim result() As Variant
//!     Dim i As Long
//!     Dim count As Long
//!     
//!     count = 0
//!     For i = LBound(arr) To UBound(arr)
//!         If arr(i) <> filterValue Then
//!             ReDim Preserve result(0 To count)
//!             result(count) = arr(i)
//!             count = count + 1
//!         End If
//!     Next i
//!     
//!     ArrayFilter = result
//! End Function
//!
//! Public Function ArraySlice(arr() As Variant, startIndex As Long, _
//!                           endIndex As Long) As Variant()
//!     Dim result() As Variant
//!     Dim i As Long
//!     Dim idx As Long
//!     
//!     ReDim result(0 To endIndex - startIndex)
//!     
//!     idx = 0
//!     For i = startIndex To endIndex
//!         result(idx) = arr(i)
//!         idx = idx + 1
//!     Next i
//!     
//!     ArraySlice = result
//! End Function
//! ```
//!
//! ### Example 3: Matrix Operations Class
//!
//! ```vb6
//! ' Class: MatrixOperations
//! ' Performs operations on 2D arrays
//! Option Explicit
//!
//! Public Function GetRowCount(matrix As Variant) As Long
//!     On Error Resume Next
//!     GetRowCount = UBound(matrix, 1) - LBound(matrix, 1) + 1
//!     If Err.Number <> 0 Then GetRowCount = 0
//! End Function
//!
//! Public Function GetColumnCount(matrix As Variant) As Long
//!     On Error Resume Next
//!     GetColumnCount = UBound(matrix, 2) - LBound(matrix, 2) + 1
//!     If Err.Number <> 0 Then GetColumnCount = 0
//! End Function
//!
//! Public Function GetRow(matrix As Variant, rowIndex As Long) As Variant()
//!     Dim result() As Variant
//!     Dim col As Long
//!     Dim idx As Long
//!     
//!     ReDim result(LBound(matrix, 2) To UBound(matrix, 2))
//!     
//!     For col = LBound(matrix, 2) To UBound(matrix, 2)
//!         result(col) = matrix(rowIndex, col)
//!     Next col
//!     
//!     GetRow = result
//! End Function
//!
//! Public Function GetColumn(matrix As Variant, colIndex As Long) As Variant()
//!     Dim result() As Variant
//!     Dim row As Long
//!     
//!     ReDim result(LBound(matrix, 1) To UBound(matrix, 1))
//!     
//!     For row = LBound(matrix, 1) To UBound(matrix, 1)
//!         result(row) = matrix(row, colIndex)
//!     Next row
//!     
//!     GetColumn = result
//! End Function
//!
//! Public Function TransposeMatrix(matrix As Variant) As Variant
//!     Dim result() As Variant
//!     Dim row As Long
//!     Dim col As Long
//!     
//!     ReDim result(LBound(matrix, 2) To UBound(matrix, 2), _
//!                  LBound(matrix, 1) To UBound(matrix, 1))
//!     
//!     For row = LBound(matrix, 1) To UBound(matrix, 1)
//!         For col = LBound(matrix, 2) To UBound(matrix, 2)
//!             result(col, row) = matrix(row, col)
//!         Next col
//!     Next row
//!     
//!     TransposeMatrix = result
//! End Function
//! ```
//!
//! ### Example 4: Collection to Array Converter
//!
//! ```vb6
//! ' Module: CollectionConverter
//! ' Converts between Collections and Arrays
//! Option Explicit
//!
//! Public Function CollectionToArray(col As Collection) As Variant()
//!     Dim result() As Variant
//!     Dim i As Long
//!     
//!     If col.Count = 0 Then
//!         CollectionToArray = Array()
//!         Exit Function
//!     End If
//!     
//!     ReDim result(1 To col.Count)
//!     
//!     For i = 1 To col.Count
//!         If IsObject(col(i)) Then
//!             Set result(i) = col(i)
//!         Else
//!             result(i) = col(i)
//!         End If
//!     Next i
//!     
//!     CollectionToArray = result
//! End Function
//!
//! Public Function ArrayToCollection(arr() As Variant) As Collection
//!     Dim result As New Collection
//!     Dim i As Long
//!     
//!     For i = LBound(arr) To UBound(arr)
//!         result.Add arr(i)
//!     Next i
//!     
//!     Set ArrayToCollection = result
//! End Function
//!
//! Public Function MergeArrays(ParamArray arrays() As Variant) As Variant()
//!     Dim result() As Variant
//!     Dim totalSize As Long
//!     Dim currentIndex As Long
//!     Dim i As Long
//!     Dim j As Long
//!     Dim arr As Variant
//!     
//!     ' Calculate total size
//!     totalSize = 0
//!     For i = LBound(arrays) To UBound(arrays)
//!         arr = arrays(i)
//!         totalSize = totalSize + (UBound(arr) - LBound(arr) + 1)
//!     Next i
//!     
//!     ' Merge arrays
//!     ReDim result(0 To totalSize - 1)
//!     currentIndex = 0
//!     
//!     For i = LBound(arrays) To UBound(arrays)
//!         arr = arrays(i)
//!         For j = LBound(arr) To UBound(arr)
//!             result(currentIndex) = arr(j)
//!             currentIndex = currentIndex + 1
//!         Next j
//!     Next i
//!     
//!     MergeArrays = result
//! End Function
//! ```
//!
//! ## Error Handling
//!
//! The `UBound` function can raise the following errors:
//!
//! - **Error 9 (Subscript out of range)**: If the array has not been initialized (for dynamic arrays)
//! - **Error 9 (Subscript out of range)**: If `dimension` is less than 1 or greater than the array's number of dimensions
//! - **Error 13 (Type mismatch)**: If the variable is not an array
//! - **Error 5 (Invalid procedure call or argument)**: If dimension parameter is invalid
//!
//! ## Performance Notes
//!
//! - Very fast O(1) operation - directly returns array metadata
//! - No performance difference between dimensions
//! - Safe to call repeatedly in loops
//! - Consider caching value if used extensively in tight loops
//! - No memory allocation or copying involved
//!
//! ## Best Practices
//!
//! 1. **Always use with `LBound`** for complete array bounds information
//! 2. **Check for initialization** with On Error Resume Next for dynamic arrays
//! 3. **Use in For loops** instead of hardcoding array sizes
//! 4. **Specify dimension** explicitly for multi-dimensional arrays
//! 5. **Cache in variables** if used multiple times in tight loops
//! 6. **Validate dimension parameter** when working with multi-dimensional arrays
//! 7. **Handle errors gracefully** for potentially uninitialized arrays
//! 8. **Use for `ParamArray`** to handle variable arguments
//! 9. **Document array bounds** in function comments
//! 10. **Prefer explicit bounds** in array declarations for clarity
//!
//! ## Comparison Table
//!
//! | Function | Purpose | Returns | Notes |
//! |----------|---------|---------|-------|
//! | `UBound` | Upper bound | Long | Largest valid index |
//! | `LBound` | Lower bound | Long | Smallest valid index |
//! | `Array` | Create array | Variant | Returns zero-based array |
//! | `ReDim` | Resize array | N/A | Statement, not function |
//!
//! ## Platform Notes
//!
//! - Available in VB6, VBA, and `VBScript`
//! - Behavior consistent across platforms
//! - Returns Long (32-bit signed integer)
//! - Maximum array size limited by available memory
//! - Multi-dimensional arrays limited to 60 dimensions
//!
//! ## Limitations
//!
//! - Cannot determine if array is initialized without error handling
//! - Does not return array capacity (allocated size vs. used size)
//! - No built-in way to get all dimensions at once
//! - Dimension parameter must be compile-time constant in some contexts
//! - Cannot be used on Collections or other non-array types
//! - Does not work with jagged arrays (arrays of arrays) directly

use crate::error::{VBError, VBResult};
use crate::value::VBVariant;

/// Implementation of the `UBound` function.
///
/// Returns the upper bound (largest available subscript) for the specified
/// dimension of an array.
///
/// VB6 behavior:
/// - Default dimension is 1 (first dimension) when omitted
/// - Dimension parameter is 1-based (1 = first dimension, 2 = second, ...)
/// - Raises error 9 (Subscript out of range) if the array is not initialized
///   or if `dimension` exceeds the number of array dimensions
/// - Raises error 9 if `dimension` is less than 1
pub fn ubound(array: &VBVariant, dimension: Option<i32>) -> VBResult<VBVariant> {
    // The dimension parameter is 1-based in VB6; convert to 0-based index
    let dim_index = match dimension {
        None => 0,
        Some(d) if d >= 1 => (d - 1) as usize,
        Some(_) => return Err(VBError::subscript_out_of_range()),
    };

    // Validate that the argument is an array
    let VBVariant::Array(arr) = array else {
        return Err(VBError::type_mismatch());
    };

    let upper = arr.upper_bound(dim_index)?;
    Ok(VBVariant::Long(upper))
}

#[cfg(test)]
mod tests {
    use super::ubound;
    use crate::array::{ArrayDimension, ArrayValue};
    use crate::error::err_number;
    use crate::types::VBType;
    use crate::value::VBVariant;

    fn make_array(bounds: (i32, i32)) -> VBVariant {
        let (lower, upper) = bounds;
        let dim_len = (upper - lower + 1) as usize;
        let data: Vec<VBVariant> = (0..dim_len).map(|_| VBVariant::Empty).collect();
        VBVariant::Array(ArrayValue::from_vec_with_bounds(VBType::Long, data, lower))
    }

    #[test]
    fn default_dimension_returns_first_upper_bound() {
        let arr = make_array((0, 5));
        let result = ubound(&arr, None).unwrap();
        assert_eq!(result, VBVariant::Long(5));
    }

    #[test]
    fn explicit_first_dimension() {
        let arr = make_array((0, 5));
        let result = ubound(&arr, Some(1)).unwrap();
        assert_eq!(result, VBVariant::Long(5));
    }

    #[test]
    fn non_zero_lower_bound() {
        let arr = make_array((5, 10));
        let result = ubound(&arr, None).unwrap();
        assert_eq!(result, VBVariant::Long(10));
    }

    #[test]
    fn negative_upper_bound() {
        let arr = make_array((-5, -2));
        let result = ubound(&arr, None).unwrap();
        assert_eq!(result, VBVariant::Long(-2));
    }

    #[test]
    fn multi_dimensional_first_dim() {
        let dims = [ArrayDimension::new(1, 5), ArrayDimension::new(0, 3)];
        let arr = VBVariant::Array(ArrayValue::new_fixed(VBType::Long, &dims).unwrap());
        let result = ubound(&arr, None).unwrap();
        assert_eq!(result, VBVariant::Long(5));
    }

    #[test]
    fn multi_dimensional_second_dim() {
        let dims = [ArrayDimension::new(1, 5), ArrayDimension::new(0, 3)];
        let arr = VBVariant::Array(ArrayValue::new_fixed(VBType::Long, &dims).unwrap());
        let result = ubound(&arr, Some(2)).unwrap();
        assert_eq!(result, VBVariant::Long(3));
    }

    #[test]
    fn dimension_out_of_range() {
        let dims = [ArrayDimension::new(1, 5), ArrayDimension::new(0, 3)];
        let arr = VBVariant::Array(ArrayValue::new_fixed(VBType::Long, &dims).unwrap());
        let err = ubound(&arr, Some(3)).unwrap_err();
        assert_eq!(err.number, err_number::SUBSCRIPT_OUT_OF_RANGE);
    }

    #[test]
    fn zero_dimension_is_error() {
        let arr = make_array((0, 5));
        let err = ubound(&arr, Some(0)).unwrap_err();
        assert_eq!(err.number, err_number::SUBSCRIPT_OUT_OF_RANGE);
    }

    #[test]
    fn negative_dimension_is_error() {
        let arr = make_array((0, 5));
        let err = ubound(&arr, Some(-1)).unwrap_err();
        assert_eq!(err.number, err_number::SUBSCRIPT_OUT_OF_RANGE);
    }

    #[test]
    fn uninitialized_dynamic_array_errors() {
        let arr = VBVariant::Array(ArrayValue::new_dynamic(VBType::Long));
        let err = ubound(&arr, None).unwrap_err();
        assert_eq!(err.number, err_number::SUBSCRIPT_OUT_OF_RANGE);
    }

    #[test]
    fn non_array_is_type_mismatch() {
        let err = ubound(&VBVariant::from_string("not an array"), None).unwrap_err();
        assert_eq!(err.number, err_number::TYPE_MISMATCH);
    }

    #[test]
    fn paramarray_has_correct_upper_bound() {
        // ParamArray always has LBound = 0 and UBound = n-1
        let data: Vec<VBVariant> = vec![VBVariant::from_string("a"); 3];
        let arr = VBVariant::Array(ArrayValue::from_vec_with_bounds(VBType::Variant, data, 0));
        let result = ubound(&arr, None).unwrap();
        assert_eq!(result, VBVariant::Long(2));
    }
}
