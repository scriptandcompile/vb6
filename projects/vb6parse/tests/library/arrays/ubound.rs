const SNAPSHOT_PATH: &str = "../../../snapshots/parsers/cst/library/arrays/ubound";

use vb6parse::ConcreteSyntaxTree;

#[test]
fn ubound_basic() {
    let source = r"
Sub Test()
    upper = UBound(myArray)
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
fn ubound_variable_assignment() {
    let source = r"
Sub Test()
    Dim maxIndex As Long
    maxIndex = UBound(values)
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
fn ubound_with_dimension() {
    let source = r"
Sub Test()
    rows = UBound(matrix, 1)
    cols = UBound(matrix, 2)
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
fn ubound_for_loop() {
    let source = r"
Sub Test()
    For i = LBound(arr) To UBound(arr)
        Process arr(i)
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
fn ubound_function_return() {
    let source = r"
Function GetArraySize(arr() As Variant) As Long
    GetArraySize = UBound(arr) - LBound(arr) + 1
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
fn ubound_if_statement() {
    let source = r"
Sub Test()
    If index > UBound(data) Then
        Err.Raise 9
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
fn ubound_msgbox() {
    let source = r#"
Sub Test()
    MsgBox "Array size: " & (UBound(arr) - LBound(arr) + 1)
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
#[allow(clippy::too_many_lines)]
fn ubound_select_case() {
    let source = r#"
Sub Test()
    Select Case UBound(items)
        Case 0 To 10
            category = "Small"
        Case Else
            category = "Large"
    End Select
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
fn ubound_redim() {
    let source = r"
Sub Test()
    ReDim Preserve arr(LBound(arr) To UBound(arr) + 1)
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
fn ubound_function_argument() {
    let source = r"
Sub Test()
    Call ProcessArray(data, UBound(data))
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
fn ubound_comparison() {
    let source = r"
Sub Test()
    If UBound(arr1) > UBound(arr2) Then
        larger = arr1
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
fn ubound_debug_print() {
    let source = r#"
Sub Test()
    Debug.Print "Upper bound: " & UBound(values)
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
fn ubound_do_while() {
    let source = r"
Sub Test()
    Do While i <= UBound(items)
        Process items(i)
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
fn ubound_do_until() {
    let source = r"
Sub Test()
    Do Until i > UBound(data)
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
fn ubound_while_wend() {
    let source = r"
Sub Test()
    While idx <= UBound(arr)
        idx = idx + 1
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
fn ubound_iif() {
    let source = r#"
Sub Test()
    size = IIf(UBound(arr) > 10, "Large", "Small")
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
fn ubound_with_statement() {
    let source = r"
Sub Test()
    With arrayManager
        .MaxIndex = UBound(.Data)
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
fn ubound_parentheses() {
    let source = r"
Sub Test()
    result = (UBound(arr) + 1) * 2
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
fn ubound_error_handling() {
    let source = r"
Sub Test()
    On Error Resume Next
    maxIdx = UBound(dynamicArray)
    If Err.Number <> 0 Then
        maxIdx = -1
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
fn ubound_property_assignment() {
    let source = r"
Sub Test()
    obj.UpperBound = UBound(obj.Items)
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
fn ubound_concatenation() {
    let source = r#"
Sub Test()
    message = "Array has " & UBound(arr) + 1 & " elements"
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
fn ubound_arithmetic() {
    let source = r"
Sub Test()
    lastIndex = UBound(values) - 1
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
fn ubound_print_statement() {
    let source = r"
Sub Test()
    Print #1, UBound(data)
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
fn ubound_class_usage() {
    let source = r"
Sub Test()
    Set manager = New ArrayManager
    manager.Size = UBound(manager.Data) + 1
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
fn ubound_array_bounds() {
    let source = r"
Sub Test()
    lastElement = arr(UBound(arr))
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
fn ubound_elseif() {
    let source = r#"
Sub Test()
    If UBound(arr) = 0 Then
        result = "Empty"
    ElseIf UBound(arr) < 10 Then
        result = "Small"
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
#[allow(clippy::too_many_lines)]
fn ubound_nested_arrays() {
    let source = r"
Sub Test()
    For i = LBound(matrix, 1) To UBound(matrix, 1)
        For j = LBound(matrix, 2) To UBound(matrix, 2)
            total = total + matrix(i, j)
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
#[allow(clippy::too_many_lines)]
fn ubound_paramarray() {
    let source = r"
Function Sum(ParamArray values() As Variant) As Double
    For i = LBound(values) To UBound(values)
        total = total + values(i)
    Next i
    Sum = total
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
