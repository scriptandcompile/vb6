//! ## `Array` Function
//!
//! Returns a `Variant` containing an array formed from the comma-delimited arg list of values
//! passed into the function.
//!
//! ## Syntax
//!
//! ```text
//! Array(arglist)
//! ```
//!
//! ## Parameters
//!
//! - `arglist`: Required. A comma-delimited list of values that are assigned to the elements
//!   of the array contained within the `Variant`. If no arguments are specified, an array of zero
//!   length is created.
//!
//! ## Return Value
//!
//! Returns a `Variant` whose subtype is `Array` containing the specified elements.
//!
//! ## Remarks
//!
//! - `Variant Array`: The `Array` function returns a `Variant` that contains an array. The array
//!   elements are `Variants` that can hold any data type.
//! - `Zero-Based`: The array created by the `Array` function is zero-based. The first element
//!   has an index of 0.
//! - `Dynamic Size`: The size of the array is determined by the number of arguments provided.
//! - `Mixed Types`: `Array` elements can be of different types since they are stored as `Variants`.
//! - `Assignment`: The result must be assigned to a `Variant` variable, not an array declared
//!   with specific dimensions.
//! - `Empty Array`: Calling `Array()` with no arguments creates a zero-length array.
//! - `LBound and UBound`: You can use `LBound` and `UBound` to determine the array bounds.
//!   `LBound` always returns 0, `UBound` returns (number of elements - 1).
//! - `Option Base`: The `Array` function is not affected by `Option Base` statements; it always
//!   creates zero-based arrays.
//!
//! ## Important Characteristics
//!
//! ### Assignment Requirements
//!
//! ```vb
//! ' Correct - assign to Variant
//! Dim v As Variant
//! v = Array(1, 2, 3)  ' OK
//!
//! ' Incorrect - cannot assign to typed array
//! Dim arr(2) As Integer
//! arr = Array(1, 2, 3)  ' ERROR: Type mismatch
//! ```
//!
//! ### Zero-Based Indexing
//!
//! ```vb
//! Dim arr As Variant
//! arr = Array("A", "B", "C")
//! Debug.Print LBound(arr)  ' Always 0
//! Debug.Print UBound(arr)  ' 2 (not 3!)
//!
//! ' First element is arr(0), last is arr(2)
//! ```
//!
//! ### Performance Considerations
//!
//! - `Array()` creates a `Variant` array, which has more overhead than typed arrays
//! - For large arrays with known types, consider using `ReDim` instead
//! - `Array()` is best for small, temporary arrays or mixed-type collections
//! - Each element is a `Variant`, which uses more memory than native types
//!
//! ## Related Functions
//!
//! - `Split`: Splits a string into an array of substrings
//! - `Join`: Concatenates array elements into a string
//! - `LBound`: Returns the lowest available subscript for an array dimension
//! - `UBound`: Returns the highest available subscript for an array dimension
//! - `IsArray`: Determines whether a variable is an array
//! - `Filter`: Returns a zero-based array containing a subset of a string array
//!
//! ## Examples
//!
//! ### Basic Array Creation
//!
//! ```vb
//! Dim myArray As Variant
//! myArray = Array(1, 2, 3, 4, 5)
//! ' myArray contains: [1, 2, 3, 4, 5]
//! ' LBound(myArray) = 0, UBound(myArray) = 4
//! ```
//!
//! ### Mixed Data Types
//!
//! ```vb
//! Dim mixed As Variant
//! mixed = Array("Hello", 42, True, #1/1/2025#, 3.14)
//! ' Array can hold different types
//! ```
//!
//! ### String Array
//!
//! ```vb
//! Dim names As Variant
//! names = Array("Alice", "Bob", "Charlie")
//! Debug.Print names(0)  ' Prints: Alice
//! ```
//!
//! ### Empty Array
//!
//! ```vb
//! Dim emptyArr As Variant
//! emptyArr = Array()
//! ' Creates a zero-length array
//! ' UBound(emptyArr) = -1
//! ```
//!
//! ### Using For Each
//!
//! ```vb
//! Dim values As Variant
//! values = Array(10, 20, 30, 40)
//!
//! Dim item As Variant
//! For Each item In values
//!     Debug.Print item
//! Next item
//! ```
//!
//! ### Array as Function Return
//!
//! ```vb
//! Function GetColors() As Variant
//!     GetColors = Array("Red", "Green", "Blue")
//! End Function
//! ```
//!
//! ### Accessing Elements
//!
//! ```vb
//! Dim data As Variant
//! data = Array("A", "B", "C")
//! Debug.Print data(0)  ' A
//! Debug.Print data(1)  ' B
//! Debug.Print data(2)  ' C
//! ```
//!
//! ## Common Patterns
//!
//! ### Initialize Lookup Table
//!
//! ```vb
//! Function GetMonthName(monthNum As Integer) As String
//!     Dim months As Variant
//!     months = Array("Jan", "Feb", "Mar", "Apr", "May", "Jun", _
//!                    "Jul", "Aug", "Sep", "Oct", "Nov", "Dec")
//!     
//!     If monthNum >= 1 And monthNum <= 12 Then
//!         GetMonthName = months(monthNum - 1)
//!     Else
//!         GetMonthName = ""
//!     End If
//! End Function
//! ```
//!
//! ### Configuration Data
//!
//! ```vb
//! Sub ProcessFiles()
//!     Dim extensions As Variant
//!     extensions = Array(".txt", ".doc", ".pdf", ".xls")
//!     
//!     Dim ext As Variant
//!     For Each ext In extensions
//!         ProcessFileType CStr(ext)
//!     Next ext
//! End Sub
//! ```
//!
//! ### Quick Test Data
//!
//! ```vb
//! Sub TestFunction()
//!     Dim testCases As Variant
//!     testCases = Array(0, 1, 10, 100, -1, -100)
//!     
//!     Dim testValue As Variant
//!     For Each testValue In testCases
//!         Debug.Print "Testing: " & testValue & " -> " & MyFunction(testValue)
//!     Next testValue
//! End Sub
//! ```
//!
//! ### Passing Multiple Values
//!
//! ```vb
//! Sub UpdateRecord()
//!     SaveData Array("Name", "John"), _
//!             Array("Age", 30), _
//!             Array("City", "NYC")
//! End Sub
//!
//! Sub SaveData(ParamArray fields())
//!     Dim field As Variant
//!     For Each field In fields
//!         Debug.Print field(0) & ": " & field(1)
//!     Next field
//! End Sub
//! ```
//!
//! ### Enumeration Substitute
//!
//! ```vb
//! Function GetStatusText(status As Integer) As String
//!     Dim statuses As Variant
//!     statuses = Array("Pending", "Processing", "Complete", "Failed")
//!     
//!     If status >= 0 And status <= 3 Then
//!         GetStatusText = statuses(status)
//!     Else
//!         GetStatusText = "Unknown"
//!     End If
//! End Function
//! ```
//!
//! ### Split Alternative (VB6 Early Versions)
//!
//! ```vb
//! ' Before Split function was widely available
//! Function GetHeaderFields() As Variant
//!     GetHeaderFields = Array("ID", "Name", "Date", "Status")
//! End Function
//! ```
//!
//! ### Matrix/Grid Data
//!
//! ```vb
//! Sub CreateGrid()
//!     Dim row1 As Variant, row2 As Variant, row3 As Variant
//!     row1 = Array(1, 2, 3)
//!     row2 = Array(4, 5, 6)
//!     row3 = Array(7, 8, 9)
//!     
//!     Dim grid As Variant
//!     grid = Array(row1, row2, row3)
//!     
//!     ' Access: grid(0)(0) = 1, grid(1)(2) = 6, etc.
//! End Sub
//! ```
//!
//! ### Default Values
//!
//! ```vb
//! Function GetDefaults() As Variant
//!     GetDefaults = Array(0, "", False, Null, Empty)
//! End Function
//! ```
//!
//! ## Error Handling
//!
//! The `Array` function does not typically raise errors:
//!
//! - An empty argument list returns an empty array (not an error)
//! - Nested arrays are supported (arrays containing arrays)
//! - All argument types are accepted without restriction
//!
//! ## Performance Notes
//!
//! - Fast operation with O(n) complexity where n is the number of elements
//! - Creates a new array each time; consider caching for repeated use
//! - Best suited for small collections (typically < 100 elements)
//! - For larger datasets, consider using `ReDim` or other data structures
//!
//! ## Platform Notes
//!
//! - Available in VB6, VBA, and VBScript
//! - Always creates zero-based arrays regardless of `Option Base` setting
//! - Returns a `Variant` subtype array (not a strongly-typed array)
//! - Behavior is consistent across all VB platforms

