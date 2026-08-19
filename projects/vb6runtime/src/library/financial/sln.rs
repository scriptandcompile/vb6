//! # SLN Function
//!
//! Returns a Double specifying the straight-line depreciation of an asset for a single period.
//!
//! ## Syntax
//!
//! ```vb
//! SLN(cost, salvage, life)
//! ```
//!
//! ## Parameters
//!
//! - `cost` - Required. Double specifying initial cost of the asset.
//! - `salvage` - Required. Double specifying value of the asset at the end of its useful life.
//! - `life` - Required. Double specifying length of the useful life of the asset.
//!
//! ## Return Value
//!
//! Returns a Double representing the depreciation of an asset for a single period using the straight-line method.
//!
//! The formula is:
//! ```text
//! SLN = (cost - salvage) / life
//! ```
//!
//! ## Remarks
//!
//! The SLN function calculates straight-line depreciation, which is the simplest and most commonly used depreciation method. It assumes that the asset depreciates by the same amount each period over its useful life.
//!
//! Key characteristics:
//! - Depreciation is constant for each period
//! - Total depreciation = cost - salvage value
//! - Each period's depreciation = (cost - salvage) / life
//! - All three arguments must be positive
//! - Life represents the number of periods (years, months, etc.)
//! - Units must be consistent (if life is in years, result is yearly depreciation)
//!
//! Straight-line depreciation is used when:
//! - Asset provides uniform benefit over its life
//! - Simple, easy-to-understand method is preferred
//! - Tax regulations require or allow straight-line method
//! - Asset doesn't become obsolete quickly
//! - Usage is relatively constant over time
//!
//! The straight-line method is one of several depreciation methods:
//! - **SLN**: Straight-line (this function) - constant depreciation
//! - **DDB**: Double-declining balance - accelerated depreciation
//! - **SYD**: Sum-of-years digits - accelerated depreciation
//!
//! ## Typical Uses
//!
//! 1. **Asset Depreciation**: Calculate annual depreciation expense
//! 2. **Financial Planning**: Project future asset values
//! 3. **Tax Calculations**: Determine tax deductions
//! 4. **Budgeting**: Estimate replacement costs
//! 5. **Accounting**: Prepare financial statements
//! 6. **Asset Management**: Track asset value over time
//! 7. **Cost Analysis**: Calculate total ownership cost
//! 8. **Investment Analysis**: Evaluate asset investments
//!
//! ## Basic Examples
//!
//! ```vb
//! ' Example 1: Calculate yearly depreciation for equipment
//! Dim equipmentCost As Double
//! Dim salvageValue As Double
//! Dim usefulLife As Double
//! Dim yearlyDepreciation As Double
//!
//! equipmentCost = 50000      ' $50,000 initial cost
//! salvageValue = 5000        ' $5,000 salvage value
//! usefulLife = 5             ' 5 years useful life
//!
//! yearlyDepreciation = SLN(equipmentCost, salvageValue, usefulLife)
//! ' Returns 9000 ($9,000 per year)
//! ```
//!
//! ```vb
//! ' Example 2: Calculate monthly depreciation
//! Dim cost As Double
//! Dim salvage As Double
//! Dim months As Double
//! Dim monthlyDepreciation As Double
//!
//! cost = 24000
//! salvage = 2000
//! months = 36  ' 3 years = 36 months
//!
//! monthlyDepreciation = SLN(cost, salvage, months)
//! ' Returns 611.11 (approximately)
//! ```
//!
//! ```vb
//! ' Example 3: Calculate book value after n periods
//! Dim initialCost As Double
//! Dim salvage As Double
//! Dim life As Double
//! Dim periods As Integer
//! Dim annualDepreciation As Double
//! Dim bookValue As Double
//!
//! initialCost = 100000
//! salvage = 10000
//! life = 10
//! periods = 3  ' After 3 years
//!
//! annualDepreciation = SLN(initialCost, salvage, life)
//! bookValue = initialCost - (annualDepreciation * periods)
//! ' bookValue = 73000
//! ```
//!
//! ```vb
//! ' Example 4: Display depreciation schedule
//! Dim cost As Double
//! Dim salvage As Double
//! Dim life As Integer
//! Dim depreciation As Double
//! Dim i As Integer
//!
//! cost = 30000
//! salvage = 3000
//! life = 5
//! depreciation = SLN(cost, salvage, life)
//!
//! For i = 1 To life
//!     Debug.Print "Year " & i & ": $" & depreciation
//! Next i
//! ```
//!
//! ## Common Patterns
//!
//! ### Pattern 1: `CalculateBookValue`
//! Calculate asset book value after specified periods
//! ```vb
//! Function CalculateBookValue(cost As Double, salvage As Double, _
//!                             life As Double, periodsElapsed As Integer) As Double
//!     Dim annualDepreciation As Double
//!     Dim totalDepreciation As Double
//!     
//!     annualDepreciation = SLN(cost, salvage, life)
//!     totalDepreciation = annualDepreciation * periodsElapsed
//!     
//!     ' Ensure book value doesn't go below salvage value
//!     If totalDepreciation > (cost - salvage) Then
//!         CalculateBookValue = salvage
//!     Else
//!         CalculateBookValue = cost - totalDepreciation
//!     End If
//! End Function
//! ```
//!
//! ### Pattern 2: `GenerateDepreciationSchedule`
//! Create complete depreciation schedule
//! ```vb
//! Sub GenerateDepreciationSchedule(cost As Double, salvage As Double, _
//!                                  life As Integer)
//!     Dim depreciation As Double
//!     Dim bookValue As Double
//!     Dim i As Integer
//!     
//!     depreciation = SLN(cost, salvage, life)
//!     bookValue = cost
//!     
//!     Debug.Print "Year" & vbTab & "Depreciation" & vbTab & "Book Value"
//!     Debug.Print "0" & vbTab & "0" & vbTab & bookValue
//!     
//!     For i = 1 To life
//!         bookValue = bookValue - depreciation
//!         Debug.Print i & vbTab & depreciation & vbTab & bookValue
//!     Next i
//! End Sub
//! ```
//!
//! ### Pattern 3: `CompareDepreciationMethods`
//! Compare straight-line with other methods
//! ```vb
//! Sub CompareDepreciationMethods(cost As Double, salvage As Double, _
//!                                life As Integer, year As Integer)
//!     Dim slnDepreciation As Double
//!     Dim ddbDepreciation As Double
//!     
//!     slnDepreciation = SLN(cost, salvage, life)
//!     ddbDepreciation = DDB(cost, salvage, life, year)
//!     
//!     Debug.Print "Year " & year
//!     Debug.Print "Straight-Line: $" & Format(slnDepreciation, "#,##0.00")
//!     Debug.Print "Double-Declining: $" & Format(ddbDepreciation, "#,##0.00")
//! End Sub
//! ```
//!
//! ### Pattern 4: `CalculateTotalDepreciation`
//! Calculate total depreciation over asset life
//! ```vb
//! Function CalculateTotalDepreciation(cost As Double, salvage As Double) As Double
//!     CalculateTotalDepreciation = cost - salvage
//! End Function
//! ```
//!
//! ### Pattern 5: `ValidateDepreciationInputs`
//! Validate inputs before calculating depreciation
//! ```vb
//! Function ValidateDepreciationInputs(cost As Double, salvage As Double, _
//!                                     life As Double) As Boolean
//!     ValidateDepreciationInputs = False
//!     
//!     If cost <= 0 Then
//!         MsgBox "Cost must be positive", vbExclamation
//!         Exit Function
//!     End If
//!     
//!     If salvage < 0 Then
//!         MsgBox "Salvage value cannot be negative", vbExclamation
//!         Exit Function
//!     End If
//!     
//!     If salvage >= cost Then
//!         MsgBox "Salvage value must be less than cost", vbExclamation
//!         Exit Function
//!     End If
//!     
//!     If life <= 0 Then
//!         MsgBox "Life must be positive", vbExclamation
//!         Exit Function
//!     End If
//!     
//!     ValidateDepreciationInputs = True
//! End Function
//! ```
//!
//! ### Pattern 6: `CalculateMonthlyDepreciation`
//! Convert annual to monthly depreciation
//! ```vb
//! Function CalculateMonthlyDepreciation(cost As Double, salvage As Double, _
//!                                       yearsLife As Double) As Double
//!     Dim annualDepreciation As Double
//!     
//!     annualDepreciation = SLN(cost, salvage, yearsLife)
//!     CalculateMonthlyDepreciation = annualDepreciation / 12
//! End Function
//! ```
//!
//! ### Pattern 7: `CalculateDepreciationRate`
//! Calculate depreciation rate as percentage
//! ```vb
//! Function CalculateDepreciationRate(cost As Double, salvage As Double, _
//!                                    life As Double) As Double
//!     Dim annualDepreciation As Double
//!     
//!     annualDepreciation = SLN(cost, salvage, life)
//!     CalculateDepreciationRate = (annualDepreciation / cost) * 100
//! End Function
//! ```
//!
//! ### Pattern 8: `CalculateReplacementYear`
//! Determine when asset should be replaced
//! ```vb
//! Function CalculateReplacementYear(cost As Double, salvage As Double, _
//!                                   life As Double, _
//!                                   minimumValue As Double) As Integer
//!     Dim depreciation As Double
//!     Dim bookValue As Double
//!     Dim year As Integer
//!     
//!     depreciation = SLN(cost, salvage, life)
//!     bookValue = cost
//!     
//!     For year = 1 To life
//!         bookValue = bookValue - depreciation
//!         If bookValue <= minimumValue Then
//!             CalculateReplacementYear = year
//!             Exit Function
//!         End If
//!     Next year
//!     
//!     CalculateReplacementYear = life
//! End Function
//! ```
//!
//! ### Pattern 9: `CalculateAccumulatedDepreciation`
//! Calculate accumulated depreciation at specific period
//! ```vb
//! Function CalculateAccumulatedDepreciation(cost As Double, salvage As Double, _
//!                                           life As Double, _
//!                                           period As Integer) As Double
//!     Dim annualDepreciation As Double
//!     
//!     annualDepreciation = SLN(cost, salvage, life)
//!     
//!     If period >= life Then
//!         CalculateAccumulatedDepreciation = cost - salvage
//!     Else
//!         CalculateAccumulatedDepreciation = annualDepreciation * period
//!     End If
//! End Function
//! ```
//!
//! ### Pattern 10: `FormatDepreciationReport`
//! Format depreciation information for display
//! ```vb
//! Function FormatDepreciationReport(cost As Double, salvage As Double, _
//!                                   life As Double, year As Integer) As String
//!     Dim depreciation As Double
//!     Dim accumulated As Double
//!     Dim bookValue As Double
//!     Dim report As String
//!     
//!     depreciation = SLN(cost, salvage, life)
//!     accumulated = depreciation * year
//!     bookValue = cost - accumulated
//!     
//!     report = "Depreciation Report - Year " & year & vbCrLf
//!     report = report & "Initial Cost: $" & Format(cost, "#,##0.00") & vbCrLf
//!     report = report & "Annual Depreciation: $" & Format(depreciation, "#,##0.00") & vbCrLf
//!     report = report & "Accumulated Depreciation: $" & Format(accumulated, "#,##0.00") & vbCrLf
//!     report = report & "Book Value: $" & Format(bookValue, "#,##0.00") & vbCrLf
//!     
//!     FormatDepreciationReport = report
//! End Function
//! ```
//!
//! ## Advanced Usage
//!
//! ### Example 1: `AssetDepreciationTracker` Class
//! Track depreciation for multiple assets
//! ```vb
//! ' Class: AssetDepreciationTracker
//! Private Type AssetInfo
//!     AssetName As String
//!     Cost As Double
//!     Salvage As Double
//!     Life As Double
//!     PurchaseDate As Date
//!     AnnualDepreciation As Double
//! End Type
//!
//! Private m_assets() As AssetInfo
//! Private m_assetCount As Integer
//!
//! Private Sub Class_Initialize()
//!     m_assetCount = 0
//!     ReDim m_assets(0 To 9)
//! End Sub
//!
//! Public Sub AddAsset(assetName As String, cost As Double, _
//!                     salvage As Double, life As Double, _
//!                     purchaseDate As Date)
//!     If m_assetCount > UBound(m_assets) Then
//!         ReDim Preserve m_assets(0 To m_assetCount * 2)
//!     End If
//!     
//!     With m_assets(m_assetCount)
//!         .AssetName = assetName
//!         .Cost = cost
//!         .Salvage = salvage
//!         .Life = life
//!         .PurchaseDate = purchaseDate
//!         .AnnualDepreciation = SLN(cost, salvage, life)
//!     End With
//!     
//!     m_assetCount = m_assetCount + 1
//! End Sub
//!
//! Public Function GetAssetBookValue(assetIndex As Integer, _
//!                                   asOfDate As Date) As Double
//!     Dim yearsElapsed As Double
//!     Dim totalDepreciation As Double
//!     Dim bookValue As Double
//!     
//!     If assetIndex < 0 Or assetIndex >= m_assetCount Then
//!         GetAssetBookValue = 0
//!         Exit Function
//!     End If
//!     
//!     With m_assets(assetIndex)
//!         yearsElapsed = DateDiff("d", .PurchaseDate, asOfDate) / 365.25
//!         
//!         If yearsElapsed >= .Life Then
//!             GetAssetBookValue = .Salvage
//!         Else
//!             totalDepreciation = .AnnualDepreciation * yearsElapsed
//!             bookValue = .Cost - totalDepreciation
//!             
//!             If bookValue < .Salvage Then
//!                 GetAssetBookValue = .Salvage
//!             Else
//!                 GetAssetBookValue = bookValue
//!             End If
//!         End If
//!     End With
//! End Function
//!
//! Public Function GetTotalBookValue(asOfDate As Date) As Double
//!     Dim i As Integer
//!     Dim total As Double
//!     
//!     total = 0
//!     For i = 0 To m_assetCount - 1
//!         total = total + GetAssetBookValue(i, asOfDate)
//!     Next i
//!     
//!     GetTotalBookValue = total
//! End Function
//!
//! Public Function GetAnnualDepreciationExpense() As Double
//!     Dim i As Integer
//!     Dim total As Double
//!     
//!     total = 0
//!     For i = 0 To m_assetCount - 1
//!         total = total + m_assets(i).AnnualDepreciation
//!     Next i
//!     
//!     GetAnnualDepreciationExpense = total
//! End Function
//!
//! Public Function GetAssetCount() As Integer
//!     GetAssetCount = m_assetCount
//! End Function
//! ```
//!
//! ### Example 2: `DepreciationScheduleGenerator` Module
//! Generate detailed depreciation schedules
//! ```vb
//! ' Module: DepreciationScheduleGenerator
//!
//! Public Function GenerateScheduleArray(cost As Double, salvage As Double, _
//!                                       life As Integer) As Variant
//!     Dim schedule() As Variant
//!     Dim depreciation As Double
//!     Dim bookValue As Double
//!     Dim accumulated As Double
//!     Dim i As Integer
//!     
//!     ReDim schedule(0 To life, 0 To 4)
//!     
//!     ' Headers
//!     schedule(0, 0) = "Year"
//!     schedule(0, 1) = "Beginning Value"
//!     schedule(0, 2) = "Depreciation"
//!     schedule(0, 3) = "Accumulated"
//!     schedule(0, 4) = "Ending Value"
//!     
//!     depreciation = SLN(cost, salvage, life)
//!     bookValue = cost
//!     accumulated = 0
//!     
//!     For i = 1 To life
//!         schedule(i, 0) = i
//!         schedule(i, 1) = bookValue
//!         schedule(i, 2) = depreciation
//!         accumulated = accumulated + depreciation
//!         schedule(i, 3) = accumulated
//!         bookValue = cost - accumulated
//!         schedule(i, 4) = bookValue
//!     Next i
//!     
//!     GenerateScheduleArray = schedule
//! End Function
//!
//! Public Sub ExportScheduleToCSV(cost As Double, salvage As Double, _
//!                                life As Integer, filePath As String)
//!     Dim schedule As Variant
//!     Dim fileNum As Integer
//!     Dim i As Integer
//!     Dim j As Integer
//!     Dim line As String
//!     
//!     schedule = GenerateScheduleArray(cost, salvage, life)
//!     
//!     fileNum = FreeFile
//!     Open filePath For Output As #fileNum
//!     
//!     For i = 0 To life
//!         line = ""
//!         For j = 0 To 4
//!             If j > 0 Then line = line & ","
//!             line = line & schedule(i, j)
//!         Next j
//!         Print #fileNum, line
//!     Next i
//!     
//!     Close #fileNum
//! End Sub
//!
//! Public Function CalculateQuarterlyDepreciation(cost As Double, salvage As Double, _
//!                                                yearsLife As Integer) As Double()
//!     Dim annualDepreciation As Double
//!     Dim quarterlyDepreciation As Double
//!     Dim quarters() As Double
//!     Dim i As Integer
//!     
//!     annualDepreciation = SLN(cost, salvage, yearsLife)
//!     quarterlyDepreciation = annualDepreciation / 4
//!     
//!     ReDim quarters(1 To yearsLife * 4)
//!     
//!     For i = 1 To yearsLife * 4
//!         quarters(i) = quarterlyDepreciation
//!     Next i
//!     
//!     CalculateQuarterlyDepreciation = quarters
//! End Function
//! ```
//!
//! ### Example 3: `DepreciationComparison` Class
//! Compare different depreciation methods
//! ```vb
//! ' Class: DepreciationComparison
//! Private m_cost As Double
//! Private m_salvage As Double
//! Private m_life As Integer
//!
//! Public Sub Initialize(cost As Double, salvage As Double, life As Integer)
//!     m_cost = cost
//!     m_salvage = salvage
//!     m_life = life
//! End Sub
//!
//! Public Function GetStraightLineDepreciation(year As Integer) As Double
//!     GetStraightLineDepreciation = SLN(m_cost, m_salvage, m_life)
//! End Function
//!
//! Public Function GetDoubleDecliningDepreciation(year As Integer) As Double
//!     GetDoubleDecliningDepreciation = DDB(m_cost, m_salvage, m_life, year)
//! End Function
//!
//! Public Function GetStraightLineBookValue(year As Integer) As Double
//!     Dim depreciation As Double
//!     depreciation = SLN(m_cost, m_salvage, m_life)
//!     GetStraightLineBookValue = m_cost - (depreciation * year)
//! End Function
//!
//! Public Function GetTotalDifference(year As Integer) As Double
//!     Dim slnTotal As Double
//!     Dim ddbTotal As Double
//!     Dim i As Integer
//!     
//!     slnTotal = 0
//!     ddbTotal = 0
//!     
//!     For i = 1 To year
//!         slnTotal = slnTotal + GetStraightLineDepreciation(i)
//!         ddbTotal = ddbTotal + GetDoubleDecliningDepreciation(i)
//!     Next i
//!     
//!     GetTotalDifference = ddbTotal - slnTotal
//! End Function
//!
//! Public Function GenerateComparisonReport(maxYears As Integer) As String
//!     Dim report As String
//!     Dim i As Integer
//!     
//!     report = "Depreciation Method Comparison" & vbCrLf
//!     report = report & "Cost: $" & Format(m_cost, "#,##0.00") & vbCrLf
//!     report = report & "Salvage: $" & Format(m_salvage, "#,##0.00") & vbCrLf
//!     report = report & "Life: " & m_life & " years" & vbCrLf & vbCrLf
//!     
//!     report = report & "Year" & vbTab & "SLN" & vbTab & "DDB" & vbCrLf
//!     
//!     For i = 1 To maxYears
//!         report = report & i & vbTab
//!         report = report & Format(GetStraightLineDepreciation(i), "#,##0") & vbTab
//!         report = report & Format(GetDoubleDecliningDepreciation(i), "#,##0") & vbCrLf
//!     Next i
//!     
//!     GenerateComparisonReport = report
//! End Function
//! ```
//!
//! ### Example 4: `FinancialPlanner` Module
//! Financial planning with depreciation
//! ```vb
//! ' Module: FinancialPlanner
//!
//! Public Function CalculateReplacementFund(cost As Double, salvage As Double, _
//!                                         life As Double) As Double
//!     ' Calculate annual savings needed to replace asset
//!     Dim replacementCost As Double
//!     
//!     ' Assume replacement cost increases by inflation
//!     replacementCost = cost * 1.03 ^ life  ' 3% annual inflation
//!     
//!     CalculateReplacementFund = (replacementCost - salvage) / life
//! End Function
//!
//! Public Function CalculateTaxSavings(cost As Double, salvage As Double, _
//!                                     life As Double, taxRate As Double) As Double
//!     ' Calculate annual tax savings from depreciation
//!     Dim annualDepreciation As Double
//!     
//!     annualDepreciation = SLN(cost, salvage, life)
//!     CalculateTaxSavings = annualDepreciation * taxRate
//! End Function
//!
//! Public Function CalculateNetPresentValue(cost As Double, salvage As Double, _
//!                                          life As Integer, _
//!                                          discountRate As Double) As Double
//!     ' Calculate NPV of depreciation tax shield
//!     Dim annualDepreciation As Double
//!     Dim taxShield As Double
//!     Dim npv As Double
//!     Dim i As Integer
//!     Const TAX_RATE As Double = 0.3  ' 30% tax rate
//!     
//!     annualDepreciation = SLN(cost, salvage, life)
//!     taxShield = annualDepreciation * TAX_RATE
//!     npv = 0
//!     
//!     For i = 1 To life
//!         npv = npv + taxShield / ((1 + discountRate) ^ i)
//!     Next i
//!     
//!     CalculateNetPresentValue = npv
//! End Function
//!
//! Public Function ShouldReplaceAsset(currentBookValue As Double, _
//!                                    replacementCost As Double, _
//!                                    yearsRemaining As Double, _
//!                                    expectedSavings As Double) As Boolean
//!     ' Determine if asset should be replaced now
//!     Dim replacementBenefit As Double
//!     
//!     replacementBenefit = expectedSavings * yearsRemaining
//!     ShouldReplaceAsset = replacementBenefit > replacementCost
//! End Function
//! ```
//!
//! ## Error Handling
//!
//! The SLN function can generate the following errors:
//!
//! - **Error 5** (Invalid procedure call or argument): If life is zero or negative
//! - **Error 6** (Overflow): If result exceeds Double range
//! - **Error 13** (Type mismatch): Arguments not numeric
//!
//! Always validate inputs before calling SLN:
//! ```vb
//! On Error Resume Next
//! depreciation = SLN(cost, salvage, life)
//! If Err.Number <> 0 Then
//!     MsgBox "Error calculating depreciation: " & Err.Description
//! End If
//! ```
//!
//! ## Performance Considerations
//!
//! - SLN is a very fast calculation (simple division)
//! - No iterative calculations required
//! - Can be called repeatedly without performance concerns
//! - Consider caching result if used multiple times with same inputs
//!
//! ## Best Practices
//!
//! 1. **Validate Inputs**: Check cost > salvage, life > 0
//! 2. **Consistent Units**: Ensure life matches desired period (years, months)
//! 3. **Handle Edge Cases**: Check for zero life, negative values
//! 4. **Document Assumptions**: Clearly state depreciation period
//! 5. **Salvage Value**: Use realistic salvage value estimates
//! 6. **Format Output**: Use Format function for currency display
//! 7. **Complete Schedules**: Generate full schedules for planning
//! 8. **Compare Methods**: Evaluate if straight-line is most appropriate
//! 9. **Tax Compliance**: Verify method meets tax requirements
//! 10. **Regular Review**: Update assumptions periodically
//!
//! ## Comparison with Related Functions
//!
//! | Function | Method | Depreciation Pattern | Best For |
//! |----------|--------|---------------------|----------|
//! | SLN | Straight-line | Constant each period | Uniform usage assets |
//! | DDB | Double-declining balance | Accelerated (higher early) | Tech, vehicles |
//! | SYD | Sum-of-years digits | Accelerated (graduated) | Equipment with declining efficiency |
//!
//! ## Platform Considerations
//!
//! - Available in VB6, VBA (all versions)
//! - Part of financial functions library
//! - Returns Double for precision
//! - Consistent across platforms
//!
//! ## Limitations
//!
//! - Assumes constant depreciation (may not reflect reality)
//! - Doesn't account for accelerated wear
//! - No adjustment for partial periods
//! - Salvage value is an estimate
//! - Doesn't consider tax implications directly
//! - Can't handle mid-period asset purchases without adjustment
//!
//! ## Related Functions
//!
//! - `DDB`: Returns depreciation using double-declining balance method
//! - `SYD`: Returns depreciation using sum-of-years digits method
//! - `FV`: Calculates future value (inverse concept)
//! - `PV`: Calculates present value
//!

