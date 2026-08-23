//! VB6 `TypeName` Function
//!
//! The `TypeName` function returns a string that provides information about the data type of a variable or expression.
//!
//! ## Syntax
//! ```vb6
//! TypeName(varname)
//! ```
//!
//! ## Parameters
//! - `varname`: Required. Name of a variable or expression whose data type is to be determined.
//!
//! ## Returns
//! Returns a `String` describing the data type of the variable or expression. Common return values include:
//! - "Boolean"
//! - "Byte"
//! - "Integer"
//! - "Long"
//! - "Single"
//! - "Double"
//! - "Currency"
//! - "Date"
//! - "String"
//! - "Object"
//! - "Error"
//! - "Empty"
//! - "Null"
//! - "Nothing"
//! - "Variant"
//! - "Unknown"
//! - Custom class or user-defined type names
//!
//! ## Remarks
//! - Returns the type as a string, not the actual type.
//! - For objects, returns the class name or interface name.
//! - For arrays, returns the base type name with "()" appended (e.g., "`Integer()`", "`String()`").
//! - For objects not instantiated, returns "Nothing".
//! - For Null, returns "Null"; for Empty, returns "Empty".
//! - For user-defined types, returns the type name.
//! - For Variant variables, returns the underlying type.
//! - Useful for debugging, logging, and type checking at runtime.
//! - Not case-sensitive.
//!
//! ## Typical Uses
//! 1. Debugging variable types
//! 2. Logging type information
//! 3. Type checking in generic code
//! 4. Handling Variant variables
//! 5. Validating function arguments
//! 6. Reflection-like operations
//! 7. Error handling and reporting
//! 8. Determining array types
//!
//! ## Basic Examples
//!
//! ### Example 1: Get type of variable
//! ```vb6
//! Dim x As Integer
//! MsgBox TypeName(x) ' "Integer"
//! ```
//!
//! ### Example 2: Get type of object
//! ```vb6
//! Dim c As Collection
//! Set c = New Collection
//! MsgBox TypeName(c) ' "Collection"
//! ```
//!
//! ### Example 3: Get type of array
//! ```vb6
//! Dim arr(1 To 5) As String
//! MsgBox TypeName(arr) ' "String()"
//! ```
//!
//! ### Example 4: Get type of Variant
//! ```vb6
//! Dim v As Variant
//! v = 123
//! MsgBox TypeName(v) ' "Integer"
//! ```
//!
//! ## Common Patterns
//!
//! ### Pattern 1: Check for array
//! ```vb6
//! If Right$(TypeName(var), 2) = "()" Then
//!     MsgBox "It's an array!"
//! End If
//! ```
//!
//! ### Pattern 2: Check for object
//! ```vb6
//! If TypeName(obj) = "Nothing" Then
//!     MsgBox "Object not set!"
//! End If
//! ```
//!
//! ### Pattern 3: Handle Variant types
//! ```vb6
//! If TypeName(v) = "String" Then
//!     ' Handle string
//! End If
//! ```
//!
//! ### Pattern 4: Log variable types
//! ```vb6
//! Debug.Print "Type: " & TypeName(x)
//! ```
//!
//! ### Pattern 5: Validate argument type
//! ```vb6
//! Sub Foo(arg As Variant)
//!     If TypeName(arg) <> "String" Then Err.Raise 5
//! End Sub
//! ```
//!
//! ### Pattern 6: Reflection-like usage
//! ```vb6
//! Dim t As String
//! t = TypeName(obj)
//! If t = "MyClass" Then
//!     ' Do something
//! End If
//! ```
//!
//! ### Pattern 7: Handle Null and Empty
//! ```vb6
//! If TypeName(v) = "Null" Then
//!     ' Handle Null
//! ElseIf TypeName(v) = "Empty" Then
//!     ' Handle Empty
//! End If
//! ```
//!
//! ### Pattern 8: Array type detection
//! ```vb6
//! If InStr(TypeName(arr), "()") > 0 Then
//!     Debug.Print "Array of type: " & Left$(TypeName(arr), Len(TypeName(arr)) - 2)
//! End If
//! ```
//!
//! ### Pattern 9: User-defined type
//! ```vb6
//! Type MyType
//!     x As Integer
//! End Type
//! Dim t As MyType
//! MsgBox TypeName(t) ' "MyType"
//! ```
//!
//! ### Pattern 10: Class type detection
//! ```vb6
//! If TypeName(obj) = "MyClass" Then
//!     ' Handle MyClass
//! End If
//! ```
//!
//! ## Advanced Usage
//!
//! ### Example 1: Type checking in generic function
//! ```vb6
//! Function IsString(val As Variant) As Boolean
//!     IsString = (TypeName(val) = "String")
//! End Function
//! ```
//!
//! ### Example 2: Logging all argument types
//! ```vb6
//! Sub LogTypes(ParamArray args() As Variant)
//!     Dim i As Integer
//!     For i = LBound(args) To UBound(args)
//!         Debug.Print "Arg " & i & ": " & TypeName(args(i))
//!     Next i
//! End Sub
//! ```
//!
//! ### Example 3: Reflection for class methods
//! ```vb6
//! If TypeName(obj) = "MyClass" Then
//!     obj.SpecialMethod
//! End If
//! ```
//!
//! ### Example 4: Variant array detection
//! ```vb6
//! Dim v As Variant
//! v = Array(1, 2, 3)
//! If Right$(TypeName(v), 2) = "()" Then
//!     Debug.Print "Variant array"
//! End If
//! ```
//!
//! ## Error Handling
//! - Returns "Unknown" for unsupported types.
//! - Returns "Nothing" for uninitialized object variables.
//! - Returns "Null" for Null values.
//! - Returns "Empty" for uninitialized variables.
//!
//! ## Performance Notes
//! - Fast, constant time O(1).
//! - No side effects.
//!
//! ## Best Practices
//! 1. Use for debugging and logging.
//! 2. Do not use for strict type enforcement.
//! 3. Handle "Nothing", "Null", and "Empty" cases.
//! 4. Use with Variant variables for type safety.
//! 5. Use for generic code and utilities.
//! 6. Document expected type strings.
//! 7. Use with arrays for type detection.
//! 8. Avoid using as a substitute for type declarations.
//! 9. Use for runtime checks, not compile-time.
//! 10. Combine with `VarType` for more detail.
//!
//! ## Comparison Table
//!
//! | Function   | Purpose                | Input      | Returns        |
//! |------------|------------------------|------------|----------------|
//! | `TypeName` | Get type as string     | variable   | String         |
//! | `VarType`  | Get type as constant   | variable   | Integer        |
//! | `IsObject` | Check if is object     | variable   | Boolean        |
//! | `IsArray`  | Check if is array      | variable   | Boolean        |
//!
//! ## Platform Notes
//! - Available in VB6, VBA, `VBScript`
//! - Consistent across platforms
//! - Returns type names in English
//!
//! ## Limitations
//! - Returns only type name as string
//! - Not locale-sensitive
//! - Returns "Unknown" for unsupported types
//! - Not for compile-time type checking
//! - May return user-defined type/class names

