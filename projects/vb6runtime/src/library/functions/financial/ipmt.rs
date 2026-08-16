//! # `IPmt` Function
//!
//! Returns a `Double` specifying the interest payment for a given period of an annuity based on periodic, fixed payments and a fixed interest rate.
//!
//! ## Syntax
//!
//! ```vb
//! IPmt(rate, per, nper, pv[, fv[, type]])
//! ```
//!
//! ## Parameters
//!
//! - `rate` (Required): `Double` specifying interest rate per period. For example, if you get a car loan at an annual percentage rate (APR) of 10 percent and make monthly payments, the rate per period is 0.1/12, or 0.0083
//! - `per` (Required): `Double` specifying payment period in the range 1 through nper
//! - `nper` (Required): `Double` specifying total number of payment periods in the annuity. For example, if you make monthly payments on a four-year car loan, your loan has 4 * 12 (or 48) payment periods
//! - `pv` (Required): `Double` specifying present value, or value today, of a series of future payments or receipts. For example, when you borrow money to buy a car, the loan amount is the present value to the lender of the monthly car payments you will make
//! - `fv` (Optional): `Variant` specifying future value or cash balance you want after you've made the final payment. For example, the future value of a loan is $0 because that's its value after the final payment. However, if you want to save $50,000 over 18 years for your child's education, then $50,000 is the future value. If omitted, 0 is assumed
//! - `type` (Optional): `Variant` specifying when payments are due. Use 0 if payments are due at the end of the payment period, or use 1 if payments are due at the beginning of the period. If omitted, 0 is assumed
//!
//! ## Return Value
//!
//! Returns a `Double` representing the interest payment for the specified period:
//! - Negative value indicates money paid out (such as loan interest payments)
//! - Positive value indicates money received (such as investment interest earnings)
//! - The sum of interest payments (`IPmt`) and principal payments (`PPmt`) equals the total payment for a period
//!
//! ## Remarks
//!
//! An annuity is a series of fixed cash payments made over a period of time:
//!
//! - Used to calculate the interest portion of a specific payment
//! - Commonly used for loan amortization calculations
//! - Interest payment decreases over the life of a loan (more principal paid later)
//! - The `per` argument must be in the range 1 through `nper`
//! - `rate` and `nper` must be calculated using payment periods in the same units
//! - For monthly payments, divide annual rate by 12, multiply years by 12
//! - For quarterly payments, divide annual rate by 4, multiply years by 4
//! - All arguments referring to cash paid out are negative; cash received is positive
//! - The interest payment varies by period (unlike fixed total payment)
//! - Use `PPmt` to calculate the principal portion of a payment
//! - Use `Pmt` to calculate the total payment amount
//!
//! ## Typical Uses
//!
//! 1. **Loan Amortization**: Calculate interest portion of loan payments
//! 2. **Mortgage Analysis**: Determine interest paid per payment period
//! 3. **Investment Analysis**: Calculate interest earned per period
//! 4. **Financial Planning**: Project interest expenses over time
//! 5. **Tax Calculations**: Determine deductible interest for tax purposes
//! 6. **Budget Planning**: Forecast interest costs
//! 7. **Refinancing Analysis**: Compare interest costs between loans
//! 8. **Amortization Schedules**: Build payment schedules showing interest breakdown
//!
//! ## Basic Usage Examples
//!
//! ```vb
//! ' Example 1: Calculate first month's interest on a car loan
//! Dim monthlyRate As Double
//! Dim totalMonths As Integer
//! Dim loanAmount As Double
//! Dim interestPayment As Double
//!
//! monthlyRate = 0.08 / 12  ' 8% annual rate
//! totalMonths = 48         ' 4-year loan
//! loanAmount = -20000      ' $20,000 borrowed (negative = cash received)
//!
//! interestPayment = IPmt(monthlyRate, 1, totalMonths, loanAmount)
//! Debug.Print "First month interest: " & Format$(interestPayment, "Currency")
//! ' Prints approximately: -$133.33
//!
//! ' Example 2: Calculate last payment's interest
//! interestPayment = IPmt(monthlyRate, 48, totalMonths, loanAmount)
//! Debug.Print "Last month interest: " & Format$(interestPayment, "Currency")
//! ' Prints approximately: -$3.26 (much less than first month)
//!
//! ' Example 3: Calculate total interest paid in first year
//! Dim totalInterest As Double
//! Dim i As Integer
//! totalInterest = 0
//! For i = 1 To 12
//!     totalInterest = totalInterest + IPmt(monthlyRate, i, totalMonths, loanAmount)
//! Next i
//! Debug.Print "First year interest: " & Format$(totalInterest, "Currency")
//!
//! ' Example 4: Monthly mortgage interest payment
//! Dim mortgageRate As Double
//! Dim mortgageMonths As Integer
//! Dim mortgageAmount As Double
//!
//! mortgageRate = 0.06 / 12  ' 6% annual rate
//! mortgageMonths = 30 * 12  ' 30-year mortgage
//! mortgageAmount = -200000  ' $200,000 loan
//!
//! interestPayment = IPmt(mortgageRate, 1, mortgageMonths, mortgageAmount)
//! Debug.Print "First mortgage payment interest: " & Format$(interestPayment, "Currency")
//! ```
//!
//! ## Common Patterns
//!
//! ```vb
//! ' Pattern 1: Calculate interest for specific payment
//! Function CalculateInterestPayment(loanAmount As Double, annualRate As Double, _
//!                                   years As Integer, paymentNumber As Integer) As Double
//!     Dim monthlyRate As Double
//!     Dim totalPayments As Integer
//!     
//!     monthlyRate = annualRate / 12
//!     totalPayments = years * 12
//!     
//!     CalculateInterestPayment = IPmt(monthlyRate, paymentNumber, totalPayments, -loanAmount)
//! End Function
//!
//! ' Pattern 2: Calculate total interest for a year
//! Function CalculateAnnualInterest(loanAmount As Double, annualRate As Double, _
//!                                  years As Integer, year As Integer) As Double
//!     Dim monthlyRate As Double
//!     Dim totalPayments As Integer
//!     Dim startMonth As Integer
//!     Dim endMonth As Integer
//!     Dim totalInterest As Double
//!     Dim i As Integer
//!     
//!     monthlyRate = annualRate / 12
//!     totalPayments = years * 12
//!     startMonth = (year - 1) * 12 + 1
//!     endMonth = year * 12
//!     
//!     totalInterest = 0
//!     For i = startMonth To endMonth
//!         If i <= totalPayments Then
//!             totalInterest = totalInterest + IPmt(monthlyRate, i, totalPayments, -loanAmount)
//!         End If
//!     Next i
//!     
//!     CalculateAnnualInterest = totalInterest
//! End Function
//!
//! ' Pattern 3: Generate amortization schedule entry
//! Type AmortizationEntry
//!     PaymentNumber As Integer
//!     TotalPayment As Double
//!     InterestPayment As Double
//!     PrincipalPayment As Double
//!     Balance As Double
//! End Type
//!
//! Function GetAmortizationEntry(loanAmount As Double, annualRate As Double, _
//!                               years As Integer, paymentNumber As Integer) As AmortizationEntry
//!     Dim monthlyRate As Double
//!     Dim totalPayments As Integer
//!     Dim entry As AmortizationEntry
//!     
//!     monthlyRate = annualRate / 12
//!     totalPayments = years * 12
//!     
//!     entry.PaymentNumber = paymentNumber
//!     entry.TotalPayment = Pmt(monthlyRate, totalPayments, -loanAmount)
//!     entry.InterestPayment = IPmt(monthlyRate, paymentNumber, totalPayments, -loanAmount)
//!     entry.PrincipalPayment = PPmt(monthlyRate, paymentNumber, totalPayments, -loanAmount)
//!     
//!     GetAmortizationEntry = entry
//! End Function
//!
//! ' Pattern 4: Compare interest costs between loans
//! Function CompareInterestCosts(loan1Amount As Double, loan1Rate As Double, loan1Years As Integer, _
//!                               loan2Amount As Double, loan2Rate As Double, loan2Years As Integer) As String
//!     Dim loan1Interest As Double
//!     Dim loan2Interest As Double
//!     Dim i As Integer
//!     
//!     ' Calculate total interest for loan 1
//!     For i = 1 To loan1Years * 12
//!         loan1Interest = loan1Interest + IPmt(loan1Rate / 12, i, loan1Years * 12, -loan1Amount)
//!     Next i
//!     
//!     ' Calculate total interest for loan 2
//!     For i = 1 To loan2Years * 12
//!         loan2Interest = loan2Interest + IPmt(loan2Rate / 12, i, loan2Years * 12, -loan2Amount)
//!     Next i
//!     
//!     CompareInterestCosts = "Loan 1 total interest: " & Format$(loan1Interest, "Currency") & vbCrLf & _
//!                           "Loan 2 total interest: " & Format$(loan2Interest, "Currency")
//! End Function
//!
//! ' Pattern 5: Calculate deductible interest for tax year
//! Function CalculateTaxDeductibleInterest(loanAmount As Double, annualRate As Double, _
//!                                        totalYears As Integer, taxYear As Integer) As Double
//!     ' Assumes loan started at beginning of year 1
//!     CalculateTaxDeductibleInterest = CalculateAnnualInterest(loanAmount, annualRate, totalYears, taxYear)
//! End Function
//!
//! ' Pattern 6: Determine when interest drops below threshold
//! Function FindPaymentWhenInterestBelow(loanAmount As Double, annualRate As Double, _
//!                                       years As Integer, threshold As Double) As Integer
//!     Dim monthlyRate As Double
//!     Dim totalPayments As Integer
//!     Dim i As Integer
//!     Dim interestPayment As Double
//!     
//!     monthlyRate = annualRate / 12
//!     totalPayments = years * 12
//!     
//!     For i = 1 To totalPayments
//!         interestPayment = Abs(IPmt(monthlyRate, i, totalPayments, -loanAmount))
//!         If interestPayment < threshold Then
//!             FindPaymentWhenInterestBelow = i
//!             Exit Function
//!         End If
//!     Next i
//!     
//!     FindPaymentWhenInterestBelow = -1  ' Never drops below threshold
//! End Function
//!
//! ' Pattern 7: Calculate average monthly interest
//! Function CalculateAverageMonthlyInterest(loanAmount As Double, annualRate As Double, _
//!                                          years As Integer) As Double
//!     Dim totalInterest As Double
//!     Dim totalPayments As Integer
//!     Dim i As Integer
//!     
//!     totalPayments = years * 12
//!     
//!     For i = 1 To totalPayments
//!         totalInterest = totalInterest + IPmt(annualRate / 12, i, totalPayments, -loanAmount)
//!     Next i
//!     
//!     CalculateAverageMonthlyInterest = totalInterest / totalPayments
//! End Function
//!
//! ' Pattern 8: Interest payment with balloon payment
//! Function CalculateInterestWithBalloon(loanAmount As Double, annualRate As Double, _
//!                                       years As Integer, paymentNumber As Integer, _
//!                                       balloonAmount As Double) As Double
//!     Dim monthlyRate As Double
//!     Dim totalPayments As Integer
//!     
//!     monthlyRate = annualRate / 12
//!     totalPayments = years * 12
//!     
//!     CalculateInterestWithBalloon = IPmt(monthlyRate, paymentNumber, totalPayments, _
//!                                         -loanAmount, -balloonAmount)
//! End Function
//!
//! ' Pattern 9: Interest for payment due at beginning
//! Function CalculateInterestPaymentBeginning(loanAmount As Double, annualRate As Double, _
//!                                            years As Integer, paymentNumber As Integer) As Double
//!     Dim monthlyRate As Double
//!     Dim totalPayments As Integer
//!     
//!     monthlyRate = annualRate / 12
//!     totalPayments = years * 12
//!     
//!     ' Type = 1 means payment at beginning of period
//!     CalculateInterestPaymentBeginning = IPmt(monthlyRate, paymentNumber, totalPayments, _
//!                                              -loanAmount, 0, 1)
//! End Function
//!
//! ' Pattern 10: Validate interest payment calculation
//! Function ValidateInterestPayment(loanAmount As Double, annualRate As Double, _
//!                                  years As Integer, paymentNumber As Integer) As Boolean
//!     Dim totalPayment As Double
//!     Dim interestPayment As Double
//!     Dim principalPayment As Double
//!     Dim monthlyRate As Double
//!     Dim totalPayments As Integer
//!     
//!     monthlyRate = annualRate / 12
//!     totalPayments = years * 12
//!     
//!     totalPayment = Pmt(monthlyRate, totalPayments, -loanAmount)
//!     interestPayment = IPmt(monthlyRate, paymentNumber, totalPayments, -loanAmount)
//!     principalPayment = PPmt(monthlyRate, paymentNumber, totalPayments, -loanAmount)
//!     
//!     ' Verify that interest + principal = total payment (within rounding tolerance)
//!     ValidateInterestPayment = (Abs(totalPayment - (interestPayment + principalPayment)) < 0.01)
//! End Function
//! ```
//!
//! ## Advanced Usage Examples
//!
//! ```vb
//! ' Example 1: Complete amortization schedule generator
//! Public Class AmortizationSchedule
//!     Private m_loanAmount As Double
//!     Private m_annualRate As Double
//!     Private m_years As Integer
//!     Private m_schedule As Collection
//!     
//!     Public Sub Initialize(loanAmount As Double, annualRate As Double, years As Integer)
//!         m_loanAmount = loanAmount
//!         m_annualRate = annualRate
//!         m_years = years
//!         GenerateSchedule
//!     End Sub
//!     
//!     Private Sub GenerateSchedule()
//!         Dim monthlyRate As Double
//!         Dim totalPayments As Integer
//!         Dim i As Integer
//!         Dim entry As AmortizationEntry
//!         Dim balance As Double
//!         
//!         Set m_schedule = New Collection
//!         monthlyRate = m_annualRate / 12
//!         totalPayments = m_years * 12
//!         balance = m_loanAmount
//!         
//!         For i = 1 To totalPayments
//!             entry.PaymentNumber = i
//!             entry.TotalPayment = Pmt(monthlyRate, totalPayments, -m_loanAmount)
//!             entry.InterestPayment = IPmt(monthlyRate, i, totalPayments, -m_loanAmount)
//!             entry.PrincipalPayment = PPmt(monthlyRate, i, totalPayments, -m_loanAmount)
//!             balance = balance - entry.PrincipalPayment
//!             entry.Balance = balance
//!             
//!             m_schedule.Add entry
//!         Next i
//!     End Sub
//!     
//!     Public Function GetPayment(paymentNumber As Integer) As AmortizationEntry
//!         If paymentNumber >= 1 And paymentNumber <= m_schedule.Count Then
//!             GetPayment = m_schedule(paymentNumber)
//!         End If
//!     End Function
//!     
//!     Public Function GetTotalInterest() As Double
//!         Dim i As Integer
//!         Dim total As Double
//!         Dim entry As AmortizationEntry
//!         
//!         For i = 1 To m_schedule.Count
//!             entry = m_schedule(i)
//!             total = total + entry.InterestPayment
//!         Next i
//!         
//!         GetTotalInterest = total
//!     End Function
//! End Class
//!
//! ' Example 2: Loan comparison calculator
//! Public Class LoanComparer
//!     Public Function CompareLoanOptions(loanAmount As Double) As String
//!         Dim result As String
//!         Dim option1Interest As Double
//!         Dim option2Interest As Double
//!         Dim option3Interest As Double
//!         Dim i As Integer
//!         
//!         result = "Loan Amount: " & Format$(loanAmount, "Currency") & vbCrLf & vbCrLf
//!         
//!         ' Option 1: 15-year at 5.5%
//!         For i = 1 To 15 * 12
//!             option1Interest = option1Interest + IPmt(0.055 / 12, i, 15 * 12, -loanAmount)
//!         Next i
//!         result = result & "15-year at 5.5%: " & Format$(option1Interest, "Currency") & vbCrLf
//!         
//!         ' Option 2: 20-year at 6.0%
//!         For i = 1 To 20 * 12
//!             option2Interest = option2Interest + IPmt(0.06 / 12, i, 20 * 12, -loanAmount)
//!         Next i
//!         result = result & "20-year at 6.0%: " & Format$(option2Interest, "Currency") & vbCrLf
//!         
//!         ' Option 3: 30-year at 6.5%
//!         For i = 1 To 30 * 12
//!             option3Interest = option3Interest + IPmt(0.065 / 12, i, 30 * 12, -loanAmount)
//!         Next i
//!         result = result & "30-year at 6.5%: " & Format$(option3Interest, "Currency")
//!         
//!         CompareLoanOptions = result
//!     End Function
//! End Class
//!
//! ' Example 3: Interest payment tracker
//! Public Class InterestTracker
//!     Private m_loanAmount As Double
//!     Private m_annualRate As Double
//!     Private m_years As Integer
//!     Private m_currentPayment As Integer
//!     
//!     Public Sub Initialize(loanAmount As Double, annualRate As Double, years As Integer)
//!         m_loanAmount = loanAmount
//!         m_annualRate = annualRate
//!         m_years = years
//!         m_currentPayment = 0
//!     End Sub
//!     
//!     Public Function GetNextInterestPayment() As Double
//!         m_currentPayment = m_currentPayment + 1
//!         If m_currentPayment <= m_years * 12 Then
//!             GetNextInterestPayment = IPmt(m_annualRate / 12, m_currentPayment, _
//!                                          m_years * 12, -m_loanAmount)
//!         Else
//!             GetNextInterestPayment = 0
//!         End If
//!     End Function
//!     
//!     Public Function GetInterestForPayment(paymentNumber As Integer) As Double
//!         If paymentNumber >= 1 And paymentNumber <= m_years * 12 Then
//!             GetInterestForPayment = IPmt(m_annualRate / 12, paymentNumber, _
//!                                         m_years * 12, -m_loanAmount)
//!         Else
//!             GetInterestForPayment = 0
//!         End If
//!     End Function
//!     
//!     Public Function GetYearToDateInterest() As Double
//!         Dim currentYear As Integer
//!         Dim startMonth As Integer
//!         Dim endMonth As Integer
//!         Dim total As Double
//!         Dim i As Integer
//!         
//!         currentYear = Int((m_currentPayment - 1) / 12) + 1
//!         startMonth = (currentYear - 1) * 12 + 1
//!         endMonth = m_currentPayment
//!         
//!         For i = startMonth To endMonth
//!             total = total + IPmt(m_annualRate / 12, i, m_years * 12, -m_loanAmount)
//!         Next i
//!         
//!         GetYearToDateInterest = total
//!     End Function
//! End Class
//!
//! ' Example 4: Refinancing analyzer
//! Function AnalyzeRefinancing(currentLoanBalance As Double, currentRate As Double, _
//!                            remainingYears As Integer, newRate As Double, _
//!                            newYears As Integer, closingCosts As Double) As String
//!     Dim currentTotalInterest As Double
//!     Dim newTotalInterest As Double
//!     Dim i As Integer
//!     Dim result As String
//!     
//!     ' Calculate remaining interest on current loan
//!     For i = 1 To remainingYears * 12
//!         currentTotalInterest = currentTotalInterest + _
//!             IPmt(currentRate / 12, i, remainingYears * 12, -currentLoanBalance)
//!     Next i
//!     
//!     ' Calculate total interest on new loan
//!     For i = 1 To newYears * 12
//!         newTotalInterest = newTotalInterest + _
//!             IPmt(newRate / 12, i, newYears * 12, -currentLoanBalance)
//!     Next i
//!     
//!     result = "Current loan interest: " & Format$(currentTotalInterest, "Currency") & vbCrLf
//!     result = result & "New loan interest: " & Format$(newTotalInterest, "Currency") & vbCrLf
//!     result = result & "Closing costs: " & Format$(closingCosts, "Currency") & vbCrLf
//!     result = result & "Net savings: " & _
//!              Format$(currentTotalInterest - newTotalInterest - closingCosts, "Currency")
//!     
//!     AnalyzeRefinancing = result
//! End Function
//! ```
//!
//! ## Error Handling
//!
//! The `IPmt` function can raise errors:
//!
//! - **Invalid procedure call (Error 5)**: If `per` is less than 1 or greater than `nper`
//! - **Type Mismatch (Error 13)**: If arguments are not numeric
//! - **Overflow (Error 6)**: If result exceeds Double range
//!
//! ```vb
//! On Error GoTo ErrorHandler
//! Dim interestPayment As Double
//!
//! interestPayment = IPmt(0.08 / 12, 1, 48, -20000)
//! Debug.Print "Interest payment: " & Format$(interestPayment, "Currency")
//! Exit Sub
//!
//! ErrorHandler:
//!     MsgBox "Error calculating interest: " & Err.Description, vbCritical
//! ```
//!
//! ## Performance Considerations
//!
//! - **Calculation Intensity**: `IPmt` involves complex financial calculations
//! - **Loop Performance**: Calculating all payments can be slow for long-term loans
//! - **Caching**: Consider caching amortization schedules rather than recalculating
//! - **Precision**: Uses `Double` precision for accurate financial calculations
//!
//! ## Best Practices
//!
//! 1. **Consistent Periods**: Ensure rate and nper use same time units (monthly, quarterly, etc.)
//! 2. **Sign Convention**: Use negative for cash paid, positive for cash received
//! 3. **Validate Period**: Check that `per` is between 1 and `nper`
//! 4. **Error Handling**: Wrap financial calculations in error handlers
//! 5. **Rounding**: Round currency values appropriately for display
//! 6. **Documentation**: Document assumptions about payment timing (beginning/end of period)
//! 7. **Testing**: Verify that `IPmt` + `PPmt` = `Pmt` for each period
//!
//! ## Platform and Version Notes
//!
//! - Available in all VB6 versions
//! - Part of VBA financial functions
//! - Uses `Double` precision (not Currency type)
//! - Consistent with Excel's `IPMT` function
//! - Sign convention follows financial standards
//!
//! ## Limitations
//!
//! - Does not handle variable interest rates
//! - Assumes constant payment amounts
//! - Does not account for fees or other charges
//! - Limited to fixed annuity calculations
//! - No built-in support for skip payments or extra payments
//!
//! ## Related Functions
//!
//! - `PPmt`: Principal payment for a period
//! - `Pmt`: Total payment for a period
//! - `Rate`: Interest rate per period
//! - `NPer`: Number of periods
//! - `PV`: Present value
//! - `FV`: Future value

