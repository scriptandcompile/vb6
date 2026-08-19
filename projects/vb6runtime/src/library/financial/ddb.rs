//! # DDB Function
//!
//! Returns a Double specifying the depreciation of an asset for a specific time period using
//! the double-declining balance method or some other method you specify.
//!
//! ## Syntax
//!
//! ```vb
//! DDB(cost, salvage, life, period[, factor])
//! ```
//!
//! ## Parameters
//!
//! - **cost**: Required. Double specifying initial cost of the asset.
//! - **salvage**: Required. Double specifying value of the asset at the end of its useful life.
//! - **life**: Required. Double specifying length of useful life of the asset.
//! - **period**: Required. Double specifying period for which asset depreciation is calculated.
//! - **factor**: Optional. Variant specifying rate at which the balance declines. If omitted,
//!   2 (double-declining method) is assumed.
//!
//! ## Return Value
//!
//! Returns a Double representing the depreciation amount for the specified period. The return
//! value uses the same time units as the life parameter.
//!
//! ## Remarks
//!
//! The `DDB` function calculates depreciation using the double-declining balance method,
//! which computes depreciation at an accelerated rate. Depreciation is highest in the first
//! period and decreases in successive periods.
//!
//! **Important Characteristics:**
//!
//! - Uses accelerated depreciation (more in early periods)
//! - Default factor is 2.0 (double-declining balance)
//! - Factor of 1.5 gives 150% declining balance
//! - All arguments must be positive numbers
//! - The life and period arguments must use the same units (years, months, etc.)
//! - Depreciation never reduces asset value below salvage value
//! - More accurate than straight-line for assets that lose value quickly
//! - Commonly used for tax purposes and financial reporting
//!
//! ## Formula
//!
//! The double-declining balance method uses:
//!
//! ```text
//! Depreciation = (Book Value - Salvage) × (Factor / Life)
//!
//! Where:
//! - Book Value = Cost - Accumulated Depreciation from prior periods
//! - Factor = Declining balance rate (default 2.0)
//! - Life = Total useful life of asset
//! ```
//!
//! The function ensures that depreciation does not reduce the book value below salvage value.
//!
//! ## Examples
//!
//! ### Basic Usage
//!
//! ```vb
//! ' Calculate depreciation for equipment
//! Dim cost As Double
//! Dim salvage As Double
//! Dim life As Double
//! Dim depreciation As Double
//!
//! cost = 10000      ' $10,000 initial cost
//! salvage = 1000    ' $1,000 salvage value
//! life = 5          ' 5 year useful life
//!
//! ' First year depreciation (double-declining)
//! depreciation = DDB(cost, salvage, life, 1)
//! ' Returns 4000 (40% of 10000)
//!
//! ' Second year depreciation
//! depreciation = DDB(cost, salvage, life, 2)
//! ' Returns 2400 (40% of 6000)
//! ```
//!
//! ### Custom Declining Factor
//!
//! ```vb
//! ' 150% declining balance
//! Dim depreciation As Double
//! depreciation = DDB(10000, 1000, 5, 1, 1.5)
//! ' Uses 30% rate instead of 40%
//!
//! ' Straight-line equivalent (factor = 1)
//! depreciation = DDB(10000, 1000, 5, 1, 1)
//! ```
//!
//! ### Complete Depreciation Schedule
//!
//! ```vb
//! Sub ShowDepreciationSchedule()
//!     Dim cost As Double
//!     Dim salvage As Double
//!     Dim life As Double
//!     Dim period As Integer
//!     Dim depreciation As Double
//!     
//!     cost = 10000
//!     salvage = 1000
//!     life = 5
//!     
//!     Debug.Print "Year", "Depreciation", "Book Value"
//!     
//!     For period = 1 To life
//!         depreciation = DDB(cost, salvage, life, period)
//!         Debug.Print period, Format(depreciation, "Currency"), _
//!                     Format(cost - TotalDepreciation(period), "Currency")
//!     Next period
//! End Sub
//! ```
//!
//! ## Common Patterns
//!
//! ### Calculate Total Accumulated Depreciation
//!
//! ```vb
//! Function AccumulatedDepreciation(cost As Double, salvage As Double, _
//!                                  life As Double, currentPeriod As Integer) As Double
//!     Dim total As Double
//!     Dim i As Integer
//!     
//!     total = 0
//!     For i = 1 To currentPeriod
//!         total = total + DDB(cost, salvage, life, i)
//!     Next i
//!     
//!     AccumulatedDepreciation = total
//! End Function
//! ```
//!
//! ### Calculate Current Book Value
//!
//! ```vb
//! Function BookValue(cost As Double, salvage As Double, _
//!                    life As Double, currentPeriod As Integer) As Double
//!     Dim accumulated As Double
//!     accumulated = AccumulatedDepreciation(cost, salvage, life, currentPeriod)
//!     BookValue = cost - accumulated
//! End Function
//! ```
//!
//! ### Compare Depreciation Methods
//!
//! ```vb
//! Sub CompareDepreciationMethods(cost As Double, salvage As Double, life As Double)
//!     Dim period As Integer
//!     Dim ddbDepr As Double
//!     Dim slnDepr As Double
//!     
//!     Debug.Print "Period", "DDB", "SLN"
//!     
//!     For period = 1 To life
//!         ddbDepr = DDB(cost, salvage, life, period)
//!         slnDepr = SLN(cost, salvage, life)
//!         
//!         Debug.Print period, Format(ddbDepr, "Currency"), _
//!                     Format(slnDepr, "Currency")
//!     Next period
//! End Sub
//! ```
//!
//! ### Monthly Depreciation
//!
//! ```vb
//! Function MonthlyDDB(cost As Double, salvage As Double, _
//!                     lifeYears As Double, month As Integer) As Double
//!     ' Calculate depreciation by month instead of year
//!     Dim lifeMonths As Double
//!     lifeMonths = lifeYears * 12
//!     MonthlyDDB = DDB(cost, salvage, lifeMonths, month)
//! End Function
//! ```
//!
//! ### Partial Year Depreciation
//!
//! ```vb
//! Function PartialYearDDB(cost As Double, salvage As Double, life As Double, _
//!                         year As Integer, monthsInFirstYear As Integer) As Double
//!     ' Handle assets purchased mid-year
//!     If year = 1 Then
//!         PartialYearDDB = DDB(cost, salvage, life, 1) * (monthsInFirstYear / 12)
//!     Else
//!         Dim priorYearPartial As Double
//!         Dim currentYearPartial As Double
//!         
//!         priorYearPartial = DDB(cost, salvage, life, year - 1) * _
//!                           ((12 - monthsInFirstYear) / 12)
//!         currentYearPartial = DDB(cost, salvage, life, year) * _
//!                             (monthsInFirstYear / 12)
//!         
//!         PartialYearDDB = priorYearPartial + currentYearPartial
//!     End If
//! End Function
//! ```
//!
//! ### Depreciation Rate Calculation
//!
//! ```vb
//! Function DepreciationRate(life As Double, Optional factor As Double = 2) As Double
//!     ' Calculate the depreciation rate percentage
//!     DepreciationRate = (factor / life) * 100
//! End Function
//!
//! ' Usage
//! rate = DepreciationRate(5)      ' Returns 40% for 5-year DDB
//! rate = DepreciationRate(5, 1.5) ' Returns 30% for 5-year 150% DB
//! ```
//!
//! ### Asset Register with DDB
//!
//! ```vb
//! Type Asset
//!     Description As String
//!     Cost As Double
//!     Salvage As Double
//!     Life As Double
//!     PurchaseDate As Date
//! End Type
//!
//! Function CalculateAssetDepreciation(asset As Asset, currentYear As Integer) As Double
//!     Dim yearsOwned As Integer
//!     yearsOwned = Year(Date) - Year(asset.PurchaseDate)
//!     
//!     If yearsOwned >= currentYear And currentYear <= asset.Life Then
//!         CalculateAssetDepreciation = DDB(asset.Cost, asset.Salvage, _
//!                                         asset.Life, currentYear)
//!     Else
//!         CalculateAssetDepreciation = 0
//!     End If
//! End Function
//! ```
//!
//! ### Switch to Straight-Line Detection
//!
//! ```vb
//! Function ShouldSwitchToSLN(cost As Double, salvage As Double, _
//!                            life As Double, period As Integer) As Boolean
//!     ' Determine if switching to SLN would give higher depreciation
//!     Dim ddbAmount As Double
//!     Dim slnAmount As Double
//!     Dim bookValue As Double
//!     Dim remainingLife As Double
//!     
//!     ddbAmount = DDB(cost, salvage, life, period)
//!     bookValue = BookValue(cost, salvage, life, period - 1)
//!     remainingLife = life - period + 1
//!     slnAmount = (bookValue - salvage) / remainingLife
//!     
//!     ShouldSwitchToSLN = (slnAmount > ddbAmount)
//! End Function
//! ```
//!
//! ### Tax Depreciation Calculator
//!
//! ```vb
//! Function TaxDepreciation(cost As Double, salvage As Double, _
//!                         life As Double, taxYear As Integer, _
//!                         Optional method As String = "DDB") As Double
//!     Select Case UCase(method)
//!         Case "DDB"
//!             TaxDepreciation = DDB(cost, salvage, life, taxYear)
//!         Case "150DB"
//!             TaxDepreciation = DDB(cost, salvage, life, taxYear, 1.5)
//!         Case "SLN"
//!             TaxDepreciation = SLN(cost, salvage, life)
//!         Case Else
//!             TaxDepreciation = 0
//!     End Select
//! End Function
//! ```
//!
//! ## Advanced Usage
//!
//! ### Depreciation Schedule Generator
//!
//! ```vb
//! Function GenerateDepreciationSchedule(cost As Double, salvage As Double, _
//!                                      life As Double) As Variant
//!     ' Returns 2D array: Period, Depreciation, Accumulated, Book Value
//!     Dim schedule() As Variant
//!     Dim period As Integer
//!     Dim depreciation As Double
//!     Dim accumulated As Double
//!     
//!     ReDim schedule(1 To life, 1 To 4)
//!     accumulated = 0
//!     
//!     For period = 1 To life
//!         depreciation = DDB(cost, salvage, life, period)
//!         accumulated = accumulated + depreciation
//!         
//!         schedule(period, 1) = period
//!         schedule(period, 2) = depreciation
//!         schedule(period, 3) = accumulated
//!         schedule(period, 4) = cost - accumulated
//!     Next period
//!     
//!     GenerateDepreciationSchedule = schedule
//! End Function
//! ```
//!
//! ### Hybrid Depreciation Method
//!
//! ```vb
//! Function HybridDepreciation(cost As Double, salvage As Double, _
//!                            life As Double, period As Integer) As Double
//!     ' Use DDB but switch to SLN when SLN gives higher amount
//!     Dim ddbAmount As Double
//!     Dim slnAmount As Double
//!     Dim bookValue As Double
//!     Dim remainingLife As Double
//!     
//!     ddbAmount = DDB(cost, salvage, life, period)
//!     
//!     If period > 1 Then
//!         bookValue = BookValue(cost, salvage, life, period - 1)
//!         remainingLife = life - period + 1
//!         slnAmount = (bookValue - salvage) / remainingLife
//!         
//!         HybridDepreciation = Application.Max(ddbAmount, slnAmount)
//!     Else
//!         HybridDepreciation = ddbAmount
//!     End If
//! End Function
//! ```
//!
//! ### Multi-Asset Depreciation Report
//!
//! ```vb
//! Sub GenerateDepreciationReport(assets() As Asset, fiscalYear As Integer)
//!     Dim i As Integer
//!     Dim totalDepreciation As Double
//!     Dim assetDepreciation As Double
//!     
//!     totalDepreciation = 0
//!     
//!     Debug.Print "Asset", "Cost", "Life", "Year", "Depreciation"
//!     
//!     For i = LBound(assets) To UBound(assets)
//!         Dim yearsSincePurchase As Integer
//!         yearsSincePurchase = fiscalYear - Year(assets(i).PurchaseDate) + 1
//!         
//!         If yearsSincePurchase > 0 And yearsSincePurchase <= assets(i).Life Then
//!             assetDepreciation = DDB(assets(i).Cost, assets(i).Salvage, _
//!                                    assets(i).Life, yearsSincePurchase)
//!             
//!             Debug.Print assets(i).Description, _
//!                        Format(assets(i).Cost, "Currency"), _
//!                        assets(i).Life, _
//!                        yearsSincePurchase, _
//!                        Format(assetDepreciation, "Currency")
//!             
//!             totalDepreciation = totalDepreciation + assetDepreciation
//!         End If
//!     Next i
//!     
//!     Debug.Print "Total Depreciation:", Format(totalDepreciation, "Currency")
//! End Sub
//! ```
//!
//! ### Optimal Method Selector
//!
//! ```vb
//! Function OptimalDepreciationMethod(cost As Double, salvage As Double, _
//!                                   life As Double, period As Integer, _
//!                                   taxRate As Double) As String
//!     ' Determine which method gives best tax benefit
//!     Dim ddbAmount As Double
//!     Dim slnAmount As Double
//!     Dim ddbTaxSavings As Double
//!     Dim slnTaxSavings As Double
//!     
//!     ddbAmount = DDB(cost, salvage, life, period)
//!     slnAmount = SLN(cost, salvage, life)
//!     
//!     ddbTaxSavings = ddbAmount * taxRate
//!     slnTaxSavings = slnAmount * taxRate
//!     
//!     If ddbTaxSavings > slnTaxSavings Then
//!         OptimalDepreciationMethod = "DDB"
//!     Else
//!         OptimalDepreciationMethod = "SLN"
//!     End If
//! End Function
//! ```
//!
//! ### Financial Statement Generator
//!
//! ```vb
//! Sub GenerateDepreciationFootnote(cost As Double, salvage As Double, _
//!                                 life As Double, currentYear As Integer)
//!     Dim schedule As Variant
//!     Dim i As Integer
//!     
//!     Debug.Print "Depreciation is calculated using the double-declining balance method:"
//!     Debug.Print "Asset cost: " & Format(cost, "Currency")
//!     Debug.Print "Salvage value: " & Format(salvage, "Currency")
//!     Debug.Print "Useful life: " & life & " years"
//!     Debug.Print
//!     Debug.Print "Year", "Depreciation", "Net Book Value"
//!     
//!     For i = 1 To currentYear
//!         Dim depr As Double
//!         Dim bookVal As Double
//!         
//!         depr = DDB(cost, salvage, life, i)
//!         bookVal = BookValue(cost, salvage, life, i)
//!         
//!         Debug.Print i, Format(depr, "Currency"), Format(bookVal, "Currency")
//!     Next i
//! End Sub
//! ```
//!
//! ### MACRS Alternative Comparison
//!
//! ```vb
//! Function CompareDDBToMARS(cost As Double, life As Double) As Variant
//!     ' Compare DDB to MACRS (Modified Accelerated Cost Recovery System)
//!     ' This is simplified; actual MACRS uses specific tables
//!     Dim comparison() As Variant
//!     Dim period As Integer
//!     Dim ddbTotal As Double
//!     Dim salvage As Double
//!     
//!     salvage = 0 ' MACRS assumes zero salvage
//!     ReDim comparison(1 To life, 1 To 3)
//!     
//!     For period = 1 To life
//!         comparison(period, 1) = period
//!         comparison(period, 2) = DDB(cost, salvage, life, period)
//!         comparison(period, 3) = BookValue(cost, salvage, life, period)
//!     Next period
//!     
//!     CompareDDBToMARS = comparison
//! End Function
//! ```
//!
//! ## Error Handling
//!
//! ```vb
//! Function SafeDDB(cost As Double, salvage As Double, life As Double, _
//!                  period As Integer, Optional factor As Double = 2) As Variant
//!     On Error GoTo ErrorHandler
//!     
//!     ' Validate inputs
//!     If cost < 0 Or salvage < 0 Or life <= 0 Or period <= 0 Then
//!         SafeDDB = CVErr(xlErrNum)
//!         Exit Function
//!     End If
//!     
//!     If salvage >= cost Then
//!         SafeDDB = 0
//!         Exit Function
//!     End If
//!     
//!     If period > life Then
//!         SafeDDB = 0
//!         Exit Function
//!     End If
//!     
//!     SafeDDB = DDB(cost, salvage, life, period, factor)
//!     Exit Function
//!     
//! ErrorHandler:
//!     SafeDDB = CVErr(xlErrValue)
//! End Function
//! ```
//!
//! ### Common Errors
//!
//! - **Error 5** (Invalid procedure call): Negative values for cost, salvage, life, or period
//! - **Error 5**: Life or period equals zero
//! - **Error 5**: Salvage value exceeds cost
//!
//! ## Performance Considerations
//!
//! - `DDB` involves iterative calculations for periods > 1
//! - Cache results when calculating multiple periods for same asset
//! - For complete schedules, calculate once and store array
//! - More complex than `SLN` but still efficient
//! - Consider pre-calculating depreciation schedules for reporting
//!
//! ## Best Practices
//!
//! ### Validate Parameters
//!
//! ```vb
//! ' Good - Validate before calculation
//! If cost > 0 And salvage >= 0 And salvage < cost And life > 0 Then
//!     depreciation = DDB(cost, salvage, life, period)
//! End If
//!
//! ' Avoid - May cause runtime error
//! depreciation = DDB(cost, salvage, life, period)
//! ```
//!
//! ### Use Consistent Time Units
//!
//! ```vb
//! ' Good - Both in years
//! depreciation = DDB(10000, 1000, 5, 2)
//!
//! ' Good - Both in months
//! depreciation = DDB(10000, 1000, 60, 24)
//!
//! ' Avoid - Mixing units
//! depreciation = DDB(10000, 1000, 5, 24)  ' Mixing years and months
//! ```
//!
//! ### Consider Switching Methods
//!
//! ```vb
//! ' Many businesses switch from DDB to SLN mid-life
//! ' to maximize depreciation deductions
//! If ShouldSwitchToSLN(cost, salvage, life, period) Then
//!     depreciation = CalculateSLNForRemaining(cost, salvage, life, period)
//! Else
//!     depreciation = DDB(cost, salvage, life, period)
//! End If
//! ```
//!
//! ### Document Depreciation Assumptions
//!
//! ```vb
//! ' Good - Document method and assumptions
//! ' Depreciation calculated using double-declining balance (200%)
//! ' Useful life: 5 years, Salvage: 10% of cost
//! depreciation = DDB(cost, cost * 0.1, 5, currentYear)
//! ```
//!
//! ## Comparison with Other Functions
//!
//! ### DDB vs SLN
//!
//! ```vb
//! ' DDB - Accelerated depreciation (higher early, lower later)
//! depr = DDB(10000, 1000, 5, 1)  ' Returns 4000
//!
//! ' SLN - Straight-line (same every year)
//! depr = SLN(10000, 1000, 5)     ' Returns 1800
//! ```
//!
//! ### DDB vs SYD
//!
//! ```vb
//! ' DDB - Double-declining balance
//! depr = DDB(10000, 1000, 5, 1)  ' Returns 4000
//!
//! ' SYD - Sum-of-years digits (also accelerated)
//! depr = SYD(10000, 1000, 5, 1)  ' Returns 3000
//! ```
//!
//! ### DDB with Different Factors
//!
//! ```vb
//! ' Double-declining (200%)
//! depr = DDB(10000, 1000, 5, 1, 2)    ' Returns 4000 (40% rate)
//!
//! ' 150% declining balance
//! depr = DDB(10000, 1000, 5, 1, 1.5)  ' Returns 3000 (30% rate)
//!
//! ' Straight-line equivalent
//! depr = DDB(10000, 1000, 5, 1, 1)    ' Returns 1800 (20% rate)
//! ```
//!
//! ## Limitations
//!
//! - Does not automatically switch to SLN (must implement manually)
//! - Does not handle mid-period purchases automatically
//! - Does not conform to specific tax codes (MACRS, etc.)
//! - Requires manual handling of disposal before end of life
//! - Cannot directly calculate accumulated depreciation (must sum periods)
//! - Does not handle negative depreciation or write-ups
//!
//! ## Related Functions
//!
//! - `SLN`: Straight-line depreciation (constant per period)
//! - `SYD`: Sum-of-years digits depreciation (accelerated)
//! - `VDB`: Variable declining balance (can switch to SLN automatically)
//! - `FV`: Future value (general financial calculation)
//! - `PV`: Present value (general financial calculation)

