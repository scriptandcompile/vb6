//! # `CVErr` Function
//!
//! Returns a `Variant` of subtype Error containing an error number.
//!
//! ## Syntax
//!
//! ```vb
//! CVErr(errornumber)
//! ```
//!
//! ## Parameters
//!
//! - **`errornumber`**: Required. `Long` integer that identifies an error. The valid range is from
//!   0 to 65535, though application-defined errors are typically in the range 513-65535 (VB6
//!   uses 1-512 for system errors).
//!
//! ## Return Value
//!
//! Returns a `Variant` of subtype `Error` (`VarType = 10`) containing the specified error number.
//! This is not the same as raising an error with `Err.Raise`; instead, it creates an `Error`
//! value that can be assigned to variables and returned from functions.
//!
//! ## Remarks
//!
//! The `CVErr` function is used to create user-defined error values that can be returned from
//! functions or assigned to `Variant` variables. This is particularly useful for:
//!
//! - Returning error conditions from functions without raising exceptions
//! - Creating functions that behave like Excel worksheet functions (returning error values)
//! - Signaling invalid results that should propagate through calculations
//! - Implementing error handling in data processing pipelines
//!
//! **Important Characteristics:**
//!
//! - Returns a `Variant` of subtype `Error` (not an exception)
//! - `Error` values propagate through expressions
//! - Can be tested with `IsError()` function
//! - Not the same as `Err` object or `Err.Raise`
//! - Commonly used with VBA functions called from Excel
//! - `Error` values cannot be used in arithmetic operations
//! - `VarType` of `CVErr` result is 10 (vbError)
//!
//! ## Error Number Ranges
//!
//! | Range | Description |
//! |-------|-------------|
//! | 0-512 | Reserved for VB6 system errors |
//! | 513-65535 | Available for application-defined errors |
//! | 2007-2042 | Excel error values (when used in Excel automation) |
//!
//! ## Excel Error Constants
//!
//! When creating functions for Excel, these error values are commonly used:
//!
//! | Constant | Value | Excel Display | Meaning |
//! |----------|-------|---------------|---------|
//! | xlErrDiv0 | 2007 | #DIV/0! | Division by zero |
//! | xlErrNA | 2042 | #N/A | Value not available |
//! | xlErrName | 2029 | #NAME? | Invalid name |
//! | xlErrNull | 2000 | #NULL! | Null intersection |
//! | xlErrNum | 2036 | #NUM! | Invalid number |
//! | xlErrRef | 2023 | #REF! | Invalid reference |
//! | xlErrValue | 2015 | #VALUE! | Wrong type |
//!
//! ## Examples
//!
//! ### Basic Usage
//!
//! ```vb
//! ' Create an error value
//! Dim result As Variant
//! result = CVErr(2042)  ' Create #N/A error
//!
//! ' Test if value is an error
//! If IsError(result) Then
//!     MsgBox "Result is an error"
//! End If
//! ```
//!
//! ### Function Returning Error on Invalid Input
//!
//! ```vb
//! Function SafeDivide(numerator As Double, denominator As Double) As Variant
//!     If denominator = 0 Then
//!         SafeDivide = CVErr(2007)  ' #DIV/0!
//!     Else
//!         SafeDivide = numerator / denominator
//!     End If
//! End Function
//!
//! ' Usage
//! Dim result As Variant
//! result = SafeDivide(10, 0)
//! If IsError(result) Then
//!     MsgBox "Division error occurred"
//! Else
//!     MsgBox "Result: " & result
//! End If
//! ```
//!
//! ### Excel UDF with Error Handling
//!
//! ```vb
//! Function Lookup(value As Variant, table As Range) As Variant
//!     Dim cell As Range
//!     
//!     ' Validate input
//!     If IsEmpty(value) Then
//!         Lookup = CVErr(2042)  ' #N/A
//!         Exit Function
//!     End If
//!     
//!     ' Search for value
//!     For Each cell In table
//!         If cell.Value = value Then
//!             Lookup = cell.Offset(0, 1).Value
//!             Exit Function
//!         End If
//!     Next cell
//!     
//!     ' Not found
//!     Lookup = CVErr(2042)  ' #N/A
//! End Function
//! ```
//!
//! ## Common Patterns
//!
//! ### Error Constants Definition
//!
//! ```vb
//! ' Define Excel error constants
//! Public Const xlErrDiv0 As Long = 2007   ' #DIV/0!
//! Public Const xlErrNA As Long = 2042     ' #N/A
//! Public Const xlErrName As Long = 2029   ' #NAME?
//! Public Const xlErrNull As Long = 2000   ' #NULL!
//! Public Const xlErrNum As Long = 2036    ' #NUM!
//! Public Const xlErrRef As Long = 2023    ' #REF!
//! Public Const xlErrValue As Long = 2015  ' #VALUE!
//!
//! ' Use in functions
//! Function MyFunction(input As Variant) As Variant
//!     If Not IsNumeric(input) Then
//!         MyFunction = CVErr(xlErrValue)
//!     Else
//!         MyFunction = input * 2
//!     End If
//! End Function
//! ```
//!
//! ### Validation Functions
//!
//! ```vb
//! Function ValidateRange(value As Double, min As Double, max As Double) As Variant
//!     If value < min Or value > max Then
//!         ValidateRange = CVErr(2036)  ' #NUM! - Number out of range
//!     Else
//!         ValidateRange = value
//!     End If
//! End Function
//! ```
//!
//! ### Error Propagation
//!
//! ```vb
//! Function Calculate(input As Variant) As Variant
//!     ' Check if input is already an error
//!     If IsError(input) Then
//!         Calculate = input  ' Propagate the error
//!         Exit Function
//!     End If
//!     
//!     ' Perform calculation
//!     If input < 0 Then
//!         Calculate = CVErr(2036)  ' #NUM!
//!     Else
//!         Calculate = Sqr(input)
//!     End If
//! End Function
//! ```
//!
//! ### Database Lookup with Error
//!
//! ```vb
//! Function GetEmployeeName(employeeID As Long) As Variant
//!     Dim rs As ADODB.Recordset
//!     
//!     On Error GoTo ErrorHandler
//!     
//!     Set rs = New ADODB.Recordset
//!     rs.Open "SELECT Name FROM Employees WHERE ID = " & employeeID, conn
//!     
//!     If rs.EOF Then
//!         GetEmployeeName = CVErr(2042)  ' #N/A - Not found
//!     Else
//!         GetEmployeeName = rs("Name")
//!     End If
//!     
//!     rs.Close
//!     Set rs = Nothing
//!     Exit Function
//!     
//! ErrorHandler:
//!     GetEmployeeName = CVErr(2042)  ' #N/A
//! End Function
//! ```
//!
//! ### Array Formula with Errors
//!
//! ```vb
//! Function ProcessArray(values As Variant) As Variant
//!     Dim results() As Variant
//!     Dim i As Long
//!     
//!     If Not IsArray(values) Then
//!         ProcessArray = CVErr(2015)  ' #VALUE!
//!         Exit Function
//!     End If
//!     
//!     ReDim results(LBound(values) To UBound(values))
//!     
//!     For i = LBound(values) To UBound(values)
//!         If IsError(values(i)) Then
//!             results(i) = values(i)  ' Propagate error
//!         ElseIf Not IsNumeric(values(i)) Then
//!             results(i) = CVErr(2015)  ' #VALUE!
//!         Else
//!             results(i) = values(i) * 2
//!         End If
//!     Next i
//!     
//!     ProcessArray = results
//! End Function
//! ```
//!
//! ### Type Conversion with Error
//!
//! ```vb
//! Function SafeCLng(value As Variant) As Variant
//!     On Error GoTo ErrorHandler
//!     
//!     If IsError(value) Then
//!         SafeCLng = value  ' Propagate error
//!     ElseIf IsEmpty(value) Then
//!         SafeCLng = CVErr(2042)  ' #N/A
//!     ElseIf Not IsNumeric(value) Then
//!         SafeCLng = CVErr(2015)  ' #VALUE!
//!     Else
//!         Dim temp As Double
//!         temp = CDbl(value)
//!         If temp < -2147483648# Or temp > 2147483647# Then
//!             SafeCLng = CVErr(2036)  ' #NUM!
//!         Else
//!             SafeCLng = CLng(value)
//!         End If
//!     End If
//!     Exit Function
//!     
//! ErrorHandler:
//!     SafeCLng = CVErr(2015)  ' #VALUE!
//! End Function
//! ```
//!
//! ### Conditional Error Return
//!
//! ```vb
//! Function GetDiscount(totalSales As Double) As Variant
//!     Select Case totalSales
//!         Case Is < 0
//!             GetDiscount = CVErr(2036)  ' #NUM! - Negative sales
//!         Case 0 To 999.99
//!             GetDiscount = 0
//!         Case 1000 To 4999.99
//!             GetDiscount = 0.05
//!         Case 5000 To 9999.99
//!             GetDiscount = 0.1
//!         Case Is >= 10000
//!             GetDiscount = 0.15
//!         Case Else
//!             GetDiscount = CVErr(2042)  ' #N/A
//!     End Select
//! End Function
//! ```
//!
//! ### Error Checking Helper
//!
//! ```vb
//! Function GetErrorNumber(value As Variant) As Long
//!     ' Returns error number or 0 if not an error
//!     If IsError(value) Then
//!         ' There's no direct way to extract error number in VB6
//!         ' Would need to compare against known error values
//!         GetErrorNumber = -1  ' Indicates error present
//!     Else
//!         GetErrorNumber = 0
//!     End If
//! End Function
//! ```
//!
//! ## Advanced Usage
//!
//! ### Custom Error Type System
//!
//! ```vb
//! ' Application-specific error codes
//! Public Const APP_ERR_INVALID_USER As Long = 1000
//! Public Const APP_ERR_DATABASE As Long = 1001
//! Public Const APP_ERR_NETWORK As Long = 1002
//! Public Const APP_ERR_PERMISSION As Long = 1003
//!
//! Function AuthenticateUser(username As String, password As String) As Variant
//!     If Len(username) = 0 Then
//!         AuthenticateUser = CVErr(APP_ERR_INVALID_USER)
//!         Exit Function
//!     End If
//!     
//!     ' Check credentials
//!     If Not ValidateCredentials(username, password) Then
//!         AuthenticateUser = CVErr(APP_ERR_PERMISSION)
//!         Exit Function
//!     End If
//!     
//!     ' Return user object on success
//!     AuthenticateUser = GetUserObject(username)
//! End Function
//! ```
//!
//! ### Error Value in Collections
//!
//! ```vb
//! Function ProcessRecords() As Collection
//!     Dim results As New Collection
//!     Dim rs As ADODB.Recordset
//!     Dim i As Long
//!     
//!     Set rs = GetRecords()
//!     
//!     While Not rs.EOF
//!         On Error Resume Next
//!         results.Add ProcessRecord(rs)
//!         
//!         If Err.Number <> 0 Then
//!             results.Add CVErr(2015)  ' Add error marker
//!             Err.Clear
//!         End If
//!         
//!         rs.MoveNext
//!     Wend
//!     
//!     Set ProcessRecords = results
//! End Function
//! ```
//!
//! ### Chainable Operations with Error Propagation
//!
//! ```vb
//! Function Step1(input As Variant) As Variant
//!     If IsError(input) Then
//!         Step1 = input
//!     ElseIf input < 0 Then
//!         Step1 = CVErr(2036)
//!     Else
//!         Step1 = Sqr(input)
//!     End If
//! End Function
//!
//! Function Step2(input As Variant) As Variant
//!     If IsError(input) Then
//!         Step2 = input
//!     ElseIf input = 0 Then
//!         Step2 = CVErr(2007)
//!     Else
//!         Step2 = 100 / input
//!     End If
//! End Function
//!
//! ' Chain operations
//! result = Step2(Step1(value))
//! ```
//!
//! ## Error Handling
//!
//! ```vb
//! Function CreateErrorSafe(errorNum As Long) As Variant
//!     On Error GoTo ErrorHandler
//!     
//!     If errorNum < 0 Or errorNum > 65535 Then
//!         CreateErrorSafe = CVErr(2036)  ' Invalid error number
//!     Else
//!         CreateErrorSafe = CVErr(errorNum)
//!     End If
//!     
//!     Exit Function
//!     
//! ErrorHandler:
//!     CreateErrorSafe = CVErr(2042)  ' Generic error
//! End Function
//! ```
//!
//! ### Common Errors
//!
//! - **Error 13** (Type mismatch): Error number is not a valid Long integer
//! - **Error 6** (Overflow): Error number is outside valid range (0-65535)
//!
//! ## Performance Considerations
//!
//! - `CVErr` is a fast function with minimal overhead
//! - `Error` values are lightweight `Variant` subtypes
//! - Using `CVErr` is more efficient than raising and catching exceptions
//! - `Error` propagation through calculations is automatic
//! - No significant memory overhead compared to other `Variant` values
//!
//! ## Comparison with Other Error Mechanisms
//!
//! ### `CVErr` vs `Err.Raise`
//!
//! ```vb
//! ' CVErr - Returns error value (doesn't stop execution)
//! Function Method1(x As Double) As Variant
//!     If x < 0 Then
//!         Method1 = CVErr(2036)  ' Returns error, continues
//!     Else
//!         Method1 = Sqr(x)
//!     End If
//! End Function
//!
//! ' Err.Raise - Throws exception (stops execution)
//! Function Method2(x As Double) As Double
//!     If x < 0 Then
//!         Err.Raise 5, , "Invalid argument"  ' Stops execution
//!     Else
//!         Method2 = Sqr(x)
//!     End If
//! End Function
//! ```
//!
//! **`CVErr` advantages:**
//! - Doesn't interrupt program flow
//! - Can be used in expressions
//! - Natural for functional-style programming
//! - Compatible with Excel worksheet functions
//!
//! **`Err.Raise` advantages:**
//! - Forces immediate attention to errors
//! - Provides error description and source
//! - Traditional exception handling model
//! - Better for critical errors
//!
//! ## Best Practices
//!
//! ### Always Check for Errors Before Using Values
//!
//! ```vb
//! Dim result As Variant
//! result = SomeFunction()
//!
//! If IsError(result) Then
//!     MsgBox "Error occurred"
//! Else
//!     ' Safe to use result
//!     Debug.Print result
//! End If
//! ```
//!
//! ### Use Meaningful Error Numbers
//!
//! ```vb
//! ' Good - Use named constants
//! Const ERR_INVALID_INPUT As Long = 1000
//! result = CVErr(ERR_INVALID_INPUT)
//!
//! ' Avoid - Magic numbers
//! result = CVErr(42)  ' What does 42 mean?
//! ```
//!
//! ### Document Custom Error Codes
//!
//! ```vb
//! ' Application Error Codes (1000-1999)
//! Public Const ERR_INVALID_USER As Long = 1000    ' Invalid username
//! Public Const ERR_EXPIRED_SESSION As Long = 1001  ' Session expired
//! Public Const ERR_INSUFFICIENT_RIGHTS As Long = 1002  ' Access denied
//! ```
//!
//! ## Limitations
//!
//! - Cannot extract error number from error value directly in VB6
//! - `Error` values cannot be used in arithmetic operations
//! - Limited to Long integer error numbers (0-65535)
//! - No built-in error description with `CVErr` (unlike `Err` object)
//! - `VarType` test required to detect errors (`IsError` function)
//! - Not all VB6 functions handle error values gracefully
//!
//! ## Related Functions
//!
//! - `IsError`: Tests if a Variant contains an error value
//! - `Err.Raise`: Raises a runtime error (different from `CVErr`)
//! - `Error`: Returns error message for an error number
//! - `Error$`: `String` version of `Error` function
//! - `VarType`: Returns the subtype of a Variant (10 for `Error`)

