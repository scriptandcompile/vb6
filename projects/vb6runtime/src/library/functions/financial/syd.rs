//! VB6 `SYD` Function
//!
//! The `SYD` function returns a Double specifying the sum-of-years digits depreciation of an asset for a specified period.
//!
//! ## Syntax
//! ```vb6
//! SYD(cost, salvage, life, period)
//! ```
//!
//! ## Parameters
//! - `cost`: Required. Double specifying initial cost of the asset.
//! - `salvage`: Required. Double specifying value of the asset at the end of its useful life.
//! - `life`: Required. Double specifying length of the useful life of the asset.
//! - `period`: Required. Double specifying period for which asset depreciation is calculated.
//!
//! All arguments must be positive numbers. The `period` argument must be in the same units as the `life` argument.
//!
//! ## Returns
//! Returns a `Double` specifying the depreciation of an asset for a specific period when using the sum-of-years digits method.
//!
//! ## Remarks
//! The `SYD` function calculates depreciation using the sum-of-years digits method:
//!
//! - **Accelerated depreciation**: More depreciation in early years, less in later years
//! - **Sum-of-years calculation**: For life of 5 years, sum = 5+4+3+2+1 = 15
//! - **Period weighting**: Year 1 uses 5/15, Year 2 uses 4/15, etc.
//! - **Formula**: `SYD = ((cost - salvage) * (life - period + 1) * 2) / (life * (life + 1))`
//! - **Depreciable base**: `cost - salvage` (total amount to depreciate)
//! - **Declining fraction**: Remaining life / sum-of-years digits
//! - **Period must be valid**: Must be between 1 and `life` (inclusive)
//! - **Consistent units**: `period` and `life` must use same time units (years, months, etc.)
//!
//! ### Sum-of-Years Digits Method
//! The sum-of-years digits (SYD) method is an accelerated depreciation technique:
//! 1. Calculate the sum of all years: `sum = life * (life + 1) / 2`
//! 2. For each period, the depreciation fraction is: `(life - period + 1) / sum`
//! 3. Multiply the fraction by the depreciable amount: `(cost - salvage)`
//!
//! ### Example Calculation
//! For an asset with cost=$10,000, salvage=$1,000, life=5 years:
//! - Depreciable amount = $10,000 - $1,000 = $9,000
//! - Sum of years = 5+4+3+2+1 = 15
//! - Year 1: 5/15 × $9,000 = $3,000
//! - Year 2: 4/15 × $9,000 = $2,400
//! - Year 3: 3/15 × $9,000 = $1,800
//! - Year 4: 2/15 × $9,000 = $1,200
//! - Year 5: 1/15 × $9,000 = $600
//! - Total: $9,000 (fully depreciated to salvage value)
//!
//! ### When to Use SYD
//! - **Technology assets**: Equipment that loses value quickly initially
//! - **Vehicles**: Cars and trucks that depreciate faster when new
//! - **Tax advantages**: When accelerated depreciation provides tax benefits
//! - **Matching principle**: When asset productivity is higher in early years
//! - **Alternative to DDB**: Less aggressive than double-declining balance
//!
//! ## Typical Uses
//! 1. **Asset Depreciation**: Calculate annual depreciation for financial statements
//! 2. **Tax Calculations**: Determine tax-deductible depreciation amounts
//! 3. **Book Value Tracking**: Track declining book value of assets over time
//! 4. **Financial Reporting**: Generate depreciation schedules for reports
//! 5. **Budget Planning**: Estimate future depreciation expenses
//! 6. **Asset Management**: Track depreciation for multiple assets
//! 7. **Comparison Analysis**: Compare SYD with straight-line or DDB methods
//! 8. **Period Calculations**: Calculate depreciation for partial periods
//!
//! ## Basic Examples
//!
//! ### Example 1: Simple Annual Depreciation
//! ```vb6
//! Dim depreciation As Double
//! Dim cost As Double
//! Dim salvage As Double
//! Dim life As Double
//! Dim year As Double
//!
//! cost = 10000      ' Initial cost
//! salvage = 1000    ' Salvage value
//! life = 5          ' 5-year life
//! year = 1          ' First year
//!
//! depreciation = SYD(cost, salvage, life, year)
//! ' depreciation = 3000 (5/15 of 9000)
//! ```
//!
//! ### Example 2: Complete Depreciation Schedule
//! ```vb6
//! Sub ShowDepreciationSchedule()
//!     Dim cost As Double
//!     Dim salvage As Double
//!     Dim life As Integer
//!     Dim year As Integer
//!     Dim depreciation As Double
//!     Dim totalDep As Double
//!     
//!     cost = 50000
//!     salvage = 5000
//!     life = 10
//!     totalDep = 0
//!     
//!     For year = 1 To life
//!         depreciation = SYD(cost, salvage, life, year)
//!         totalDep = totalDep + depreciation
//!         Debug.Print "Year " & year & ": $" & Format$(depreciation, "#,##0.00")
//!     Next year
//!     
//!     Debug.Print "Total Depreciation: $" & Format$(totalDep, "#,##0.00")
//! End Sub
//! ```
//!
//! ### Example 3: Monthly Depreciation
//! ```vb6
//! Function CalculateMonthlyDepreciation(cost As Double, salvage As Double, _
//!                                       lifeYears As Integer, month As Integer) As Double
//!     Dim lifeMonths As Integer
//!     lifeMonths = lifeYears * 12
//!     
//!     ' Calculate depreciation for the specific month
//!     CalculateMonthlyDepreciation = SYD(cost, salvage, lifeMonths, month)
//! End Function
//! ```
//!
//! ### Example 4: Book Value Calculation
//! ```vb6
//! Function CalculateBookValue(cost As Double, salvage As Double, _
//!                            life As Integer, currentPeriod As Integer) As Double
//!     Dim period As Integer
//!     Dim totalDepreciation As Double
//!     
//!     totalDepreciation = 0
//!     For period = 1 To currentPeriod
//!         totalDepreciation = totalDepreciation + SYD(cost, salvage, life, period)
//!     Next period
//!     
//!     CalculateBookValue = cost - totalDepreciation
//! End Function
//! ```
//!
//! ## Common Patterns
//!
//! ### Pattern 1: Depreciation Schedule Generator
//! ```vb6
//! Function GenerateDepreciationSchedule(cost As Double, salvage As Double, _
//!                                       life As Integer) As Variant
//!     Dim schedule() As Double
//!     Dim period As Integer
//!     
//!     ReDim schedule(1 To life)
//!     
//!     For period = 1 To life
//!         schedule(period) = SYD(cost, salvage, life, period)
//!     Next period
//!     
//!     GenerateDepreciationSchedule = schedule
//! End Function
//! ```
//!
//! ### Pattern 2: Partial Year Depreciation
//! ```vb6
//! Function CalculatePartialYearDepreciation(cost As Double, salvage As Double, _
//!                                          life As Integer, year As Integer, _
//!                                          monthsInYear As Integer) As Double
//!     Dim fullYearDep As Double
//!     
//!     fullYearDep = SYD(cost, salvage, life, year)
//!     CalculatePartialYearDepreciation = fullYearDep * (monthsInYear / 12)
//! End Function
//! ```
//!
//! ### Pattern 3: Remaining Depreciable Amount
//! ```vb6
//! Function GetRemainingDepreciation(cost As Double, salvage As Double, _
//!                                   life As Integer, currentPeriod As Integer) As Double
//!     Dim period As Integer
//!     Dim accumulatedDep As Double
//!     
//!     accumulatedDep = 0
//!     For period = 1 To currentPeriod
//!         accumulatedDep = accumulatedDep + SYD(cost, salvage, life, period)
//!     Next period
//!     
//!     GetRemainingDepreciation = (cost - salvage) - accumulatedDep
//! End Function
//! ```
//!
//! ### Pattern 4: Compare Depreciation Methods
//! ```vb6
//! Sub CompareDepreciationMethods(cost As Double, salvage As Double, life As Integer)
//!     Dim period As Integer
//!     Dim sydDep As Double
//!     Dim slnDep As Double
//!     Dim ddbDep As Double
//!     
//!     Debug.Print "Period", "SYD", "SLN", "DDB"
//!     
//!     For period = 1 To life
//!         sydDep = SYD(cost, salvage, life, period)
//!         slnDep = SLN(cost, salvage, life)
//!         ddbDep = DDB(cost, salvage, life, period)
//!         
//!         Debug.Print period, Format$(sydDep, "#,##0.00"), _
//!                     Format$(slnDep, "#,##0.00"), _
//!                     Format$(ddbDep, "#,##0.00")
//!     Next period
//! End Sub
//! ```
//!
//! ### Pattern 5: Quarterly Depreciation
//! ```vb6
//! Function GetQuarterlyDepreciation(cost As Double, salvage As Double, _
//!                                  lifeYears As Integer, quarter As Integer) As Double
//!     Dim lifeQuarters As Integer
//!     lifeQuarters = lifeYears * 4
//!     
//!     GetQuarterlyDepreciation = SYD(cost, salvage, lifeQuarters, quarter)
//! End Function
//! ```
//!
//! ### Pattern 6: Accumulated Depreciation
//! ```vb6
//! Function GetAccumulatedDepreciation(cost As Double, salvage As Double, _
//!                                    life As Integer, throughPeriod As Integer) As Double
//!     Dim period As Integer
//!     Dim total As Double
//!     
//!     total = 0
//!     For period = 1 To throughPeriod
//!         total = total + SYD(cost, salvage, life, period)
//!     Next period
//!     
//!     GetAccumulatedDepreciation = total
//! End Function
//! ```
//!
//! ### Pattern 7: Depreciation Percentage
//! ```vb6
//! Function GetDepreciationPercentage(cost As Double, salvage As Double, _
//!                                   life As Integer, period As Integer) As Double
//!     Dim depreciableBase As Double
//!     Dim periodDep As Double
//!     
//!     depreciableBase = cost - salvage
//!     periodDep = SYD(cost, salvage, life, period)
//!     
//!     If depreciableBase > 0 Then
//!         GetDepreciationPercentage = (periodDep / depreciableBase) * 100
//!     Else
//!         GetDepreciationPercentage = 0
//!     End If
//! End Function
//! ```
//!
//! ### Pattern 8: Multi-Asset Depreciation
//! ```vb6
//! Function CalculateTotalDepreciation(costs() As Double, salvages() As Double, _
//!                                    lives() As Integer, period As Integer) As Double
//!     Dim i As Integer
//!     Dim total As Double
//!     
//!     total = 0
//!     For i = LBound(costs) To UBound(costs)
//!         total = total + SYD(costs(i), salvages(i), lives(i), period)
//!     Next i
//!     
//!     CalculateTotalDepreciation = total
//! End Function
//! ```
//!
//! ### Pattern 9: Validate Depreciation Parameters
//! ```vb6
//! Function ValidateDepreciationParams(cost As Double, salvage As Double, _
//!                                    life As Double, period As Double) As Boolean
//!     ValidateDepreciationParams = (cost > 0) And (salvage >= 0) And _
//!                                  (life > 0) And (period > 0) And _
//!                                  (period <= life) And (cost > salvage)
//! End Function
//! ```
//!
//! ### Pattern 10: Format Depreciation Report
//! ```vb6
//! Function FormatDepreciationLine(period As Integer, cost As Double, _
//!                                salvage As Double, life As Integer) As String
//!     Dim depreciation As Double
//!     Dim accumulated As Double
//!     Dim bookValue As Double
//!     Dim i As Integer
//!     
//!     depreciation = SYD(cost, salvage, life, period)
//!     
//!     accumulated = 0
//!     For i = 1 To period
//!         accumulated = accumulated + SYD(cost, salvage, life, i)
//!     Next i
//!     
//!     bookValue = cost - accumulated
//!     
//!     FormatDepreciationLine = Format$(period, "0") & vbTab & _
//!                             Format$(depreciation, "#,##0.00") & vbTab & _
//!                             Format$(accumulated, "#,##0.00") & vbTab & _
//!                             Format$(bookValue, "#,##0.00")
//! End Function
//! ```
//!
//! ## Advanced Usage
//!
//! ### Example 1: Asset Depreciation Manager Class
//! ```vb6
//! ' Class: AssetDepreciationManager
//! ' Manages depreciation calculations for assets using SYD method
//! Option Explicit
//!
//! Private m_Cost As Double
//! Private m_Salvage As Double
//! Private m_Life As Integer
//! Private m_CurrentPeriod As Integer
//!
//! Public Sub Initialize(cost As Double, salvage As Double, life As Integer)
//!     If cost <= salvage Then
//!         Err.Raise 5, , "Cost must be greater than salvage value"
//!     End If
//!     If life <= 0 Then
//!         Err.Raise 5, , "Life must be greater than zero"
//!     End If
//!     
//!     m_Cost = cost
//!     m_Salvage = salvage
//!     m_Life = life
//!     m_CurrentPeriod = 0
//! End Sub
//!
//! Public Function GetDepreciation(period As Integer) As Double
//!     If period < 1 Or period > m_Life Then
//!         Err.Raise 5, , "Period must be between 1 and " & m_Life
//!     End If
//!     
//!     GetDepreciation = SYD(m_Cost, m_Salvage, m_Life, period)
//! End Function
//!
//! Public Function GetAccumulatedDepreciation(throughPeriod As Integer) As Double
//!     Dim period As Integer
//!     Dim total As Double
//!     
//!     total = 0
//!     For period = 1 To throughPeriod
//!         total = total + GetDepreciation(period)
//!     Next period
//!     
//!     GetAccumulatedDepreciation = total
//! End Function
//!
//! Public Function GetBookValue(atPeriod As Integer) As Double
//!     GetBookValue = m_Cost - GetAccumulatedDepreciation(atPeriod)
//! End Function
//!
//! Public Function GetDepreciationSchedule() As Variant
//!     Dim schedule() As Variant
//!     Dim period As Integer
//!     Dim accumulated As Double
//!     
//!     ReDim schedule(0 To m_Life, 0 To 3) ' Period, Depreciation, Accumulated, Book Value
//!     
//!     schedule(0, 0) = "Period"
//!     schedule(0, 1) = "Depreciation"
//!     schedule(0, 2) = "Accumulated"
//!     schedule(0, 3) = "Book Value"
//!     
//!     accumulated = 0
//!     For period = 1 To m_Life
//!         Dim dep As Double
//!         dep = GetDepreciation(period)
//!         accumulated = accumulated + dep
//!         
//!         schedule(period, 0) = period
//!         schedule(period, 1) = dep
//!         schedule(period, 2) = accumulated
//!         schedule(period, 3) = m_Cost - accumulated
//!     Next period
//!     
//!     GetDepreciationSchedule = schedule
//! End Function
//!
//! Public Property Get Cost() As Double
//!     Cost = m_Cost
//! End Property
//!
//! Public Property Get SalvageValue() As Double
//!     SalvageValue = m_Salvage
//! End Property
//!
//! Public Property Get UsefulLife() As Integer
//!     UsefulLife = m_Life
//! End Property
//! ```
//!
//! ### Example 2: Depreciation Calculator Module
//! ```vb6
//! ' Module: DepreciationCalculator
//! ' Provides comprehensive depreciation calculation utilities
//! Option Explicit
//!
//! Public Function CalculateFullSchedule(cost As Double, salvage As Double, _
//!                                      life As Integer) As String
//!     Dim period As Integer
//!     Dim output As String
//!     Dim depreciation As Double
//!     Dim accumulated As Double
//!     Dim bookValue As Double
//!     
//!     output = "Period" & vbTab & "Depreciation" & vbTab & _
//!              "Accumulated" & vbTab & "Book Value" & vbCrLf
//!     output = output & String(60, "-") & vbCrLf
//!     
//!     accumulated = 0
//!     For period = 1 To life
//!         depreciation = SYD(cost, salvage, life, period)
//!         accumulated = accumulated + depreciation
//!         bookValue = cost - accumulated
//!         
//!         output = output & period & vbTab & _
//!                  Format$(depreciation, "$#,##0.00") & vbTab & _
//!                  Format$(accumulated, "$#,##0.00") & vbTab & _
//!                  Format$(bookValue, "$#,##0.00") & vbCrLf
//!     Next period
//!     
//!     CalculateFullSchedule = output
//! End Function
//!
//! Public Function CompareToStraightLine(cost As Double, salvage As Double, _
//!                                      life As Integer, period As Integer) As Double
//!     Dim sydDep As Double
//!     Dim slnDep As Double
//!     
//!     sydDep = SYD(cost, salvage, life, period)
//!     slnDep = SLN(cost, salvage, life)
//!     
//!     CompareToStraightLine = sydDep - slnDep
//! End Function
//!
//! Public Function CalculateFirstYearDepreciation(cost As Double, salvage As Double, _
//!                                               life As Integer, _
//!                                               purchaseMonth As Integer) As Double
//!     Dim monthsInFirstYear As Integer
//!     Dim fullYearDep As Double
//!     
//!     monthsInFirstYear = 13 - purchaseMonth
//!     fullYearDep = SYD(cost, salvage, life, 1)
//!     
//!     CalculateFirstYearDepreciation = fullYearDep * (monthsInFirstYear / 12)
//! End Function
//!
//! Public Function GetDepreciationRate(life As Integer, period As Integer) As Double
//!     Dim sumOfYears As Integer
//!     Dim remainingLife As Integer
//!     
//!     sumOfYears = life * (life + 1) / 2
//!     remainingLife = life - period + 1
//!     
//!     GetDepreciationRate = remainingLife / sumOfYears
//! End Function
//! ```
//!
//! ### Example 3: Multi-Asset Tracker Class
//! ```vb6
//! ' Class: MultiAssetTracker
//! ' Tracks depreciation for multiple assets
//! Option Explicit
//!
//! Private Type AssetInfo
//!     Name As String
//!     Cost As Double
//!     Salvage As Double
//!     Life As Integer
//!     PurchaseDate As Date
//! End Type
//!
//! Private m_Assets() As AssetInfo
//! Private m_AssetCount As Integer
//!
//! Public Sub Initialize()
//!     m_AssetCount = 0
//!     ReDim m_Assets(0 To 9)
//! End Sub
//!
//! Public Sub AddAsset(name As String, cost As Double, salvage As Double, _
//!                    life As Integer, purchaseDate As Date)
//!     If m_AssetCount >= UBound(m_Assets) Then
//!         ReDim Preserve m_Assets(0 To UBound(m_Assets) * 2)
//!     End If
//!     
//!     With m_Assets(m_AssetCount)
//!         .Name = name
//!         .Cost = cost
//!         .Salvage = salvage
//!         .Life = life
//!         .PurchaseDate = purchaseDate
//!     End With
//!     
//!     m_AssetCount = m_AssetCount + 1
//! End Sub
//!
//! Public Function GetTotalDepreciation(forYear As Integer) As Double
//!     Dim i As Integer
//!     Dim total As Double
//!     Dim period As Integer
//!     
//!     total = 0
//!     For i = 0 To m_AssetCount - 1
//!         period = forYear - Year(m_Assets(i).PurchaseDate) + 1
//!         If period >= 1 And period <= m_Assets(i).Life Then
//!             total = total + SYD(m_Assets(i).Cost, m_Assets(i).Salvage, _
//!                                m_Assets(i).Life, period)
//!         End If
//!     Next i
//!     
//!     GetTotalDepreciation = total
//! End Function
//!
//! Public Function GetAssetDepreciation(assetIndex As Integer, period As Integer) As Double
//!     If assetIndex < 0 Or assetIndex >= m_AssetCount Then
//!         Err.Raise 9, , "Invalid asset index"
//!     End If
//!     
//!     With m_Assets(assetIndex)
//!         If period < 1 Or period > .Life Then
//!             GetAssetDepreciation = 0
//!         Else
//!             GetAssetDepreciation = SYD(.Cost, .Salvage, .Life, period)
//!         End If
//!     End With
//! End Function
//!
//! Public Property Get AssetCount() As Integer
//!     AssetCount = m_AssetCount
//! End Property
//! ```
//!
//! ### Example 4: Tax Depreciation Reporter
//! ```vb6
//! ' Module: TaxDepreciationReporter
//! ' Generates tax depreciation reports using SYD method
//! Option Explicit
//!
//! Public Function GenerateTaxReport(assetName As String, cost As Double, _
//!                                  salvage As Double, life As Integer, _
//!                                  taxYear As Integer) As String
//!     Dim report As String
//!     Dim currentYear As Integer
//!     Dim depreciation As Double
//!     Dim accumulated As Double
//!     
//!     report = "Tax Depreciation Report - " & assetName & vbCrLf
//!     report = report & "Method: Sum-of-Years Digits (SYD)" & vbCrLf
//!     report = report & "Cost: " & Format$(cost, "$#,##0.00") & vbCrLf
//!     report = report & "Salvage: " & Format$(salvage, "$#,##0.00") & vbCrLf
//!     report = report & "Life: " & life & " years" & vbCrLf & vbCrLf
//!     
//!     accumulated = 0
//!     For currentYear = 1 To taxYear
//!         depreciation = SYD(cost, salvage, life, currentYear)
//!         accumulated = accumulated + depreciation
//!     Next currentYear
//!     
//!     report = report & "Depreciation for Year " & taxYear & ": " & _
//!              Format$(SYD(cost, salvage, life, taxYear), "$#,##0.00") & vbCrLf
//!     report = report & "Accumulated Depreciation: " & _
//!              Format$(accumulated, "$#,##0.00") & vbCrLf
//!     report = report & "Book Value: " & _
//!              Format$(cost - accumulated, "$#,##0.00") & vbCrLf
//!     
//!     GenerateTaxReport = report
//! End Function
//!
//! Public Function ExportToCSV(cost As Double, salvage As Double, life As Integer) As String
//!     Dim csv As String
//!     Dim period As Integer
//!     Dim depreciation As Double
//!     Dim accumulated As Double
//!     
//!     csv = "Period,Depreciation,Accumulated,Book Value" & vbCrLf
//!     
//!     accumulated = 0
//!     For period = 1 To life
//!         depreciation = SYD(cost, salvage, life, period)
//!         accumulated = accumulated + depreciation
//!         
//!         csv = csv & period & "," & _
//!               Round(depreciation, 2) & "," & _
//!               Round(accumulated, 2) & "," & _
//!               Round(cost - accumulated, 2) & vbCrLf
//!     Next period
//!     
//!     ExportToCSV = csv
//! End Function
//!
//! Public Function CalculateTaxSavings(cost As Double, salvage As Double, _
//!                                    life As Integer, taxRate As Double) As Variant
//!     Dim period As Integer
//!     Dim savings() As Double
//!     Dim depreciation As Double
//!     
//!     ReDim savings(1 To life)
//!     
//!     For period = 1 To life
//!         depreciation = SYD(cost, salvage, life, period)
//!         savings(period) = depreciation * taxRate
//!     Next period
//!     
//!     CalculateTaxSavings = savings
//! End Function
//! ```
//!
//! ## Error Handling
//! The `SYD` function can raise the following errors:
//!
//! - **Error 5 (Invalid procedure call or argument)**: If any argument is negative, or if `period > life`, or if `cost <= salvage`
//! - **Error 11 (Division by zero)**: If `life = 0`
//! - **Error 13 (Type mismatch)**: If arguments are not numeric
//!
//! ## Performance Notes
//! - Very fast calculation using direct formula
//! - No iterative computation required (unlike accumulated depreciation)
//! - Constant time O(1) for single period calculation
//! - For full schedule, O(n) where n is the life of the asset
//! - More efficient than DDB for certain tax scenarios
//!
//! ## Best Practices
//! 1. **Validate inputs** before calling SYD (cost > salvage, life > 0, period valid)
//! 2. **Use consistent units** for period and life (both in years, months, or quarters)
//! 3. **Handle salvage = 0** as valid (fully depreciate to zero)
//! 4. **Cache schedules** if calculating multiple periods for same asset
//! 5. **Round appropriately** for financial reporting (typically 2 decimal places)
//! 6. **Document assumptions** about partial periods and mid-year conventions
//! 7. **Compare methods** (SYD, SLN, DDB) to choose appropriate one
//! 8. **Consider tax implications** when choosing depreciation method
//! 9. **Track accumulated depreciation** separately for audit purposes
//! 10. **Validate period range** to avoid errors (1 to life inclusive)
//!
//! ## Comparison Table
//!
//! | Method | Pattern | Early Years | Later Years | Calculation |
//! |--------|---------|-------------|-------------|-------------|
//! | **SYD** | Accelerated | Higher | Lower | Sum-of-years formula |
//! | **DDB** | Accelerated | Highest | Lowest | Double rate |
//! | **SLN** | Straight-line | Equal | Equal | (Cost-Salvage)/Life |
//!
//! ## Platform Notes
//! - Available in VB6 and VBA
//! - Not available in `VBScript`
//! - Returns Double precision floating-point
//! - Part of the Financial functions library
//! - Requires all arguments to be positive (except salvage can be zero)
//!
//! ## Limitations
//! - Cannot handle negative values for cost, salvage, or life
//! - Period must be between 1 and life (inclusive)
//! - Does not handle mid-period conventions automatically
//! - No built-in support for asset disposals or write-offs
//! - Salvage value must be less than cost
//! - No automatic switching to straight-line method
//! - Does not account for bonus depreciation or special tax rules

