//! # IRR Function
//!
//! Returns a Double specifying the internal rate of return for a series of periodic cash flows
//! (payments and receipts).
//!
//! ## Syntax
//!
//! ```vb
//! IRR(values()[, guess])
//! ```
//!
//! ## Parameters
//!
//! - `values()` (Required): Array of Double specifying cash flow values. The array must contain
//!   at least one positive value (receipt) and one negative value (payment).
//! - `guess` (Optional): Variant specifying value you estimate will be returned by IRR. If omitted,
//!   guess is 0.1 (10 percent).
//!
//! ## Return Value
//!
//! Returns a Double representing the internal rate of return:
//! - Expressed as a decimal (0.1 = 10%)
//! - The discount rate that makes the net present value (NPV) of all cash flows equal to zero
//! - Used to evaluate the profitability of potential investments
//! - Higher IRR indicates more desirable investment
//!
//! ## Remarks
//!
//! The internal rate of return is the interest rate received for an investment consisting of
//! payments and receipts that occur at regular intervals.
//!
//! - Uses Newton-Raphson iteration to find the rate where NPV equals zero
//! - Begins with guess value and iterates until result is accurate within 0.00001
//! - Fails after 20 iterations if no convergence (Error 5)
//! - Array must contain at least one positive and one negative value
//! - Cash flows must occur at regular intervals
//!
//! ## Typical Uses
//!
//! 1. **Investment Analysis**: Evaluate profitability of potential investments
//! 2. **Project Evaluation**: Compare multiple projects to select most profitable
//! 3. **Capital Budgeting**: Assess capital expenditure decisions
//! 4. **Business Case Analysis**: Justify business investments with `ROI` calculations
//! 5. **Equipment Purchase**: Evaluate cost savings from new equipment
//! 6. **Real Estate Investment**: Analyze property investment returns
//! 7. **Lease vs Buy**: Compare financial impact of leasing versus purchasing
//! 8. **Portfolio Management**: Assess historical returns on investments
//!
//! ## Basic Usage Examples
//!
//! ```vb
//! ' Example 1: Simple investment analysis
//! Dim cashFlows(0 To 4) As Double
//! Dim returnRate As Double
//!
//! cashFlows(0) = -10000  ' Initial investment (negative = cash out)
//! cashFlows(1) = 3000    ' Year 1 return
//! cashFlows(2) = 3500    ' Year 2 return
//! cashFlows(3) = 4000    ' Year 3 return
//! cashFlows(4) = 4500    ' Year 4 return
//!
//! returnRate = IRR(cashFlows)
//! Debug.Print "Internal Rate of Return: " & Format$(returnRate * 100, "0.00") & "%"
//! ' Prints approximately: 28.09%
//!
//! ' Example 2: Equipment purchase evaluation
//! Dim equipmentCosts(0 To 5) As Double
//! equipmentCosts(0) = -50000  ' Equipment cost
//! equipmentCosts(1) = 12000   ' Year 1 savings
//! equipmentCosts(2) = 15000   ' Year 2 savings
//! equipmentCosts(3) = 18000   ' Year 3 savings
//! equipmentCosts(4) = 21000   ' Year 4 savings
//! equipmentCosts(5) = 24000   ' Year 5 savings
//!
//! returnRate = IRR(equipmentCosts)
//! If returnRate > 0.15 Then  ' 15% hurdle rate
//!     MsgBox "Equipment purchase approved - IRR: " & Format$(returnRate * 100, "0.00") & "%"
//! Else
//!     MsgBox "Equipment purchase rejected - IRR too low"
//! End If
//!
//! ' Example 3: Comparing two projects
//! Dim projectA(0 To 3) As Double
//! Dim projectB(0 To 3) As Double
//!
//! projectA(0) = -25000: projectA(1) = 10000: projectA(2) = 12000: projectA(3) = 15000
//! projectB(0) = -30000: projectB(1) = 15000: projectB(2) = 14000: projectB(3) = 13000
//!
//! Dim irrA As Double, irrB As Double
//! irrA = IRR(projectA)
//! irrB = IRR(projectB)
//!
//! Debug.Print "Project A IRR: " & Format$(irrA * 100, "0.00") & "%"
//! Debug.Print "Project B IRR: " & Format$(irrB * 100, "0.00") & "%"
//!
//! If irrA > irrB Then
//!     MsgBox "Select Project A"
//! Else
//!     MsgBox "Select Project B"
//! End If
//!
//! ' Example 4: Using guess parameter for difficult calculations
//! Dim complexFlows(0 To 6) As Double
//! complexFlows(0) = -100000
//! complexFlows(1) = -50000   ' Additional investment in year 2
//! complexFlows(2) = 20000
//! complexFlows(3) = 40000
//! complexFlows(4) = 50000
//! complexFlows(5) = 60000
//! complexFlows(6) = 70000
//!
//! ' Provide guess to help convergence
//! On Error Resume Next
//! returnRate = IRR(complexFlows, 0.2)  ' Start with 20% guess
//! If Err.Number = 0 Then
//!     Debug.Print "Complex IRR: " & Format$(returnRate * 100, "0.00") & "%"
//! Else
//!     Debug.Print "Could not calculate IRR"
//! End If
//! On Error GoTo 0
//! ```
//!
//! ## Common Patterns
//!
//! ```vb
//! ' Pattern 1: Calculate IRR for investment
//! Function CalculateInvestmentIRR(initialInvestment As Double, returns() As Double) As Double
//!     Dim cashFlows() As Double
//!     Dim i As Integer
//!     
//!     ReDim cashFlows(0 To UBound(returns) + 1)
//!     cashFlows(0) = -Abs(initialInvestment)  ' Ensure negative
//!     
//!     For i = 0 To UBound(returns)
//!         cashFlows(i + 1) = returns(i)
//!     Next i
//!     
//!     CalculateInvestmentIRR = IRR(cashFlows)
//! End Function
//!
//! ' Pattern 2: IRR with hurdle rate comparison
//! Function MeetsHurdleRate(cashFlows() As Double, hurdleRate As Double) As Boolean
//!     On Error Resume Next
//!     Dim rate As Double
//!     rate = IRR(cashFlows)
//!     
//!     If Err.Number = 0 Then
//!         MeetsHurdleRate = (rate >= hurdleRate)
//!     Else
//!         MeetsHurdleRate = False
//!     End If
//!     On Error GoTo 0
//! End Function
//!
//! ' Pattern 3: Format IRR as percentage
//! Function FormatIRR(cashFlows() As Double) As String
//!     On Error Resume Next
//!     Dim rate As Double
//!     rate = IRR(cashFlows)
//!     
//!     If Err.Number = 0 Then
//!         FormatIRR = Format$(rate * 100, "0.00") & "%"
//!     Else
//!         FormatIRR = "N/A"
//!     End If
//!     On Error GoTo 0
//! End Function
//!
//! ' Pattern 4: Select best investment from multiple options
//! Function SelectBestInvestment(investments As Collection) As Integer
//!     Dim bestIRR As Double
//!     Dim bestIndex As Integer
//!     Dim currentIRR As Double
//!     Dim i As Integer
//!     
//!     bestIRR = -999999
//!     bestIndex = -1
//!     
//!     For i = 1 To investments.Count
//!         On Error Resume Next
//!         currentIRR = IRR(investments(i))
//!         
//!         If Err.Number = 0 And currentIRR > bestIRR Then
//!             bestIRR = currentIRR
//!             bestIndex = i
//!         End If
//!         On Error GoTo 0
//!     Next i
//!     
//!     SelectBestInvestment = bestIndex
//! End Function
//!
//! ' Pattern 5: Calculate IRR with validation
//! Function SafeIRR(cashFlows() As Double, Optional guess As Double = 0.1) As Variant
//!     Dim hasPositive As Boolean
//!     Dim hasNegative As Boolean
//!     Dim i As Integer
//!     
//!     ' Validate array has both positive and negative values
//!     For i = LBound(cashFlows) To UBound(cashFlows)
//!         If cashFlows(i) > 0 Then hasPositive = True
//!         If cashFlows(i) < 0 Then hasNegative = True
//!     Next i
//!     
//!     If Not (hasPositive And hasNegative) Then
//!         SafeIRR = Null
//!         Exit Function
//!     End If
//!     
//!     On Error Resume Next
//!     SafeIRR = IRR(cashFlows, guess)
//!     If Err.Number <> 0 Then SafeIRR = Null
//!     On Error GoTo 0
//! End Function
//!
//! ' Pattern 6: Compare project IRRs
//! Sub CompareProjects(project1() As Double, project2() As Double)
//!     Dim irr1 As Double, irr2 As Double
//!     
//!     irr1 = IRR(project1)
//!     irr2 = IRR(project2)
//!     
//!     Debug.Print "Project 1 IRR: " & Format$(irr1 * 100, "0.00") & "%"
//!     Debug.Print "Project 2 IRR: " & Format$(irr2 * 100, "0.00") & "%"
//!     Debug.Print "Difference: " & Format$((irr1 - irr2) * 100, "0.00") & " percentage points"
//! End Sub
//!
//! ' Pattern 7: Calculate breakeven IRR
//! Function GetBreakevenIRR(costOfCapital As Double, cashFlows() As Double) As String
//!     Dim projectIRR As Double
//!     projectIRR = IRR(cashFlows)
//!     
//!     If projectIRR > costOfCapital Then
//!         GetBreakevenIRR = "Project exceeds cost of capital by " & _
//!                          Format$((projectIRR - costOfCapital) * 100, "0.00") & "%"
//!     ElseIf projectIRR < costOfCapital Then
//!         GetBreakevenIRR = "Project falls short of cost of capital by " & _
//!                          Format$((costOfCapital - projectIRR) * 100, "0.00") & "%"
//!     Else
//!         GetBreakevenIRR = "Project exactly meets cost of capital"
//!     End If
//! End Function
//!
//! ' Pattern 8: IRR for monthly cash flows
//! Function MonthlyIRR(monthlyCashFlows() As Double) As Double
//!     ' Returns annualized IRR from monthly cash flows
//!     Dim monthlyRate As Double
//!     monthlyRate = IRR(monthlyCashFlows)
//!     MonthlyIRR = ((1 + monthlyRate) ^ 12) - 1  ' Convert to annual rate
//! End Function
//!
//! ' Pattern 9: Try multiple guesses if IRR fails
//! Function RobustIRR(cashFlows() As Double) As Variant
//!     Dim guesses As Variant
//!     Dim i As Integer
//!     Dim result As Double
//!     
//!     guesses = Array(0.1, 0.2, 0.5, -0.1, -0.2, 0.01, 0.9)
//!     
//!     For i = 0 To UBound(guesses)
//!         On Error Resume Next
//!         result = IRR(cashFlows, guesses(i))
//!         
//!         If Err.Number = 0 Then
//!             RobustIRR = result
//!             On Error GoTo 0
//!             Exit Function
//!         End If
//!         On Error GoTo 0
//!     Next i
//!     
//!     RobustIRR = Null  ' Could not calculate
//! End Function
//!
//! ' Pattern 10: Incremental IRR analysis
//! Function IncrementalIRR(baseProject() As Double, incrementalProject() As Double) As Double
//!     Dim incrementalFlows() As Double
//!     Dim i As Integer
//!     Dim maxIndex As Integer
//!     
//!     ' Calculate incremental cash flows
//!     maxIndex = IIf(UBound(baseProject) > UBound(incrementalProject), _
//!                    UBound(baseProject), UBound(incrementalProject))
//!     
//!     ReDim incrementalFlows(0 To maxIndex)
//!     
//!     For i = 0 To maxIndex
//!         incrementalFlows(i) = 0
//!         If i <= UBound(incrementalProject) Then
//!             incrementalFlows(i) = incrementalFlows(i) + incrementalProject(i)
//!         End If
//!         If i <= UBound(baseProject) Then
//!             incrementalFlows(i) = incrementalFlows(i) - baseProject(i)
//!         End If
//!     Next i
//!     
//!     IncrementalIRR = IRR(incrementalFlows)
//! End Function
//! ```
//!
//! ## Advanced Usage Examples
//!
//! ```vb
//! ' Example 1: Investment analyzer class
//! Public Class InvestmentAnalyzer
//!     Private m_cashFlows() As Double
//!     Private m_irr As Variant
//!     Private m_calculated As Boolean
//!     
//!     Public Sub SetCashFlows(cashFlows() As Double)
//!         Dim i As Integer
//!         ReDim m_cashFlows(LBound(cashFlows) To UBound(cashFlows))
//!         
//!         For i = LBound(cashFlows) To UBound(cashFlows)
//!             m_cashFlows(i) = cashFlows(i)
//!         Next i
//!         
//!         m_calculated = False
//!     End Sub
//!     
//!     Public Function GetIRR() As Variant
//!         If Not m_calculated Then
//!             On Error Resume Next
//!             m_irr = IRR(m_cashFlows)
//!             If Err.Number <> 0 Then m_irr = Null
//!             On Error GoTo 0
//!             m_calculated = True
//!         End If
//!         GetIRR = m_irr
//!     End Function
//!     
//!     Public Function GetFormattedIRR() As String
//!         Dim rate As Variant
//!         rate = GetIRR()
//!         
//!         If IsNull(rate) Then
//!             GetFormattedIRR = "N/A"
//!         Else
//!             GetFormattedIRR = Format$(rate * 100, "0.00") & "%"
//!         End If
//!     End Function
//!     
//!     Public Function IsAcceptable(hurdleRate As Double) As Boolean
//!         Dim rate As Variant
//!         rate = GetIRR()
//!         
//!         If IsNull(rate) Then
//!             IsAcceptable = False
//!         Else
//!             IsAcceptable = (rate >= hurdleRate)
//!         End If
//!     End Function
//!     
//!     Public Function CompareToRate(targetRate As Double) As String
//!         Dim rate As Variant
//!         rate = GetIRR()
//!         
//!         If IsNull(rate) Then
//!             CompareToRate = "Unable to calculate IRR"
//!         ElseIf rate > targetRate Then
//!             CompareToRate = "Exceeds target by " & _
//!                            Format$((rate - targetRate) * 100, "0.00") & "%"
//!         ElseIf rate < targetRate Then
//!             CompareToRate = "Below target by " & _
//!                            Format$((targetRate - rate) * 100, "0.00") & "%"
//!         Else
//!             CompareToRate = "Exactly meets target"
//!         End If
//!     End Function
//! End Class
//!
//! ' Example 2: Project portfolio manager
//! Public Class ProjectPortfolio
//!     Private m_projects As Collection
//!     
//!     Private Sub Class_Initialize()
//!         Set m_projects = New Collection
//!     End Sub
//!     
//!     Public Sub AddProject(projectName As String, cashFlows() As Double)
//!         Dim projectData As Variant
//!         projectData = Array(projectName, cashFlows)
//!         m_projects.Add projectData
//!     End Sub
//!     
//!     Public Function GetBestProject() As String
//!         Dim bestIRR As Double
//!         Dim bestName As String
//!         Dim currentIRR As Double
//!         Dim i As Integer
//!         Dim projectData As Variant
//!         
//!         bestIRR = -999999
//!         bestName = ""
//!         
//!         For i = 1 To m_projects.Count
//!             projectData = m_projects(i)
//!             
//!             On Error Resume Next
//!             currentIRR = IRR(projectData(1))
//!             
//!             If Err.Number = 0 And currentIRR > bestIRR Then
//!                 bestIRR = currentIRR
//!                 bestName = projectData(0)
//!             End If
//!             On Error GoTo 0
//!         Next i
//!         
//!         GetBestProject = bestName & " (IRR: " & Format$(bestIRR * 100, "0.00") & "%)"
//!     End Function
//!     
//!     Public Function GetRankedProjects() As String
//!         Dim rankings() As Variant
//!         Dim i As Integer, j As Integer
//!         Dim temp As Variant
//!         Dim result As String
//!         Dim projectData As Variant
//!         Dim projectIRR As Double
//!         
//!         ReDim rankings(1 To m_projects.Count)
//!         
//!         ' Build array of project names and IRRs
//!         For i = 1 To m_projects.Count
//!             projectData = m_projects(i)
//!             
//!             On Error Resume Next
//!             projectIRR = IRR(projectData(1))
//!             If Err.Number <> 0 Then projectIRR = -999999
//!             On Error GoTo 0
//!             
//!             rankings(i) = Array(projectData(0), projectIRR)
//!         Next i
//!         
//!         ' Sort by IRR (descending)
//!         For i = 1 To UBound(rankings) - 1
//!             For j = i + 1 To UBound(rankings)
//!                 If rankings(j)(1) > rankings(i)(1) Then
//!                     temp = rankings(i)
//!                     rankings(i) = rankings(j)
//!                     rankings(j) = temp
//!                 End If
//!             Next j
//!         Next i
//!         
//!         ' Build result string
//!         result = "Project Rankings:" & vbCrLf
//!         For i = 1 To UBound(rankings)
//!             result = result & i & ". " & rankings(i)(0) & ": " & _
//!                      Format$(rankings(i)(1) * 100, "0.00") & "%" & vbCrLf
//!         Next i
//!         
//!         GetRankedProjects = result
//!     End Function
//! End Class
//!
//! ' Example 3: Capital budgeting calculator
//! Function EvaluateCapitalProject(initialCost As Double, annualSavings As Double, _
//!                                years As Integer, salvageValue As Double, _
//!                                hurdleRate As Double) As String
//!     Dim cashFlows() As Double
//!     Dim i As Integer
//!     Dim projectIRR As Double
//!     Dim result As String
//!     
//!     ReDim cashFlows(0 To years)
//!     cashFlows(0) = -Abs(initialCost)
//!     
//!     For i = 1 To years - 1
//!         cashFlows(i) = annualSavings
//!     Next i
//!     
//!     cashFlows(years) = annualSavings + salvageValue
//!     
//!     projectIRR = IRR(cashFlows)
//!     
//!     result = "Capital Project Evaluation" & vbCrLf
//!     result = result & "Initial Cost: " & Format$(initialCost, "Currency") & vbCrLf
//!     result = result & "Annual Savings: " & Format$(annualSavings, "Currency") & vbCrLf
//!     result = result & "Project Life: " & years & " years" & vbCrLf
//!     result = result & "Salvage Value: " & Format$(salvageValue, "Currency") & vbCrLf
//!     result = result & "IRR: " & Format$(projectIRR * 100, "0.00") & "%" & vbCrLf
//!     result = result & "Hurdle Rate: " & Format$(hurdleRate * 100, "0.00") & "%" & vbCrLf
//!     
//!     If projectIRR >= hurdleRate Then
//!         result = result & "Recommendation: APPROVE"
//!     Else
//!         result = result & "Recommendation: REJECT"
//!     End If
//!     
//!     EvaluateCapitalProject = result
//! End Function
//!
//! ' Example 4: Real estate investment analyzer
//! Function AnalyzeRealEstateInvestment(purchasePrice As Double, downPayment As Double, _
//!                                      monthlyRent As Double, monthlyExpenses As Double, _
//!                                      years As Integer, appreciationRate As Double) As String
//!     Dim cashFlows() As Double
//!     Dim i As Integer
//!     Dim salePrice As Double
//!     Dim annualIRR As Double
//!     Dim result As String
//!     
//!     ReDim cashFlows(0 To years)
//!     
//!     ' Initial investment (down payment)
//!     cashFlows(0) = -Abs(downPayment)
//!     
//!     ' Annual net cash flows
//!     For i = 1 To years - 1
//!         cashFlows(i) = (monthlyRent - monthlyExpenses) * 12
//!     Next i
//!     
//!     ' Final year includes sale
//!     salePrice = purchasePrice * ((1 + appreciationRate) ^ years)
//!     cashFlows(years) = (monthlyRent - monthlyExpenses) * 12 + salePrice - (purchasePrice - downPayment)
//!     
//!     annualIRR = IRR(cashFlows)
//!     
//!     result = "Real Estate Investment Analysis" & vbCrLf
//!     result = result & "Purchase Price: " & Format$(purchasePrice, "Currency") & vbCrLf
//!     result = result & "Down Payment: " & Format$(downPayment, "Currency") & vbCrLf
//!     result = result & "Monthly Rent: " & Format$(monthlyRent, "Currency") & vbCrLf
//!     result = result & "Monthly Expenses: " & Format$(monthlyExpenses, "Currency") & vbCrLf
//!     result = result & "Holding Period: " & years & " years" & vbCrLf
//!     result = result & "Annual IRR: " & Format$(annualIRR * 100, "0.00") & "%"
//!     
//!     AnalyzeRealEstateInvestment = result
//! End Function
//! ```
//!
//! ## Error Handling
//!
//! The IRR function can raise errors:
//!
//! - **Invalid procedure call (Error 5)**: If IRR can't find a result after 20 iterations, or if array doesn't contain at least one positive and one negative value
//! - **Type Mismatch (Error 13)**: If values array is not numeric
//! - **Subscript out of range (Error 9)**: If array is invalid
//!
//! ```vb
//! On Error GoTo ErrorHandler
//! Dim cashFlows(0 To 4) As Double
//! Dim rate As Double
//!
//! cashFlows(0) = -10000
//! cashFlows(1) = 3000
//! cashFlows(2) = 3500
//! cashFlows(3) = 4000
//! cashFlows(4) = 4500
//!
//! rate = IRR(cashFlows)
//! Debug.Print "IRR: " & Format$(rate * 100, "0.00") & "%"
//! Exit Sub
//!
//! ErrorHandler:
//!     If Err.Number = 5 Then
//!         MsgBox "Unable to calculate IRR. Try a different guess value.", vbCritical
//!     Else
//!         MsgBox "Error calculating IRR: " & Err.Description, vbCritical
//!     End If
//! ```
//!
//! ## Performance Considerations
//!
//! - **Iterative Calculation**: `IRR` uses iterative algorithm that can be slow for complex cash flows
//! - **Convergence**: May require multiple iterations; providing good `guess` can improve performance
//! - **Array Size**: Larger arrays take longer to process
//! - **Caching**: Cache calculated `IRR` values rather than recalculating repeatedly
//!
//! ## Best Practices
//!
//! 1. **Validate Input**: Ensure array contains at least one positive and one negative value
//! 2. **Error Handling**: Always wrap `IRR` in error handler as it may fail to converge
//! 3. **Sign Convention**: Use negative for cash outflows (investments), positive for inflows (returns)
//! 4. **Provide Guess**: For complex cash flows or when default fails, provide appropriate guess value
//! 5. **Regular Intervals**: Ensure cash flows occur at regular, consistent intervals
//! 6. **Order Matters**: Values must be in chronological order in the array
//! 7. **Hurdle Rate**: Compare `IRR` to hurdle rate or cost of capital to make decisions
//! 8. **Multiple IRRs**: Be aware that some cash flow patterns can have multiple valid `IRR`s
//! 9. **Complement with NPV**: Use `NPV` alongside `IRR` for complete investment analysis
//! 10. **Format for Display**: Multiply by 100 and format as percentage for user display
//!
//! ## Comparison with Related Functions
//!
//! | Function | Purpose | Return Value | Use Case |
//! |----------|---------|--------------|----------|
//! | `IRR` | Internal rate of return | Rate (`Decimal`) | Evaluate single investment profitability |
//! | `MIRR` | Modified `IRR` | Rate (`Decimal`) | Handle reinvestment assumptions |
//! | `NPV` | Net present value | `Currency` amount | Calculate dollar value at given rate |
//! | `PV` | Present value | `Currency` amount | Simple annuity present value |
//! | `FV` | Future value | `Currency` amount | Simple annuity future value |
//!
//! ## Platform and Version Notes
//!
//! - Available in all VB6 versions
//! - Part of VBA financial functions
//! - Uses `Double` precision
//! - Consistent with Excel's `IRR` function
//! - Maximum 20 iterations for convergence
//!
//! ## Limitations
//!
//! - Assumes cash flows occur at regular intervals
//! - May fail to converge for certain cash flow patterns
//! - Assumes reinvestment at the `IRR` rate (use `MIRR` for different assumption)
//! - Cannot handle irregular time periods between cash flows (use `XIRR` in Excel for that)
//! - Multiple `IRR`s possible for some cash flow patterns (multiple sign changes)
//! - Does not account for risk differences between projects
//!
//! ## Related Functions
//!
//! - `MIRR`: Modified internal rate of return with reinvestment rate
//! - `NPV`: Net present value
//! - `PV`: Present value
//! - `FV`: Future value
//! - `Rate`: Interest rate per period

