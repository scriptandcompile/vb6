//! # `NPer` Function
//!
//! Returns a Double specifying the number of periods for an annuity based on periodic, fixed payments and a fixed interest rate.
//!
//! ## Syntax
//!
//! ```vb
//! NPer(rate, pmt, pv, [fv], [type])
//! ```
//!
//! ## Parameters
//!
//! - **rate** (Required) - Double specifying interest rate per period. For example, if you get a car loan at an annual percentage rate (APR) of 10 percent and make monthly payments, the rate per period is 0.1/12, or 0.0083.
//! - **pmt** (Required) - Double specifying payment to be made each period. Payments usually contain principal and interest that doesn't change over the life of the annuity.
//! - **pv** (Required) - Double specifying present value, or value today, of a series of future payments or receipts. For example, when you borrow money to buy a car, the loan amount is the present value to the lender of the monthly car payments you will make.
//! - **fv** (Optional) - Variant specifying future value or cash balance you want after you've made the final payment. For example, the future value of a loan is $0 because that's its value after the final payment. However, if you want to save $50,000 over 18 years for your child's education, then $50,000 is the future value. If omitted, 0 is assumed.
//! - **type** (Optional) - Variant specifying when payments are due. Use 0 if payments are due at the end of the payment period, or use 1 if payments are due at the beginning of the period. If omitted, 0 is assumed.
//!
//! ## Return Value
//!
//! Returns a **Double** specifying the number of payment periods in an annuity.
//!
//! ## Remarks
//!
//! The `NPer` function calculates how many periods (usually months or years) it will take to pay off a loan or reach a savings goal, given a fixed payment amount and interest rate.
//!
//! ### Key Characteristics:
//! - Returns number of periods (can be fractional)
//! - Assumes constant payment amounts
//! - Assumes constant interest rate
//! - For loans, pv is positive (amount borrowed), pmt is negative (payment out)
//! - For investments, pv is negative (deposit), fv is positive (goal)
//! - Type parameter: 0 = end of period (default), 1 = beginning of period
//! - Rate must be expressed per period (annual rate / periods per year)
//! - Commonly used with PMT, PV, FV, and Rate functions
//!
//! ### Common Use Cases:
//! - Calculate loan payoff time
//! - Determine how long to reach savings goal
//! - Plan retirement savings duration
//! - Calculate time to pay off credit card debt
//! - Determine mortgage term needed
//! - Investment planning timelines
//! - Debt consolidation analysis
//! - Education savings planning
//!
//! ## Typical Uses
//!
//! 1. **Loan Payoff Time** - Calculate how many months to pay off a loan
//! 2. **Savings Goal Timeline** - Determine time to reach a savings target
//! 3. **Mortgage Planning** - Calculate term needed for specific payments
//! 4. **Credit Card Payoff** - Estimate months to pay off credit card balance
//! 5. **Investment Duration** - Calculate time to reach investment goal
//! 6. **Retirement Planning** - Determine years needed to save for retirement
//! 7. **Debt Analysis** - Compare payoff timelines for different payment amounts
//! 8. **What-If Scenarios** - Model different payment scenarios
//!
//! ## Basic Examples
//!
//! ```vb
//! ' Example 1: How many months to pay off a $10,000 loan at 8% APR with $200/month payments?
//! Dim months As Double
//! months = NPer(0.08 / 12, -200, 10000)
//! ' Result: approximately 62.6 months (5.2 years)
//! ```
//!
//! ```vb
//! ' Example 2: How many years to save $50,000 with $300/month at 6% annual return?
//! Dim years As Double
//! years = NPer(0.06 / 12, -300, 0, 50000) / 12
//! ' Result: approximately 10.8 years
//! ```
//!
//! ```vb
//! ' Example 3: Time to pay off credit card with minimum payments
//! Dim balance As Double
//! Dim monthlyPayment As Double
//! Dim apr As Double
//! Dim payoffMonths As Double
//!
//! balance = 5000
//! monthlyPayment = -100
//! apr = 0.1899 ' 18.99% APR
//! payoffMonths = NPer(apr / 12, monthlyPayment, balance)
//! ' Result: approximately 79 months
//! ```
//!
//! ```vb
//! ' Example 4: Retirement savings timeline
//! Dim monthlyDeposit As Double
//! Dim retirementGoal As Double
//! Dim annualReturn As Double
//! Dim yearsNeeded As Double
//!
//! monthlyDeposit = -500
//! retirementGoal = 1000000
//! annualReturn = 0.07
//! yearsNeeded = NPer(annualReturn / 12, monthlyDeposit, 0, retirementGoal) / 12
//! ' Result: approximately 38.3 years
//! ```
//!
//! ## Common Patterns
//!
//! ```vb
//! ' Pattern 1: Loan payoff calculator
//! Function CalculatePayoffMonths(loanAmount As Double, _
//!                                monthlyPayment As Double, _
//!                                apr As Double) As Double
//!     Dim monthlyRate As Double
//!     monthlyRate = apr / 12
//!     CalculatePayoffMonths = NPer(monthlyRate, -monthlyPayment, loanAmount)
//! End Function
//! ```
//!
//! ```vb
//! ' Pattern 2: Savings goal timeline
//! Function YearsToReachGoal(monthlyDeposit As Double, _
//!                          currentBalance As Double, _
//!                          targetAmount As Double, _
//!                          annualReturn As Double) As Double
//!     Dim months As Double
//!     months = NPer(annualReturn / 12, -monthlyDeposit, -currentBalance, targetAmount)
//!     YearsToReachGoal = months / 12
//! End Function
//! ```
//!
//! ```vb
//! ' Pattern 3: Compare payment scenarios
//! Sub ComparePaymentScenarios(balance As Double, apr As Double)
//!     Dim payment As Double
//!     Dim months As Double
//!     Dim i As Integer
//!     
//!     For i = 1 To 5
//!         payment = balance * 0.02 * i ' 2%, 4%, 6%, 8%, 10% of balance
//!         months = NPer(apr / 12, -payment, balance)
//!         Debug.Print "Payment: $" & Format(payment, "0.00") & _
//!                     " - Months: " & Format(months, "0.0")
//!     Next i
//! End Sub
//! ```
//!
//! ```vb
//! ' Pattern 4: Interest rate impact on duration
//! Function CalculateRateImpact(loanAmount As Double, _
//!                             monthlyPayment As Double) As String
//!     Dim result As String
//!     Dim rate As Double
//!     Dim months As Double
//!     
//!     result = "Interest Rate Impact:" & vbCrLf
//!     
//!     For rate = 0.03 To 0.15 Step 0.02
//!         months = NPer(rate / 12, -monthlyPayment, loanAmount)
//!         result = result & Format(rate * 100, "0.0") & "% APR: " & _
//!                  Format(months, "0.0") & " months" & vbCrLf
//!     Next rate
//!     
//!     CalculateRateImpact = result
//! End Function
//! ```
//!
//! ```vb
//! ' Pattern 5: Minimum payment warning
//! Function IsMinimumPaymentTooLow(balance As Double, _
//!                                 payment As Double, _
//!                                 apr As Double) As Boolean
//!     Dim months As Double
//!     Dim maxYears As Double
//!     
//!     maxYears = 10
//!     months = NPer(apr / 12, -payment, balance)
//!     
//!     IsMinimumPaymentTooLow = (months > maxYears * 12)
//! End Function
//! ```
//!
//! ```vb
//! ' Pattern 6: Early payoff calculation
//! Function MonthsSavedByExtraPayment(balance As Double, _
//!                                    regularPayment As Double, _
//!                                    extraPayment As Double, _
//!                                    apr As Double) As Double
//!     Dim regularMonths As Double
//!     Dim acceleratedMonths As Double
//!     
//!     regularMonths = NPer(apr / 12, -regularPayment, balance)
//!     acceleratedMonths = NPer(apr / 12, -(regularPayment + extraPayment), balance)
//!     
//!     MonthsSavedByExtraPayment = regularMonths - acceleratedMonths
//! End Function
//! ```
//!
//! ```vb
//! ' Pattern 7: Formatted duration display
//! Function FormatPayoffTime(months As Double) As String
//!     Dim years As Long
//!     Dim remainingMonths As Long
//!     
//!     years = Int(months / 12)
//!     remainingMonths = Int(months Mod 12)
//!     
//!     If years > 0 Then
//!         FormatPayoffTime = years & " year"
//!         If years > 1 Then FormatPayoffTime = FormatPayoffTime & "s"
//!         If remainingMonths > 0 Then
//!             FormatPayoffTime = FormatPayoffTime & ", " & remainingMonths & " month"
//!             If remainingMonths > 1 Then FormatPayoffTime = FormatPayoffTime & "s"
//!         End If
//!     Else
//!         FormatPayoffTime = remainingMonths & " month"
//!         If remainingMonths <> 1 Then FormatPayoffTime = FormatPayoffTime & "s"
//!     End If
//! End Function
//! ```
//!
//! ```vb
//! ' Pattern 8: College savings timeline
//! Function CalculateCollegeSavingsTime(currentAge As Integer, _
//!                                      currentSavings As Double, _
//!                                      monthlyDeposit As Double, _
//!                                      collegeGoal As Double, _
//!                                      annualReturn As Double) As Double
//!     Dim months As Double
//!     Dim yearsUntilCollege As Integer
//!     
//!     months = NPer(annualReturn / 12, -monthlyDeposit, -currentSavings, collegeGoal)
//!     yearsUntilCollege = 18 - currentAge
//!     
//!     CalculateCollegeSavingsTime = months / 12
//! End Function
//! ```
//!
//! ```vb
//! ' Pattern 9: Debt consolidation comparison
//! Function CompareConsolidation(debt1 As Double, rate1 As Double, pmt1 As Double, _
//!                              debt2 As Double, rate2 As Double, pmt2 As Double, _
//!                              consolidatedRate As Double) As String
//!     Dim currentMonths As Double
//!     Dim consolidatedMonths As Double
//!     Dim totalDebt As Double
//!     Dim totalPayment As Double
//!     Dim result As String
//!     
//!     ' Calculate current payoff time
//!     currentMonths = Application.Max( _
//!         NPer(rate1 / 12, -pmt1, debt1), _
//!         NPer(rate2 / 12, -pmt2, debt2))
//!     
//!     ' Calculate consolidated payoff time
//!     totalDebt = debt1 + debt2
//!     totalPayment = pmt1 + pmt2
//!     consolidatedMonths = NPer(consolidatedRate / 12, -totalPayment, totalDebt)
//!     
//!     result = "Current: " & Format(currentMonths, "0.0") & " months" & vbCrLf
//!     result = result & "Consolidated: " & Format(consolidatedMonths, "0.0") & " months" & vbCrLf
//!     result = result & "Savings: " & Format(currentMonths - consolidatedMonths, "0.0") & " months"
//!     
//!     CompareConsolidation = result
//! End Function
//! ```
//!
//! ```vb
//! ' Pattern 10: Payment sufficiency check
//! Function IsPaymentSufficient(balance As Double, _
//!                             payment As Double, _
//!                             apr As Double) As Boolean
//!     Dim monthlyInterest As Double
//!     
//!     ' Check if payment exceeds monthly interest
//!     monthlyInterest = balance * (apr / 12)
//!     
//!     If payment <= monthlyInterest Then
//!         IsPaymentSufficient = False ' Will never pay off
//!     Else
//!         On Error Resume Next
//!         Dim periods As Double
//!         periods = NPer(apr / 12, -payment, balance)
//!         IsPaymentSufficient = (Err.Number = 0)
//!         On Error GoTo 0
//!     End If
//! End Function
//! ```
//!
//! ## Advanced Usage
//!
//! ### Example 1: Loan Analyzer Class
//!
//! ```vb
//! ' Class: LoanAnalyzer
//! ' Comprehensive loan analysis with payoff scenarios
//!
//! Option Explicit
//!
//! Private m_principal As Double
//! Private m_apr As Double
//! Private m_monthlyPayment As Double
//!
//! Public Sub Initialize(principal As Double, apr As Double, monthlyPayment As Double)
//!     m_principal = principal
//!     m_apr = apr
//!     m_monthlyPayment = monthlyPayment
//! End Sub
//!
//! Public Function GetPayoffMonths() As Double
//!     GetPayoffMonths = NPer(m_apr / 12, -m_monthlyPayment, m_principal)
//! End Function
//!
//! Public Function GetPayoffYears() As Double
//!     GetPayoffYears = GetPayoffMonths() / 12
//! End Function
//!
//! Public Function GetTotalInterest() As Double
//!     Dim months As Double
//!     Dim totalPaid As Double
//!     
//!     months = GetPayoffMonths()
//!     totalPaid = m_monthlyPayment * months
//!     GetTotalInterest = totalPaid - m_principal
//! End Function
//!
//! Public Function GetPayoffDate() As Date
//!     Dim months As Double
//!     months = GetPayoffMonths()
//!     GetPayoffDate = DateAdd("m", Int(months), Date)
//! End Function
//!
//! Public Function AnalyzeExtraPayment(extraMonthly As Double) As String
//!     Dim baseMonths As Double
//!     Dim acceleratedMonths As Double
//!     Dim monthsSaved As Double
//!     Dim interestSaved As Double
//!     Dim result As String
//!     
//!     baseMonths = NPer(m_apr / 12, -m_monthlyPayment, m_principal)
//!     acceleratedMonths = NPer(m_apr / 12, -(m_monthlyPayment + extraMonthly), m_principal)
//!     
//!     monthsSaved = baseMonths - acceleratedMonths
//!     interestSaved = (m_monthlyPayment * baseMonths - m_principal) - _
//!                     ((m_monthlyPayment + extraMonthly) * acceleratedMonths - m_principal)
//!     
//!     result = "Extra Payment Analysis:" & vbCrLf
//!     result = result & "Extra payment: $" & Format(extraMonthly, "#,##0.00") & vbCrLf
//!     result = result & "Time saved: " & Format(monthsSaved, "0.0") & " months" & vbCrLf
//!     result = result & "Interest saved: $" & Format(interestSaved, "#,##0.00")
//!     
//!     AnalyzeExtraPayment = result
//! End Function
//!
//! Public Function GenerateAmortizationSummary() As String
//!     Dim summary As String
//!     Dim months As Double
//!     Dim totalPaid As Double
//!     Dim totalInterest As Double
//!     
//!     months = GetPayoffMonths()
//!     totalPaid = m_monthlyPayment * months
//!     totalInterest = totalPaid - m_principal
//!     
//!     summary = "Loan Amortization Summary" & vbCrLf
//!     summary = summary & String(50, "-") & vbCrLf
//!     summary = summary & "Principal: $" & Format(m_principal, "#,##0.00") & vbCrLf
//!     summary = summary & "APR: " & Format(m_apr * 100, "0.00") & "%" & vbCrLf
//!     summary = summary & "Monthly Payment: $" & Format(m_monthlyPayment, "#,##0.00") & vbCrLf
//!     summary = summary & "Payoff Time: " & Format(months, "0.0") & " months (" & _
//!                        Format(months / 12, "0.0") & " years)" & vbCrLf
//!     summary = summary & "Total Paid: $" & Format(totalPaid, "#,##0.00") & vbCrLf
//!     summary = summary & "Total Interest: $" & Format(totalInterest, "#,##0.00") & vbCrLf
//!     summary = summary & "Payoff Date: " & Format(GetPayoffDate(), "mmm dd, yyyy")
//!     
//!     GenerateAmortizationSummary = summary
//! End Function
//! ```
//!
//! ### Example 2: Retirement Planner Class
//!
//! ```vb
//! ' Class: RetirementPlanner
//! ' Plans retirement savings timeline and scenarios
//!
//! Option Explicit
//!
//! Private m_currentAge As Integer
//! Private m_retirementAge As Integer
//! Private m_currentSavings As Double
//! Private m_monthlyContribution As Double
//! Private m_expectedReturn As Double
//! Private m_retirementGoal As Double
//!
//! Public Sub Initialize(currentAge As Integer, _
//!                      retirementAge As Integer, _
//!                      currentSavings As Double, _
//!                      monthlyContribution As Double, _
//!                      expectedReturn As Double, _
//!                      retirementGoal As Double)
//!     m_currentAge = currentAge
//!     m_retirementAge = retirementAge
//!     m_currentSavings = currentSavings
//!     m_monthlyContribution = monthlyContribution
//!     m_expectedReturn = expectedReturn
//!     m_retirementGoal = retirementGoal
//! End Sub
//!
//! Public Function GetYearsToGoal() As Double
//!     Dim months As Double
//!     months = NPer(m_expectedReturn / 12, -m_monthlyContribution, -m_currentSavings, m_retirementGoal)
//!     GetYearsToGoal = months / 12
//! End Function
//!
//! Public Function WillReachGoal() As Boolean
//!     Dim yearsNeeded As Double
//!     Dim yearsAvailable As Integer
//!     
//!     yearsNeeded = GetYearsToGoal()
//!     yearsAvailable = m_retirementAge - m_currentAge
//!     
//!     WillReachGoal = (yearsNeeded <= yearsAvailable)
//! End Function
//!
//! Public Function GetRequiredMonthlyContribution() As Double
//!     Dim monthsAvailable As Long
//!     monthsAvailable = (m_retirementAge - m_currentAge) * 12
//!     
//!     GetRequiredMonthlyContribution = -PMT(m_expectedReturn / 12, _
//!                                          monthsAvailable, _
//!                                          -m_currentSavings, _
//!                                          m_retirementGoal)
//! End Function
//!
//! Public Function GenerateScenarioAnalysis() As String
//!     Dim result As String
//!     Dim yearsNeeded As Double
//!     Dim yearsAvailable As Integer
//!     Dim shortfall As Double
//!     
//!     yearsNeeded = GetYearsToGoal()
//!     yearsAvailable = m_retirementAge - m_currentAge
//!     
//!     result = "Retirement Scenario Analysis" & vbCrLf
//!     result = result & String(50, "-") & vbCrLf
//!     result = result & "Current Age: " & m_currentAge & vbCrLf
//!     result = result & "Retirement Age: " & m_retirementAge & vbCrLf
//!     result = result & "Years Available: " & yearsAvailable & vbCrLf
//!     result = result & "Current Savings: $" & Format(m_currentSavings, "#,##0.00") & vbCrLf
//!     result = result & "Monthly Contribution: $" & Format(m_monthlyContribution, "#,##0.00") & vbCrLf
//!     result = result & "Expected Return: " & Format(m_expectedReturn * 100, "0.00") & "%" & vbCrLf
//!     result = result & "Retirement Goal: $" & Format(m_retirementGoal, "#,##0.00") & vbCrLf
//!     result = result & vbCrLf
//!     result = result & "Years Needed: " & Format(yearsNeeded, "0.0") & vbCrLf
//!     
//!     If WillReachGoal() Then
//!         result = result & "Status: On track! " & Format(yearsAvailable - yearsNeeded, "0.0") & _
//!                  " years ahead of schedule."
//!     Else
//!         shortfall = yearsNeeded - yearsAvailable
//!         result = result & "Status: Behind schedule by " & Format(shortfall, "0.0") & " years." & vbCrLf
//!         result = result & "Required Monthly: $" & _
//!                  Format(GetRequiredMonthlyContribution(), "#,##0.00")
//!     End If
//!     
//!     GenerateScenarioAnalysis = result
//! End Function
//! ```
//!
//! ### Example 3: Debt Payoff Optimizer Module
//!
//! ```vb
//! ' Module: DebtPayoffOptimizer
//! ' Optimizes debt payoff strategies
//!
//! Option Explicit
//!
//! Private Type DebtInfo
//!     name As String
//!     balance As Double
//!     apr As Double
//!     minimumPayment As Double
//!     payoffMonths As Double
//! End Type
//!
//! Public Function AnalyzeSnowballMethod(debts() As DebtInfo, _
//!                                       extraPayment As Double) As String
//!     Dim i As Integer
//!     Dim result As String
//!     Dim totalMonths As Double
//!     
//!     ' Sort debts by balance (smallest first)
//!     SortDebtsByBalance debts
//!     
//!     result = "Debt Snowball Method Analysis" & vbCrLf
//!     result = result & String(60, "-") & vbCrLf
//!     
//!     totalMonths = 0
//!     
//!     For i = LBound(debts) To UBound(debts)
//!         If i = LBound(debts) Then
//!             debts(i).payoffMonths = NPer(debts(i).apr / 12, _
//!                                          -(debts(i).minimumPayment + extraPayment), _
//!                                          debts(i).balance)
//!         Else
//!             ' Add freed-up payment from previous debt
//!             Dim availablePayment As Double
//!             availablePayment = debts(i).minimumPayment + extraPayment
//!             
//!             For j = LBound(debts) To i - 1
//!                 availablePayment = availablePayment + debts(j).minimumPayment
//!             Next j
//!             
//!             debts(i).payoffMonths = totalMonths + _
//!                 NPer(debts(i).apr / 12, -availablePayment, debts(i).balance)
//!         End If
//!         
//!         totalMonths = debts(i).payoffMonths
//!         
//!         result = result & debts(i).name & ": " & _
//!                  Format(debts(i).payoffMonths, "0.0") & " months" & vbCrLf
//!     Next i
//!     
//!     result = result & vbCrLf & "Total Time: " & Format(totalMonths, "0.0") & " months"
//!     
//!     AnalyzeSnowballMethod = result
//! End Function
//!
//! Public Function AnalyzeAvalancheMethod(debts() As DebtInfo, _
//!                                        extraPayment As Double) As String
//!     ' Sort debts by APR (highest first)
//!     SortDebtsByRate debts
//!     
//!     ' Use same logic as snowball but with different sort
//!     AnalyzeAvalancheMethod = AnalyzeSnowballMethod(debts, extraPayment)
//! End Function
//!
//! Private Sub SortDebtsByBalance(debts() As DebtInfo)
//!     ' Simple bubble sort
//!     Dim i As Integer, j As Integer
//!     Dim temp As DebtInfo
//!     
//!     For i = LBound(debts) To UBound(debts) - 1
//!         For j = i + 1 To UBound(debts)
//!             If debts(i).balance > debts(j).balance Then
//!                 temp = debts(i)
//!                 debts(i) = debts(j)
//!                 debts(j) = temp
//!             End If
//!         Next j
//!     Next i
//! End Sub
//!
//! Private Sub SortDebtsByRate(debts() As DebtInfo)
//!     ' Simple bubble sort
//!     Dim i As Integer, j As Integer
//!     Dim temp As DebtInfo
//!     
//!     For i = LBound(debts) To UBound(debts) - 1
//!         For j = i + 1 To UBound(debts)
//!             If debts(i).apr < debts(j).apr Then
//!                 temp = debts(i)
//!                 debts(i) = debts(j)
//!                 debts(j) = temp
//!             End If
//!         Next j
//!     Next i
//! End Sub
//! ```
//!
//! ### Example 4: Financial Goal Tracker
//!
//! ```vb
//! ' Class: FinancialGoalTracker
//! ' Tracks progress toward multiple financial goals
//!
//! Option Explicit
//!
//! Private Type Goal
//!     name As String
//!     targetAmount As Double
//!     currentAmount As Double
//!     monthlyDeposit As Double
//!     expectedReturn As Double
//!     targetDate As Date
//!     isOnTrack As Boolean
//!     monthsNeeded As Double
//! End Type
//!
//! Private m_goals As Collection
//!
//! Private Sub Class_Initialize()
//!     Set m_goals = New Collection
//! End Sub
//!
//! Public Sub AddGoal(name As String, _
//!                   targetAmount As Double, _
//!                   currentAmount As Double, _
//!                   monthlyDeposit As Double, _
//!                   expectedReturn As Double, _
//!                   targetDate As Date)
//!     Dim goal As Goal
//!     
//!     goal.name = name
//!     goal.targetAmount = targetAmount
//!     goal.currentAmount = currentAmount
//!     goal.monthlyDeposit = monthlyDeposit
//!     goal.expectedReturn = expectedReturn
//!     goal.targetDate = targetDate
//!     
//!     ' Calculate if on track
//!     goal.monthsNeeded = NPer(expectedReturn / 12, -monthlyDeposit, -currentAmount, targetAmount)
//!     
//!     Dim monthsAvailable As Long
//!     monthsAvailable = DateDiff("m", Date, targetDate)
//!     goal.isOnTrack = (goal.monthsNeeded <= monthsAvailable)
//!     
//!     m_goals.Add goal, name
//! End Sub
//!
//! Public Function GetGoalStatus(goalName As String) As String
//!     Dim goal As Goal
//!     Dim monthsAvailable As Long
//!     Dim result As String
//!     
//!     On Error Resume Next
//!     goal = m_goals(goalName)
//!     
//!     If Err.Number <> 0 Then
//!         GetGoalStatus = "Goal not found"
//!         Exit Function
//!     End If
//!     On Error GoTo 0
//!     
//!     monthsAvailable = DateDiff("m", Date, goal.targetDate)
//!     
//!     result = "Goal: " & goal.name & vbCrLf
//!     result = result & "Target: $" & Format(goal.targetAmount, "#,##0.00") & vbCrLf
//!     result = result & "Current: $" & Format(goal.currentAmount, "#,##0.00") & vbCrLf
//!     result = result & "Months Available: " & monthsAvailable & vbCrLf
//!     result = result & "Months Needed: " & Format(goal.monthsNeeded, "0.0") & vbCrLf
//!     
//!     If goal.isOnTrack Then
//!         result = result & "Status: On track!"
//!     Else
//!         Dim requiredMonthly As Double
//!         requiredMonthly = -PMT(goal.expectedReturn / 12, monthsAvailable, _
//!                               -goal.currentAmount, goal.targetAmount)
//!         result = result & "Status: Behind - Need $" & _
//!                  Format(requiredMonthly, "#,##0.00") & "/month"
//!     End If
//!     
//!     GetGoalStatus = result
//! End Function
//!
//! Public Function GenerateAllGoalsReport() As String
//!     Dim report As String
//!     Dim goal As Goal
//!     Dim i As Long
//!     
//!     report = "Financial Goals Progress Report" & vbCrLf
//!     report = report & String(60, "=") & vbCrLf & vbCrLf
//!     
//!     For i = 1 To m_goals.Count
//!         goal = m_goals(i)
//!         report = report & GetGoalStatus(goal.name) & vbCrLf & vbCrLf
//!     Next i
//!     
//!     GenerateAllGoalsReport = report
//! End Function
//! ```
//!
//! ## Error Handling
//!
//! ```vb
//! On Error Resume Next
//! Dim periods As Double
//! periods = NPer(rate, pmt, pv, fv, type)
//! If Err.Number <> 0 Then
//!     MsgBox "Error calculating periods: " & Err.Description & vbCrLf & _
//!            "This may occur if payment is too small to ever pay off the balance."
//!     ' Payment must exceed interest to avoid infinite periods
//! End If
//! On Error GoTo 0
//! ```
//!
//! ## Performance Considerations
//!
//! - `NPer` uses iterative calculation - reasonably fast
//! - More complex than simple arithmetic operations
//! - For many calculations, consider caching results
//! - Typically completes in microseconds
//! - Performance similar to other financial functions (PMT, PV, FV)
//!
//! ## Best Practices
//!
//! 1. **Use consistent periods** - Match rate and payment periods (monthly, yearly, etc.)
//! 2. **Sign conventions** - Money out is negative, money in is positive
//! 3. **Validate inputs** - Check that payment exceeds interest charge
//! 4. **Handle errors** - Wrap in error handling for invalid scenarios
//! 5. **Round results** - Round to appropriate precision for display
//! 6. **Document assumptions** - State whether payments are start/end of period
//! 7. **Validate reasonableness** - Check that results make sense
//! 8. **Use with other functions** - Combine with PMT, PV, FV for analysis
//! 9. **Consider type parameter** - Specify payment timing when relevant
//! 10. **Format for users** - Convert to years/months for clarity
//!
//! ## Comparison with Related Functions
//!
//! | Function | Calculates | Given |
//! |----------|-----------|-------|
//! | **`NPer`** | Number of periods | Rate, payment, present value, future value |
//! | **PMT** | Payment amount | Rate, periods, present value, future value |
//! | **PV** | Present value | Rate, periods, payment, future value |
//! | **FV** | Future value | Rate, periods, payment, present value |
//! | **Rate** | Interest rate | Periods, payment, present value, future value |
//!
//! ## Platform Notes
//!
//! - Available in VBA (Excel, Access, Word, etc.)
//! - Available in VB6
//! - **Not available in `VBScript`**
//! - Uses iterative algorithm to solve
//! - Consistent with Excel's NPER function
//! - Part of VBA financial functions library
//!
//! ## Limitations
//!
//! - Assumes constant payment amounts
//! - Assumes constant interest rate
//! - Payment must exceed periodic interest or will fail/return error
//! - Does not account for fees, taxes, or insurance
//! - Cannot handle irregular payment schedules
//! - Type parameter limited to 0 or 1
//! - May return fractional periods
//!
//! ## Related Functions
//!
//! - **PMT** - Calculates payment amount for a loan
//! - **PV** - Calculates present value of an investment
//! - **FV** - Calculates future value of an investment
//! - **Rate** - Calculates interest rate per period
//! - **`IPmt`** - Calculates interest payment for a specific period
//! - **`PPmt`** - Calculates principal payment for a specific period

