//! VB6 `Tan` Function
//!
//! The `Tan` function returns the tangent of an angle specified in radians.
//!
//! ## Syntax
//! ```vb6
//! Tan(number)
//! ```
//!
//! ## Parameters
//! - `number`: Required. A numeric expression representing an angle in radians.
//!
//! ## Returns
//! Returns a `Double` representing the tangent of the angle.
//!
//! ## Remarks
//! - The argument must be in radians, not degrees. To convert degrees to radians, multiply by `Pi/180`.
//! - Returns a `Double` value.
//! - If the argument is a multiple of π/2 (except 0), the result is undefined (overflow error).
//! - Returns Null if the argument is Null.
//! - Use `Atn` to get the arctangent (inverse tangent).
//! - The tangent function is periodic with period π.
//! - For very large or very small arguments, floating-point rounding may affect results.
//!
//! ## Typical Uses
//! 1. Trigonometric calculations
//! 2. Geometry and graphics
//! 3. Physics and engineering formulas
//! 4. Calculating slopes and angles
//! 5. Animation and simulation
//! 6. Signal processing
//! 7. Scientific computation
//! 8. Converting between coordinate systems
//!
//! ## Basic Examples
//!
//! ### Example 1: Tangent of 45 degrees
//! ```vb6
//! result = Tan(45 * 3.14159265358979 / 180)
//! ' result = 1
//! ```
//!
//! ### Example 2: Tangent of Pi/4 radians
//! ```vb6
//! result = Tan(3.14159265358979 / 4)
//! ' result = 1
//! ```
//!
//! ### Example 3: Using with Atn
//! ```vb6
//! angle = Atn(1)
//! result = Tan(angle)
//! ' result = 1
//! ```
//!
//! ### Example 4: Handling Null
//! ```vb6
//! result = Tan(Null)
//! ' result = Null
//! ```
//!
//! ## Common Patterns
//!
//! ### Pattern 1: Convert degrees to radians
//! ```vb6
//! Function DegreesToRadians(degrees As Double) As Double
//!     DegreesToRadians = degrees * 3.14159265358979 / 180
//! End Function
//! result = Tan(DegreesToRadians(60))
//! ```
//!
//! ### Pattern 2: Calculate slope from angle
//! ```vb6
//! Function SlopeFromAngle(angleRadians As Double) As Double
//!     SlopeFromAngle = Tan(angleRadians)
//! End Function
//! ```
//!
//! ### Pattern 3: Use in triangle calculations
//! ```vb6
//! Function OppositeFromAdjacent(adjacent As Double, angleRadians As Double) As Double
//!     OppositeFromAdjacent = adjacent * Tan(angleRadians)
//! End Function
//! ```
//!
//! ### Pattern 4: Animation rotation
//! ```vb6
//! angle = t * 3.14159265358979 / 180
//! y = Tan(angle) * x
//! ```
//!
//! ### Pattern 5: Periodic function
//! ```vb6
//! For i = 0 To 360 Step 45
//!     Debug.Print Tan(i * 3.14159265358979 / 180)
//! Next i
//! ```
//!
//! ### Pattern 6: Error handling for undefined values
//! ```vb6
//! On Error Resume Next
//! result = Tan(3.14159265358979 / 2)
//! If Err.Number <> 0 Then
//!     Debug.Print "Overflow error"
//! End If
//! On Error GoTo 0
//! ```
//!
//! ### Pattern 7: Use with arrays
//! ```vb6
//! For i = LBound(arr) To UBound(arr)
//!     arr(i) = Tan(arr(i))
//! Next i
//! ```
//!
//! ### Pattern 8: Inverse calculation
//! ```vb6
//! angle = Atn(Tan(x))
//! ```
//!
//! ### Pattern 9: Normalize angle
//! ```vb6
//! angle = angle Mod (2 * 3.14159265358979)
//! result = Tan(angle)
//! ```
//!
//! ### Pattern 10: Use in coordinate conversion
//! ```vb6
//! y = r * Tan(theta)
//! ```
//!
//! ## Advanced Usage
//!
//! ### Example 1: Trigonometric Table
//! ```vb6
//! For i = 0 To 90 Step 15
//!     Debug.Print "Tan(" & i & ") = " & Tan(i * 3.14159265358979 / 180)
//! Next i
//! ```
//!
//! ### Example 2: Slope Calculation
//! ```vb6
//! Function Slope(degrees As Double) As Double
//!     Slope = Tan(degrees * 3.14159265358979 / 180)
//! End Function
//! ```
//!
//! ### Example 3: Handling Undefined Values
//! ```vb6
//! On Error Resume Next
//! result = Tan(3.14159265358979 / 2)
//! If Err.Number <> 0 Then
//!     result = Null
//! End If
//! On Error GoTo 0
//! ```
//!
//! ### Example 4: Use in Physics Formula
//! ```vb6
//! ' Calculate projectile height
//! height = distance * Tan(angleRadians)
//! ```
//!
//! ## Error Handling
//! - Returns Null if argument is Null.
//! - Overflow error if argument is a multiple of π/2 (except 0).
//!
//! ## Performance Notes
//! - Fast, constant time O(1).
//! - Floating-point rounding may affect results for large/small arguments.
//!
//! ## Best Practices
//! 1. Always use radians, not degrees.
//! 2. Convert degrees to radians as needed.
//! 3. Handle possible overflow for undefined values.
//! 4. Use error handling for edge cases.
//! 5. Test with a range of values.
//! 6. Use with Atn for inverse calculations.
//! 7. Document expected input range.
//! 8. Avoid using with multiples of π/2.
//! 9. Use with arrays for batch calculations.
//! 10. Normalize angles for periodicity.
//!
//! ## Comparison Table
//!
//! | Function | Purpose | Input | Returns |
//! |----------|---------|-------|---------|
//! | `Tan`    | Tangent | radians | Double |
//! | `Atn`    | Arctangent | number | Double |
//! | `Sin`    | Sine | radians | Double |
//! | `Cos`    | Cosine | radians | Double |
//!
//! ## Platform Notes
//! - Available in VB6, VBA, `VBScript`
//! - Consistent across platforms
//! - Returns Double
//!
//! ## Limitations
//! - Argument must be in radians
//! - Undefined for odd multiples of π/2 (except 0)
//! - Returns Null for Null input
//! - No support for complex numbers
//! - Floating-point rounding errors possible

