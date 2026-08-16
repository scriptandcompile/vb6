//! # PV Function
//!
//! Returns a Double specifying the present value of an annuity based on periodic, fixed payments to be paid in the future and a fixed interest rate.
//!
//! ## Syntax
//!
//! ```vb
//! PV(rate, nper, pmt, [fv], [type])
//! ```
//!
//! ## Parameters
//!
//! - `rate` - Required. Double specifying interest rate per period. For example, if you get a car loan at an annual percentage rate (APR) of 10% and make monthly payments, the rate per period is 0.1/12, or 0.0083.
//! - `nper` - Required. Integer specifying total number of payment periods in the annuity. For example, if you make monthly payments on a 4-year car loan, your loan has 4 * 12 (or 48) payment periods.
//! - `pmt` - Required. Double specifying payment to be made each period. Payments usually contain principal and interest that does not change over the life of the annuity.
//! - `fv` - Optional. Variant specifying future value or cash balance you want after you've made the final payment. For example, the future value of a loan is $0 because that's its value after the final payment. However, if you want to save $50,000 over 18 years for your child's education, then $50,000 is the future value. If omitted, 0 is assumed.
//! - `type` - Optional. Variant specifying when payments are due. Use 0 if payments are due at the end of the payment period, or use 1 if payments are due at the beginning of the period. If omitted, 0 is assumed.
//!
//! ## Return Value
//!
//! Returns a `Double` specifying the present value of an annuity. The present value is the current value of a series of future payments or the current value of a future lump sum.
//!
//! ## Remarks
//!
//! The `PV` function is the inverse of the `FV` function. While `FV` calculates what a series of payments will be worth in the future,
//! `PV` calculates what those same future payments are worth today, discounted by a rate of return.
//!
//! An annuity is a series of fixed cash payments made over a period of time. An annuity can be a loan (such as a home mortgage)
//! or an investment (such as a monthly savings plan).
//!
//! The `rate` and `nper` arguments must be calculated using payment periods expressed in the same units. For example, if `rate`
//! is calculated using months, `nper` must also be calculated using months.
//!
//! For all arguments, cash paid out (such as deposits to savings) is represented by negative numbers; cash received
//! (such as dividend checks) is represented by positive numbers.
//!
//! **Important Uses**:
//! - **Loan Affordability**: Calculate how much you can borrow given a specific payment amount
//! - **Investment Valuation**: Determine current value of future cash flows
//! - **Annuity Pricing**: Calculate lump sum value of periodic payments
//! - **Lease Analysis**: Determine present value of lease payments
//!
//! ## Typical Uses
//!
//! 1. **Loan Affordability**: Calculate maximum loan amount based on affordable payment
//! 2. **Investment Valuation**: Determine present value of future investment returns
//! 3. **Annuity Valuation**: Calculate lump sum value of annuity payments
//! 4. **Bond Pricing**: Value bonds based on coupon payments and face value
//! 5. **Lease vs Buy Analysis**: Compare present value of lease payments to purchase price
//! 6. **Pension Valuation**: Calculate current value of future pension payments
//! 7. **Structured Settlement**: Determine lump sum value of periodic payments
//! 8. **Capital Budgeting**: Evaluate present value of project cash flows
//!
//! ## Basic Examples
//!
//! ### Example 1: Loan Affordability
//! ```vb
//! ' How much can you borrow if you can afford $500/month for 5 years at 6% APR?
//! Dim loanAmount As Double
//! loanAmount = Abs(PV(0.06 / 12, 5 * 12, -500))
//! ' Returns approximately $25,775 (negative payment = money you pay out)
//! ```
//!
//! ### Example 2: Investment Present Value
//! ```vb
//! ' What's the present value of receiving $1,000/month for 10 years at 5% return?
//! Dim presentValue As Double
//! presentValue = Abs(PV(0.05 / 12, 10 * 12, 1000))
//! ' Returns approximately $94,289 (positive payment = money you receive)
//! ```
//!
//! ### Example 3: Annuity Valuation
//! ```vb
//! ' Value of annuity paying $2,000/month for 20 years at 4% discount rate
//! Dim annuityValue As Double
//! annuityValue = Abs(PV(0.04 / 12, 20 * 12, 2000))
//! ' Returns the lump sum equivalent value
//! ```
//!
//! ### Example 4: Lump Sum with Future Value
//! ```vb
//! ' Present value of $50,000 in 10 years at 6% annual return (no periodic payments)
//! Dim presentValue As Double
//! presentValue = Abs(PV(0.06, 10, 0, -50000))
//! ' Returns approximately $27,920 (what you'd need to invest today)
//! ```
//!
//! ## Common Patterns
//!
//! ### Pattern 1: `CalculateAffordableLoan`
//! ```vb
//! Function CalculateAffordableLoan(monthlyPayment As Double, _
//!                                  annualRate As Double, _
//!                                  years As Integer) As Double
//!     Dim monthlyRate As Double
//!     Dim numPayments As Integer
//!     
//!     monthlyRate = annualRate / 12
//!     numPayments = years * 12
//!     
//!     ' Negative payment because it's money flowing out
//!     CalculateAffordableLoan = Abs(PV(monthlyRate, numPayments, -monthlyPayment))
//! End Function
//! ```
//!
//! ### Pattern 2: `ComparePaymentOptions`
//! ```vb
//! Function ComparePaymentOptions(payment1 As Double, years1 As Integer, _
//!                                payment2 As Double, years2 As Integer, _
//!                                rate As Double) As String
//!     Dim pv1 As Double
//!     Dim pv2 As Double
//!     
//!     pv1 = Abs(PV(rate / 12, years1 * 12, -payment1))
//!     pv2 = Abs(PV(rate / 12, years2 * 12, -payment2))
//!     
//!     If pv1 > pv2 Then
//!         ComparePaymentOptions = "Option 1 allows borrowing $" & _
//!                                Format(pv1 - pv2, "#,##0") & " more"
//!     Else
//!         ComparePaymentOptions = "Option 2 allows borrowing $" & _
//!                                Format(pv2 - pv1, "#,##0") & " more"
//!     End If
//! End Function
//! ```
//!
//! ### Pattern 3: `CalculateLumpSumEquivalent`
//! ```vb
//! Function CalculateLumpSumEquivalent(monthlyPayment As Double, _
//!                                     years As Integer, _
//!                                     discountRate As Double) As Double
//!     ' Calculate what a stream of payments is worth as a lump sum today
//!     Dim monthlyRate As Double
//!     
//!     monthlyRate = discountRate / 12
//!     CalculateLumpSumEquivalent = Abs(PV(monthlyRate, years * 12, monthlyPayment))
//! End Function
//! ```
//!
//! ### Pattern 4: `ValidatePVParameters`
//! ```vb
//! Function ValidatePVParameters(rate As Double, nper As Integer, _
//!                               pmt As Double) As Boolean
//!     ValidatePVParameters = False
//!     
//!     If nper <= 0 Then
//!         MsgBox "Number of periods must be positive"
//!         Exit Function
//!     End If
//!     
//!     If rate < -1 Then
//!         MsgBox "Interest rate cannot be less than -100%"
//!         Exit Function
//!     End If
//!     
//!     ValidatePVParameters = True
//! End Function
//! ```
//!
//! ### Pattern 5: `CalculateBreakEvenLoanAmount`
//! ```vb
//! Function CalculateBreakEvenLoanAmount(payment As Double, _
//!                                       rate As Double, _
//!                                       years As Integer, _
//!                                       upfrontCosts As Double) As Double
//!     ' Calculate loan amount where total cost equals upfront costs
//!     Dim loanPV As Double
//!     
//!     loanPV = Abs(PV(rate / 12, years * 12, -payment))
//!     CalculateBreakEvenLoanAmount = loanPV - upfrontCosts
//! End Function
//! ```
//!
//! ### Pattern 6: `PVOfMixedCashFlows`
//! ```vb
//! Function PVOfMixedCashFlows(regularPayment As Double, _
//!                             rate As Double, _
//!                             nper As Integer, _
//!                             futureValue As Double) As Double
//!     ' Calculate PV when you have both regular payments and a lump sum
//!     PVOfMixedCashFlows = Abs(PV(rate, nper, regularPayment, futureValue))
//! End Function
//! ```
//!
//! ### Pattern 7: `CalculateRequiredDownPayment`
//! ```vb
//! Function CalculateRequiredDownPayment(homePrice As Double, _
//!                                       monthlyPayment As Double, _
//!                                       rate As Double, _
//!                                       years As Integer) As Double
//!     Dim maxLoan As Double
//!     
//!     maxLoan = Abs(PV(rate / 12, years * 12, -monthlyPayment))
//!     
//!     If maxLoan >= homePrice Then
//!         CalculateRequiredDownPayment = 0
//!     Else
//!         CalculateRequiredDownPayment = homePrice - maxLoan
//!     End If
//! End Function
//! ```
//!
//! ### Pattern 8: `CompareLumpSumVsAnnuity`
//! ```vb
//! Function CompareLumpSumVsAnnuity(lumpSum As Double, _
//!                                  annuityPayment As Double, _
//!                                  years As Integer, _
//!                                  discountRate As Double) As String
//!     Dim annuityPV As Double
//!     Dim difference As Double
//!     
//!     annuityPV = Abs(PV(discountRate / 12, years * 12, annuityPayment))
//!     difference = lumpSum - annuityPV
//!     
//!     If difference > 0 Then
//!         CompareLumpSumVsAnnuity = "Lump sum is better by $" & Format(difference, "#,##0")
//!     ElseIf difference < 0 Then
//!         CompareLumpSumVsAnnuity = "Annuity is better by $" & Format(Abs(difference), "#,##0")
//!     Else
//!         CompareLumpSumVsAnnuity = "Both options are equal"
//!     End If
//! End Function
//! ```
//!
//! ### Pattern 9: `CalculateLeaseValue`
//! ```vb
//! Function CalculateLeaseValue(monthlyLease As Double, _
//!                              leaseTermMonths As Integer, _
//!                              discountRate As Double) As Double
//!     ' Calculate present value of all lease payments
//!     CalculateLeaseValue = Abs(PV(discountRate / 12, leaseTermMonths, -monthlyLease))
//! End Function
//! ```
//!
//! ### Pattern 10: `FindAffordablePayment`
//! ```vb
//! Function FindAffordablePayment(desiredLoan As Double, _
//!                                rate As Double, _
//!                                nper As Integer) As Double
//!     ' Reverse calculation: find payment from desired loan amount
//!     ' This uses Pmt, but demonstrates PV relationship
//!     FindAffordablePayment = Abs(Pmt(rate, nper, desiredLoan))
//! End Function
//! ```
//!
//! ## Advanced Usage
//!
//! ### Example 1: Comprehensive Loan Calculator
//! ```vb
//! ' Calculate loan amounts based on payment affordability
//! Class LoanAffordabilityCalculator
//!     Private m_monthlyIncome As Double
//!     Private m_monthlyDebts As Double
//!     Private m_annualRate As Double
//!     Private m_loanYears As Integer
//!     Private m_debtToIncomeRatio As Double
//!     
//!     Public Sub Initialize(monthlyIncome As Double, monthlyDebts As Double, _
//!                          annualRate As Double, loanYears As Integer)
//!         m_monthlyIncome = monthlyIncome
//!         m_monthlyDebts = monthlyDebts
//!         m_annualRate = annualRate
//!         m_loanYears = loanYears
//!         m_debtToIncomeRatio = 0.43  ' Standard 43% DTI ratio
//!     End Sub
//!     
//!     Public Function GetMaxMonthlyPayment() As Double
//!         Dim maxTotalDebt As Double
//!         Dim maxPayment As Double
//!         
//!         maxTotalDebt = m_monthlyIncome * m_debtToIncomeRatio
//!         maxPayment = maxTotalDebt - m_monthlyDebts
//!         
//!         If maxPayment < 0 Then maxPayment = 0
//!         GetMaxMonthlyPayment = maxPayment
//!     End Function
//!     
//!     Public Function GetMaxLoanAmount() As Double
//!         Dim maxPayment As Double
//!         Dim monthlyRate As Double
//!         Dim numPayments As Integer
//!         
//!         maxPayment = GetMaxMonthlyPayment()
//!         monthlyRate = m_annualRate / 12
//!         numPayments = m_loanYears * 12
//!         
//!         ' Use PV to find how much can be borrowed
//!         GetMaxLoanAmount = Abs(PV(monthlyRate, numPayments, -maxPayment))
//!     End Function
//!     
//!     Public Function GetLoanWithDownPayment(downPayment As Double) As Double
//!         GetLoanWithDownPayment = GetMaxLoanAmount() + downPayment
//!     End Function
//!     
//!     Public Function GetRequiredDownPayment(homePrice As Double) As Double
//!         Dim maxLoan As Double
//!         
//!         maxLoan = GetMaxLoanAmount()
//!         
//!         If maxLoan >= homePrice Then
//!             GetRequiredDownPayment = 0
//!         Else
//!             GetRequiredDownPayment = homePrice - maxLoan
//!         End If
//!     End Function
//!     
//!     Public Function GenerateAffordabilityReport() As String
//!         Dim report As String
//!         Dim maxPayment As Double
//!         Dim maxLoan As Double
//!         
//!         maxPayment = GetMaxMonthlyPayment()
//!         maxLoan = GetMaxLoanAmount()
//!         
//!         report = "Loan Affordability Analysis" & vbCrLf
//!         report = report & String(50, "=") & vbCrLf
//!         report = report & "Monthly Income: $" & Format(m_monthlyIncome, "#,##0") & vbCrLf
//!         report = report & "Existing Debts: $" & Format(m_monthlyDebts, "#,##0") & vbCrLf
//!         report = report & "DTI Ratio: " & Format(m_debtToIncomeRatio * 100, "0") & "%" & vbCrLf
//!         report = report & String(50, "-") & vbCrLf
//!         report = report & "Max Monthly Payment: $" & Format(maxPayment, "#,##0") & vbCrLf
//!         report = report & "Interest Rate: " & Format(m_annualRate * 100, "0.00") & "%" & vbCrLf
//!         report = report & "Loan Term: " & m_loanYears & " years" & vbCrLf
//!         report = report & String(50, "-") & vbCrLf
//!         report = report & "Maximum Loan Amount: $" & Format(maxLoan, "#,##0")
//!         
//!         GenerateAffordabilityReport = report
//!     End Function
//! End Class
//! ```
//!
//! ### Example 2: Annuity Comparison Tool
//! ```vb
//! ' Compare different annuity and lump sum options
//! Module AnnuityComparison
//!     Private Type AnnuityOption
//!         Name As String
//!         Payment As Double
//!         Years As Integer
//!         IsLumpSum As Boolean
//!         LumpSumAmount As Double
//!     End Type
//!     
//!     Public Function CompareOptions(options() As AnnuityOption, _
//!                                   discountRate As Double) As String
//!         Dim report As String
//!         Dim i As Integer
//!         Dim pv As Double
//!         Dim monthlyRate As Double
//!         Dim bestValue As Double
//!         Dim bestIndex As Integer
//!         
//!         monthlyRate = discountRate / 12
//!         bestValue = 0
//!         bestIndex = LBound(options)
//!         
//!         report = "Annuity Option Comparison" & vbCrLf
//!         report = report & "Discount Rate: " & Format(discountRate * 100, "0.0") & "%" & vbCrLf
//!         report = report & String(60, "=") & vbCrLf
//!         report = report & "Option              Type        Present Value" & vbCrLf
//!         report = report & String(60, "-") & vbCrLf
//!         
//!         For i = LBound(options) To UBound(options)
//!             If options(i).IsLumpSum Then
//!                 pv = options(i).LumpSumAmount
//!             Else
//!                 pv = Abs(PV(monthlyRate, options(i).Years * 12, options(i).Payment))
//!             End If
//!             
//!             If pv > bestValue Then
//!                 bestValue = pv
//!                 bestIndex = i
//!             End If
//!             
//!             report = report & Left(options(i).Name & Space(20), 20) & _
//!                      IIf(options(i).IsLumpSum, "Lump Sum    ", "Annuity     ") & _
//!                      "$" & Format(pv, "#,##0")
//!             
//!             If i = bestIndex Then report = report & " *BEST*"
//!             report = report & vbCrLf
//!         Next i
//!         
//!         report = report & String(60, "-") & vbCrLf
//!         report = report & "Recommended: " & options(bestIndex).Name
//!         
//!         CompareOptions = report
//!     End Function
//!     
//!     Public Function CalculateAnnuityYield(lumpSum As Double, _
//!                                          monthlyPayment As Double, _
//!                                          years As Integer) As Double
//!         ' Find the discount rate that makes PV equal to lump sum
//!         ' This is a simplified approximation
//!         Dim rate As Double
//!         Dim pv As Double
//!         Dim diff As Double
//!         
//!         rate = 0.05  ' Starting guess
//!         Do
//!             pv = Abs(PV(rate / 12, years * 12, monthlyPayment))
//!             diff = pv - lumpSum
//!             
//!             If Abs(diff) < 0.01 Then Exit Do
//!             
//!             ' Adjust rate
//!             If diff > 0 Then
//!                 rate = rate + 0.0001
//!             Else
//!                 rate = rate - 0.0001
//!             End If
//!         Loop While Abs(diff) > 0.01
//!         
//!         CalculateAnnuityYield = rate
//!     End Function
//! End Module
//! ```
//!
//! ### Example 3: Lease vs Buy Analyzer
//! ```vb
//! ' Compare leasing vs buying with present value analysis
//! Class LeaseVsBuyAnalyzer
//!     Private m_purchasePrice As Double
//!     Private m_monthlyLease As Double
//!     Private m_leaseTermMonths As Integer
//!     Private m_discountRate As Double
//!     Private m_residualValue As Double
//!     
//!     Public Sub Initialize(purchasePrice As Double, monthlyLease As Double, _
//!                          leaseTermMonths As Integer, discountRate As Double, _
//!                          residualValue As Double)
//!         m_purchasePrice = purchasePrice
//!         m_monthlyLease = monthlyLease
//!         m_leaseTermMonths = leaseTermMonths
//!         m_discountRate = discountRate
//!         m_residualValue = residualValue
//!     End Sub
//!     
//!     Public Function GetLeasePresentValue() As Double
//!         ' Calculate PV of all lease payments
//!         GetLeasePresentValue = Abs(PV(m_discountRate / 12, m_leaseTermMonths, -m_monthlyLease))
//!     End Function
//!     
//!     Public Function GetBuyPresentValue() As Double
//!         ' Calculate PV of buying (purchase price minus PV of residual value)
//!         Dim pvResidual As Double
//!         
//!         ' PV of residual value (what it's worth after lease term)
//!         pvResidual = Abs(PV(m_discountRate / 12, m_leaseTermMonths, 0, -m_residualValue))
//!         
//!         GetBuyPresentValue = m_purchasePrice - pvResidual
//!     End Function
//!     
//!     Public Function GetRecommendation() As String
//!         Dim leasePV As Double
//!         Dim buyPV As Double
//!         Dim difference As Double
//!         
//!         leasePV = GetLeasePresentValue()
//!         buyPV = GetBuyPresentValue()
//!         difference = Abs(buyPV - leasePV)
//!         
//!         If leasePV < buyPV Then
//!             GetRecommendation = "LEASE - Saves $" & Format(difference, "#,##0") & " in PV"
//!         ElseIf leasePV > buyPV Then
//!             GetRecommendation = "BUY - Saves $" & Format(difference, "#,##0") & " in PV"
//!         Else
//!             GetRecommendation = "Either option - PV is equal"
//!         End If
//!     End Function
//!     
//!     Public Function GenerateAnalysis() As String
//!         Dim analysis As String
//!         Dim leasePV As Double
//!         Dim buyPV As Double
//!         
//!         leasePV = GetLeasePresentValue()
//!         buyPV = GetBuyPresentValue()
//!         
//!         analysis = "Lease vs Buy Analysis" & vbCrLf
//!         analysis = analysis & String(50, "=") & vbCrLf
//!         analysis = analysis & "Purchase Price: $" & Format(m_purchasePrice, "#,##0") & vbCrLf
//!         analysis = analysis & "Monthly Lease: $" & Format(m_monthlyLease, "#,##0") & vbCrLf
//!         analysis = analysis & "Lease Term: " & m_leaseTermMonths & " months" & vbCrLf
//!         analysis = analysis & "Discount Rate: " & Format(m_discountRate * 100, "0.0") & "%" & vbCrLf
//!         analysis = analysis & "Residual Value: $" & Format(m_residualValue, "#,##0") & vbCrLf
//!         analysis = analysis & String(50, "-") & vbCrLf
//!         analysis = analysis & "Lease PV: $" & Format(leasePV, "#,##0") & vbCrLf
//!         analysis = analysis & "Buy PV: $" & Format(buyPV, "#,##0") & vbCrLf
//!         analysis = analysis & String(50, "-") & vbCrLf
//!         analysis = analysis & "Recommendation: " & GetRecommendation()
//!         
//!         GenerateAnalysis = analysis
//!     End Function
//! End Class
//! ```
//!
//! ### Example 4: Pension Valuation Calculator
//! ```vb
//! ' Calculate present value of pension benefits
//! Class PensionValuator
//!     Private m_monthlyPension As Double
//!     Private m_yearsToRetirement As Integer
//!     Private m_yearsOfPayments As Integer
//!     Private m_discountRate As Double
//!     Private m_inflationRate As Double
//!     
//!     Public Sub SetPensionDetails(monthlyPension As Double, _
//!                                 yearsToRetirement As Integer, _
//!                                 yearsOfPayments As Integer)
//!         m_monthlyPension = monthlyPension
//!         m_yearsToRetirement = yearsToRetirement
//!         m_yearsOfPayments = yearsOfPayments
//!     End Sub
//!     
//!     Public Sub SetEconomicAssumptions(discountRate As Double, inflationRate As Double)
//!         m_discountRate = discountRate
//!         m_inflationRate = inflationRate
//!     End Sub
//!     
//!     Public Function GetPensionPresentValue() As Double
//!         Dim monthlyRate As Double
//!         Dim numPayments As Integer
//!         Dim pvAtRetirement As Double
//!         Dim pvToday As Double
//!         
//!         monthlyRate = m_discountRate / 12
//!         numPayments = m_yearsOfPayments * 12
//!         
//!         ' Calculate PV at retirement
//!         pvAtRetirement = Abs(PV(monthlyRate, numPayments, m_monthlyPension))
//!         
//!         ' Discount back to today
//!         pvToday = Abs(PV(m_discountRate, m_yearsToRetirement, 0, -pvAtRetirement))
//!         
//!         GetPensionPresentValue = pvToday
//!     End Function
//!     
//!     Public Function GetInflationAdjustedValue() As Double
//!         Dim realRate As Double
//!         Dim monthlyRate As Double
//!         Dim numPayments As Integer
//!         Dim pvAtRetirement As Double
//!         Dim pvToday As Double
//!         
//!         ' Fisher equation: (1 + nominal) = (1 + real)(1 + inflation)
//!         realRate = ((1 + m_discountRate) / (1 + m_inflationRate)) - 1
//!         monthlyRate = realRate / 12
//!         numPayments = m_yearsOfPayments * 12
//!         
//!         pvAtRetirement = Abs(PV(monthlyRate, numPayments, m_monthlyPension))
//!         pvToday = Abs(PV(realRate, m_yearsToRetirement, 0, -pvAtRetirement))
//!         
//!         GetInflationAdjustedValue = pvToday
//!     End Function
//!     
//!     Public Function GenerateValuationReport() As String
//!         Dim report As String
//!         Dim nominalPV As Double
//!         Dim realPV As Double
//!         Dim totalPayments As Double
//!         
//!         nominalPV = GetPensionPresentValue()
//!         realPV = GetInflationAdjustedValue()
//!         totalPayments = m_monthlyPension * m_yearsOfPayments * 12
//!         
//!         report = "Pension Valuation Report" & vbCrLf
//!         report = report & String(50, "=") & vbCrLf
//!         report = report & "Monthly Pension: $" & Format(m_monthlyPension, "#,##0") & vbCrLf
//!         report = report & "Years to Retirement: " & m_yearsToRetirement & vbCrLf
//!         report = report & "Years of Payments: " & m_yearsOfPayments & vbCrLf
//!         report = report & "Discount Rate: " & Format(m_discountRate * 100, "0.0") & "%" & vbCrLf
//!         report = report & "Inflation Rate: " & Format(m_inflationRate * 100, "0.0") & "%" & vbCrLf
//!         report = report & String(50, "-") & vbCrLf
//!         report = report & "Total Nominal Payments: $" & Format(totalPayments, "#,##0") & vbCrLf
//!         report = report & "Present Value (Nominal): $" & Format(nominalPV, "#,##0") & vbCrLf
//!         report = report & "Present Value (Real): $" & Format(realPV, "#,##0") & vbCrLf
//!         report = report & String(50, "-") & vbCrLf
//!         report = report & "Value Reduction from Inflation: $" & _
//!                  Format(nominalPV - realPV, "#,##0") & " (" & _
//!                  Format(((nominalPV - realPV) / nominalPV) * 100, "0.0") & "%)"
//!         
//!         GenerateValuationReport = report
//!     End Function
//! End Class
//! ```
//!
//! ## Error Handling
//!
//! The `PV` function can raise errors in the following situations:
//!
//! - **Invalid Procedure Call (Error 5)**: When:
//!   - `nper` is 0 or negative
//!   - `rate` is -1 (causes division by zero in the formula)
//! - **Type Mismatch (Error 13)**: When arguments cannot be converted to numeric values
//! - **Overflow (Error 6)**: When calculated values exceed Double range
//!
//! Always validate input parameters:
//!
//! ```vb
//! On Error Resume Next
//! presentValue = PV(rate, nper, pmt, fv, type)
//! If Err.Number <> 0 Then
//!     MsgBox "Error calculating present value: " & Err.Description
//!     Err.Clear
//! End If
//! On Error GoTo 0
//! ```
//!
//! ## Performance Considerations
//!
//! - The `PV` function is very fast for individual calculations
//! - Avoid calling repeatedly in tight loops if parameters don't change
//! - Pre-calculate monthly rates and other constants outside loops
//! - For sensitivity analysis, consider caching results
//!
//! ## Best Practices
//!
//! 1. **Convert Rates Properly**: Always divide annual rates by 12 for monthly calculations
//! 2. **Match Time Units**: Ensure rate and nper use the same time period
//! 3. **Use Absolute Value**: Use `Abs()` to display positive values to users
//! 4. **Validate Inputs**: Check that nper > 0 and rate is reasonable
//! 5. **Handle Sign Conventions**: Remember negative = outflow, positive = inflow
//! 6. **Round for Display**: Use `Format()` for currency display
//! 7. **Document Assumptions**: Clearly state discount rates and time periods
//! 8. **Consider Inflation**: Use real rates for inflation-adjusted analysis
//! 9. **Test Edge Cases**: Verify behavior with 0% rate, very long terms
//! 10. **Compare with Pmt**: Understand the inverse relationship between PV and Pmt
//!
//! ## Comparison with Related Functions
//!
//! | Function | Purpose | Returns | Use Case |
//! |----------|---------|---------|----------|
//! | **PV** | Present value of annuity | Double (current value) | Loan affordability, investment valuation |
//! | **FV** | Future value of annuity | Double (future value) | Investment growth, savings goals |
//! | **Pmt** | Periodic payment | Double (payment amount) | Loan payments, inverse of PV |
//! | **NPV** | Net present value | Double (NPV) | Project evaluation with irregular cash flows |
//! | **`NPer`** | Number of periods | Double (period count) | Time to goal calculation |
//! | **Rate** | Interest rate | Double (rate per period) | Finding effective rate |
//!
//! ## Platform and Version Notes
//!
//! - Available in all versions of VBA and VB6
//! - Behavior is consistent across Windows platforms
//! - Uses standard present value formulas from financial mathematics
//! - For zero interest rates, PV is simply pmt * nper + fv
//! - Maximum precision limited by Double data type
//!
//! ## Limitations
//!
//! - Assumes constant interest rate over entire period
//! - Assumes equal payment amounts (standard annuity)
//! - Does not account for taxes, fees, or transaction costs
//! - Cannot handle variable rate scenarios without recalculation
//! - Does not consider payment frequency other than what you specify
//! - Sign convention can be confusing (negative for outflows)
//!
//! ## Related Functions
//!
use crate::error::{VBError, VBResult};
use crate::value::VBVariant;

