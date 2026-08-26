#[cfg(test)]
mod tests {
    use crate::*;
    use vb6parse::ConcreteSyntaxTree;

    #[test]
    fn join_basic() {
        let source = r"
Sub Test()
    result = Join(myArray)
End Sub
";
        let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        let cst = cst_opt.expect("CST should be parsed");
        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/arrays/join");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn join_with_delimiter() {
        let source = r#"
Sub Test()
    result = Join(items, ", ")
End Sub
"#;
        let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        let cst = cst_opt.expect("CST should be parsed");
        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/arrays/join");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn join_if_statement() {
        let source = r#"
Sub Test()
    If Len(Join(parts, "-")) > 0 Then
        ProcessResult
    End If
End Sub
"#;
        let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        let cst = cst_opt.expect("CST should be parsed");
        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/arrays/join");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn join_function_return() {
        let source = r#"
Function GetCSV(fields As Variant) As String
    GetCSV = Join(fields, ",")
End Function
"#;
        let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        let cst = cst_opt.expect("CST should be parsed");
        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/arrays/join");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn join_debug_print() {
        let source = r#"
Sub Test()
    Debug.Print Join(values, " | ")
End Sub
"#;
        let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        let cst = cst_opt.expect("CST should be parsed");
        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/arrays/join");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn join_msgbox() {
        let source = r"
Sub Test()
    MsgBox Join(names, vbCrLf)
End Sub
";
        let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        let cst = cst_opt.expect("CST should be parsed");
        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/arrays/join");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn join_variable_assignment() {
        let source = r#"
Sub Test()
    Dim combined As String
    combined = Join(words, " ")
End Sub
"#;
        let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        let cst = cst_opt.expect("CST should be parsed");
        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/arrays/join");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn join_property_assignment() {
        let source = r"
Sub Test()
    obj.DisplayText = Join(obj.Lines, vbCrLf)
End Sub
";
        let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        let cst = cst_opt.expect("CST should be parsed");
        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/arrays/join");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn join_concatenation() {
        let source = r#"
Sub Test()
    result = "Values: " & Join(data, ", ")
End Sub
"#;
        let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        let cst = cst_opt.expect("CST should be parsed");
        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/arrays/join");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn join_in_class() {
        let source = r#"
Private Sub Class_Initialize()
    m_text = Join(m_parts, "")
End Sub
"#;
        let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        let cst = cst_opt.expect("CST should be parsed");
        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/arrays/join");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn join_with_statement() {
        let source = r"
Sub Test()
    With builder
        .Output = Join(.Parts, .Delimiter)
    End With
End Sub
";
        let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        let cst = cst_opt.expect("CST should be parsed");
        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/arrays/join");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn join_function_argument() {
        let source = r"
Sub Test()
    Call ProcessText(Join(lines, vbCrLf))
End Sub
";
        let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        let cst = cst_opt.expect("CST should be parsed");
        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/arrays/join");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn join_select_case() {
        let source = r#"
Sub Test()
    Select Case Join(tags, ",")
        Case "A,B,C"
            HandleABC
        Case Else
            HandleOther
    End Select
End Sub
"#;
        let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        let cst = cst_opt.expect("CST should be parsed");
        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/arrays/join");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn join_for_loop() {
        let source = r#"
Sub Test()
    Dim i As Integer
    For i = 0 To UBound(dataRows)
        lines(i) = Join(dataRows(i), ",")
    Next i
End Sub
"#;
        let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        let cst = cst_opt.expect("CST should be parsed");
        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/arrays/join");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn join_elseif() {
        let source = r#"
Sub Test()
    If format = "csv" Then
        output = Join(data, ",")
    ElseIf format = "tsv" Then
        output = Join(data, vbTab)
    End If
End Sub
"#;
        let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        let cst = cst_opt.expect("CST should be parsed");
        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/arrays/join");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn join_iif() {
        let source = r#"
Sub Test()
    result = IIf(useComma, Join(arr, ","), Join(arr, ";"))
End Sub
"#;
        let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        let cst = cst_opt.expect("CST should be parsed");
        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/arrays/join");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn join_parentheses() {
        let source = r#"
Sub Test()
    result = (Join(values, "-"))
End Sub
"#;
        let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        let cst = cst_opt.expect("CST should be parsed");
        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/arrays/join");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn join_array_assignment() {
        let source = r#"
Sub Test()
    csvRows(i) = Join(fields(i), ",")
End Sub
"#;
        let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        let cst = cst_opt.expect("CST should be parsed");
        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/arrays/join");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn join_collection_add() {
        let source = r"
Sub Test()
    lines.Add Join(row, vbTab)
End Sub
";
        let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        let cst = cst_opt.expect("CST should be parsed");
        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/arrays/join");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn join_comparison() {
        let source = r#"
Sub Test()
    If Join(actual, ",") = Join(expected, ",") Then
        MsgBox "Match!"
    End If
End Sub
"#;
        let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        let cst = cst_opt.expect("CST should be parsed");
        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/arrays/join");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn join_nested_call() {
        let source = r#"
Sub Test()
    result = UCase(Join(names, " "))
End Sub
"#;
        let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        let cst = cst_opt.expect("CST should be parsed");
        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/arrays/join");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn join_while_wend() {
        let source = r#"
Sub Test()
    While Len(Join(buffer, "")) < maxLen
        buffer.Add GetNextItem()
    Wend
End Sub
"#;
        let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        let cst = cst_opt.expect("CST should be parsed");
        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/arrays/join");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn join_do_while() {
        let source = r#"
Sub Test()
    Do While Len(Join(parts, "")) > 0
        ProcessParts
    Loop
End Sub
"#;
        let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        let cst = cst_opt.expect("CST should be parsed");
        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/arrays/join");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn join_do_until() {
        let source = r#"
Sub Test()
    Do Until Join(fields, "") = ""
        fields = GetNextFields()
    Loop
End Sub
"#;
        let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        let cst = cst_opt.expect("CST should be parsed");
        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/arrays/join");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn join_with_split() {
        let source = r#"
Sub Test()
    parts = Split(text, "-")
    result = Join(parts, " ")
End Sub
"#;
        let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        let cst = cst_opt.expect("CST should be parsed");
        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/arrays/join");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn join_csv_builder() {
        let source = r#"
Function BuildCSV(fields As Variant) As String
    BuildCSV = Join(fields, ",")
End Function
"#;
        let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        let cst = cst_opt.expect("CST should be parsed");
        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/arrays/join");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn join_empty_delimiter() {
        let source = r#"
Sub Test()
    concatenated = Join(chars, "")
End Sub
"#;
        let (cst_opt, _failure) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        let cst = cst_opt.expect("CST should be parsed");
        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/arrays/join");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }
}