use crate::error::{VBError, VBResult};
use crate::value::VBVariant;

/// Implementation of the `IPmt` function.
///
/// Calculates the interest payment for a given period of an annuity based on
/// periodic, fixed payments and a fixed interest rate.
///
/// VB6 behavior:
/// - `rate` is the interest rate per period
/// - `per` must be in range [1, nper]; if outside, raises error 5
/// - `nper` must be positive; if <= 0, raises error 5
/// - `pv` is the present value (loan amount)
/// - `fv` defaults to 0 if omitted
/// - `type` defaults to 0 (end of period) if omitted; use 1 for beginning of period
/// - Returns a `Double`
pub fn ipmt(
    rate: &VBVariant,
    per: &VBVariant,
    nper: &VBVariant,
    pv: &VBVariant,
    fv: Option<&VBVariant>,
    type_: Option<&VBVariant>,
) -> VBResult<VBVariant> {
    let rate_val = rate.as_f64()?;
    let per_val = per.as_f64()?;
    let nper_val = nper.as_f64()?;
    let pv_val = pv.as_f64()?;
    let fv_val = fv.map(|f| f.as_f64()).transpose()?.unwrap_or(0.0);
    let type_val = type_.map(|t| t.as_i16()).transpose()?.unwrap_or(0);

    // Validate inputs per VB6 behavior
    if nper_val <= 0.0 {
        return Err(VBError::new(5));
    }

    if per_val < 1.0 || per_val > nper_val {
        return Err(VBError::new(5));
    }

    // IPmt is the interest charged on the outstanding balance for the period,
    // following the same rules as VB6:
    // - zero interest rate: no interest is charged
    // - payments at the beginning of the period (type 1): the first payment
    //   carries no interest; later ones accrue on the balance after the
    //   previous payment
    // - otherwise: interest accrues on the balance at the start of the period
    let ipmt_val = if rate_val == 0.0 || (type_val != 0 && per_val == 1.0) {
        // Zero interest: no interest is charged.
        // Beginning-of-period first payment: made immediately, no interest yet.
        0.0
    } else {
        // Calculate the periodic payment using Pmt formula
        let pmt_val = if type_val == 0 {
            let factor = (1.0_f64 + rate_val).powf(nper_val);
            -(pv_val * factor + fv_val) / ((factor - 1.0) / rate_val)
        } else {
            let factor = (1.0_f64 + rate_val).powf(nper_val);
            -(pv_val * factor + fv_val) / ((factor - 1.0) / rate_val * (1.0_f64 + rate_val))
        };

        if type_val != 0 {
            // Balance after the previous payment, using FV formula
            let balance = fv_formula(rate_val, per_val - 2.0, pmt_val, pv_val + pmt_val);
            balance * rate_val
        } else {
            // Balance at start of period `per` using FV formula
            // Balance = FV(rate, per-1, pmt, pv)
            let balance = fv_formula(rate_val, per_val - 1.0, pmt_val, pv_val);
            balance * rate_val
        }
    };

    Ok(VBVariant::from_double(ipmt_val))
}

