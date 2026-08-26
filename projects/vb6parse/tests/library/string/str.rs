#[cfg(test)]
mod tests {
    use crate::*;
    use vb6parse::ConcreteSyntaxTree;

    #[test]
    fn str_basic() {
        let source = r"
Sub Test()
    result = Str(123)
End Sub
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/str");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn str_variable_assignment() {
        let source = r"
Sub Test()
    Dim s As String
    s = Str(value)
End Sub
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/str");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn str_negative_number() {
        let source = r"
Sub Test()
    result = Str(-456)
End Sub
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/str");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn str_decimal() {
        let source = r"
Sub Test()
    result = Str(3.14159)
End Sub
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/str");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn str_with_ltrim() {
        let source = r"
Sub Test()
    result = LTrim$(Str(value))
End Sub
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/str");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn str_concatenation() {
        let source = r#"
Sub Test()
    message = "Count:" & Str(count)
End Sub
"#;
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/str");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn str_in_print() {
        let source = r"
Sub Test()
    Print #1, Str(value)
End Sub
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/str");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn str_debug_print() {
        let source = r"
Sub Test()
    Debug.Print Str(x)
End Sub
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/str");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn str_if_statement() {
        let source = r#"
Sub Test()
    If Str(value) = " 0" Then
        MsgBox "Zero"
    End If
End Sub
"#;
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/str");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn str_for_loop() {
        let source = r"
Sub Test()
    For i = 1 To 10
        result = result & Str(i)
    Next i
End Sub
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/str");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn str_array_assignment() {
        let source = r"
Sub Test()
    arr(i) = Str(values(i))
End Sub
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/str");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn str_function_argument() {
        let source = r"
Sub Test()
    Call ProcessString(Str(number))
End Sub
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/str");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn str_function_return() {
        let source = r"
Function GetNumberString() As String
    GetNumberString = Str(value)
End Function
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/str");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn str_comparison() {
        let source = r#"
Sub Test()
    If Len(Str(num)) > 5 Then
        MsgBox "Long number"
    End If
End Sub
"#;
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/str");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn str_select_case() {
        let source = r#"
Sub Test()
    Select Case Str(value)
        Case " 0"
            MsgBox "Zero"
        Case Else
            MsgBox "Other"
    End Select
End Sub
"#;
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/str");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn str_with_statement() {
        let source = r"
Sub Test()
    With obj
        .Text = Str(.Value)
    End With
End Sub
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/str");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn str_do_while() {
        let source = r"
Sub Test()
    Do While Len(Str(counter)) < 10
        counter = counter + 1
    Loop
End Sub
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/str");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn str_do_until() {
        let source = r#"
Sub Test()
    Do Until Str(value) = " 100"
        value = value + 1
    Loop
End Sub
"#;
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/str");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn str_while_wend() {
        let source = r"
Sub Test()
    While i < 10
        output = output & Str(i)
        i = i + 1
    Wend
End Sub
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/str");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn str_iif() {
        let source = r"
Sub Test()
    result = IIf(flag, Str(value1), Str(value2))
End Sub
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/str");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn str_parentheses() {
        let source = r"
Sub Test()
    result = Str((x + y) * z)
End Sub
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/str");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn str_msgbox() {
        let source = r#"
Sub Test()
    MsgBox "Value: " & Str(total)
End Sub
"#;
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/str");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn str_property_assignment() {
        let source = r"
Sub Test()
    obj.Caption = Str(count)
End Sub
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/str");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn str_error_handling() {
        let source = r#"
Sub Test()
    On Error Resume Next
    result = Str(varValue)
    If Err.Number <> 0 Then
        result = ""
    End If
End Sub
"#;
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/str");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn str_class_usage() {
        let source = r"
Sub Test()
    Set obj = New MyClass
    obj.SetValue Str(counter)
End Sub
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/str");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn str_currency_conversion() {
        let source = r"
Sub Test()
    Dim amount As Currency
    amount = 123.45
    result = Str(amount)
End Sub
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/str");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn str_csv_building() {
        let source = r#"
Sub Test()
    csvLine = LTrim$(Str(id)) & "," & LTrim$(Str(qty)) & "," & LTrim$(Str(price))
End Sub
"#;
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/str");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }
}
