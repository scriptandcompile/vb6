#[cfg(test)]
mod tests {
    use crate::*;
    use vb6parse::ConcreteSyntaxTree;

    #[test]
    fn rtrim_dollar_simple() {
        let source = r#"
Sub Main()
    result = RTrim$("Hello   ")
End Sub
"#;
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/rtrim_dollar");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn rtrim_dollar_assignment() {
        let source = r"
Sub Main()
    Dim cleaned As String
    cleaned = RTrim$(userInput)
End Sub
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/rtrim_dollar");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn rtrim_dollar_variable() {
        let source = r#"
Sub Main()
    Dim text As String
    Dim result As String
    text = "Sample  "
    result = RTrim$(text)
End Sub
"#;
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/rtrim_dollar");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn rtrim_dollar_display_format() {
        let source = r#"
Sub DisplayData()
    Dim dataField As String
    dataField = "Value    "
    Debug.Print "|" & RTrim$(dataField) & "|"
End Sub
"#;
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/rtrim_dollar");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn rtrim_dollar_database_field() {
        let source = r"
Function GetFieldValue(fieldValue As String) As String
    GetFieldValue = RTrim$(fieldValue)
End Function
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/rtrim_dollar");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn rtrim_dollar_in_condition() {
        let source = r#"
Sub Main()
    If RTrim$(dataValue) = "Expected" Then
        Debug.Print "Match found"
    End If
End Sub
"#;
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/rtrim_dollar");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn rtrim_dollar_fixed_width() {
        let source = r"
Function ParseFixedField(dataLine As String, startPos As Integer, fieldWidth As Integer) As String
    Dim rawField As String
    rawField = Mid$(dataLine, startPos, fieldWidth)
    ParseFixedField = RTrim$(rawField)
End Function
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/rtrim_dollar");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn rtrim_dollar_comparison() {
        let source = r"
Function CompareValues(value1 As String, value2 As String) As Boolean
    CompareValues = (RTrim$(value1) = RTrim$(value2))
End Function
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/rtrim_dollar");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn rtrim_dollar_array_cleanup() {
        let source = r"
Sub CleanStringArray(arr() As String)
    Dim i As Integer
    For i = LBound(arr) To UBound(arr)
        arr(i) = RTrim$(arr(i))
    Next i
End Sub
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/rtrim_dollar");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn rtrim_dollar_multiple_uses() {
        let source = r"
Sub ProcessData()
    Dim firstName As String
    Dim lastName As String
    firstName = RTrim$(rawFirst)
    lastName = RTrim$(rawLast)
End Sub
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/rtrim_dollar");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn rtrim_dollar_select_case() {
        let source = r#"
Sub Main()
    Select Case RTrim$(status)
        Case "Active"
            Debug.Print "Active record"
        Case "Inactive"
            Debug.Print "Inactive record"
    End Select
End Sub
"#;
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/rtrim_dollar");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn rtrim_dollar_concatenation() {
        let source = r#"
Sub Main()
    Dim output As String
    output = "Name: " & RTrim$(nameField)
End Sub
"#;
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/rtrim_dollar");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn rtrim_dollar_with_ltrim() {
        let source = r"
Sub Main()
    Dim fullyClean As String
    fullyClean = LTrim$(RTrim$(input))
End Sub
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/rtrim_dollar");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn rtrim_dollar_report_format() {
        let source = r"
Function FormatReportLine(textLabel As String, value As String) As String
    Dim paddedLabel As String
    paddedLabel = textLabel & Space(30)
    FormatReportLine = Left$(RTrim$(paddedLabel), 30) & value
End Function
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/rtrim_dollar");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn rtrim_dollar_logging() {
        let source = r#"
Sub LogMessage(message As String)
    Dim cleanMsg As String
    cleanMsg = RTrim$(message)
    Debug.Print Now & " - " & cleanMsg
End Sub
"#;
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/rtrim_dollar");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn rtrim_dollar_in_function() {
        let source = r"
Function CleanInput(userInput As String) As String
    CleanInput = RTrim$(userInput)
End Function
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/rtrim_dollar");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn rtrim_dollar_config_value() {
        let source = r#"
Function GetConfigValue(key As String) As String
    Dim rawValue As String
    rawValue = GetINIString("Settings", key, "")
    GetConfigValue = RTrim$(rawValue)
End Function
"#;
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/rtrim_dollar");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn rtrim_dollar_empty_check() {
        let source = r#"
Sub Main()
    If Len(RTrim$(input)) > 0 Then
        Debug.Print "Has content"
    End If
End Sub
"#;
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/rtrim_dollar");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn rtrim_dollar_file_processing() {
        let source = r"
Function ReadCleanLine(fileNum As Integer) As String
    Dim rawLine As String
    Line Input #fileNum, rawLine
    ReadCleanLine = RTrim$(rawLine)
End Function
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/rtrim_dollar");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn rtrim_dollar_loop_processing() {
        let source = r"
Sub ProcessLines()
    Dim i As Integer
    Dim cleanLine As String
    For i = 1 To 10
        cleanLine = RTrim$(lines(i))
        Debug.Print cleanLine
    Next i
End Sub
";
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert_eq!(failures.len(), 0, "Expected no parse failures.");
        let cst = cst_opt.expect("CST should be parsed");

        let tree = cst.to_serializable();

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../../../snapshots/tests/library/string/rtrim_dollar");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
        insta::assert_yaml_snapshot!(tree);
    }
}