/// Calculate future value using the same formula as the FV function.
fn fv_formula(rate: f64, nper: f64, pmt: f64, pv: f64) -> f64 {
    if nper == 0.0 {
        return -pv;
    }
    let factor = (1.0_f64 + rate).powf(nper);
    let pv_part = -pv * factor;
    let pmt_part = -pmt * (factor - 1.0) / rate;
    pv_part + pmt_part
}

#[cfg(test)]
mod tests {
    use super::ipmt;
    use crate::value::VBVariant;

    fn ipmt_defaults(rate: f64, per: f64, nper: f64, pv: f64) -> VBVariant {
        ipmt(
            &VBVariant::from_double(rate),
            &VBVariant::from_double(per),
            &VBVariant::from_double(nper),
            &VBVariant::from_double(pv),
            None,
            None,
        )
        .unwrap()
    }

    fn ipmt_with_fv(rate: f64, per: f64, nper: f64, pv: f64, fv: f64) -> VBVariant {
        ipmt(
            &VBVariant::from_double(rate),
            &VBVariant::from_double(per),
            &VBVariant::from_double(nper),
            &VBVariant::from_double(pv),
            Some(&VBVariant::from_double(fv)),
            None,
        )
        .unwrap()
    }

    fn ipmt_with_type(rate: f64, per: f64, nper: f64, pv: f64, fv: f64, ptype: i16) -> VBVariant {
        ipmt(
            &VBVariant::from_double(rate),
            &VBVariant::from_double(per),
            &VBVariant::from_double(nper),
            &VBVariant::from_double(pv),
            Some(&VBVariant::from_double(fv)),
            Some(&VBVariant::from_integer(ptype)),
        )
        .unwrap()
    }