use crate::{error::VBResult, value::VBVariant};

/// Implementation of the tangent (Tan) function.
///
/// VB6 behavior:
/// - `Tan(Null)` returns `Null`
/// - other values are coerced with numeric conversion rules and return `Double`
pub fn tan(value: &VBVariant) -> VBResult<VBVariant> {
    if value.is_null() {
        return Ok(VBVariant::Null);
    }

    let numeric = value.as_f64()?;
    Ok(VBVariant::from_double(numeric.tan()))
}

#[cfg(test)]
mod tests {
    use super::tan;
    use crate::{error::err_number, value::VBVariant};

    fn assert_approx_eq(actual: f64, expected: f64) {
        let diff = (actual - expected).abs();
        assert!(
            diff < 1e-12,
            "expected {expected}, got {actual}, diff {diff}"
        );
    }

    #[test]
    fn returns_null_for_null() {
        assert_eq!(tan(&VBVariant::Null).unwrap(), VBVariant::Null);
    }

    #[test]
    fn returns_zero_for_empty() {
        assert_eq!(tan(&VBVariant::Empty).unwrap(), VBVariant::from_double(0.0));
    }

    #[test]
    fn returns_double_for_numeric_inputs() {
        let result = tan(&VBVariant::from_byte(5)).unwrap();
        assert_eq!(result, VBVariant::from_double((5.0_f64).tan()));

        let result = tan(&VBVariant::from_integer(-123)).unwrap();
        assert_eq!(result, VBVariant::from_double((-123.0_f64).tan()));

        let result = tan(&VBVariant::from_long(-12345)).unwrap();
        assert_eq!(result, VBVariant::from_double((-12345.0_f64).tan()));

        let result = tan(&VBVariant::from_single(-12.5)).unwrap();
        assert_eq!(result, VBVariant::from_double((-12.5_f64).tan()));

        let result = tan(&VBVariant::from_double(-12.5)).unwrap();
        assert_eq!(result, VBVariant::from_double((-12.5_f64).tan()));

        let result = tan(&VBVariant::from_currency_scaled(-12_345)).unwrap();
        assert_eq!(result, VBVariant::from_double((-1.2345_f64).tan()));
    }

    #[test]
    fn returns_expected_special_angles() {
        let VBVariant::Double(v) = tan(&VBVariant::from_double(0.0)).unwrap() else {
            panic!("expected Double")
        };
        assert_approx_eq(v, 0.0);

        let VBVariant::Double(v) = tan(&VBVariant::from_double(1.0)).unwrap() else {
            panic!("expected Double")
        };
        assert_approx_eq(v, (1.0_f64).tan());

        let VBVariant::Double(v) = tan(&VBVariant::from_double(-1.0)).unwrap() else {
            panic!("expected Double")
        };
        assert_approx_eq(v, (-1.0_f64).tan());
    }

    #[test]
    fn rejects_non_numeric_values() {
        let err = tan(&VBVariant::from_string("not-a-number")).unwrap_err();
        assert_eq!(err.number, err_number::TYPE_MISMATCH);
    }

    #[test]
    fn accepts_numeric_strings() {
        let result = tan(&VBVariant::from_string("1.5")).unwrap();
        assert_eq!(result, VBVariant::from_double((1.5_f64).tan()));
    }
}
