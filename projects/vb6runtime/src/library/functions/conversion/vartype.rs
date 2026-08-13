//! # `VarType` Function
//!
//! Returns an integer constant indicating the subtype of a Variant variable or expression.
//!
//! ## Syntax
//!
//! ```vb6
//! VarType(varname)
//! ```
//!
//! ## Parameters
//!
//! - `varname`: Required. Name of a variable or expression whose Variant subtype is to be determined.
//!
//! ## Returns
//!
//! Returns an `Integer` constant representing the Variant subtype. Common return values:
//! - 0 - vbEmpty (uninitialized)
//! - 1 - vbNull (Null)
//! - 2 - vbInteger
//! - 3 - vbLong
//! - 4 - vbSingle
//! - 5 - vbDouble
//! - 6 - vbCurrency
//! - 7 - vbDate
//! - 8 - vbString
//! - 9 - vbObject
//! - 10 - vbError
//! - 11 - vbBoolean
//! - 12 - vbVariant (for arrays)
//! - 13 - vbDataObject
//! - 14 - vbDecimal
//! - 17 - vbByte
//! - 20 - vbLongLong (64-bit platforms only).
//! - 36 - vbUserDefinedType
//! - 8192 - vbArray (bitwise OR with base type)
//! - ...other values are possible.
//!
//! ## Remarks
//!
//! - Returns a numeric constant, not a string.
//! - For arrays, returns vbArray (8192) bitwise OR'd with the base type (e.g., vbArray + vbInteger = 8194).
//! - For objects, returns vbObject (9) or vbDataObject (13).
//! - For user-defined types, returns vbUserDefinedType (36).
//! - For Empty, returns vbEmpty (0); for Null, returns vbNull (1).
//! - For non-Variant variables, returns the corresponding type constant.
//! - Useful for type checking, debugging, and generic code.
//! - Use with `TypeName` for string representation.
//!
//! ## Typical Uses
//!
//! 1. Type checking in generic code
//! 2. Handling Variant variables
//! 3. Debugging and logging
//! 4. Validating function arguments
//! 5. Detecting arrays and base types
//! 6. Reflection-like operations
//! 7. Error handling and reporting
//! 8. Working with COM objects
//!
//! ## Basic Examples
//!
//! ### Example 1: Get `VarType` of Integer
//!
//! ```vb6
//! Dim x As Integer
//! Debug.Print VarType(x) ' 2 (vbInteger)
//! ```
//!
//! ### Example 2: Get `VarType` of String
//!
//! ```vb6
//! Dim s As String
//! Debug.Print VarType(s) ' 8 (vbString)
//! ```
//!
//! ### Example 3: Get `VarType` of Array
//!
//! ```vb6
//! Dim arr(1 To 5) As Double
//! Debug.Print VarType(arr) ' 8197 (vbArray OR'd with vbDouble)
//! ```
//!
//! ### Example 4: Get `VarType` of Variant
//!
//! ```vb6
//! Dim v As Variant
//! v = 123
//! Debug.Print VarType(v) ' 2 (vbInteger)
//! ```
//!
//! ## Common Patterns
//!
//! ### Pattern 1: Check for array
//!
//! ```vb6
//! If VarType(var) And vbArray Then
//!     Debug.Print "It's an array!"
//! End If
//! ```
//!
//! ### Pattern 2: Check for string
//!
//! ```vb6
//! If VarType(x) = vbString Then
//!     ' Handle string
//! End If
//! ```
//!
//! ### Pattern 3: Handle Variant types
//!
//! ```vb6
//! If VarType(v) = vbInteger Then
//!     ' Handle integer
//! End If
//! ```
//!
//! ### Pattern 4: Log variable types
//!
//! ```vb6
//! Debug.Print "VarType: " & VarType(x)
//! ```
//!
//! ### Pattern 5: Validate argument type
//!
//! ```vb6
//! Sub Foo(arg As Variant)
//!     If VarType(arg) <> vbString Then Err.Raise 5
//! End Sub
//! ```
//!
//! ### Pattern 6: Reflection-like usage
//!
//! ```vb6
//! Dim t As Integer
//! t = VarType(obj)
//! If t = vbObject Then
//!     ' Do something
//! End If
//! ```
//!
//! ### Pattern 7: Handle Null and Empty
//!
//! ```vb6
//! If VarType(v) = vbNull Then
//!     ' Handle Null
//! ElseIf VarType(v) = vbEmpty Then
//!     ' Handle Empty
//! End If
//! ```
//!
//! ### Pattern 8: Array type detection
//!
//! ```vb6
//! If (VarType(arr) And vbArray) Then
//!     Debug.Print "Array base type: " & (VarType(arr) - vbArray)
//! End If
//! ```
//!
//! ### Pattern 9: User-defined type
//!
//! ```vb6
//! Type MyType
//!     x As Integer
//! End Type
//! Dim t As MyType
//! Debug.Print VarType(t) ' 36 (vbUserDefinedType)
//! ```
//!
//! ### Pattern 10: Class type detection
//!
//! ```vb6
//! If VarType(obj) = vbObject Then
//!     ' Handle object
//! End If
//! ```
//!
//! ## Advanced Usage
//!
//! ### Example 1: Type checking in generic function
//!
//! ```vb6
//! Function IsString(val As Variant) As Boolean
//!     IsString = (VarType(val) = vbString)
//! End Function
//! ```
//!
//! ### Example 2: Logging all argument types
//!
//! ```vb6
//! Sub LogTypes(ParamArray args() As Variant)
//!     Dim i As Integer
//!     For i = LBound(args) To UBound(args)
//!         Debug.Print "Arg " & i & ": " & VarType(args(i))
//!     Next i
//! End Sub
//! ```
//!
//! ### Example 3: Reflection for class methods
//!
//! ```vb6
//! If VarType(obj) = vbObject Then
//!     obj.SpecialMethod
//! End If
//! ```
//!
//! ### Example 4: Variant array detection
//!
//! ```vb6
//! Dim v As Variant
//! v = Array(1, 2, 3)
//! If (VarType(v) And vbArray) Then
//!     Debug.Print "Variant array"
//! End If
//! ```
//!
//! ## Error Handling
//!
//! - Returns vbError (10) for error values.
//! - Returns vbUnknown (0) for unsupported types.
//! - Returns vbEmpty (0) for uninitialized variables.
//! - Returns vbNull (1) for Null values.
//!
//! ## Performance Notes
//!
//! - Fast, constant time O(1).
//! - No side effects.
//!
//! ## Best Practices
//!
//! 1. Use for debugging and logging.
//! 2. Use bitwise AND with vbArray to detect arrays.
//! 3. Use with `TypeName` for string representation.
//! 4. Handle vbNull, vbEmpty, and vbError cases.
//! 5. Use for generic code and utilities.
//! 6. Document expected type constants.
//! 7. Use for runtime checks, not compile-time.
//! 8. Combine with `TypeName` for more detail.
//! 9. Use for Variant and object variables.
//! 10. Avoid using as a substitute for type declarations.
//!
//! ## Comparison Table
//!
//! | Function   | Purpose                | Input      | Returns        |
//! |------------|------------------------|------------|----------------|
//! | `VarType`  | Get type as constant   | variable   | Integer        |
//! | `TypeName` | Get type as string     | variable   | String         |
//! | `IsObject` | Check if is object     | variable   | Boolean        |
//! | `IsArray`  | Check if is array      | variable   | Boolean        |
//!
//! ## Platform Notes
//!
//! - Available in VB6, VBA, `VBScript`
//! - Consistent across platforms
//! - Returns type constants in English
//!
//! ## Limitations
//!
//! - Returns only type constant as integer
//! - Not locale-sensitive
//! - Returns vbUnknown (0) for unsupported types
//! - Not for compile-time type checking
//! - May return user-defined type/class constants

