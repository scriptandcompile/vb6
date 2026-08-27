//! Tests for With statement parsing.
//!
//! This module verifies parsing of VB6 With blocks for simplified object property access:
//! - `With...End With` - Reference an object multiple times without repeating its name

#[cfg(test)]
mod tests {
    const SNAPSHOT_PATH: &str = "../../snapshots/parsers/cst/objects/with_block";

    use vb6parse::ConcreteSyntaxTree;

    #[test]
    fn with_statement_simple() {
        let source = r"
Sub Test()
    With myObject
        .Property = 123
    End With
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
    fn with_statement_nested_property() {
        let source = r"
Sub Test()
    With myObject
        .NestedObject.Property = 123
    End With
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
    fn with_statement_method_call() {
        let source = r#"
Sub Test()
    With myObject
        .Method "argument"
    End With
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
    fn with_statement_nested() {
        let source = r"
Sub Test()
    With obj1
        .Property1 = 1
        With .NestedObj
            .Property2 = 2
        End With
    End With
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
    fn with_statement_multiple_properties() {
        let source = r"
Sub Test()
    With myObject
        .Property1 = 1
        .Property2 = 2
        .Property3 = 3
    End With
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
    fn with_statement_with_if() {
        let source = r"
Sub Test()
    With myObject
        If .Property > 0 Then
            .Method
        End If
    End With
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
    fn with_statement_with_loop() {
        let source = r"
Sub Test()
    With myObject
        For i = 1 To 10
            .Process i
        Next i
    End With
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
    fn with_statement_array_access() {
        let source = r"
Sub Test()
    With myObject(index)
        .Property = 123
    End With
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
    fn with_statement_function_result() {
        let source = r"
Sub Test()
    With GetObject()
        .Property = 123
    End With
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
    fn with_statement_empty() {
        let source = r"
Sub Test()
    With myObject
    End With
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
    fn with_statement_sequential() {
        let source = r"
Sub Test()
    With obj1
        .Property = 1
    End With
    With obj2
        .Property = 2
    End With
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
    fn with_statement_preserves_whitespace() {
        let source = r"
Sub Test()
    With   myObject
        .Property   =   123
    End   With
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
    fn with_statement_new_object() {
        let source = r"
Sub Test()
    With New MyClass
        .Property = 123
    End With
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
    fn with_statement_at_module_level() {
        let source = r"
With globalObject
    .Property = 123
End With
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
