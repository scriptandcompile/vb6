//! # `Int` Function
//!
//! Returns the integer portion of a number.
//!
//! ## Syntax
//!
//! ```vb
//! Int(number)
//! ```
//!
//! ## Parameters
//!
//! - `number` (Required): Any valid numeric expression. If `number` contains `Null`, `Null` is returned
//!
//! ## Return Value
//!
//! Returns the integer portion of a number:
//! - For positive numbers: Returns the largest integer less than or equal to `number`
//! - For negative numbers: Returns the first negative integer less than or equal to `number`
//! - If `number` is `Null`: Returns `Null`
//! - Return type matches the input type (Integer, Long, Single, Double, Currency, Decimal)
//!
//! ## Remarks
//!
//! The `Int` function truncates toward negative infinity:
//!
//! - Removes the fractional part of a number
//! - For positive numbers, behaves like truncation (same as `Fix`)
//! - For negative numbers, rounds DOWN (toward negative infinity)
//! - `Fix` rounds toward zero (always truncates), `Int` rounds down
//! - `Int`(-8.4) returns -9, `Fix`(-8.4) returns -8
//! - `Int`(8.4) returns 8, `Fix`(8.4) returns 8
//! - Does not round to nearest integer (use `Round` for rounding)
//! - The return type preserves the input numeric type
//! - Commonly used with `Rnd` for generating random integers
//! - For currency calculations, consider using `Round` or `CCur` instead
//!
//! ## Typical Uses
//!
//! 1. **Remove Decimals**: Strip fractional part from numbers
//! 2. **Random Integers**: Generate random integer values with `Rnd`
//! 3. **Array Indices**: Convert floats to valid array indices
//! 4. **Loop Counters**: Ensure integer values for loops
//! 5. **Division Results**: Get whole number quotients
//! 6. **Coordinate Rounding**: Round pixel coordinates
//! 7. **Pagination**: Calculate page numbers
//! 8. **Quantity Calculations**: Ensure whole unit quantities
//!
//! ## Basic Usage Examples
//!
//! ```vb
//! ' Example 1: Remove decimal portion
//! Dim result As Integer
//! result = Int(8.7)
//! Debug.Print result  ' Prints: 8
//!
//! ' Example 2: Negative number behavior
//! Dim result As Integer
//! result = Int(-8.7)
//! Debug.Print result  ' Prints: -9 (rounds down, not toward zero)
//!
//! ' Example 3: Random integer between 1 and 100
//! Dim randomNum As Integer
//! Randomize
//! randomNum = Int(Rnd * 100) + 1
//!
//! ' Example 4: Calculate whole pages
//! Dim totalItems As Long
//! Dim itemsPerPage As Long
//! Dim totalPages As Long
//! totalItems = 47
//! itemsPerPage = 10
//! totalPages = Int(totalItems / itemsPerPage) + 1
//! Debug.Print totalPages  ' Prints: 5
//! ```
//!
//! ## Common Patterns
//!
//! ```vb
//! ' Pattern 1: Random integer in range
//! Function RandomInteger(minValue As Long, maxValue As Long) As Long
//!     Randomize
//!     RandomInteger = Int((maxValue - minValue + 1) * Rnd) + minValue
//! End Function
//!
//! ' Pattern 2: Get whole number portion
//! Function GetWholeNumber(value As Double) As Long
//!     If value >= 0 Then
//!         GetWholeNumber = Int(value)
//!     Else
//!         ' For negative numbers, Int rounds down
//!         ' Use Fix if you want to truncate toward zero
//!         GetWholeNumber = Int(value)
//!     End If
//! End Function
//!
//! ' Pattern 3: Calculate pages needed
//! Function CalculatePages(totalItems As Long, itemsPerPage As Long) As Long
//!     If itemsPerPage <= 0 Then
//!         CalculatePages = 0
//!         Exit Function
//!     End If
//!     
//!     CalculatePages = Int((totalItems - 1) / itemsPerPage) + 1
//! End Function
//!
//! ' Pattern 4: Round down to nearest multiple
//! Function RoundDownToMultiple(value As Double, multiple As Double) As Double
//!     If multiple = 0 Then
//!         RoundDownToMultiple = value
//!     Else
//!         RoundDownToMultiple = Int(value / multiple) * multiple
//!     End If
//! End Function
//!
//! ' Pattern 5: Extract integer part for display
//! Function FormatNumber(value As Double) As String
//!     Dim wholePart As Long
//!     Dim decimalPart As Double
//!     
//!     wholePart = Int(Abs(value))
//!     decimalPart = Abs(value) - wholePart
//!     
//!     FormatNumber = CStr(wholePart) & "." & _
//!                    Format$(decimalPart, "00")
//! End Function
//!
//! ' Pattern 6: Generate random array index
//! Function RandomArrayIndex(arr As Variant) As Long
//!     Dim lowerBound As Long
//!     Dim upperBound As Long
//!     
//!     lowerBound = LBound(arr)
//!     upperBound = UBound(arr)
//!     
//!     RandomArrayIndex = Int((upperBound - lowerBound + 1) * Rnd) + lowerBound
//! End Function
//!
//! ' Pattern 7: Calculate grid position
//! Sub GetGridPosition(pixelX As Double, pixelY As Double, _
//!                     gridSize As Double, _
//!                     ByRef gridX As Long, ByRef gridY As Long)
//!     gridX = Int(pixelX / gridSize)
//!     gridY = Int(pixelY / gridSize)
//! End Sub
//!
//! ' Pattern 8: Divide and get quotient
//! Function IntegerDivision(dividend As Long, divisor As Long) As Long
//!     If divisor = 0 Then
//!         Err.Raise 11, , "Division by zero"
//!     End If
//!     
//!     IntegerDivision = Int(dividend / divisor)
//! End Function
//!
//! ' Pattern 9: Time to whole seconds
//! Function GetWholeSeconds(timeValue As Double) As Long
//!     Dim secondsDecimal As Double
//!     secondsDecimal = timeValue * 86400  ' Convert days to seconds
//!     GetWholeSeconds = Int(secondsDecimal)
//! End Function
//!
//! ' Pattern 10: Percentage to whole number
//! Function GetWholePercent(value As Double, total As Double) As Long
//!     If total = 0 Then
//!         GetWholePercent = 0
//!     Else
//!         GetWholePercent = Int((value / total) * 100)
//!     End If
//! End Function
//! ```
//!
//! ## Advanced Usage Examples
//!
//! ```vb
//! ' Example 1: Random number generator class
//! Public Class RandomNumberGenerator
//!     Private m_initialized As Boolean
//!     
//!     Private Sub EnsureInitialized()
//!         If Not m_initialized Then
//!             Randomize
//!             m_initialized = True
//!         End If
//!     End Sub
//!     
//!     Public Function NextInteger(minValue As Long, maxValue As Long) As Long
//!         EnsureInitialized
//!         
//!         If minValue > maxValue Then
//!             Err.Raise 5, , "minValue cannot be greater than maxValue"
//!         End If
//!         
//!         NextInteger = Int((maxValue - minValue + 1) * Rnd) + minValue
//!     End Function
//!     
//!     Public Function NextDouble() As Double
//!         EnsureInitialized
//!         NextDouble = Rnd
//!     End Function
//!     
//!     Public Function NextBoolean() As Boolean
//!         EnsureInitialized
//!         NextBoolean = (Int(Rnd * 2) = 1)
//!     End Function
//!     
//!     Public Function Shuffle(arr As Variant) As Variant
//!         Dim i As Long
//!         Dim j As Long
//!         Dim temp As Variant
//!         Dim result() As Variant
//!         
//!         EnsureInitialized
//!         
//!         ' Copy array
//!         ReDim result(LBound(arr) To UBound(arr))
//!         For i = LBound(arr) To UBound(arr)
//!             result(i) = arr(i)
//!         Next i
//!         
//!         ' Fisher-Yates shuffle
//!         For i = UBound(result) To LBound(result) + 1 Step -1
//!             j = Int((i - LBound(result) + 1) * Rnd) + LBound(result)
//!             temp = result(i)
//!             result(i) = result(j)
//!             result(j) = temp
//!         Next i
//!         
//!         Shuffle = result
//!     End Function
//! End Class
//!
//! ' Example 2: Pagination calculator
//! Public Class PaginationHelper
//!     Private m_totalItems As Long
//!     Private m_itemsPerPage As Long
//!     
//!     Public Property Let TotalItems(value As Long)
//!         m_totalItems = value
//!     End Property
//!     
//!     Public Property Let ItemsPerPage(value As Long)
//!         If value <= 0 Then
//!             Err.Raise 5, , "ItemsPerPage must be greater than zero"
//!         End If
//!         m_itemsPerPage = value
//!     End Property
//!     
//!     Public Property Get PageCount() As Long
//!         If m_itemsPerPage = 0 Then
//!             PageCount = 0
//!         Else
//!             PageCount = Int((m_totalItems - 1) / m_itemsPerPage) + 1
//!         End If
//!     End Property
//!     
//!     Public Function GetPageStartIndex(pageNumber As Long) As Long
//!         If pageNumber < 1 Or pageNumber > PageCount Then
//!             GetPageStartIndex = -1
//!         Else
//!             GetPageStartIndex = (pageNumber - 1) * m_itemsPerPage
//!         End If
//!     End Function
//!     
//!     Public Function GetPageEndIndex(pageNumber As Long) As Long
//!         Dim startIndex As Long
//!         startIndex = GetPageStartIndex(pageNumber)
//!         
//!         If startIndex = -1 Then
//!             GetPageEndIndex = -1
//!         Else
//!             GetPageEndIndex = startIndex + m_itemsPerPage - 1
//!             If GetPageEndIndex >= m_totalItems Then
//!                 GetPageEndIndex = m_totalItems - 1
//!             End If
//!         End If
//!     End Function
//!     
//!     Public Function GetPageForItem(itemIndex As Long) As Long
//!         If itemIndex < 0 Or itemIndex >= m_totalItems Then
//!             GetPageForItem = -1
//!         Else
//!             GetPageForItem = Int(itemIndex / m_itemsPerPage) + 1
//!         End If
//!     End Function
//! End Class
//!
//! ' Example 3: Grid coordinate mapper
//! Public Class GridMapper
//!     Private m_cellWidth As Double
//!     Private m_cellHeight As Double
//!     
//!     Public Sub Initialize(cellWidth As Double, cellHeight As Double)
//!         m_cellWidth = cellWidth
//!         m_cellHeight = cellHeight
//!     End Sub
//!     
//!     Public Sub PixelToGrid(pixelX As Double, pixelY As Double, _
//!                           ByRef gridX As Long, ByRef gridY As Long)
//!         gridX = Int(pixelX / m_cellWidth)
//!         gridY = Int(pixelY / m_cellHeight)
//!     End Sub
//!     
//!     Public Sub GridToPixel(gridX As Long, gridY As Long, _
//!                           ByRef pixelX As Double, ByRef pixelY As Double)
//!         pixelX = gridX * m_cellWidth
//!         pixelY = gridY * m_cellHeight
//!     End Sub
//!     
//!     Public Function SnapToGrid(pixelX As Double, pixelY As Double) As Variant
//!         Dim gridX As Long
//!         Dim gridY As Long
//!         Dim snappedX As Double
//!         Dim snappedY As Double
//!         
//!         PixelToGrid pixelX, pixelY, gridX, gridY
//!         GridToPixel gridX, gridY, snappedX, snappedY
//!         
//!         SnapToGrid = Array(snappedX, snappedY)
//!     End Function
//! End Class
//!
//! ' Example 4: Dice roller simulator
//! Public Class DiceRoller
//!     Public Function Roll(sides As Long, Optional count As Long = 1) As Long
//!         Dim i As Long
//!         Dim total As Long
//!         
//!         Randomize
//!         total = 0
//!         
//!         For i = 1 To count
//!             total = total + Int(Rnd * sides) + 1
//!         Next i
//!         
//!         Roll = total
//!     End Function
//!     
//!     Public Function RollMultiple(sides As Long, count As Long) As Collection
//!         Dim i As Long
//!         Dim result As New Collection
//!         
//!         Randomize
//!         
//!         For i = 1 To count
//!             result.Add Int(Rnd * sides) + 1
//!         Next i
//!         
//!         Set RollMultiple = result
//!     End Function
//!     
//!     Public Function RollWithAdvantage(sides As Long) As Long
//!         Dim roll1 As Long
//!         Dim roll2 As Long
//!         
//!         Randomize
//!         roll1 = Int(Rnd * sides) + 1
//!         roll2 = Int(Rnd * sides) + 1
//!         
//!         RollWithAdvantage = IIf(roll1 > roll2, roll1, roll2)
//!     End Function
//! End Class
//! ```
//!
//! ## Error Handling
//!
//! The `Int` function can raise errors or return `Null`:
//!
//! - **Type Mismatch (Error 13)**: If `number` is not a numeric expression
//! - **Invalid use of Null (Error 94)**: If `number` is `Null` and result is assigned to non-Variant
//! - **Overflow (Error 6)**: If result exceeds the range of the target data type
//!
//! ```vb
//! On Error GoTo ErrorHandler
//! Dim result As Long
//! Dim value As Double
//!
//! value = 12.75
//! result = Int(value)
//!
//! Debug.Print "Integer portion: " & result
//! Exit Sub
//!
//! ErrorHandler:
//!     MsgBox "Error in Int: " & Err.Description, vbCritical
//! ```
//!
//! ## Performance Considerations
//!
//! - **Fast Operation**: `Int` is a very fast built-in function
//! - **Type Preservation**: Return type matches input type
//! - **No Rounding**: Faster than `Round` (no complex calculation)
//! - **Alternative**: For truncation toward zero, `Fix` is equivalent for positive numbers
//! - **Currency**: For financial calculations, consider `Round` or `CCur`
//!
//! ## Best Practices
//!
//! 1. **Understand Behavior**: Know that `Int` rounds DOWN (toward negative infinity)
//! 2. **Fix vs Int**: Use `Fix` for truncation toward zero, `Int` for floor operation
//! 3. **Random Numbers**: Always Randomize before using `Rnd` with `Int`
//! 4. **Type Awareness**: Be aware of return type matching input type
//! 5. **Null Handling**: Use Variant if input might be `Null`
//! 6. **Array Bounds**: Ensure `Int` result is within array bounds
//! 7. **Division**: For integer division, consider using \ operator instead
//!
//! ## Comparison with Other Functions
//!
//! | Function | Behavior | Example |
//! |----------|----------|---------|
//! | `Int` | Rounds down (floor) | Int(-8.7) = -9 |
//! | `Fix` | Truncates toward zero | Fix(-8.7) = -8 |
//! | `Round` | Rounds to nearest | Round(-8.7) = -9 |
//! | `CLng` | Converts to Long with rounding | CLng(-8.7) = -9 |
//! | `CInt` | Converts to Integer with rounding | CInt(-8.7) = -9 |
//! | \ | Integer division | -87 \ 10 = -8 |
//!
//! ## Platform and Version Notes
//!
//! - Available in all VB6 versions
//! - Consistent behavior across platforms
//! - Return type matches input numeric type
//! - Different from many languages `int()` which truncates toward zero
//! - Equivalent to `Math.floor()` in many other languages
//!
//! ## Limitations
//!
//! - Does not round to nearest (use Round for that)
//! - Behavior with negative numbers can be unexpected (use Fix for truncation)
//! - Return type depends on input type (can cause overflow)
//! - Cannot specify decimal places (always removes all decimals)
//! - No control over rounding direction (always down)
//!
//! ## Related Functions
//!
//! - `Fix`: Returns integer portion, truncating toward zero
//! - `Round`: Rounds to nearest integer or specified decimal places
//! - `CInt`: Converts to Integer with rounding
//! - `CLng`: Converts to Long with rounding
//! - `Rnd`: Random number generator (often used with Int)
//! - `\`: Integer division operator