/// Implementation of the `PV` function.
///
/// Calculates the present value of an annuity based on periodic, fixed payments
/// and a fixed interest rate. The calculation supports both ordinary annuities
/// (payments at end of period) and annuities due (payments at beginning of period).
///
/// VB6 behavior:
/// - `rate` is the interest rate per period
/// - `nper` must be positive; if <= 0, raises error 5 (Invalid procedure call)
/// - `fv` defaults to 0 if omitted
/// - `type` defaults to 0 (end of period) if omitted; use 1 for beginning of period
/// - Returns a `Double`
pub fn pv(
    rate: &VBVariant,
    nper: &VBVariant,
    pmt: &VBVariant,
    fv: Option<&VBVariant>,
    type_: Option<&VBVariant>,
) -> VBResult<VBVariant> {
    let rate_val = rate.as_f64()?;
    let nper_val = nper.as_f64()?;
    let pmt_val = pmt.as_f64()?;
    let fv_val = fv.map(|f| f.as_f64()).transpose()?.unwrap_or(0.0);
    let type_val = type_.map(|t| t.as_i16()).transpose()?.unwrap_or(0);

    // Validate inputs per VB6 behavior
    if nper_val <= 0.0 {
        return Err(VBError::new(5));
    }

    let pv_val = if rate_val == 0.0 {
        // Special case: zero interest rate
        -(pmt_val * nper_val + fv_val)
    } else if type_val == 0 {
        // Payments at end of period (ordinary annuity)
        let factor = (1.0_f64 + rate_val).powi(nper_val as i32);
        let pmt_part = pmt_val * (1.0 - 1.0 / factor) / rate_val;
        let fv_part = -fv_val / factor;
        pmt_part + fv_part
    } else {
        // Payments at beginning of period (annuity due)
        let factor = (1.0_f64 + rate_val).powi(nper_val as i32);
        let pmt_part = pmt_val * (1.0 - 1.0 / factor) / rate_val * (1.0 + rate_val);
        let fv_part = -fv_val / factor;
        pmt_part + fv_part
    };

    Ok(VBVariant::from_double(pv_val))
}

