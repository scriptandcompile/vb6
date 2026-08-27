#[cfg(test)]
mod tests {
    const SNAPSHOT_PATH: &str = "../../../snapshots/parsers/cst/library/string/lenb";

    use vb6parse::ConcreteSyntaxTree;

    #[test]
    fn lenb_simple() {
        let source = r#"
Sub Test()
    Dim size As Long
    size = LenB("Hello")
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
    fn lenb_with_variable() {
        let source = r"
Sub Test()
    Dim text As String
    Dim size As Long
    size = LenB(text)
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
    fn lenb_comparison() {
        let source = r"
Sub Test()
    Dim text As String
    If LenB(text) > 100 Then
        ' Do something
    End If
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
    fn lenb_buffer_size() {
        let source = r"
Sub Test()
    Dim buffer As String
    Dim bufferSize As Long
    buffer = Space$(256)
    bufferSize = LenB(buffer)
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
    fn lenb_in_expression() {
        let source = r"
Sub Test()
    Dim s As String
    Dim total As Long
    total = LenB(s) * 2 + 10
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
    fn lenb_array_element() {
        let source = r"
Sub Test()
    Dim arr(10) As String
    Dim size As Long
    size = LenB(arr(5))
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
    fn lenb_loop_condition() {
        let source = r"
Sub Test()
    Dim data As String
    Dim i As Long
    For i = 1 To LenB(data) Step 2
        ' Process bytes
    Next i
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
    fn lenb_validation() {
        let source = r"
Sub Test()
    Dim packet As String
    If LenB(packet) < 64 Then
        Exit Sub
    End If
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
    fn lenb_with_midb() {
        let source = r"
Sub Test()
    Dim data As String
    Dim chunk As String
    Dim i As Long
    For i = 1 To LenB(data) Step 100
        chunk = MidB(data, i, 100)
    Next i
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
    fn lenb_empty_check() {
        let source = r"
Sub Test()
    Dim s As String
    Dim isEmpty As Boolean
    isEmpty = (LenB(s) = 0)
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
    fn lenb_function_parameter() {
        let source = r"
Function GetByteSize(text As String) As Long
    GetByteSize = LenB(text)
End Function
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
    fn lenb_nested_function() {
        let source = r#"
Sub Test()
    Dim size As Long
    size = LenB(UCase$("hello"))
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
    fn lenb_select_case() {
        let source = r"
Sub Test()
    Dim data As String
    Select Case LenB(data)
        Case 0
            ' Empty
        Case Is > 1000
            ' Large
        Case Else
            ' Normal
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
    fn lenb_binary_data() {
        let source = r"
Sub Test()
    Dim binaryData As String
    Dim headerSize As Long
    Dim payloadSize As Long
    headerSize = 16
    payloadSize = LenB(binaryData) - headerSize
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
    fn lenb_redim_array() {
        let source = r"
Sub Test()
    Dim data As String
    Dim bytes() As Byte
    ReDim bytes(LenB(data) - 1)
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
    fn lenb_type_field() {
        let source = r"
Type BinaryRecord
    Data As String
    ID As String
End Type

Sub Test()
    Dim rec As BinaryRecord
    Dim size As Long
    size = LenB(rec.Data)
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
    fn lenb_concatenation_result() {
        let source = r"
Sub Test()
    Dim s1 As String
    Dim s2 As String
    Dim totalSize As Long
    totalSize = LenB(s1 & s2)
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
    fn lenb_multiple_calls() {
        let source = r"
Sub Test()
    Dim a As String
    Dim b As String
    Dim result As Long
    result = LenB(a) + LenB(b)
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
    fn lenb_print_statement() {
        let source = r"
Sub Test()
    Dim text As String
    Debug.Print LenB(text)
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
    fn lenb_division() {
        let source = r"
Sub Test()
    Dim unicodeStr As String
    Dim charCount As Long
    charCount = LenB(unicodeStr) / 2
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
}
