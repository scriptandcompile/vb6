use vb6parse::ConcreteSyntaxTree;

#[test]
fn midb_simple() {
    let source = r#"
Sub Test()
    MidB(text, 5, 3) = "abc"
End Sub
"#;
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings
        .set_snapshot_path("../../../snapshots/syntax/statements/string_manipulation/midb");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn midb_at_module_level() {
    let source = r#"MidB(globalStr, 1, 5) = "START""#;
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings
        .set_snapshot_path("../../../snapshots/syntax/statements/string_manipulation/midb");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn midb_without_length() {
    let source = r"
Sub Test()
    MidB(s, 10) = replacement
End Sub
";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings
        .set_snapshot_path("../../../snapshots/syntax/statements/string_manipulation/midb");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn midb_with_expressions() {
    let source = r"
Sub Test()
    MidB(arr(i), startPos + 1, LenB(newStr)) = newStr
End Sub
";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings
        .set_snapshot_path("../../../snapshots/syntax/statements/string_manipulation/midb");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn midb_preserves_whitespace() {
    let source = "    MidB  (  myString  ,  3  ,  2  )  =  \"XX\"    \n";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings
        .set_snapshot_path("../../../snapshots/syntax/statements/string_manipulation/midb");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn midb_with_comment() {
    let source = r"
Sub Test()
    MidB(buffer, pos, 10) = data ' Replace 10 bytes
End Sub
";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings
        .set_snapshot_path("../../../snapshots/syntax/statements/string_manipulation/midb");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn midb_in_if_statement() {
    let source = r#"
Sub Test()
    If needsUpdate Then
        MidB(statusText, 1, 7) = "UPDATED"
    End If
End Sub
"#;
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings
        .set_snapshot_path("../../../snapshots/syntax/statements/string_manipulation/midb");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn midb_inline_if() {
    let source = r#"
Sub Test()
    If valid Then MidB(s, 1, 1) = "A"
End Sub
"#;
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings
        .set_snapshot_path("../../../snapshots/syntax/statements/string_manipulation/midb");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn multiple_midb_statements() {
    let source = r#"
Sub ReplaceBytes()
    MidB(line1, 5) = "HELLO"
    MidB(line2, 1, 3) = "ABC"
    MidB(line3, 2, 4) = "TEST"
End Sub
"#;
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings
        .set_snapshot_path("../../../snapshots/syntax/statements/string_manipulation/midb");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn midb_dbcs_example() {
    let source = r#"
Sub Test()
    Dim dbcsStr As String
    MidB(dbcsStr, 1, 2) = "XX"
End Sub
"#;
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings
        .set_snapshot_path("../../../snapshots/syntax/statements/string_manipulation/midb");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn midb_with_member_access() {
    let source = r"
Sub Test()
    MidB(obj.Data, 1, 10) = newData
End Sub
";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings
        .set_snapshot_path("../../../snapshots/syntax/statements/string_manipulation/midb");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn midb_with_concatenation() {
    let source = r"
Sub Test()
    MidB(fullText, pos, 5) = prefix & suffix
End Sub
";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings
        .set_snapshot_path("../../../snapshots/syntax/statements/string_manipulation/midb");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}