    #[test]
    fn ipmt_first_period_loan() {
        // 8% annual rate, monthly payments, 48 months, $20,000 loan
        // First month interest should be approximately $133.33.
        // With a negative pv (deposit), the interest payment is positive.
        let result = ipmt_defaults(0.08 / 12.0, 1.0, 48.0, -20000.0);
        let ipmt_val = result.as_f64().unwrap();
        assert!((ipmt_val - 133.33).abs() < 0.01);
    }

    #[test]
    fn ipmt_last_period_less_interest() {
        // Last period should have much less interest than first
        let first = ipmt_defaults(0.08 / 12.0, 1.0, 48.0, -20000.0);
        let last = ipmt_defaults(0.08 / 12.0, 48.0, 48.0, -20000.0);

        let first_val = first.as_f64().unwrap();
        let last_val = last.as_f64().unwrap();

        // Interest decreases over time
        assert!((first_val - 133.33).abs() < 0.01);
        assert!((last_val - 3.23).abs() < 0.01);
        assert!(last_val < first_val);
    }

    #[test]
    fn ipmt_zero_rate_returns_zero() {
        let result = ipmt_defaults(0.0, 1.0, 48.0, -20000.0);
        assert_eq!(result.as_f64().unwrap(), 0.0);
    }

    #[test]
    fn ipmt_with_beginning_of_period() {
        // Payments due at beginning of period: the first payment carries no
        // interest (it is made immediately, before any interest accrues).
        let result = ipmt_with_type(0.08 / 12.0, 1.0, 48.0, -20000.0, 0.0, 1);
        let val = result.as_f64().unwrap();
        assert_eq!(val, 0.0);
    }