use crate::error::{VBError, VBResult};
use crate::value::VBVariant;
use vb6core::error::err_number;

/// Implementation of the `NPer` function.
///
/// Calculates the number of periods required for payments to reach a future value
/// given a present value, payment amount, and interest rate. Uses iteration to solve
/// for the unknown period count.
///
/// VB6 behavior:
/// - `rate` is the interest rate per period
/// - `pmt` is the payment made each period
/// - `pv` is the present value (lump sum) of the series of payments
/// - `fv` defaults to 0 if omitted
/// - `type` defaults to 0 (end of period) if omitted; use 1 for beginning of period
/// - Returns a `Double` representing the number of periods
/// - Raises error 5 if:
///   - rate is negative
///   - pmt equals zero and pv equals zero (no payment or investment)
///   - nper would be infinite (payment too small to ever reach fv)
pub fn nper(
    rate: &VBVariant,
    pmt: &VBVariant,
    pv: &VBVariant,
    fv: Option<&VBVariant>,
    type_: Option<&VBVariant>,
) -> VBResult<VBVariant> {
    let rate = rate.as_f64()?;
    let pmt = pmt.as_f64()?;
    let pv = pv.as_f64()?;
    let fv = fv.map(|f| f.as_f64()).transpose()?.unwrap_or(0.0);
    let type_ = type_.map(|t| t.as_i16()).transpose()?.unwrap_or(0);

    // Validate inputs per VB6 behavior
    if rate < 0.0 {
        return Err(VBError::new(err_number::INVALID_PROCEDURE_CALL));
    }

    // Special case: zero interest rate
    // When rate is 0, nper = -(pv + fv) / pmt
    // But this requires pmt != 0
    if rate == 0.0 {
        if pmt == 0.0 {
            return Err(VBError::new(err_number::INVALID_PROCEDURE_CALL));
        }
        let nper = -(pv + fv) / pmt;
        if nper.is_nan() || nper.is_infinite() {
            return Err(VBError::new(err_number::INVALID_PROCEDURE_CALL));
        }
        return Ok(VBVariant::from_double(nper));
    }

    // For non-zero rate, solve using logarithms.
    // Derivation from the FV = 0 formula:
    //   0 = -pv * base^n - pmt * (base^n - 1)/rate
    // => pv * base^n + pmt * (base^n - 1)/rate = 0
    // => pv * rate * base^n + pmt * base^n - pmt = 0
    // => (pv * rate + pmt) * base^n = pmt
    // => base^n = pmt / (pv * rate + pmt)
    // => n = Log(pmt / (pv * rate + pmt)) / Log(1+rate)
    // But we also need to incorporate fv, so the general formula is:
    //   0 = -pv * base^n - pmt * (base^n - 1)/rate - fv
    // => (pv * rate + pmt) * base^n = pmt - fv * rate
    // => n = Log((pmt - fv*rate) / (pv*rate + pmt)) / Log(1+rate)

    let base = 1.0_f64 + rate;
    let log_base = base.ln();

    if log_base.abs() < f64::EPSILON {
        return Err(VBError::new(err_number::INVALID_PROCEDURE_CALL));
    }

    let nper_val = if type_ == 0 {
        // Payments at end of period (ordinary annuity)
        // Derived from FV formula where fv is the target value
        // n = Log((pmt - fv * rate) / (pv * rate + pmt)) / Log(1+rate)

        let numerator = pmt - fv * rate;
        let denominator = pv * rate + pmt;

        if denominator == 0.0 {
            return Err(VBError::new(err_number::INVALID_PROCEDURE_CALL));
        }

        let ratio = numerator / denominator;

        // The log of a negative number is undefined, so check for valid ratio
        if ratio <= 0.0 {
            // This case shouldn't normally happen in valid scenarios,
            // but handle it gracefully
            return Err(VBError::new(err_number::INVALID_PROCEDURE_CALL));
        }

        let nper = ratio.ln() / log_base;

        if nper.is_nan() || nper.is_infinite() {
            return Err(VBError::new(err_number::INVALID_PROCEDURE_CALL));
        }
        nper
    } else {
        // Payments at beginning of period (annuity due)
        // Adjust the formula: pmt becomes pmt * (1+rate) for annuity due
        // n = Log((pmt - fv*rate*(1+rate)) / ((pv*rate + pmt)*(1+rate))) / Log(1+rate)

        let numerator = pmt - fv * rate * base;
        let denominator = (pv * rate + pmt) * base;

        if denominator == 0.0 {
            return Err(VBError::new(err_number::INVALID_PROCEDURE_CALL));
        }

        let ratio = numerator / denominator;

        if ratio <= 0.0 {
            return Err(VBError::new(err_number::INVALID_PROCEDURE_CALL));
        }

        let nper = ratio.ln() / log_base;

        if nper.is_nan() || nper.is_infinite() {
            return Err(VBError::new(err_number::INVALID_PROCEDURE_CALL));
        }
        nper
    };

    Ok(VBVariant::from_double(nper_val))
}