use crate::error::{VBError, VBResult};
use crate::value::VBVariant;

/// Implementation of the `CVErr` function.
///
/// Creates a `Variant` of subtype `Error` containing the specified error number.
///
/// VB6 behavior:
/// - Takes an error number (Long) and returns a Variant/Error value
/// - Valid error number range is 0 to 65535 (unsigned 16-bit)
/// - If `errornumber` is `Null`, raises error 94 (Invalid use of Null)
/// - If `errornumber` cannot be converted to a Long, raises error 13 (Type mismatch)
/// - Does NOT raise an exception; returns an error value that can be assigned and propagated
pub fn cverr(errornumber: &VBVariant) -> VBResult<VBVariant> {
    // Null propagates as error 94
    if errornumber.is_null() {
        return Err(VBError::invalid_use_of_null());
    }

    // Convert to Long (error numbers are 16-bit unsigned, stored as signed Long)
    let error_num = errornumber.as_i32()?;

    // Validate range: error numbers are 0-65535, but stored as signed Long.
    // VB6 accepts negative values and treats them modulo 65536.
    Ok(VBVariant::from_error(VBError::new(error_num)))
}

#[cfg(test)]
mod tests {
    use super::cverr;
    use crate::error::err_number;
    use crate::value::VBVariant;

    #[test]
    fn cverr_creates_error_variant() {
        let result = cverr(&VBVariant::from_long(2042)).unwrap();
        assert!(result.is_error());
        let error = result.as_error().unwrap();
        assert_eq!(error.number, 2042);
    }