use crate::{error::VBResult, value::VBVariant};

/// Implementation of the `TypeName` function.
///
/// VB6 behavior:
/// - returns `"Empty"` for uninitialized `Variant`s and `"Null"` for `Null`
/// - returns `"Nothing"` for an object reference that is not set
/// - returns the object's class name (e.g. `"Collection"`) for object references
/// - returns the element type name followed by `()` for arrays
///   (e.g. `"Integer()"`, `"Variant()"`)
/// - never raises an error, regardless of the input value
pub fn type_name(value: &VBVariant) -> VBResult<VBVariant> {
    let name = match value {
        VBVariant::Empty => "Empty",
        VBVariant::Null => "Null",
        VBVariant::Nothing => "Nothing",
        VBVariant::Byte(_) => "Byte",
        VBVariant::Integer(_) => "Integer",
        VBVariant::Long(_) => "Long",
        VBVariant::Single(_) => "Single",
        VBVariant::Double(_) => "Double",
        VBVariant::Currency(_) => "Currency",
        VBVariant::Date(_) => "Date",
        VBVariant::String(_) => "String",
        VBVariant::Boolean(_) => "Boolean",
        VBVariant::Error(_) => "Error",
        VBVariant::Object(object) => object.type_name(),
        VBVariant::Array(array) => {
            return Ok(VBVariant::from_string(format!(
                "{}()",
                array.element_type().name()
            )));
        }
    };
    Ok(VBVariant::from_string(name))
}