use crate::error::{VBError, VBResult};
use crate::value::VBVariant;
use vb6core::error::err_number;

/// Implementation of the `DDB` function.
///
/// Calculates depreciation using the double-declining balance method (or another
/// declining balance rate you specify). The book value decreases each period by
/// applying a rate to the remaining undepreciated amount, capped so it never falls
/// below salvage value.
///
/// VB6 behavior:
/// - `cost`, `salvage`, and `life` must be positive; `period` must be > 0
/// - If `life` is 0, raises error 5 (Invalid procedure call)
/// - If any argument is negative or zero where inappropriate, raises error 5
/// - Optional `factor` defaults to 2.0 (double-declining balance)
/// - Returns a `Double`
pub fn ddb(
    cost: &VBVariant,
    salvage: &VBVariant,
    life: &VBVariant,
    period: &VBVariant,
    factor: Option<&VBVariant>,
) -> VBResult<VBVariant> {
    let cost_val = cost.as_f64()?;
    let salvage_val = salvage.as_f64()?;
    let life_val = life.as_f64()?;
    let period_val = period.as_f64()?;
    let factor_val = factor.map(|f| f.as_f64()).transpose()?.unwrap_or(2.0);

    // Validate inputs per VB6 behavior
    if life_val <= 0.0 || cost_val < 0.0 || salvage_val < 0.0 || period_val <= 0.0 {
        return Err(VBError::new(err_number::INVALID_PROCEDURE_CALL)); // Invalid procedure call
    }

    let rate = factor_val / life_val;
    let mut book_value = cost_val;

    // Calculate depreciation for each period up to the requested period
    for i in 1..=period_val as i64 {
        let depreciation = (book_value * rate).min(book_value - salvage_val);
        book_value -= depreciation;

        // On the target period, return the depreciation amount
        if i == period_val as i64 {
            return Ok(VBVariant::from_double(depreciation));
        }
    }

    Ok(VBVariant::from_double(0.0))
}