use crate::error::{VBError, VBResult};
use crate::value::VBVariant;
use vb6core::error::err_number;

/// Implementation of the `IRR` function.
///
/// Calculates the internal rate of return for a series of periodic cash flows
/// using iteration. The IRR is the discount rate that makes the net present value
/// (NPV) of all cash flows equal to zero.
///
/// VB6 behavior:
/// - `values` must be an array with at least one positive and one negative value
/// - Optional `guess` defaults to 0.1 (10%)
/// - Iterates up to 20 times until result is within 0.00001 of previous iteration
/// - Raises error 5 if IRR cannot be found after 20 iterations
pub fn irr(values: &VBVariant, guess: Option<&VBVariant>) -> VBResult<VBVariant> {
    let arr = values.as_array()?;
    let guess_val = guess.map(|g| g.as_f64()).transpose()?.unwrap_or(0.1);

    let cash_flows: Vec<f64> = arr
        .as_slice()
        .iter()
        .map(|v| v.as_f64())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| VBError::type_mismatch())?;

    if cash_flows.is_empty() {
        return Err(VBError::new(err_number::INVALID_PROCEDURE_CALL));
    }

    // Validate that array contains at least one positive and one negative value
    let has_positive = cash_flows.iter().any(|&v| v > 0.0);
    let has_negative = cash_flows.iter().any(|&v| v < 0.0);

    if !has_positive || !has_negative {
        return Err(VBError::new(err_number::INVALID_PROCEDURE_CALL));
    }

    // Newton-Raphson iteration to find IRR
    let mut rate = guess_val;
    const TOLERANCE: f64 = 0.00001;
    const MAX_ITERATIONS: usize = 20;

    for _ in 0..MAX_ITERATIONS {
        let npv = calculate_npv(&cash_flows, rate);
        let derivative = calculate_npv_derivative(&cash_flows, rate);

        if derivative.abs() < f64::EPSILON {
            return Err(VBError::new(err_number::INVALID_PROCEDURE_CALL));
        }

        let new_rate = rate - npv / derivative;

        if (new_rate - rate).abs() < TOLERANCE {
            return Ok(VBVariant::from_double(new_rate));
        }

        rate = new_rate;
    }

    // After 20 iterations, check if we're close enough
    let npv = calculate_npv(&cash_flows, rate);
    if npv.abs() < TOLERANCE {
        return Ok(VBVariant::from_double(rate));
    }

    Err(VBError::new(err_number::INVALID_PROCEDURE_CALL))
}