#[cfg(test)]
mod tests {
    use super::nper;
    use crate::value::VBVariant;

    fn nper_defaults(rate: f64, pmt: f64, pv: f64) -> VBVariant {
        nper(
            &VBVariant::from_double(rate),
            &VBVariant::from_double(pmt),
            &VBVariant::from_double(pv),
            None,
            None,
        )
        .unwrap()
    }

    fn nper_with_fv(rate: f64, pmt: f64, pv: f64, fv: f64) -> VBVariant {
        nper(
            &VBVariant::from_double(rate),
            &VBVariant::from_double(pmt),
            &VBVariant::from_double(pv),
            Some(&VBVariant::from_double(fv)),
            None,
        )
        .unwrap()
    }

    fn nper_with_type(rate: f64, pmt: f64, pv: f64, fv: f64, ptype: i16) -> VBVariant {
        nper(
            &VBVariant::from_double(rate),
            &VBVariant::from_double(pmt),
            &VBVariant::from_double(pv),
            Some(&VBVariant::from_double(fv)),
            Some(&VBVariant::from_integer(ptype)),
        )
        .unwrap()
    }

    #[test]
    fn nper_loan_payoff_simple() {
        // $10,000 loan at 8% APR with $200/month payments
        // Monthly rate = 0.08/12, pmt = -200, pv = 10000
        // Expected: approximately 61.02 months (derived from FV formula)
        let result = nper_defaults(0.08 / 12.0, -200.0, 10000.0);
        let nper_val = result.as_f64().unwrap();
        assert!((nper_val - 61.0).abs() < 1.0);
    }