use crate::array::ArrayValue;
use crate::types::VBType;
use crate::value::VBVariant;

/// Implementation of the `Array` function.
///
/// Creates a zero-based Variant array from the given elements.
///
/// VB6 behavior:
/// - The resulting array is always zero-based (`LBound` = 0)
/// - Elements are stored as-is (no type coercion)
/// - An empty argument list produces an empty array with `UBound` = -1
/// - Nested arrays are supported
pub fn array(elements: &[VBVariant]) -> VBVariant {
    if elements.is_empty() {
        // Empty array: bounds 0 To -1 (no elements)
        return VBVariant::Array(ArrayValue::from_vec_with_bounds(
            VBType::Variant,
            Vec::new(),
            0,
        ));
    }

    VBVariant::Array(ArrayValue::from_vec_with_bounds(
        VBType::Variant,
        elements.to_vec(),
        0,
    ))
}

#[cfg(test)]
mod tests {
    use super::array;
    use crate::error::{err_number, VBError};
    use crate::types::VBType;
    use crate::value::VBVariant;

    #[test]
    fn creates_zero_based_array() {
        let elements = vec![VBVariant::Long(1), VBVariant::Long(2), VBVariant::Long(3)];
        let result = array(&elements);

        assert!(result.is_array());
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 3);
        assert_eq!(arr.lower_bound(0).unwrap(), 0);
        assert_eq!(arr.upper_bound(0).unwrap(), 2);
    }

    #[test]
    fn preserves_element_values() {
        let elements = vec![
            VBVariant::from_string("A"),
            VBVariant::Long(42),
            VBVariant::Boolean(true),
        ];
        let result = array(&elements);
        let arr = result.as_array().unwrap();

        assert_eq!(arr.get(&[0]).unwrap(), &VBVariant::from_string("A"));
        assert_eq!(arr.get(&[1]).unwrap(), &VBVariant::Long(42));
        assert_eq!(arr.get(&[2]).unwrap(), &VBVariant::Boolean(true));
    }

    #[test]
    fn empty_array_has_negative_ubound() {
        let result = array(&[]);
        let arr = result.as_array().unwrap();

        assert_eq!(arr.lower_bound(0).unwrap(), 0);
        assert_eq!(arr.upper_bound(0).unwrap(), -1);
        assert!(arr.is_empty());
    }

    #[test]
    fn single_element_array() {
        let elements = vec![VBVariant::from_string("only")];
        let result = array(&elements);
        let arr = result.as_array().unwrap();

        assert_eq!(arr.lower_bound(0).unwrap(), 0);
        assert_eq!(arr.upper_bound(0).unwrap(), 0);
        assert_eq!(arr.get(&[0]).unwrap(), &VBVariant::from_string("only"));
    }

    #[test]
    fn mixed_types_in_array() {
        let elements = vec![
            VBVariant::Empty,
            VBVariant::Null,
            VBVariant::Long(1),
            VBVariant::from_string("text"),
            VBVariant::Boolean(false),
            VBVariant::Date(45672.0),
        ];
        let result = array(&elements);
        let arr = result.as_array().unwrap();

        assert_eq!(arr.get(&[0]).unwrap(), &VBVariant::Empty);
        assert_eq!(arr.get(&[1]).unwrap(), &VBVariant::Null);
        assert_eq!(arr.get(&[2]).unwrap(), &VBVariant::Long(1));
        assert_eq!(arr.get(&[3]).unwrap(), &VBVariant::from_string("text"));
        assert_eq!(arr.get(&[4]).unwrap(), &VBVariant::Boolean(false));
        assert_eq!(arr.get(&[5]).unwrap(), &VBVariant::Date(45672.0));
    }

    #[test]
    fn nested_arrays() {
        let inner = array(&[VBVariant::Long(1), VBVariant::Long(2)]);
        let elements = vec![inner, VBVariant::Long(3)];
        let result = array(&elements);
        let outer = result.as_array().unwrap();

        assert_eq!(outer.len(), 2);
        assert!(outer.get(&[0]).unwrap().is_array());
    }

    #[test]
    fn result_is_a_variant_array() {
        let result = array(&[VBVariant::from_string("x")]);
        let arr = result.as_array().unwrap();
        assert_eq!(arr.element_type(), &VBType::Variant);
    }

    #[test]
    fn preserves_all_variant_kinds() {
        let elements = vec![
            VBVariant::Empty,
            VBVariant::Null,
            VBVariant::Nothing,
            VBVariant::Byte(255),
            VBVariant::Integer(-1),
            VBVariant::Long(-2_000_000_000),
            VBVariant::Single(1.5),
            VBVariant::Double(123.456),
            VBVariant::from_currency(1234.56),
            VBVariant::from_string("text"),
            VBVariant::Boolean(true),
            VBVariant::Date(45_672.5),
            VBVariant::Error(VBError::new(err_number::TYPE_MISMATCH)),
        ];
        let result = array(&elements);
        let arr = result.as_array().unwrap();

        assert_eq!(arr.len(), elements.len());
        for (i, expected) in elements.iter().enumerate() {
            assert_eq!(arr.get(&[i as i32]).unwrap(), expected, "index {i}");
        }
    }

    #[test]
    fn result_is_independent_of_input() {
        let mut elements = vec![VBVariant::from_string("a"), VBVariant::Long(1)];
        let result = array(&elements);

        elements[0] = VBVariant::from_string("changed");
        elements[1] = VBVariant::Long(99);

        let arr = result.as_array().unwrap();
        assert_eq!(arr.get(&[0]).unwrap(), &VBVariant::from_string("a"));
        assert_eq!(arr.get(&[1]).unwrap(), &VBVariant::Long(1));
    }

    #[test]
    fn large_array_has_correct_bounds() {
        let elements: Vec<VBVariant> = (0..100).map(VBVariant::Long).collect();
        let result = array(&elements);
        let arr = result.as_array().unwrap();

        assert_eq!(arr.len(), 100);
        assert_eq!(arr.lower_bound(0).unwrap(), 0);
        assert_eq!(arr.upper_bound(0).unwrap(), 99);
        assert_eq!(arr.get(&[0]).unwrap(), &VBVariant::Long(0));
        assert_eq!(arr.get(&[50]).unwrap(), &VBVariant::Long(50));
        assert_eq!(arr.get(&[99]).unwrap(), &VBVariant::Long(99));
    }

    #[test]
    fn deeply_nested_arrays_are_supported() {
        let inner = array(&[VBVariant::Long(1)]);
        let middle = array(&[inner]);
        let result = array(&[middle]);

        let outer = result.as_array().unwrap();
        assert!(outer.get(&[0]).unwrap().is_array());
    }
}