use crate::{error::VBResult, value::VBVariant};

/// Implementation of the floor (Int) function.
///
/// VB6 behavior:
/// - `Fix(Null)` returns `Null`
/// - other values are coerced with numeric conversion rules and return `Double`
pub fn int(value: &VBVariant) -> VBResult<VBVariant> {
    if value.is_null() {
        return Ok(VBVariant::Null);
    }

    let numeric = value.as_f64()?;
    Ok(VBVariant::from_double(numeric.floor()))
}

#[cfg(test)]
mod tests {
    use super::int;
    use crate::{error::err_number, value::VBVariant};

    fn assert_approx_eq(actual: f64, expected: f64) {
        if (actual == f64::INFINITY && expected == f64::INFINITY)
            || (actual == f64::NEG_INFINITY && expected == f64::NEG_INFINITY)
        {
            return;
        }

        let diff = (actual - expected).abs();
        assert!(
            diff < 1e-12,
            "expected {expected}, got {actual}, diff {diff}"
        );
    }

    #[test]
    fn returns_null_for_null() {
        assert_eq!(int(&VBVariant::Null).unwrap(), VBVariant::Null);
    }

    #[test]
    fn returns_zero_for_empty() {
        assert_eq!(
            int(&VBVariant::Empty).unwrap(),
            VBVariant::from_double((0.0_f64).floor())
        );
    }