use crate::error::{VBError, VBResult};
use crate::value::VBVariant;

/// Implementation of the `SYD` function.
///
/// Calculates sum-of-years digits depreciation of an asset for a specified
/// period:
/// `((cost - salvage) * (life - period + 1) * 2) / (life * (life + 1))`
///
/// VB6 behavior:
/// - `cost` and `salvage` must be non-negative; `life` must be positive
/// - `period` must be between 1 and `life` (inclusive)
/// - `cost` must be greater than `salvage`
/// - If any argument is invalid, raises error 5 (Invalid procedure call)
/// - Returns a `Double`
pub fn syd(
    cost: &VBVariant,
    salvage: &VBVariant,
    life: &VBVariant,
    period: &VBVariant,
) -> VBResult<VBVariant> {
    let cost_val = cost.as_f64()?;
    let salvage_val = salvage.as_f64()?;
    let life_val = life.as_f64()?;
    let period_val = period.as_f64()?;

    // Validate inputs per VB6 behavior
    if life_val <= 0.0
        || cost_val < 0.0
        || salvage_val < 0.0
        || period_val < 1.0
        || period_val > life_val
        || cost_val <= salvage_val
    {
        return Err(VBError::new(5));
    }

    let depreciable_base = cost_val - salvage_val;
    let remaining_life = life_val - period_val + 1.0;
    let sum_of_years = life_val * (life_val + 1.0) / 2.0;

    Ok(VBVariant::from_double(depreciable_base * remaining_life / sum_of_years))
}

