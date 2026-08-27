use vb6parse::*;

const SNAPSHOT_PATH: &str = "../../snapshots/parsers/cst/control_flow/select";

#[test]
fn select_case_simple() {
    let source = r#"
Sub Test()
    Select Case x
        Case 1
            Debug.Print "One"
        Case 2
            Debug.Print "Two"
        Case 3
            Debug.Print "Three"
    End Select
End Sub
"#;

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn select_case_with_case_else() {
    let source = r#"
Sub Test()
    Select Case value
        Case 1
            result = "one"
        Case 2
            result = "two"
        Case Else
            result = "other"
    End Select
End Sub
"#;

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn select_case_multiple_values() {
    let source = r#"
Sub Test()
    Select Case dayOfWeek
        Case 1, 7
            Debug.Print "Weekend"
        Case 2, 3, 4, 5, 6
            Debug.Print "Weekday"
    End Select
End Sub
"#;

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn select_case_with_is() {
    let source = r#"
Sub Test()
    Select Case score
        Case Is >= 90
            grade = "A"
        Case Is >= 80
            grade = "B"
        Case Is >= 70
            grade = "C"
        Case Else
            grade = "F"
    End Select
End Sub
"#;

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn select_case_with_to() {
    let source = r#"
Sub Test()
    Select Case temperature
        Case 0 To 32
            status = "Freezing"
        Case 33 To 65
            status = "Cold"
        Case 66 To 85
            status = "Comfortable"
        Case 86 To 100
            status = "Hot"
    End Select
End Sub
"#;

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn select_case_string_comparison() {
    let source = r#"
Sub Test()
    Select Case userInput
        Case "yes", "y", "YES"
            DoSomething
        Case "no", "n", "NO"
            DoSomethingElse
        Case Else
            ShowError
    End Select
End Sub
"#;

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn select_case_nested() {
    let source = r"
Sub Test()
    Select Case x
        Case 1
            Select Case y
                Case 10
                    result = 11
                Case 20
                    result = 21
            End Select
        Case 2
            result = 2
    End Select
End Sub
";

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn select_case_with_loops() {
    let source = r#"
Sub Test()
    Select Case operation
        Case "add"
            For i = 1 To 10
                total = total + i
            Next i
        Case "multiply"
            For i = 1 To 10
                total = total * i
            Next i
    End Select
End Sub
"#;

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn select_case_with_if() {
    let source = r#"
Sub Test()
    Select Case category
        Case 1
            If value > 100 Then
                status = "high"
            Else
                status = "low"
            End If
        Case 2
            result = "category2"
    End Select
End Sub
"#;

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn select_case_empty_case() {
    let source = r"
Sub Test()
    Select Case x
        Case 1
        Case 2
            DoSomething
        Case 3
    End Select
End Sub
";

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn select_case_module_level() {
    let source = r#"
Public Sub ModuleLevelTest()
    Select Case globalVar
        Case 1
            result = "One"
        Case 2
            result = "Two"
    End Select
End Sub
"#;

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn select_case_with_function_call() {
    let source = r#"
Sub Test()
    Select Case GetValue()
        Case 1
            result = "one"
        Case 2
            result = "two"
    End Select
End Sub
"#;

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn select_case_case_is_relational() {
    let source = r#"
Sub Test()
    Select Case age
        Case Is < 13
            category = "child"
        Case Is < 20
            category = "teen"
        Case Is < 65
            category = "adult"
        Case Else
            category = "senior"
    End Select
End Sub
"#;

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn select_case_mixed_expressions() {
    let source = r#"
Sub Test()
    Select Case x
        Case 1 To 5, 10, 15 To 20
            result = "range"
        Case Is > 100
            result = "large"
        Case Else
            result = "other"
    End Select
End Sub
"#;

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn select_case_preserves_whitespace() {
    let source = r#"
Sub Test()
    Select Case x
        Case 1
            Debug.Print "test"
    End Select
End Sub
"#;

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}