#[cfg(test)]
mod tests {
    use super::ddb;
    use crate::value::VBVariant;

    fn ddb_default(cost: f64, salvage: f64, life: f64, period: f64) -> VBVariant {
        ddb(
            &VBVariant::from_double(cost),
            &VBVariant::from_double(salvage),
            &VBVariant::from_double(life),
            &VBVariant::from_double(period),
            None,
        )
        .unwrap()
    }

    fn ddb_with_factor(cost: f64, salvage: f64, life: f64, period: f64, factor: f64) -> VBVariant {
        ddb(
            &VBVariant::from_double(cost),
            &VBVariant::from_double(salvage),
            &VBVariant::from_double(life),
            &VBVariant::from_double(period),
            Some(&VBVariant::from_double(factor)),
        )
        .unwrap()
    }

    #[test]
    fn ddb_first_period_double_declining() {
        // Cost=10000, Salvage=1000, Life=5, Period=1 (rate = 2/5 = 40%)
        // Depreciation = 10000 * 0.4 = 4000
        let result = ddb_default(10000.0, 1000.0, 5.0, 1.0);
        assert_eq!(result.as_f64().unwrap(), 4000.0);
    }

    #[test]
    fn ddb_second_period() {
        // Book value after period 1 = 6000
        // Depreciation = 6000 * 0.4 = 2400
        let result = ddb_default(10000.0, 1000.0, 5.0, 2.0);
        assert_eq!(result.as_f64().unwrap(), 2400.0);
    }