#[cfg(test)]
mod tests {
    use super::syd;
    use crate::value::VBVariant;

    fn syd_default(cost: f64, salvage: f64, life: f64, period: f64) -> VBVariant {
        syd(
            &VBVariant::from_double(cost),
            &VBVariant::from_double(salvage),
            &VBVariant::from_double(life),
            &VBVariant::from_double(period),
        )
        .unwrap()
    }

    #[test]
    fn syd_first_period() {
        // Cost=10000, Salvage=1000, Life=5, Period=1
        // Sum-of-years=15, remaining=5 → 9000 * 5/15 = 3000
        let result = syd_default(10000.0, 1000.0, 5.0, 1.0);
        assert_eq!(result.as_f64().unwrap(), 3000.0);
    }

    #[test]
    fn syd_second_period() {
        // 9000 * 4/15 = 2400
        let result = syd_default(10000.0, 1000.0, 5.0, 2.0);
        assert_eq!(result.as_f64().unwrap(), 2400.0);
    }

    #[test]
    fn syd_last_period() {
        // 9000 * 1/15 = 600
        let result = syd_default(10000.0, 1000.0, 5.0, 5.0);
        assert_eq!(result.as_f64().unwrap(), 600.0);
    }

    #[test]
    fn syd_total_depreciates_to_salvage() {
        let cost = 10000.0;
        let salvage = 1000.0;
        let life = 5.0;
        let mut total = 0.0;
        for period in 1..=life as i64 {
            total += syd_default(cost, salvage, life, period as f64)
                .as_f64()
                .unwrap();
        }
        assert!((total - (cost - salvage)).abs() < 1e-9);
    }