    #[test]
    fn nper_zero_rate_simple() {
        // With zero rate, nper = -(pv + fv) / pmt
        // pv = 1000, pmt = -100, fv = 0
        // nper = -(1000 + 0) / (-100) = 10
        let result = nper_defaults(0.0, -100.0, 1000.0);
        let nper_val = result.as_f64().unwrap();
        assert!((nper_val - 10.0).abs() < 0.01);
    }

    #[test]
    fn nper_savings_goal() {
        // How many periods to save $50,000 with $300/month at 6% annual?
        // rate = 0.06/12, pmt = -300, pv = 0, fv = 50000
        let result = nper_with_fv(0.06 / 12.0, -300.0, 0.0, 50000.0);
        let nper_val = result.as_f64().unwrap();
        // Should be approximately 121.5 periods (about 10.1 years)
        assert!((nper_val - 121.5).abs() < 2.0);
    }

    #[test]
    fn nper_negative_rate_raises_error_5() {
        let err = nper(
            &VBVariant::from_double(-0.05),
            &VBVariant::from_double(-100.0),
            &VBVariant::from_double(1000.0),
            None,
            None,
        )
        .unwrap_err();
        assert_eq!(err.number, 5);
    }

    #[test]
    fn nper_zero_pmt_raises_error_5() {
        let err = nper(
            &VBVariant::from_double(0.0),
            &VBVariant::from_double(0.0),
            &VBVariant::from_double(1000.0),
            None,
            None,
        )
        .unwrap_err();
        assert_eq!(err.number, 5);
    }

