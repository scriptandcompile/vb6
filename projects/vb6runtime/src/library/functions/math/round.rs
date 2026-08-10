//! # Round Function
//!
//! Returns a number rounded to a specified number of decimal places.
//!
//! ## Syntax
//!
//! ```vb
//! Round(expression, [numdecimalplaces])
//! ```
//!
//! ## Parameters
//!
//! - `expression` - Required. Numeric expression being rounded.
//! - `numdecimalplaces` - Optional. Number indicating how many places to the right of the decimal are included in the rounding. If omitted, integers are returned.
//!
//! ## Return Value
//!
//! Returns a value of the same type as `expression` that has been rounded to the specified number of decimal places.
//!
//! ## Remarks
//!
//! The `Round` function rounds numbers using "banker's rounding" (round to even), also known as "round half to even". This differs from the typical "round half up" method taught in schools.
//!
//! **Important Notes**:
//! - Uses banker's rounding (round to nearest even number when exactly halfway)
//! - If `numdecimalplaces` is omitted, returns an integer
//! - If `numdecimalplaces` is 0, rounds to nearest integer
//! - If `numdecimalplaces` is positive, rounds to that many decimal places
//! - If `numdecimalplaces` is negative, error 5 (Invalid procedure call or argument)
//! - Returns a `Double` rounded to the specified number of decimal places
//!
//! **Banker's Rounding Examples**:
//! - Round(2.5) = 2 (rounds to even)
//! - Round(3.5) = 4 (rounds to even)
//! - Round(4.5) = 4 (rounds to even)
//! - Round(5.5) = 6 (rounds to even)
//!
//! **Rounding Behavior by numdecimalplaces**:
//!
//! | numdecimalplaces | Effect | Example |
//! |------------------|--------|---------|
//! | Omitted | Round to integer | Round(2.7) = 3 |
//! | 0 | Round to integer | Round(2.7, 0) = 3 |
//! | Positive (e.g., 2) | Round to N decimals | Round(2.748, 2) = 2.75 |
//! | Negative (e.g., -1) | Error 5 | Round(2748, -1) raises error 5 |
//!
//! ## Typical Uses
//!
//! 1. **Financial Calculations**: Round currency values to 2 decimal places
//! 2. **Display Formatting**: Round numbers for user display
//! 3. **Statistical Analysis**: Round computed values to significant digits
//! 4. **Data Normalization**: Standardize precision across datasets
//! 5. **Measurement Values**: Round sensor readings to appropriate precision
//! 6. **Grade Calculation**: Round student scores to whole numbers
//! 7. **Percentage Display**: Round percentages to desired precision
//! 8. **Scientific Notation**: Round to significant figures
//!
//! ## Basic Examples
//!
//! ### Example 1: Round to Integer
//! ```vb
//! Dim value As Double
//! Dim rounded As Integer
//!
//! value = 3.7
//! rounded = Round(value)  ' Returns 4
//! ```
//!
//! ### Example 2: Round Currency
//! ```vb
//! Dim price As Double
//! Dim roundedPrice As Double
//!
//! price = 12.3456
//! roundedPrice = Round(price, 2)  ' Returns 12.35
//! ```
//!
//! ### Example 3: Banker's Rounding
//! ```vb
//! ' Demonstrates round-to-even behavior
//! Dim result1 As Integer
//! Dim result2 As Integer
//!
//! result1 = Round(2.5)  ' Returns 2 (rounds to even)
//! result2 = Round(3.5)  ' Returns 4 (rounds to even)
//! ```
//!
//! ### Example 4: Invalid NumDecimalPlaces
//! ```vb
//! ' Negative numdecimalplaces raise error 5
//! Dim value As Long
//! Dim roundedToTens As Long
//!
//! On Error Resume Next
//! value = 2748
//! roundedToTens = Round(value, -1)  ' Raises error 5 (Invalid procedure call or argument)
//! If Err.Number = 5 Then
//!     MsgBox "Round does not accept negative numdecimalplaces"
//! End If
//! On Error GoTo 0
//! ```
//!
//! ## Common Patterns
//!
//! ### Pattern 1: `RoundCurrency`
//! ```vb
//! Function RoundCurrency(amount As Double) As Double
//!     ' Round to 2 decimal places for currency
//!     RoundCurrency = Round(amount, 2)
//! End Function
//! ```
//!
//! ### Pattern 2: `RoundPercentage`
//! ```vb
//! Function RoundPercentage(percentage As Double, _
//!                         Optional decimals As Integer = 1) As Double
//!     ' Round percentage to specified decimal places
//!     RoundPercentage = Round(percentage, decimals)
//! End Function
//! ```
//!
//! ### Pattern 3: `RoundToSignificantFigures`
//! ```vb
//! Function RoundToSignificantFigures(value As Double, _
//!                                   sigFigs As Integer) As Double
//!     ' Round to specified number of significant figures
//!     Dim magnitude As Double
//!     Dim factor As Double
//!     
//!     If value = 0 Then
//!         RoundToSignificantFigures = 0
//!         Exit Function
//!     End If
//!     
//!     magnitude = Int(Log(Abs(value)) / Log(10))
//!     factor = 10 ^ (sigFigs - magnitude - 1)
//!     
//!     RoundToSignificantFigures = Round(value * factor) / factor
//! End Function
//! ```
//!
//! ### Pattern 4: `RoundUpAlways`
//! ```vb
//! Function RoundUpAlways(value As Double, decimals As Integer) As Double
//!     ' Always round up (ceiling behavior)
//!     Dim factor As Double
//!     
//!     factor = 10 ^ decimals
//!     
//!     If value > 0 Then
//!         RoundUpAlways = Int(value * factor + 0.9999999) / factor
//!     Else
//!         RoundUpAlways = Int(value * factor) / factor
//!     End If
//! End Function
//! ```
//!
//! ### Pattern 5: `RoundDownAlways`
//! ```vb
//! Function RoundDownAlways(value As Double, decimals As Integer) As Double
//!     ' Always round down (floor behavior)
//!     Dim factor As Double
//!     
//!     factor = 10 ^ decimals
//!     
//!     If value > 0 Then
//!         RoundDownAlways = Int(value * factor) / factor
//!     Else
//!         RoundDownAlways = Int(value * factor - 0.9999999) / factor
//!     End If
//! End Function
//! ```
//!
//! ### Pattern 6: `RoundToNearest`
//! ```vb
//! Function RoundToNearest(value As Double, nearest As Double) As Double
//!     ' Round to nearest multiple of a number
//!     ' e.g., RoundToNearest(47, 5) = 45
//!     RoundToNearest = Round(value / nearest) * nearest
//! End Function
//! ```
//!
//! ### Pattern 7: `RoundArray`
//! ```vb
//! Sub RoundArray(arr() As Double, decimals As Integer)
//!     ' Round all elements in an array
//!     Dim i As Integer
//!     
//!     For i = LBound(arr) To UBound(arr)
//!         arr(i) = Round(arr(i), decimals)
//!     Next i
//! End Sub
//! ```
//!
//! ### Pattern 8: `RoundIfNeeded`
//! ```vb
//! Function RoundIfNeeded(value As Double, decimals As Integer, _
//!                       threshold As Double) As Double
//!     ' Only round if difference from rounded value exceeds threshold
//!     Dim rounded As Double
//!     
//!     rounded = Round(value, decimals)
//!     
//!     If Abs(value - rounded) > threshold Then
//!         RoundIfNeeded = rounded
//!     Else
//!         RoundIfNeeded = value
//!     End If
//! End Function
//! ```
//!
//! ### Pattern 9: `RoundForDisplay`
//! ```vb
//! Function RoundForDisplay(value As Double) As String
//!     ' Round and format for display based on magnitude
//!     If Abs(value) < 0.01 Then
//!         RoundForDisplay = Format(Round(value, 4), "0.0000")
//!     ElseIf Abs(value) < 1 Then
//!         RoundForDisplay = Format(Round(value, 3), "0.000")
//!     ElseIf Abs(value) < 100 Then
//!         RoundForDisplay = Format(Round(value, 2), "0.00")
//!     Else
//!         RoundForDisplay = Format(Round(value, 0), "0")
//!     End If
//! End Function
//! ```
//!
//! ### Pattern 10: `SymmetricRound`
//! ```vb
//! Function SymmetricRound(value As Double, decimals As Integer) As Double
//!     ' Traditional "round half up" instead of banker's rounding
//!     Dim factor As Double
//!     Dim shifted As Double
//!     
//!     factor = 10 ^ decimals
//!     shifted = value * factor
//!     
//!     If shifted >= 0 Then
//!         SymmetricRound = Int(shifted + 0.5) / factor
//!     Else
//!         SymmetricRound = Int(shifted - 0.5) / factor
//!     End If
//! End Function
//! ```
//!
//! ## Advanced Usage
//!
//! ### Example 1: Financial Calculator
//! ```vb
//! ' Precise financial calculations with proper rounding
//! Class FinancialCalculator
//!     Private m_precision As Integer
//!     
//!     Public Sub Initialize(Optional precision As Integer = 2)
//!         m_precision = precision
//!     End Sub
//!     
//!     Public Function CalculateInterest(principal As Double, _
//!                                      rate As Double, _
//!                                      periods As Integer) As Double
//!         ' Calculate simple interest with rounding
//!         Dim interest As Double
//!         
//!         interest = principal * rate * periods
//!         CalculateInterest = Round(interest, m_precision)
//!     End Function
//!     
//!     Public Function CalculatePayment(loanAmount As Double, _
//!                                     interestRate As Double, _
//!                                     numPayments As Integer) As Double
//!         ' Calculate loan payment with rounding
//!         Dim monthlyRate As Double
//!         Dim payment As Double
//!         
//!         monthlyRate = interestRate / 12
//!         
//!         If monthlyRate = 0 Then
//!             payment = loanAmount / numPayments
//!         Else
//!             payment = loanAmount * (monthlyRate * (1 + monthlyRate) ^ numPayments) / _
//!                      ((1 + monthlyRate) ^ numPayments - 1)
//!         End If
//!         
//!         CalculatePayment = Round(payment, m_precision)
//!     End Function
//!     
//!     Public Function CalculateTax(amount As Double, taxRate As Double) As Double
//!         ' Calculate tax with rounding
//!         Dim tax As Double
//!         
//!         tax = amount * taxRate
//!         CalculateTax = Round(tax, m_precision)
//!     End Function
//!     
//!     Public Function CalculateTotal(subtotal As Double, taxRate As Double) As Double
//!         ' Calculate total with tax
//!         Dim tax As Double
//!         Dim total As Double
//!         
//!         tax = Round(subtotal * taxRate, m_precision)
//!         total = subtotal + tax
//!         
//!         CalculateTotal = Round(total, m_precision)
//!     End Function
//!     
//!     Public Function SplitAmount(totalAmount As Double, _
//!                                numSplits As Integer) As Double()
//!         ' Split amount evenly with proper rounding
//!         Dim splits() As Double
//!         Dim baseAmount As Double
//!         Dim remainder As Double
//!         Dim i As Integer
//!         
//!         ReDim splits(1 To numSplits)
//!         
//!         baseAmount = Round(totalAmount / numSplits, m_precision)
//!         
//!         For i = 1 To numSplits
//!             splits(i) = baseAmount
//!         Next i
//!         
//!         ' Adjust for rounding errors
//!         remainder = Round(totalAmount - (baseAmount * numSplits), m_precision)
//!         splits(1) = Round(splits(1) + remainder, m_precision)
//!         
//!         SplitAmount = splits
//!     End Function
//!     
//!     Public Sub SetPrecision(precision As Integer)
//!         m_precision = precision
//!     End Sub
//!     
//!     Public Function GetPrecision() As Integer
//!         GetPrecision = m_precision
//!     End Function
//! End Class
//! ```
//!
//! ### Example 2: Statistical Rounder
//! ```vb
//! ' Round statistical values to appropriate precision
//! Module StatisticalRounder
//!     Public Function RoundMean(values() As Double, decimals As Integer) As Double
//!         ' Calculate and round mean
//!         Dim sum As Double
//!         Dim i As Integer
//!         
//!         sum = 0
//!         For i = LBound(values) To UBound(values)
//!             sum = sum + values(i)
//!         Next i
//!         
//!         RoundMean = Round(sum / (UBound(values) - LBound(values) + 1), decimals)
//!     End Function
//!     
//!     Public Function RoundStdDev(values() As Double, decimals As Integer) As Double
//!         ' Calculate and round standard deviation
//!         Dim mean As Double
//!         Dim sumSquaredDiff As Double
//!         Dim i As Integer
//!         Dim n As Integer
//!         
//!         n = UBound(values) - LBound(values) + 1
//!         mean = RoundMean(values, decimals + 2)
//!         
//!         sumSquaredDiff = 0
//!         For i = LBound(values) To UBound(values)
//!             sumSquaredDiff = sumSquaredDiff + (values(i) - mean) ^ 2
//!         Next i
//!         
//!         RoundStdDev = Round(Sqr(sumSquaredDiff / (n - 1)), decimals)
//!     End Function
//!     
//!     Public Function RoundPercentile(values() As Double, percentile As Double, _
//!                                    decimals As Integer) As Double
//!         ' Calculate and round percentile
//!         Dim sortedValues() As Double
//!         Dim index As Double
//!         Dim lowerIndex As Integer
//!         Dim upperIndex As Integer
//!         Dim weight As Double
//!         Dim result As Double
//!         
//!         ' Copy and sort array (simplified - would need sorting implementation)
//!         sortedValues = values
//!         
//!         index = percentile * (UBound(sortedValues) - LBound(sortedValues))
//!         lowerIndex = Int(index) + LBound(sortedValues)
//!         upperIndex = lowerIndex + 1
//!         weight = index - Int(index)
//!         
//!         If upperIndex > UBound(sortedValues) Then
//!             result = sortedValues(lowerIndex)
//!         Else
//!             result = sortedValues(lowerIndex) * (1 - weight) + _
//!                     sortedValues(upperIndex) * weight
//!         End If
//!         
//!         RoundPercentile = Round(result, decimals)
//!     End Function
//!     
//!     Public Function RoundCorrelation(values1() As Double, values2() As Double, _
//!                                     decimals As Integer) As Double
//!         ' Calculate and round correlation coefficient
//!         Dim mean1 As Double, mean2 As Double
//!         Dim sum As Double
//!         Dim sum1Sq As Double, sum2Sq As Double
//!         Dim i As Integer
//!         Dim correlation As Double
//!         
//!         mean1 = RoundMean(values1, decimals + 2)
//!         mean2 = RoundMean(values2, decimals + 2)
//!         
//!         sum = 0
//!         sum1Sq = 0
//!         sum2Sq = 0
//!         
//!         For i = LBound(values1) To UBound(values1)
//!             sum = sum + (values1(i) - mean1) * (values2(i) - mean2)
//!             sum1Sq = sum1Sq + (values1(i) - mean1) ^ 2
//!             sum2Sq = sum2Sq + (values2(i) - mean2) ^ 2
//!         Next i
//!         
//!         correlation = sum / Sqr(sum1Sq * sum2Sq)
//!         RoundCorrelation = Round(correlation, decimals)
//!     End Function
//! End Module
//! ```
//!
//! ### Example 3: Grade Calculator
//! ```vb
//! ' Calculate and round student grades
//! Class GradeCalculator
//!     Private m_roundingMode As String  ' "banker", "up", "down", "nearest"
//!     
//!     Public Sub Initialize(Optional roundingMode As String = "banker")
//!         m_roundingMode = roundingMode
//!     End Sub
//!     
//!     Public Function CalculateFinalGrade(scores() As Double, _
//!                                        weights() As Double) As Double
//!         ' Calculate weighted average grade
//!         Dim weightedSum As Double
//!         Dim totalWeight As Double
//!         Dim i As Integer
//!         
//!         weightedSum = 0
//!         totalWeight = 0
//!         
//!         For i = LBound(scores) To UBound(scores)
//!             weightedSum = weightedSum + scores(i) * weights(i)
//!             totalWeight = totalWeight + weights(i)
//!         Next i
//!         
//!         CalculateFinalGrade = RoundGrade(weightedSum / totalWeight)
//!     End Function
//!     
//!     Private Function RoundGrade(grade As Double) As Double
//!         ' Round grade based on configured mode
//!         Select Case m_roundingMode
//!             Case "banker"
//!                 RoundGrade = Round(grade, 1)
//!                 
//!             Case "up"
//!                 If grade > Int(grade) Then
//!                     RoundGrade = Int(grade) + 1
//!                 Else
//!                     RoundGrade = Int(grade)
//!                 End If
//!                 
//!             Case "down"
//!                 RoundGrade = Int(grade)
//!                 
//!             Case "nearest"
//!                 If grade - Int(grade) >= 0.5 Then
//!                     RoundGrade = Int(grade) + 1
//!                 Else
//!                     RoundGrade = Int(grade)
//!                 End If
//!                 
//!             Case Else
//!                 RoundGrade = Round(grade, 1)
//!         End Select
//!     End Function
//!     
//!     Public Function GetLetterGrade(numericGrade As Double) As String
//!         ' Convert numeric grade to letter grade
//!         Dim rounded As Double
//!         
//!         rounded = RoundGrade(numericGrade)
//!         
//!         If rounded >= 90 Then
//!             GetLetterGrade = "A"
//!         ElseIf rounded >= 80 Then
//!             GetLetterGrade = "B"
//!         ElseIf rounded >= 70 Then
//!             GetLetterGrade = "C"
//!         ElseIf rounded >= 60 Then
//!             GetLetterGrade = "D"
//!         Else
//!             GetLetterGrade = "F"
//!         End If
//!     End Function
//!     
//!     Public Sub SetRoundingMode(mode As String)
//!         m_roundingMode = mode
//!     End Sub
//!     
//!     Public Function GetRoundingMode() As String
//!         GetRoundingMode = m_roundingMode
//!     End Function
//! End Class
//! ```
//!
//! ### Example 4: Measurement Precision Manager
//! ```vb
//! ' Manage precision for different types of measurements
//! Class MeasurementPrecisionManager
//!     Private Type MeasurementType
//!         Name As String
//!         Decimals As Integer
//!         Unit As String
//!     End Type
//!     
//!     Private m_types() As MeasurementType
//!     Private m_count As Integer
//!     
//!     Public Sub Initialize()
//!         m_count = 0
//!         ReDim m_types(0 To 99)
//!         
//!         ' Add default measurement types
//!         AddMeasurementType "Temperature", 1, "°C"
//!         AddMeasurementType "Distance", 2, "m"
//!         AddMeasurementType "Weight", 3, "kg"
//!         AddMeasurementType "Pressure", 1, "kPa"
//!         AddMeasurementType "Voltage", 2, "V"
//!     End Sub
//!     
//!     Public Sub AddMeasurementType(name As String, decimals As Integer, _
//!                                   unit As String)
//!         If m_count > UBound(m_types) Then
//!             ReDim Preserve m_types(0 To UBound(m_types) + 50)
//!         End If
//!         
//!         m_types(m_count).Name = name
//!         m_types(m_count).Decimals = decimals
//!         m_types(m_count).Unit = unit
//!         m_count = m_count + 1
//!     End Sub
//!     
//!     Public Function RoundMeasurement(value As Double, _
//!                                     measurementType As String) As Double
//!         ' Round value based on measurement type
//!         Dim i As Integer
//!         
//!         For i = 0 To m_count - 1
//!             If UCase(m_types(i).Name) = UCase(measurementType) Then
//!                 RoundMeasurement = Round(value, m_types(i).Decimals)
//!                 Exit Function
//!             End If
//!         Next i
//!         
//!         ' Default to 2 decimals if type not found
//!         RoundMeasurement = Round(value, 2)
//!     End Function
//!     
//!     Public Function FormatMeasurement(value As Double, _
//!                                      measurementType As String) As String
//!         ' Round and format measurement with unit
//!         Dim rounded As Double
//!         Dim i As Integer
//!         Dim decimals As Integer
//!         Dim unit As String
//!         
//!         decimals = 2
//!         unit = ""
//!         
//!         For i = 0 To m_count - 1
//!             If UCase(m_types(i).Name) = UCase(measurementType) Then
//!                 decimals = m_types(i).Decimals
//!                 unit = m_types(i).Unit
//!                 Exit For
//!             End If
//!         Next i
//!         
//!         rounded = Round(value, decimals)
//!         FormatMeasurement = Format(rounded, "0." & String(decimals, "0")) & " " & unit
//!     End Function
//!     
//!     Public Function GetPrecision(measurementType As String) As Integer
//!         Dim i As Integer
//!         
//!         For i = 0 To m_count - 1
//!             If UCase(m_types(i).Name) = UCase(measurementType) Then
//!                 GetPrecision = m_types(i).Decimals
//!                 Exit Function
//!             End If
//!         Next i
//!         
//!         GetPrecision = 2  ' Default
//!     End Function
//! End Class
//! ```
//!
//! ## Error Handling
//!
//! The `Round` function generates errors in specific situations:
//!
//! **Error 5: Invalid procedure call or argument**
//! - Occurs when expression cannot be converted to numeric type
//! - Occurs when numdecimalplaces is excessively large
//!
//! **Error 6: Overflow**
//! - Occurs when the rounded value exceeds the range of the return type
//!
//! Example error handling:
//!
//! ```vb
//! On Error Resume Next
//! Dim result As Double
//! result = Round(userInput, decimalPlaces)
//! If Err.Number <> 0 Then
//!     MsgBox "Error rounding value: " & Err.Description
//!     result = 0
//! End If
//! On Error GoTo 0
//! ```
//!
//! ## Performance Considerations
//!
//! - `Round` is very fast for reasonable decimal place values
//! - Performance degrades with very large numdecimalplaces values
//! - Consider caching rounded values if used repeatedly with same parameters
//! - For large arrays, consider rounding in batches
//! - Banker's rounding is slightly slower than simple truncation
//!
//! ## Best Practices
//!
//! 1. **Understand Banker's Rounding**: Be aware of round-to-even behavior for .5 values
//! 2. **Document Rounding**: Clearly document which rounding method is used
//! 3. **Consistent Precision**: Use same decimal places throughout calculations
//! 4. **Round at Display Time**: Keep full precision during calculations, round for display
//! 5. **Avoid Cumulative Errors**: Be aware of rounding errors accumulating in loops
//! 6. **Test Edge Cases**: Test with .5 values to verify rounding behavior
//! 7. **Financial Calculations**: Use 2 decimals for currency, consider legal requirements
//! 8. **Validate Input**: Check that decimal places parameter is reasonable
//! 9. **Use Helper Functions**: Wrap Round in domain-specific functions
//! 10. **Consider Alternatives**: For traditional rounding, implement custom function
//!
//! ## Comparison with Related Functions
//!
//! | Function | Purpose | Behavior | Use Case |
//! |----------|---------|----------|----------|
//! | **Round** | Round to decimals | Banker's rounding | General rounding, financial |
//! | **Int** | Integer part | Truncate towards 0 | Get whole number part |
//! | **Fix** | Integer part | Truncate towards 0 | Get whole number part |
//! | **`CInt`** | Convert to integer | Round to nearest | Type conversion |
//! | **`CLng`** | Convert to long | Round to nearest | Type conversion |
//! | **Format** | Format number | Can specify decimals | Display formatting |
//!
//! ## Platform and Version Notes
//!
//! - Available in VB6 and VBA (added in VB6/Office 2000)
//! - Uses banker's rounding (IEEE 754 standard)
//! - Not available in earlier VB versions (use Int or Fix instead)
//! - In VB.NET, replaced by Math.Round with `MidpointRounding` enum
//! - Behavior differs from Excel's ROUND function (which uses "round half up")
//!
//! ## Limitations
//!
//! - Banker's rounding may be unexpected for users familiar with traditional rounding
//! - Cannot choose rounding mode (always banker's rounding)
//! - Limited precision for very large decimal place values
//! - Rounding errors can accumulate in iterative calculations
//! - Different from Excel ROUND function behavior
//! - No built-in round-up or round-down functions
//!
//! ## Related Functions
//!
//! - `Int`: Returns the integer portion of a number (truncates toward zero)
//! - `Fix`: Returns the integer portion of a number (truncates toward zero)
//! - `CInt`: Converts expression to Integer with rounding
//! - `CLng`: Converts expression to Long with rounding
//! - `Format`: Formats number with specified decimal places