    #[test]
    fn syd_zero_salvage() {
        // Cost=10000, Salvage=0, Life=5, Period=1 → 10000 * 5/15 ≈ 3333.33
        let result = syd_default(10000.0, 0.0, 5.0, 1.0);
        assert!((result.as_f64().unwrap() - 3333.3333333333335).abs() < 1e-9);
    }

    #[test]
    fn syd_with_integer_args() {
        let result = syd(
            &VBVariant::from_long(10000),
            &VBVariant::from_long(1000),
            &VBVariant::from_long(5),
            &VBVariant::from_long(2),
        )
        .unwrap();
        assert_eq!(result.as_f64().unwrap(), 2400.0);
    }

    #[test]
    fn syd_zero_life_raises_error_5() {
        let err = syd(
            &VBVariant::from_double(10000.0),
            &VBVariant::from_double(1000.0),
            &VBVariant::from_double(0.0),
            &VBVariant::from_double(1.0),
        )
        .unwrap_err();
        assert_eq!(err.number, 5);
    }

    #[test]
    fn syd_period_greater_than_life_raises_error_5() {
        let err = syd(
            &VBVariant::from_double(10000.0),
            &VBVariant::from_double(1000.0),
            &VBVariant::from_double(5.0),
            &VBVariant::from_double(6.0),
        )
        .unwrap_err();
        assert_eq!(err.number, 5);
    }