#[cfg(test)]
mod tests {
    use super::pv;
    use crate::value::VBVariant;

    fn pv_defaults(rate: f64, nper: f64, pmt: f64) -> VBVariant {
        pv(
            &VBVariant::from_double(rate),
            &VBVariant::from_double(nper),
            &VBVariant::from_double(pmt),
            None,
            None,
        )
        .unwrap()
    }

    fn pv_with_fv(rate: f64, nper: f64, pmt: f64, fv: f64) -> VBVariant {
        pv(
            &VBVariant::from_double(rate),
            &VBVariant::from_double(nper),
            &VBVariant::from_double(pmt),
            Some(&VBVariant::from_double(fv)),
            None,
        )
        .unwrap()
    }

    fn pv_with_type(rate: f64, nper: f64, pmt: f64, fv: f64, ptype: i16) -> VBVariant {
        pv(
            &VBVariant::from_double(rate),
            &VBVariant::from_double(nper),
            &VBVariant::from_double(pmt),
            Some(&VBVariant::from_double(fv)),
            Some(&VBVariant::from_integer(ptype)),
        )
        .unwrap()
    }

    #[test]
    fn pv_ordinary_annuity() {
        // $100/month for 12 months at 6% annual (0.5% monthly), no FV
        let result = pv_defaults(0.06 / 12.0, 12.0, -100.0);
        let pv_result = result.as_f64().unwrap();
        assert!((pv_result - (-1161.893)).abs() < 0.01);
    }