use crate::error::VBResult;
use crate::value::VBVariant;

/// Implementation of the `VarType` function.
///
/// Returns an `Integer` value indicating the subtype of a Variant variable
/// or expression.
///
/// VB6 behavior:
/// - Takes any value and returns its VarType code as an Integer (i16)
/// - Never raises an error; always succeeds
/// - For arrays, returns 8192 (vbArray) OR'd with the element type code
/// - For Empty, returns 0; for Null, returns 1
/// - Returns 12 (vbVariant) when the argument itself is a Variant holding another Variant
pub fn var_type(value: &VBVariant) -> VBResult<VBVariant> {
    Ok(VBVariant::from_long(value.var_type()))
}

#[cfg(test)]
mod tests {
    use super::var_type;
    use crate::{ArrayDimension, VBObject, VBType, VBVariant};

    #[derive(Debug)]
    struct TestObject(&'static str);

    impl VBObject for TestObject {
        fn type_name(&self) -> &str {
            self.0
        }
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
        fn clone_box(&self) -> Box<dyn VBObject> {
            Box::new(TestObject(self.0))
        }
    }

    #[test]
    fn vartype_empty() {
        assert_eq!(
            var_type(&VBVariant::Empty).unwrap(),
            VBVariant::from_long(0)
        );
    }

    #[test]
    fn vartype_null() {
        assert_eq!(var_type(&VBVariant::Null).unwrap(), VBVariant::from_long(1));
    }

    #[test]
    fn vartype_byte() {
        assert_eq!(
            var_type(&VBVariant::from_byte(42)).unwrap(),
            VBVariant::from_long(17)
        );
    }

    #[test]
    fn vartype_integer() {
        assert_eq!(
            var_type(&VBVariant::from_integer(-12345)).unwrap(),
            VBVariant::from_long(2)
        );
    }

    #[test]
    fn vartype_long() {
        assert_eq!(
            var_type(&VBVariant::from_long(12345678)).unwrap(),
            VBVariant::from_long(3)
        );
    }

    #[test]
    fn vartype_single() {
        assert_eq!(
            var_type(&VBVariant::from_single(std::f32::consts::PI)).unwrap(),
            VBVariant::from_long(4)
        );
    }

    #[test]
    fn vartype_double() {
        assert_eq!(
            var_type(&VBVariant::from_double(std::f64::consts::E)).unwrap(),
            VBVariant::from_long(5)
        );
    }

    #[test]
    fn vartype_currency() {
        assert_eq!(
            var_type(&VBVariant::from_currency_scaled(12345678)).unwrap(),
            VBVariant::from_long(6)
        );
    }

    #[test]
    fn vartype_string() {
        assert_eq!(
            var_type(&VBVariant::from_string("hello")).unwrap(),
            VBVariant::from_long(8)
        );
    }

    #[test]
    fn vartype_boolean_true() {
        assert_eq!(
            var_type(&VBVariant::from_bool(true)).unwrap(),
            VBVariant::from_long(11)
        );
    }

    #[test]
    fn vartype_boolean_false() {
        assert_eq!(
            var_type(&VBVariant::from_bool(false)).unwrap(),
            VBVariant::from_long(11)
        );
    }

    #[test]
    fn vartype_date() {
        assert_eq!(
            var_type(&VBVariant::from_date_serial(45000.0)).unwrap(),
            VBVariant::from_long(7)
        );
    }

    #[test]
    fn vartype_error() {
        assert_eq!(
            var_type(&VBVariant::from_error(crate::error::VBError::new(13))).unwrap(),
            VBVariant::from_long(10)
        );
    }

    #[test]
    fn vartype_object() {
        let obj = VBVariant::from_object(Box::new(TestObject("Test")));
        assert_eq!(var_type(&obj).unwrap(), VBVariant::from_long(9));
    }

    #[test]
    fn vartype_nothing() {
        assert_eq!(
            var_type(&VBVariant::Nothing).unwrap(),
            VBVariant::from_long(9)
        );
    }

    #[test]
    fn vartype_array_integer() {
        let arr = VBVariant::array_fixed(VBType::Integer, &[ArrayDimension::new(0, 4)]).unwrap();
        // vbArray (8192) | vbInteger (2) = 8194
        assert_eq!(var_type(&arr).unwrap(), VBVariant::from_long(8194));
    }

    #[test]
    fn vartype_array_double() {
        let arr = VBVariant::array_fixed(VBType::Double, &[ArrayDimension::new(0, 9)]).unwrap();
        // vbArray (8192) | vbDouble (5) = 8197
        assert_eq!(var_type(&arr).unwrap(), VBVariant::from_long(8197));
    }

    #[test]
    fn vartype_array_string() {
        let arr = VBVariant::array_fixed(VBType::String, &[ArrayDimension::new(1, 5)]).unwrap();
        // vbArray (8192) | vbString (8) = 8200
        assert_eq!(var_type(&arr).unwrap(), VBVariant::from_long(8200));
    }

    #[test]
    fn vartype_dynamic_array() {
        let arr = VBVariant::array_dynamic(VBType::Long);
        // vbArray (8192) | vbLong (3) = 8195
        assert_eq!(var_type(&arr).unwrap(), VBVariant::from_long(8195));
    }

    #[test]
    fn vartype_returns_integer_type() {
        let result = var_type(&VBVariant::from_string("test")).unwrap();
        // The result itself should be a Long (VarType returns Integer, which is i32 in our representation)
        assert_eq!(result.var_type(), 3); // vbLong
    }

    #[test]
    fn vartype_never_errors() {
        // VarType never raises an error for any input
        assert!(var_type(&VBVariant::Empty).is_ok());
        assert!(var_type(&VBVariant::Null).is_ok());
        assert!(var_type(&VBVariant::Nothing).is_ok());
        assert!(var_type(&VBVariant::from_error(crate::error::VBError::new(13))).is_ok());
        assert!(var_type(&VBVariant::from_string("test")).is_ok());
    }
}