    #[test]
    fn syd_period_less_than_one_raises_error_5() {
        let err = syd(
            &VBVariant::from_double(10000.0),
            &VBVariant::from_double(1000.0),
            &VBVariant::from_double(5.0),
            &VBVariant::from_double(0.0),
        )
        .unwrap_err();
        assert_eq!(err.number, 5);
    }

    #[test]
    fn syd_cost_less_than_salvage_raises_error_5() {
        let err = syd(
            &VBVariant::from_double(1000.0),
            &VBVariant::from_double(2000.0),
            &VBVariant::from_double(5.0),
            &VBVariant::from_double(1.0),
        )
        .unwrap_err();
        assert_eq!(err.number, 5);
    }

    #[test]
    fn syd_negative_cost_raises_error_5() {
        let err = syd(
            &VBVariant::from_double(-100.0),
            &VBVariant::from_double(0.0),
            &VBVariant::from_double(5.0),
            &VBVariant::from_double(1.0),
        )
        .unwrap_err();
        assert_eq!(err.number, 5);
    }

    #[test]
    fn syd_null_raises_invalid_use_of_null() {
        let err = syd(
            &VBVariant::Null,
            &VBVariant::Empty,
            &VBVariant::Empty,
            &VBVariant::Empty,
        )
        .unwrap_err();
        assert_eq!(err.number, 94);
    }
}