    #[test]
    fn returns_double_for_numeric_inputs() {
        let result = int(&VBVariant::from_byte(5)).unwrap();
        assert_eq!(result, VBVariant::from_double((5_f64).floor()));

        let result = int(&VBVariant::from_integer(-123)).unwrap();
        assert_eq!(result, VBVariant::from_double((-123.0_f64).floor()));

        let result = int(&VBVariant::from_long(-12345)).unwrap();
        assert_eq!(result, VBVariant::from_double((-12345.0_f64).floor()));

        let result = int(&VBVariant::from_single(-12.5)).unwrap();
        assert_eq!(result, VBVariant::from_double((-12.5_f64).floor()));

        let result = int(&VBVariant::from_double(-12.5)).unwrap();
        assert_eq!(result, VBVariant::from_double((-12.5_f64).floor()));

        let result = int(&VBVariant::from_currency_scaled(-12_345)).unwrap();
        assert_eq!(result, VBVariant::from_double((-1.2345_f64).floor()));
    }

    #[test]
    fn returns_expected_values() {
        let VBVariant::Double(v) = int(&VBVariant::from_double(0.0)).unwrap() else {
            panic!("expected Double")
        };
        assert_approx_eq(v, (0.0_f64).floor());

        let VBVariant::Double(v) = int(&VBVariant::from_double(1.0)).unwrap() else {
            panic!("expected Double")
        };
        assert_approx_eq(v, (1.0_f64).floor());

        let VBVariant::Double(v) = int(&VBVariant::from_double(-1.0)).unwrap() else {
            panic!("expected Double")
        };
        assert_approx_eq(v, (-1.0_f64).floor());

        let VBVariant::Double(v) = int(&VBVariant::from_double(-8.4)).unwrap() else {
            panic!("expected Double")
        };
        assert_approx_eq(v, (-8.4_f64).floor());
    }

    #[test]
    fn rejects_non_numeric_values() {
        let err = int(&VBVariant::from_string("not-a-number")).unwrap_err();
        assert_eq!(err.number, err_number::TYPE_MISMATCH);
    }

    #[test]
    fn accepts_numeric_strings() {
        let result = int(&VBVariant::from_string("1.5")).unwrap();
        assert_eq!(result, VBVariant::from_double(1.5_f64.floor()));
    }
}