    #[test]
    fn ipmt_zero_nper_raises_error_5() {
        let err = ipmt(
            &VBVariant::from_double(0.05),
            &VBVariant::from_double(1.0),
            &VBVariant::from_double(0.0),
            &VBVariant::from_double(-20000.0),
            None,
            None,
        )
        .unwrap_err();
        assert_eq!(err.number, 5);
    }

    #[test]
    fn ipmt_per_out_of_range_raises_error_5() {
        let err = ipmt(
            &VBVariant::from_double(0.05),
            &VBVariant::from_double(49.0), // per > nper
            &VBVariant::from_double(48.0),
            &VBVariant::from_double(-20000.0),
            None,
            None,
        )
        .unwrap_err();
        assert_eq!(err.number, 5);
    }

    #[test]
    fn ipmt_with_integer_args() {
        let result = ipmt(
            &VBVariant::from_double(0.08 / 12.0),
            &VBVariant::from_long(1),
            &VBVariant::from_long(48),
            &VBVariant::from_long(-20000),
            None,
            None,
        )
        .unwrap();
        let val = result.as_f64().unwrap();
        assert!((val - 133.33).abs() < 0.01); // Interest payment is positive for negative pv
    }

    #[test]
    fn ipmt_null_raises_invalid_use_of_null() {
        let err = ipmt(
            &VBVariant::Null,
            &VBVariant::from_double(1.0),
            &VBVariant::from_double(48.0),
            &VBVariant::from_double(-20000.0),
            None,
            None,
        )
        .unwrap_err();
        assert_eq!(err.number, 94);
    }