use crate::{
    error::{VBError, VBResult},
    value::{VBLong, VBVariant},
};

/// Implementation of the Round function.
///
/// VB6 behavior:
/// - `Round(Null)` returns `Null`
/// - other values are coerced with numeric conversion rules and return `Double`
/// - uses banker's rounding (round half to even)
/// - `numdecimalplaces` defaults to `0` when omitted
/// - a negative `numdecimalplaces` raises error 5 (`Invalid procedure call or
///   argument`), as does one so large that the rounding scale overflows
pub fn round(expression: &VBVariant, numdecimalplaces: Option<&VBLong>) -> VBResult<VBVariant> {
    if expression.is_null() {
        return Ok(VBVariant::Null);
    }

    let numeric = expression.as_f64()?;
    let digits = numdecimalplaces.map_or(0, |d| d.as_i32());

    if digits < 0 {
        return Err(VBError::invalid_procedure_call());
    }

    let factor = 10.0_f64.powi(digits);
    if !factor.is_finite() {
        return Err(VBError::invalid_procedure_call());
    }

    let scaled = numeric * factor;
    if !scaled.is_finite() {
        // So large in magnitude that no fractional digits exist to round.
        return Ok(VBVariant::from_double(numeric));
    }

    let rounded = round_half_even(scaled) / factor;
    Ok(VBVariant::from_double(rounded))
}