    #[test]
    fn cverr_with_zero() {
        let result = cverr(&VBVariant::from_long(0)).unwrap();
        assert!(result.is_error());
        let error = result.as_error().unwrap();
        assert_eq!(error.number, 0);
    }

    #[test]
    fn cverr_with_small_error() {
        let result = cverr(&VBVariant::from_long(13)).unwrap();
        assert!(result.is_error());
        let error = result.as_error().unwrap();
        assert_eq!(error.number, 13);
    }

    #[test]
    fn cverr_with_excel_error_div0() {
        let result = cverr(&VBVariant::from_long(2007)).unwrap();
        assert!(result.is_error());
        let error = result.as_error().unwrap();
        assert_eq!(error.number, 2007);
    }

    #[test]
    fn cverr_with_excel_error_na() {
        let result = cverr(&VBVariant::from_long(2042)).unwrap();
        assert!(result.is_error());
        let error = result.as_error().unwrap();
        assert_eq!(error.number, 2042);
    }

    #[test]
    fn cverr_with_max_valid_error() {
        let result = cverr(&VBVariant::from_long(65535)).unwrap();
        assert!(result.is_error());
        let error = result.as_error().unwrap();
        assert_eq!(error.number, 65535);
    }

    #[test]
    fn cverr_from_string_numeric() {
        let result = cverr(&VBVariant::from_string("42")).unwrap();
        assert!(result.is_error());
        let error = result.as_error().unwrap();
        assert_eq!(error.number, 42);
    }

