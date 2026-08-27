const SNAPSHOT_PATH: &str = "../../../snapshots/parsers/cst/library/arrays/lbound";

use vb6parse::ConcreteSyntaxTree;

#[test]
fn lbound_basic() {
    let source = r"
Sub Test()
    result = LBound(myArray)
End Sub
";
    let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn lbound_with_dimension() {
    let source = r"
Sub Test()
    result = LBound(matrix, 2)
End Sub
";
    let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn lbound_for_loop() {
    let source = r"
Sub Test()
    Dim i As Long
    For i = LBound(arr) To UBound(arr)
        Debug.Print arr(i)
    Next i
End Sub
";
    let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn lbound_function_return() {
    let source = r"
Function GetLowerBound(arr As Variant) As Long
    GetLowerBound = LBound(arr)
End Function
";
    let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn lbound_if_statement() {
    let source = r"
Sub Test()
    If LBound(data) = 0 Then
        ProcessZeroBased
    End If
End Sub
";
    let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn lbound_debug_print() {
    let source = r#"
Sub Test()
    Debug.Print "Lower bound: " & LBound(items)
End Sub
"#;
    let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn lbound_msgbox() {
    let source = r#"
Sub Test()
    MsgBox "Array starts at: " & LBound(values)
End Sub
"#;
    let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn lbound_variable_assignment() {
    let source = r"
Sub Test()
    Dim lb As Long
    lb = LBound(myArray)
End Sub
";
    let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn lbound_property_assignment() {
    let source = r"
Sub Test()
    obj.LowerBound = LBound(obj.Data)
End Sub
";
    let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn lbound_arithmetic() {
    let source = r"
Sub Test()
    size = UBound(arr) - LBound(arr) + 1
End Sub
";
    let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn lbound_in_class() {
    let source = r"
Private Sub Class_Initialize()
    m_lowerBound = LBound(m_data)
End Sub
";
    let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn lbound_with_statement() {
    let source = r"
Sub Test()
    With container
        .Start = LBound(.Items)
    End With
End Sub
";
    let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn lbound_function_argument() {
    let source = r"
Sub Test()
    Call ProcessBound(LBound(data))
End Sub
";
    let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn lbound_select_case() {
    let source = r"
Sub Test()
    Select Case LBound(arr)
        Case 0
            ZeroBasedProcessing
        Case 1
            OneBasedProcessing
    End Select
End Sub
";
    let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
#[allow(clippy::too_many_lines)]
fn lbound_elseif() {
    let source = r"
Sub Test()
    If LBound(arr) = 0 Then
        HandleZero
    ElseIf LBound(arr) = 1 Then
        HandleOne
    End If
End Sub
";
    let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn lbound_concatenation() {
    let source = r#"
Sub Test()
    info = "Range: " & LBound(arr) & " to " & UBound(arr)
End Sub
"#;
    let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn lbound_parentheses() {
    let source = r"
Sub Test()
    result = (LBound(values))
End Sub
";
    let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn lbound_array_assignment() {
    let source = r"
Sub Test()
    bounds(0) = LBound(data)
End Sub
";
    let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn lbound_collection_add() {
    let source = r"
Sub Test()
    bounds.Add LBound(arrays(i))
End Sub
";
    let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn lbound_comparison() {
    let source = r#"
Sub Test()
    If LBound(arr1) = LBound(arr2) Then
        MsgBox "Same lower bound"
    End If
End Sub
"#;
    let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn lbound_nested_call() {
    let source = r"
Sub Test()
    result = CStr(LBound(data))
End Sub
";
    let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
#[allow(clippy::too_many_lines)]
fn lbound_while_wend() {
    let source = r"
Sub Test()
    i = LBound(arr)
    While i <= UBound(arr)
        i = i + 1
    Wend
End Sub
";
    let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
#[allow(clippy::too_many_lines)]
fn lbound_do_while() {
    let source = r"
Sub Test()
    i = LBound(values)
    Do While i <= UBound(values)
        i = i + 1
    Loop
End Sub
";
    let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
#[allow(clippy::too_many_lines)]
fn lbound_do_until() {
    let source = r"
Sub Test()
    i = LBound(items)
    Do Until i > UBound(items)
        i = i + 1
    Loop
End Sub
";
    let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn lbound_redim() {
    let source = r"
Sub Test()
    Dim lb As Long
    lb = LBound(arr)
    ReDim Preserve arr(lb To lb + 20)
End Sub
";
    let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
#[allow(clippy::too_many_lines)]
fn lbound_multi_dimensional() {
    let source = r"
Sub Test()
    For i = LBound(grid, 1) To UBound(grid, 1)
        For j = LBound(grid, 2) To UBound(grid, 2)
            grid(i, j) = 0
        Next j
    Next i
End Sub
";
    let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn lbound_iif() {
    let source = r#"
Sub Test()
    start = IIf(LBound(arr) = 0, "Zero-based", "One-based")
End Sub
"#;
    let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}