    #[test]
    fn ipmt_negative_nper_raises_error_5() {
        let err = ipmt(
            &VBVariant::from_double(0.05),
            &VBVariant::from_double(1.0),
            &VBVariant::from_double(-48.0),
            &VBVariant::from_double(-20000.0),
            None,
            None,
        )
        .unwrap_err();
        assert_eq!(err.number, 5);
    }

    #[test]
    fn ipmt_with_future_value() {
        let result = ipmt_with_fv(0.08 / 12.0, 1.0, 48.0, -20000.0, -5000.0);
        let val = result.as_f64().unwrap();
        assert!((val - 133.33).abs() < 0.01);
    }

    #[test]
    fn ipmt_plus_ppmt_equals_pmt() {
        // VB6 guarantees IPmt(per) + PPmt(per) = Pmt for every period, for both
        // end-of-period and beginning-of-period payment schedules.
        let rate = 0.08 / 12.0;
        for per in 1..=48 {
            for ptype in [0, 1] {
                let pmt_val = crate::library::functions::financial::pmt::pmt(
                    VBVariant::from_double(rate),
                    VBVariant::from_double(48.0),
                    VBVariant::from_double(-20000.0),
                    None,
                    Some(&VBVariant::from_integer(ptype)),
                )
                .unwrap()
                .as_f64()
                .unwrap();

                let ipmt_val = ipmt_with_type(rate, per as f64, 48.0, -20000.0, 0.0, ptype)
                    .as_f64()
                    .unwrap();
                let ppmt_val = crate::library::functions::financial::ppmt::ppmt(
                    &VBVariant::from_double(rate),
                    &VBVariant::from_double(per as f64),
                    &VBVariant::from_double(48.0),
                    &VBVariant::from_double(-20000.0),
                    None,
                    Some(&VBVariant::from_integer(ptype)),
                )
                .unwrap()
                .as_f64()
                .unwrap();

                assert!(
                    (ipmt_val + ppmt_val - pmt_val).abs() < 1e-9,
                    "per={}, type={}: ipmt {} + ppmt {} != pmt {}",
                    per,
                    ptype,
                    ipmt_val,
                    ppmt_val,
                    pmt_val
                );
            }
        }
    }

    #[test]
    fn ipmt_per_zero_raises_error_5() {
        let err = ipmt(
            &VBVariant::from_double(0.05),
            &VBVariant::from_double(0.0),
            &VBVariant::from_double(48.0),
            &VBVariant::from_double(-20000.0),
            None,
            None,
        )
        .unwrap_err();
        assert_eq!(err.number, 5);
    }
}