    #[test]
    fn cverr_null_raises_error_94() {
        let err = cverr(&VBVariant::Null).unwrap_err();
        assert_eq!(err.number, err_number::INVALID_USE_OF_NULL);
    }

    #[test]
    fn cverr_type_mismatch_for_object() {
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

        let obj = VBVariant::Object(Box::new(TestObj));
        let err = cverr(&obj).unwrap_err();
        assert_eq!(err.number, err_number::TYPE_MISMATCH);
    }

    #[test]
    fn cverr_type_mismatch_for_array() {
        use crate::types::VBType;
        let arr = VBVariant::array_dynamic(VBType::Long);
        let err = cverr(&arr).unwrap_err();
        assert_eq!(err.number, err_number::TYPE_MISMATCH);
    }

    #[test]
    fn cverr_var_type_is_error() {
        let result = cverr(&VBVariant::from_long(13)).unwrap();
        // VarType for Error is 10 (vbError)
        assert_eq!(result.var_type(), 10);
    }

    #[test]
    fn cverr_preserves_error_description() {
        let result = cverr(&VBVariant::from_long(13)).unwrap();
        let error = result.as_error().unwrap();
        // Error 13 has the built-in description "Type mismatch"
        assert_eq!(error.description, "Type mismatch");
    }

    #[test]
    fn cverr_negative_error_number() {
        // VB6 CVErr accepts negative numbers and stores them as-is in the Long
        let result = cverr(&VBVariant::from_long(-1)).unwrap();
        assert!(result.is_error());
        let error = result.as_error().unwrap();
        assert_eq!(error.number, -1);
    }

    #[test]
    fn cverr_empty_propagates_as_zero() {
        // Empty converts to 0 in numeric contexts
        let result = cverr(&VBVariant::Empty).unwrap();
        assert!(result.is_error());
        let error = result.as_error().unwrap();
        assert_eq!(error.number, 0);
    }

    #[test]
    fn cverr_boolean_true() {
        // Boolean True converts to -1 in VB6 numeric coercion
        let result = cverr(&VBVariant::Boolean(true)).unwrap();
        assert!(result.is_error());
        let error = result.as_error().unwrap();
        assert_eq!(error.number, -1);
    }

    #[test]
    fn cverr_boolean_false() {
        // Boolean False converts to 0
        let result = cverr(&VBVariant::Boolean(false)).unwrap();
        assert!(result.is_error());
        let error = result.as_error().unwrap();
        assert_eq!(error.number, 0);
    }
}