    #[test]
    fn pv_with_future_value() {
        // Present value of -$100/month plus -$1000 FV for 12 months at 0.5% monthly
        let result = pv_with_fv(0.06 / 12.0, 12.0, -100.0, -1000.0);
        let pv_result = result.as_f64().unwrap();
        assert!((pv_result - (-219.988)).abs() < 0.01);
    }

    #[test]
    fn pv_annuity_due() {
        // Payments at beginning of period
        let result = pv_with_type(0.06 / 12.0, 12.0, -100.0, -1000.0, 1);
        let pv_result = result.as_f64().unwrap();
        assert!((pv_result - (-225.797)).abs() < 0.01);
    }

    #[test]
    fn pv_zero_rate() {
        // With zero rate, PV = -(pmt * nper + fv)
        let result = pv_defaults(0.0, 12.0, -100.0);
        let pv_result = result.as_f64().unwrap();
        assert_eq!(pv_result, 1200.0);
    }

    #[test]
    fn pv_negative_periods_raises_error_5() {
        let err = pv(
            &VBVariant::from_double(0.05),
            &VBVariant::from_double(-1.0),
            &VBVariant::from_double(-100.0),
            None,
            None,
        )
        .unwrap_err();
        assert_eq!(err.number, 5);
    }

    #[test]
    fn pv_zero_periods_raises_error_5() {
        let err = pv(
            &VBVariant::from_double(0.05),
            &VBVariant::from_double(0.0),
            &VBVariant::from_double(-100.0),
            None,
            None,
        )
        .unwrap_err();
        assert_eq!(err.number, 5);
    }