/// Calculate NPV for a given rate and cash flows.
fn calculate_npv(cash_flows: &[f64], rate: f64) -> f64 {
    let mut npv = 0.0;
    for (i, &cf) in cash_flows.iter().enumerate() {
        npv += cf / (1.0 + rate).powi(i as i32);
    }
    npv
}

/// Calculate the derivative of NPV with respect to rate.
fn calculate_npv_derivative(cash_flows: &[f64], rate: f64) -> f64 {
    let mut derivative = 0.0;
    for (i, &cf) in cash_flows.iter().enumerate() {
        let period = i as i32;
        derivative += cf * (-period as f64) / (1.0 + rate).powi(period + 1);
    }
    derivative
}

#[cfg(test)]
mod tests {
    use super::irr;
    use crate::value::VBVariant;
    use vb6core::error::err_number;

    fn make_array(values: &[f64]) -> VBVariant {
        VBVariant::Array(crate::array::ArrayValue::from_vec_with_bounds(
            crate::types::VBType::Double,
            values.iter().map(|v| VBVariant::from_double(*v)).collect(),
            0,
        ))
    }

    #[test]
    fn irr_simple_investment() {
        // -10000, 3000, 3500, 4000, 4500 -> IRR approx 0.1709 (17.09%)
        let cash_flows = make_array(&[-10000.0, 3000.0, 3500.0, 4000.0, 4500.0]);
        let result = irr(&cash_flows, None).unwrap();
        let irr_val = result.as_f64().unwrap();
        assert!((irr_val - 0.1709).abs() < 0.01);
    }