    #[test]
    fn nper_with_beginning_of_period() {
        // Compare end-of-period vs beginning-of-period
        let end = nper_defaults(0.06 / 12.0, -200.0, 10000.0);
        let begin = nper_with_type(0.06 / 12.0, -200.0, 10000.0, 0.0, 1);

        let end_val = end.as_f64().unwrap();
        let begin_val = begin.as_f64().unwrap();

        // Both should be positive and different
        assert!(end_val > 0.0);
        assert!(begin_val > 0.0);
        assert_ne!(end_val, begin_val);
    }

    #[test]
    fn nper_with_integer_args() {
        let result = nper(
            &VBVariant::from_double(0.05),
            &VBVariant::from_long(-100),
            &VBVariant::from_long(1000),
            None,
            None,
        )
        .unwrap();
        assert!(result.as_f64().unwrap() > 0.0);
    }

    #[test]
    fn nper_null_raises_invalid_use_of_null() {
        let err = nper(
            &VBVariant::Null,
            &VBVariant::from_double(-100.0),
            &VBVariant::from_double(1000.0),
            None,
            None,
        )
        .unwrap_err();
        assert_eq!(err.number, 94);
    }

    #[test]
    fn nper_round_trip_with_fv() {
        // If we calculate FV for n periods, then NPer should return approximately n
        // This tests the inverse relationship between FV and NPer
        let rate = 0.06 / 12.0;
        let pmt = -100.0;
        let pv = 0.0;

        // First calculate FV for 12 periods using the FV function formula directly
        // FV = -pv * base^n - pmt * (base^n - 1)/rate
        let factor = (1.0_f64 + rate).powf(12.0);
        let calculated_fv = -pv * factor - pmt * (factor - 1.0) / rate;

        // Now calculate NPer with that FV - should return ~12
        // Note: fv parameter should be the positive value we want to reach
        let nper_result = nper_with_fv(rate, pmt, pv, calculated_fv);
        let nper_val = nper_result.as_f64().unwrap();

        // Should be close to 12 periods
        assert!((nper_val - 12.0).abs() < 0.01);
    }
}