#[cfg(test)]
mod tests {
    use super::type_name;
    use crate::{value::VBVariant, ArrayDimension, ArrayValue, VBType};
    use vb6core::error::err_number;

    #[derive(Debug)]
    struct TestObject(&'static str);

    impl crate::VBObject for TestObject {
        fn type_name(&self) -> &str {
            self.0
        }
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
        fn clone_box(&self) -> Box<dyn crate::VBObject> {
            Box::new(TestObject(self.0))
        }
    }

    #[test]
    fn names_for_value_states() {
        assert_eq!(
            type_name(&VBVariant::Empty).unwrap(),
            VBVariant::from_string("Empty")
        );
        assert_eq!(
            type_name(&VBVariant::Null).unwrap(),
            VBVariant::from_string("Null")
        );
        assert_eq!(
            type_name(&VBVariant::Nothing).unwrap(),
            VBVariant::from_string("Nothing")
        );
    }

    #[test]
    fn names_for_primitives() {
        assert_eq!(
            type_name(&VBVariant::from_byte(0)).unwrap(),
            VBVariant::from_string("Byte")
        );
        assert_eq!(
            type_name(&VBVariant::from_integer(-5)).unwrap(),
            VBVariant::from_string("Integer")
        );
        assert_eq!(
            type_name(&VBVariant::from_long(12345)).unwrap(),
            VBVariant::from_string("Long")
        );
        assert_eq!(
            type_name(&VBVariant::from_single(1.5)).unwrap(),
            VBVariant::from_string("Single")
        );
        assert_eq!(
            type_name(&VBVariant::from_double(-2.5)).unwrap(),
            VBVariant::from_string("Double")
        );
        assert_eq!(
            type_name(&VBVariant::from_currency_scaled(-12_345)).unwrap(),
            VBVariant::from_string("Currency")
        );
        assert_eq!(
            type_name(&VBVariant::from_date_serial(0.0)).unwrap(),
            VBVariant::from_string("Date")
        );
        assert_eq!(
            type_name(&VBVariant::from_string("")).unwrap(),
            VBVariant::from_string("String")
        );
        assert_eq!(
            type_name(&VBVariant::from_bool(false)).unwrap(),
            VBVariant::from_string("Boolean")
        );
        assert_eq!(
            type_name(&VBVariant::from_error(crate::error::VBError::new(
                err_number::TYPE_MISMATCH
            )))
            .unwrap(),
            VBVariant::from_string("Error")
        );
    }

    #[test]
    fn names_object_by_its_class() {
        assert_eq!(
            type_name(&VBVariant::from_object(Box::new(TestObject("Collection")))).unwrap(),
            VBVariant::from_string("Collection")
        );
    }

    #[test]
    fn appends_parentheses_for_arrays() {
        let fixed = VBVariant::array_fixed(VBType::Integer, &[ArrayDimension::new(1, 3)]).unwrap();
        assert_eq!(
            type_name(&fixed).unwrap(),
            VBVariant::from_string("Integer()")
        );
        assert_eq!(
            type_name(&VBVariant::array_dynamic(VBType::String)).unwrap(),
            VBVariant::from_string("String()")
        );
        let variant_array =
            VBVariant::from_array(ArrayValue::from_vec(VBType::Variant, Vec::new()));
        assert_eq!(
            type_name(&variant_array).unwrap(),
            VBVariant::from_string("Variant()")
        );
    }

    #[test]
    fn never_errors() {
        assert!(type_name(&VBVariant::Empty).is_ok());
        assert!(type_name(&VBVariant::Null).is_ok());
        assert!(type_name(&VBVariant::Nothing).is_ok());
        assert!(type_name(&VBVariant::from_string("test")).is_ok());
    }
}