    #[test]
    fn irr_with_guess() {
        // Same cash flows with explicit guess
        let cash_flows = make_array(&[-10000.0, 3000.0, 3500.0, 4000.0, 4500.0]);
        let result = irr(&cash_flows, Some(&VBVariant::from_double(0.2))).unwrap();
        let irr_val = result.as_f64().unwrap();
        assert!((irr_val - 0.1709).abs() < 0.01);
    }

    #[test]
    fn irr_all_positive_raises_error_5() {
        let cash_flows = make_array(&[3000.0, 3500.0, 4000.0]);
        let err = irr(&cash_flows, None).unwrap_err();
        assert_eq!(err.number, err_number::INVALID_PROCEDURE_CALL);
    }

    #[test]
    fn irr_all_negative_raises_error_5() {
        let cash_flows = make_array(&[-3000.0, -3500.0, -4000.0]);
        let err = irr(&cash_flows, None).unwrap_err();
        assert_eq!(err.number, err_number::INVALID_PROCEDURE_CALL);
    }

    #[test]
    fn irr_two_period_simple() {
        // Simple case: -1000 invested, 1300 returned
        // IRR should be 0.3 (30%)
        let cash_flows = make_array(&[-1000.0, 1300.0]);
        let result = irr(&cash_flows, None).unwrap();
        let irr_val = result.as_f64().unwrap();
        assert!((irr_val - 0.3).abs() < 0.001);
    }

    #[test]
    fn irr_empty_array_raises_error_5() {
        let cash_flows = make_array(&[]);
        let err = irr(&cash_flows, None).unwrap_err();
        assert_eq!(err.number, err_number::INVALID_PROCEDURE_CALL);
    }
}