use crate::error::{VBError, VBResult};
use crate::value::VBVariant;
use vb6core::error::err_number;

/// Implementation of the `SLN` function.
///
/// Calculates straight-line depreciation of an asset for a single period:
/// `(cost - salvage) / life`.
///
/// VB6 behavior:
/// - `cost` and `salvage` must be non-negative; `life` must be positive
/// - If `life` is 0 or negative, raises error 5 (Invalid procedure call)
/// - If `cost` or `salvage` is negative, raises error 5
/// - Returns a `Double`
pub fn sln(cost: &VBVariant, salvage: &VBVariant, life: &VBVariant) -> VBResult<VBVariant> {
    let cost_val = cost.as_f64()?;
    let salvage_val = salvage.as_f64()?;
    let life_val = life.as_f64()?;

    // Validate inputs per VB6 behavior
    if life_val <= 0.0 || cost_val < 0.0 || salvage_val < 0.0 {
        return Err(VBError::new(err_number::INVALID_PROCEDURE_CALL));
    }

    Ok(VBVariant::from_double((cost_val - salvage_val) / life_val))
}

#[cfg(test)]
mod tests {
    use super::sln;
    use crate::value::VBVariant;

    fn sln_default(cost: f64, salvage: f64, life: f64) -> VBVariant {
        sln(
            &VBVariant::from_double(cost),
            &VBVariant::from_double(salvage),
            &VBVariant::from_double(life),
        )
        .unwrap()
    }