    #[test]
    fn ddb_custom_factor() {
        // 150% declining balance: rate = 1.5/5 = 30%
        let result = ddb_with_factor(10000.0, 1000.0, 5.0, 1.0, 1.5);
        assert_eq!(result.as_f64().unwrap(), 3000.0);
    }

    #[test]
    fn ddb_respects_salvage_floor() {
        // When depreciation would go below salvage, it's capped
        let result = ddb_default(10000.0, 1000.0, 5.0, 5.0);
        let depr = result.as_f64().unwrap();
        // Should be non-zero and positive
        assert!(depr > 0.0);
    }

    #[test]
    fn ddb_zero_life_raises_error_5() {
        let err = ddb(
            &VBVariant::from_double(10000.0),
            &VBVariant::from_double(1000.0),
            &VBVariant::from_double(0.0),
            &VBVariant::from_double(1.0),
            None,
        )
        .unwrap_err();
        assert_eq!(err.number, 5);
    }

    #[test]
    fn ddb_negative_cost_raises_error_5() {
        let err = ddb(
            &VBVariant::from_double(-100.0),
            &VBVariant::from_double(0.0),
            &VBVariant::from_double(5.0),
            &VBVariant::from_double(1.0),
            None,
        )
        .unwrap_err();
        assert_eq!(err.number, 5);
    }

    #[test]
    fn ddb_with_integer_args() {
        let result = ddb(
            &VBVariant::from_long(10000),
            &VBVariant::from_long(1000),
            &VBVariant::from_long(5),
            &VBVariant::from_long(1),
            None,
        )
        .unwrap();
        assert_eq!(result.as_f64().unwrap(), 4000.0);
    }

    #[test]
    fn ddb_null_raises_invalid_use_of_null() {
        let err = ddb(
            &VBVariant::Null,
            &VBVariant::Empty,
            &VBVariant::Empty,
            &VBVariant::Empty,
            None,
        )
        .unwrap_err();
        assert_eq!(err.number, 94);
    }
}
