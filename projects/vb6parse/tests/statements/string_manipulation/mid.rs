use vb6parse::ConcreteSyntaxTree;

#[test]
fn mid_simple() {
    let source = r#"
Sub Test()
    Mid(text, 5, 3) = "abc"
End Sub
"#;
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings
        .set_snapshot_path("../../../snapshots/syntax/statements/string_manipulation/mid");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn mid_at_module_level() {
    let source = r#"Mid(globalStr, 1, 5) = "START""#;
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings
        .set_snapshot_path("../../../snapshots/syntax/statements/string_manipulation/mid");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn mid_without_length() {
    let source = r"
Sub Test()
    Mid(s, 10) = replacement
End Sub
";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings
        .set_snapshot_path("../../../snapshots/syntax/statements/string_manipulation/mid");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn mid_with_expressions() {
    let source = r"
Sub Test()
    Mid(arr(i), startPos + 1, Len(newStr)) = newStr
End Sub
";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings
        .set_snapshot_path("../../../snapshots/syntax/statements/string_manipulation/mid");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn mid_preserves_whitespace() {
    let source = "    Mid  (  myString  ,  3  ,  2  )  =  \"XX\"    \n";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings
        .set_snapshot_path("../../../snapshots/syntax/statements/string_manipulation/mid");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn mid_with_comment() {
    let source = r"
Sub Test()
    Mid(buffer, pos, 10) = data ' Replace 10 characters
End Sub
";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings
        .set_snapshot_path("../../../snapshots/syntax/statements/string_manipulation/mid");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn mid_in_if_statement() {
    let source = r#"
Sub Test()
    If needsUpdate Then
        Mid(statusText, 1, 7) = "UPDATED"
    End If
End Sub
"#;
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings
        .set_snapshot_path("../../../snapshots/syntax/statements/string_manipulation/mid");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn mid_inline_if() {
    let source = r#"
Sub Test()
    If valid Then Mid(s, 1, 1) = "A"
End Sub
"#;
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings
        .set_snapshot_path("../../../snapshots/syntax/statements/string_manipulation/mid");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn multiple_mid_statements() {
    let source = r#"
Sub ReplaceChars()
    Mid(line1, 5) = "HELLO"
    Mid(line2, 1, 3) = "ABC"
    Mid(line3, 2, 4) = "TEST"
End Sub
"#;
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings
        .set_snapshot_path("../../../snapshots/syntax/statements/string_manipulation/mid");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn mid_replace_example() {
    let source = r#"
Sub Test()
    Dim s As String
    s = "Hello World"
    Mid(s, 7, 5) = "VB6!!"
    ' s now contains "Hello VB6!!"
End Sub
"#;
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings
        .set_snapshot_path("../../../snapshots/syntax/statements/string_manipulation/mid");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn mid_with_member_access() {
    let source = r"
Sub Test()
    Mid(obj.Name, 1, 10) = newName
End Sub
";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings
        .set_snapshot_path("../../../snapshots/syntax/statements/string_manipulation/mid");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn mid_with_concatenation() {
    let source = r"
Sub Test()
    Mid(fullText, pos, 5) = prefix & suffix
End Sub
";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings
        .set_snapshot_path("../../../snapshots/syntax/statements/string_manipulation/mid");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}