/// Round `scaled` (a value already multiplied by `10^digits`) to the nearest
/// integer, breaking exact halves toward even as VB6 does.
fn round_half_even(scaled: f64) -> f64 {
    let floor = scaled.floor();
    let diff = scaled - floor;
    if diff < 0.5 {
        floor
    } else if diff > 0.5 {
        floor + 1.0
    } else if (floor as i64) % 2 == 0 {
        floor
    } else {
        floor + 1.0
    }
}

#[cfg(test)]
mod tests {
    use super::round;
    use crate::{error::err_number, value::{VBLong, VBVariant}};

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
        assert_eq!(round(&VBVariant::Null, None).unwrap(), VBVariant::Null);
        assert_eq!(
            round(&VBVariant::Null, Some(&VBLong::from(2))).unwrap(),
            VBVariant::Null
        );
    }

    #[test]
    fn returns_zero_for_empty() {
        assert_eq!(
            round(&VBVariant::Empty, None).unwrap(),
            VBVariant::from_double(0.0)
        );
    }

    #[test]
    fn returns_double_for_numeric_inputs() {
        let result = round(&VBVariant::from_byte(5), None).unwrap();
        assert_eq!(result, VBVariant::from_double(5.0));

        let result = round(&VBVariant::from_integer(-123), None).unwrap();
        assert_eq!(result, VBVariant::from_double(-123.0));

        let result = round(&VBVariant::from_long(-12345), None).unwrap();
        assert_eq!(result, VBVariant::from_double(-12345.0));

        let result = round(&VBVariant::from_single(-12.5), None).unwrap();
        assert_eq!(result, VBVariant::from_double(-12.0));

        let result = round(&VBVariant::from_double(-12.5), None).unwrap();
        assert_eq!(result, VBVariant::from_double(-12.0));

        let result = round(&VBVariant::from_currency_scaled(-12_345), None).unwrap();
        assert_eq!(result, VBVariant::from_double(-1.0));
    }

    #[test]
    fn returns_expected_values() {
        let VBVariant::Double(v) = round(&VBVariant::from_double(0.0), None).unwrap() else {
            panic!("expected Double")
        };
        assert_approx_eq(v, 0.0);

        let VBVariant::Double(v) = round(&VBVariant::from_double(1.0), None).unwrap() else {
            panic!("expected Double")
        };
        assert_approx_eq(v, 1.0);

        let VBVariant::Double(v) = round(&VBVariant::from_double(-1.0), None).unwrap() else {
            panic!("expected Double")
        };
        assert_approx_eq(v, -1.0);

        let VBVariant::Double(v) = round(&VBVariant::from_double(1.25), None).unwrap() else {
            panic!("expected Double")
        };
        assert_approx_eq(v, 1.0);

        let VBVariant::Double(v) = round(&VBVariant::from_double(1.75), None).unwrap() else {
            panic!("expected Double")
        };
        assert_approx_eq(v, 2.0);
    }

    #[test]
    fn uses_bankers_rounding() {
        let VBVariant::Double(v) = round(&VBVariant::from_double(0.5), None).unwrap() else {
            panic!("expected Double")
        };
        assert_approx_eq(v, 0.0);

        let VBVariant::Double(v) = round(&VBVariant::from_double(1.5), None).unwrap() else {
            panic!("expected Double")
        };
        assert_approx_eq(v, 2.0);

        let VBVariant::Double(v) = round(&VBVariant::from_double(2.5), None).unwrap() else {
            panic!("expected Double")
        };
        assert_approx_eq(v, 2.0);

        let VBVariant::Double(v) = round(&VBVariant::from_double(3.5), None).unwrap() else {
            panic!("expected Double")
        };
        assert_approx_eq(v, 4.0);

        let VBVariant::Double(v) = round(&VBVariant::from_double(-2.5), None).unwrap() else {
            panic!("expected Double")
        };
        assert_approx_eq(v, -2.0);

        let VBVariant::Double(v) = round(&VBVariant::from_double(-3.5), None).unwrap() else {
            panic!("expected Double")
        };
        assert_approx_eq(v, -4.0);
    }

    #[test]
    fn rounds_to_decimal_places() {
        let VBVariant::Double(v) =
            round(&VBVariant::from_double(1.25), Some(&VBLong::from(1))).unwrap()
        else {
            panic!("expected Double")
        };
        assert_approx_eq(v, 1.2);

        let VBVariant::Double(v) =
            round(&VBVariant::from_double(1.75), Some(&VBLong::from(1))).unwrap()
        else {
            panic!("expected Double")
        };
        assert_approx_eq(v, 1.8);

        let VBVariant::Double(v) =
            round(&VBVariant::from_double(2.25), Some(&VBLong::from(1))).unwrap()
        else {
            panic!("expected Double")
        };
        assert_approx_eq(v, 2.2);

        let VBVariant::Double(v) =
            round(&VBVariant::from_double(3.75), Some(&VBLong::from(1))).unwrap()
        else {
            panic!("expected Double")
        };
        assert_approx_eq(v, 3.8);

        let VBVariant::Double(v) =
            round(&VBVariant::from_double(2.75), Some(&VBLong::from(2))).unwrap()
        else {
            panic!("expected Double")
        };
        assert_approx_eq(v, 2.75);

        let VBVariant::Double(v) =
            round(&VBVariant::from_double(-1.25), Some(&VBLong::from(1))).unwrap()
        else {
            panic!("expected Double")
        };
        assert_approx_eq(v, -1.2);
    }

    #[test]
    fn explicit_zero_decimals_matches_omitted() {
        assert_eq!(
            round(&VBVariant::from_double(2.5), Some(&VBLong::from(0))).unwrap(),
            round(&VBVariant::from_double(2.5), None).unwrap(),
        );
    }

    #[test]
    fn rejects_negative_numdecimalplaces() {
        let err = round(&VBVariant::from_double(2.5), Some(&VBLong::from(-1))).unwrap_err();
        assert_eq!(err.number, err_number::INVALID_PROCEDURE_CALL);
    }

    #[test]
    fn rejects_non_numeric_values() {
        let err = round(&VBVariant::from_string("not-a-number"), None).unwrap_err();
        assert_eq!(err.number, err_number::TYPE_MISMATCH);
    }

    #[test]
    fn accepts_numeric_strings() {
        let result = round(&VBVariant::from_string("2.5"), None).unwrap();
        assert_eq!(result, VBVariant::from_double(2.0));

        let result = round(&VBVariant::from_string("1.25"), Some(&VBLong::from(1))).unwrap();
        assert_eq!(result, VBVariant::from_double(1.2));
    }
}