    #[test]
    fn sln_basic_depreciation() {
        // Cost=50000, Salvage=5000, Life=5 → (50000-5000)/5 = 9000
        let result = sln_default(50000.0, 5000.0, 5.0);
        assert_eq!(result.as_f64().unwrap(), 9000.0);
    }

    #[test]
    fn sln_monthly_depreciation() {
        // Cost=24000, Salvage=2000, Life=36 months → 22000/36 ≈ 611.11
        let result = sln_default(24000.0, 2000.0, 36.0);
        assert!((result.as_f64().unwrap() - 611.1111111111111).abs() < 1e-9);
    }

    #[test]
    fn sln_zero_salvage() {
        let result = sln_default(10000.0, 0.0, 10.0);
        assert_eq!(result.as_f64().unwrap(), 1000.0);
    }

    #[test]
    fn sln_zero_life_raises_error_5() {
        let err = sln(
            &VBVariant::from_double(10000.0),
            &VBVariant::from_double(1000.0),
            &VBVariant::from_double(0.0),
        )
        .unwrap_err();
        assert_eq!(err.number, 5);
    }

    #[test]
    fn sln_negative_life_raises_error_5() {
        let err = sln(
            &VBVariant::from_double(10000.0),
            &VBVariant::from_double(1000.0),
            &VBVariant::from_double(-5.0),
        )
        .unwrap_err();
        assert_eq!(err.number, 5);
    }

    #[test]
    fn sln_negative_cost_raises_error_5() {
        let err = sln(
            &VBVariant::from_double(-100.0),
            &VBVariant::from_double(0.0),
            &VBVariant::from_double(5.0),
        )
        .unwrap_err();
        assert_eq!(err.number, 5);
    }

    #[test]
    fn sln_with_integer_args() {
        let result = sln(
            &VBVariant::from_long(10000),
            &VBVariant::from_long(1000),
            &VBVariant::from_long(5),
        )
        .unwrap();
        assert_eq!(result.as_f64().unwrap(), 1800.0);
    }

    #[test]
    fn sln_null_raises_invalid_use_of_null() {
        let err = sln(&VBVariant::Null, &VBVariant::Empty, &VBVariant::Empty).unwrap_err();
        assert_eq!(err.number, 94);
    }
}