    #[test]
    fn pv_with_integer_args() {
        let result = pv(
            &VBVariant::from_double(0.005),
            &VBVariant::from_long(12),
            &VBVariant::from_long(-100),
            None,
            None,
        )
        .unwrap();
        assert!((result.as_f64().unwrap() - (-1161.893)).abs() < 0.01);
    }

    #[test]
    fn pv_null_raises_invalid_use_of_null() {
        let err = pv(
            &VBVariant::Null,
            &VBVariant::from_double(12.0),
            &VBVariant::from_double(-100.0),
            None,
            None,
        )
        .unwrap_err();
        assert_eq!(err.number, 94);
    }

    #[test]
    fn pv_loan_example() {
        // Loan PV: $20,000 at 6% APR for 5 years (60 payments)
        // Monthly payment would be ~$386.66
        let result = pv_defaults(0.06 / 12.0, 60.0, -386.66);
        let pv_result = result.as_f64().unwrap();
        assert!((pv_result - (-20000.0)).abs() < 1.0);
    }

    #[test]
    fn pv_single_period() {
        // Single period: PV should be PMT / (1+r)
        let result = pv_defaults(0.05, 1.0, -100.0);
        let pv_result = result.as_f64().unwrap();
        assert!((pv_result - (-95.238)).abs() < 0.01);
    }
}
