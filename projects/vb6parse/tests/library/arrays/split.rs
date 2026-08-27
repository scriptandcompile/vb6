const SNAPSHOT_PATH: &str = "../../../snapshots/parsers/cst/library/arrays/split";

use vb6parse::ConcreteSyntaxTree;

#[test]
fn split_basic() {
    let source = r#"
Sub Test()
    Dim parts() As String
    parts = Split("a,b,c", ",")
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
fn split_with_variable() {
    let source = r#"
Sub Test()
    Dim text As String
    Dim delimiter As String
    Dim result() As String
    text = "one,two,three"
    delimiter = ","
    result = Split(text, delimiter)
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
fn split_default_delimiter() {
    let source = r#"
Sub Test()
    Dim words() As String
    words = Split("The quick brown fox")
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
fn split_with_limit() {
    let source = r#"
Sub Test()
    Dim items() As String
    items = Split("a,b,c,d,e", ",", 3)
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
fn split_if_statement() {
    let source = r#"
Sub Test()
    If UBound(Split(text, ",")) > 0 Then
        MsgBox "Multiple items"
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
fn split_function_return() {
    let source = r#"
Function ParseCSV(line As String) As String()
    ParseCSV = Split(line, ",")
End Function
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
fn split_variable_assignment() {
    let source = r#"
Sub Test()
    Dim fields() As String
    fields = Split(csvLine, ",")
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
fn split_array_access() {
    let source = r#"
Sub Test()
    Dim parts() As String
    Dim firstPart As String
    parts = Split("a,b,c", ",")
    firstPart = parts(0)
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
fn split_in_loop() {
    let source = r#"
Sub Test()
    Dim parts() As String
    Dim i As Integer
    parts = Split(data, ",")
    For i = 0 To UBound(parts)
        Debug.Print parts(i)
    Next i
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
fn split_class_usage() {
    let source = r"
Class Parser
    Public Function GetFields(line As String) As String()
        GetFields = Split(line, vbTab)
    End Function
End Class
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
fn split_with_statement() {
    let source = r"
Sub Test()
    With parser
        Dim data() As String
        data = Split(.Text, .Delimiter)
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
#[allow(clippy::too_many_lines)]
fn split_elseif() {
    let source = r#"
Sub Test()
    Dim arr() As String
    If delimiter = "," Then
        arr = Split(text, ",")
    ElseIf delimiter = ";" Then
        arr = Split(text, ";")
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
fn split_select_case() {
    let source = r#"
Sub Test()
    Dim parts() As String
    Select Case fileType
        Case "CSV"
            parts = Split(line, ",")
        Case "TSV"
            parts = Split(line, vbTab)
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
#[allow(clippy::too_many_lines)]
fn split_do_while() {
    let source = r#"
Sub Test()
    Do While lineNum <= 10
        Dim fields() As String
        fields = Split(lines(lineNum), ",")
        lineNum = lineNum + 1
    Loop
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
fn split_do_until() {
    let source = r#"
Sub Test()
    Do Until EOF(1)
        Dim data() As String
        Line Input #1, currentLine
        data = Split(currentLine, ",")
    Loop
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
fn split_while_wend() {
    let source = r#"
Sub Test()
    While i < count
        Dim tokens() As String
        tokens = Split(lines(i), " ")
        i = i + 1
    Wend
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
fn split_ubound_check() {
    let source = r#"
Sub Test()
    Dim arr() As String
    arr = Split(text, ",")
    If UBound(arr) >= 0 Then
        MsgBox arr(0)
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
fn split_lbound_ubound() {
    let source = r#"
Sub Test()
    Dim parts() As String
    Dim i As Integer
    parts = Split(data, "|")
    For i = LBound(parts) To UBound(parts)
        Debug.Print parts(i)
    Next i
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
fn split_vbcrlf() {
    let source = r"
Sub Test()
    Dim lines() As String
    lines = Split(multilineText, vbCrLf)
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
fn split_nested_function() {
    let source = r#"
Sub Test()
    Dim count As Integer
    count = UBound(Split(text, ",")) + 1
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
fn split_join_combination() {
    let source = r#"
Sub Test()
    Dim parts() As String
    Dim result As String
    parts = Split(original, ",")
    result = Join(parts, ";")
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
fn split_trim_combination() {
    let source = r#"
Sub Test()
    Dim values() As String
    Dim i As Integer
    values = Split(data, ",")
    For i = 0 To UBound(values)
        values(i) = Trim(values(i))
    Next i
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
fn split_error_handling() {
    let source = r#"
Sub Test()
    On Error Resume Next
    Dim arr() As String
    arr = Split(text, delimiter)
    If Err.Number <> 0 Then
        MsgBox "Error"
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
fn split_on_error_goto() {
    let source = r#"
Sub Test()
    On Error GoTo ErrorHandler
    Dim fields() As String
    fields = Split(csvData, ",")
    Exit Sub
ErrorHandler:
    MsgBox "Error parsing data"
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
fn split_file_path() {
    let source = r#"
Sub Test()
    Dim pathParts() As String
    Dim fileName As String
    pathParts = Split(filePath, "\")
    fileName = pathParts(UBound(pathParts))
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
fn split_compare_parameter() {
    let source = r"
Sub Test()
    Dim items() As String
    items = Split(text, delimiter, -1, vbTextCompare)
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
fn split_key_value_parsing() {
    let source = r#"
Sub Test()
    Dim kvPair As String
    Dim parts() As String
    Dim key As String
    Dim value As String
    kvPair = "name=John"
    parts = Split(kvPair, "=", 2)
    key = parts(0)
    value = parts(1)
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
