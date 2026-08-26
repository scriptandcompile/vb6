#[cfg(test)]
mod tests {
    use crate::*;
    use vb6parse::ConcreteSyntaxTree;

    #[test]
    fn nper_basic() {
        let source = r"
Dim months As Double
months = NPer(0.08 / 12, -200, 10000)
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/financial/nper");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn nper_with_fv() {
        let source = r"
Dim periods As Double
periods = NPer(0.06 / 12, -300, 0, 50000)
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/financial/nper");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn nper_with_type() {
        let source = r"
Dim n As Double
n = NPer(rate, pmt, pv, fv, 1)
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/financial/nper");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn nper_if_statement() {
        let source = r#"
If NPer(apr / 12, -payment, balance) > 60 Then
    MsgBox "Payoff will take over 5 years"
End If
"#;
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/financial/nper");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn nper_function_return() {
        let source = r"
Function CalculatePayoffMonths(loan As Double, payment As Double, rate As Double) As Double
    CalculatePayoffMonths = NPer(rate / 12, -payment, loan)
End Function
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/financial/nper");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn nper_variable_assignment() {
        let source = r"
Dim payoffYears As Double
payoffYears = NPer(0.05 / 12, -1000, 200000) / 12
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/financial/nper");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn nper_msgbox() {
        let source = r#"
MsgBox "Months to payoff: " & Format(NPer(rate, pmt, pv), "0.0")
"#;
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/financial/nper");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn nper_debug_print() {
        let source = r#"
Debug.Print "Periods: " & NPer(interestRate, monthlyPmt, principal)
"#;
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/financial/nper");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn nper_select_case() {
        let source = r#"
Select Case Int(NPer(0.08 / 12, -250, 5000) / 12)
    Case Is < 2
        MsgBox "Short term"
    Case 2 To 5
        MsgBox "Medium term"
    Case Else
        MsgBox "Long term"
End Select
"#;
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/financial/nper");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn nper_class_usage() {
        let source = r"
Private m_periods As Double

Public Sub CalculatePeriods()
    m_periods = NPer(m_rate / 12, -m_payment, m_balance)
End Sub
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/financial/nper");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn nper_with_statement() {
        let source = r"
With loanInfo
    .PayoffMonths = NPer(.Rate / 12, -.MonthlyPayment, .Principal)
End With
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/financial/nper");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn nper_elseif() {
        let source = r"
If x > 0 Then
    y = 1
ElseIf NPer(r, p, v) < 36 Then
    y = 2
End If
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/financial/nper");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn nper_for_loop() {
        let source = r"
For payment = 100 To 500 Step 50
    months = NPer(0.1 / 12, -payment, 10000)
    Debug.Print payment, months
Next payment
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/financial/nper");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn nper_do_while() {
        let source = r"
Do While NPer(rate, -payment, balance) > targetMonths
    payment = payment + 10
Loop
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/financial/nper");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn nper_do_until() {
        let source = r"
Do Until NPer(apr / 12, -pmt, bal) <= 12
    pmt = pmt + 50
Loop
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/financial/nper");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn nper_while_wend() {
        let source = r"
While NPer(0.05 / 12, -amount, 50000) > 120
    amount = amount + 25
Wend
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/financial/nper");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn nper_parentheses() {
        let source = r"
Dim result As Double
result = (NPer(rate, payment, principal))
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/financial/nper");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn nper_iif() {
        let source = r#"
Dim term As String
term = IIf(NPer(r, p, v) < 36, "Short", "Long")
"#;
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/financial/nper");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn nper_comparison() {
        let source = r#"
If NPer(rate1, pmt, bal) < NPer(rate2, pmt, bal) Then
    MsgBox "Rate 1 is faster"
End If
"#;
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/financial/nper");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn nper_array_assignment() {
        let source = r"
Dim payoffTimes(10) As Double
payoffTimes(i) = NPer(rates(i) / 12, -payment, balance)
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/financial/nper");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn nper_property_assignment() {
        let source = r"
Set obj = New LoanCalculator
obj.PayoffPeriods = NPer(obj.Rate / 12, -obj.Payment, obj.Balance)
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/financial/nper");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn nper_function_argument() {
        let source = r"
Call DisplayPayoffSchedule(NPer(apr / 12, -monthlyPmt, loanAmount))
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/financial/nper");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn nper_format() {
        let source = r#"
Dim formatted As String
formatted = Format(NPer(0.06 / 12, -500, 25000), "0.00")
"#;
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/financial/nper");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn nper_arithmetic() {
        let source = r"
Dim years As Double
years = NPer(rate / 12, -payment, principal) / 12
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/financial/nper");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn nper_concatenation() {
        let source = r#"
Dim msg As String
msg = "Payoff time: " & NPer(r, p, v) & " months"
"#;
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/financial/nper");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn nper_int_conversion() {
        let source = r"
Dim wholeMonths As Integer
wholeMonths = Int(NPer(0.08 / 12, -300, 15000))
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/financial/nper");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn nper_error_handling() {
        let source = r#"
On Error Resume Next
n = NPer(rate, payment, balance)
If Err.Number <> 0 Then
    MsgBox "Invalid calculation"
End If
On Error GoTo 0
"#;
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/financial/nper");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }
}
