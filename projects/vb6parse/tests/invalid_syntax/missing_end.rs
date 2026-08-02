use vb6parse::parsers::cst::ConcreteSyntaxTree;

fn assert_failure_count_matches_snapshot(snapshot_dir: &str, snapshot_name: &str, actual: usize) {
    let primary_path = format!("snapshots/tests/invalid_syntax/{snapshot_dir}/{snapshot_name}.snap");
    let prefixed_path = format!(
        "snapshots/tests/invalid_syntax/{snapshot_dir}/invalid_syntax__{snapshot_dir}__{snapshot_name}.snap"
    );
    let path = if std::path::Path::new(&primary_path).exists() {
        primary_path
    } else {
        prefixed_path
    };
    let content = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("Failed to read snapshot file {path}: {err}"));
    let expected = content
        .lines()
        .filter(|line| line.trim_start().starts_with("- "))
        .count();

    assert_eq!(
        actual,
        expected,
        "Unexpected failure count for snapshot {snapshot_name}"
    );
}

/// Test missing End Sub statement
#[test]
fn missing_end_sub() {
    let source = r"
Sub TestSub()
    Dim x As Integer
    x = 10
";

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    // Report current behavior: parser may or may not report failures
    eprintln!("=== Failures for missing_end_sub ===");
    eprintln!("Number of failures: {}", failures.len());
    for failure in &failures {
        failure.eprint();
    }
    eprintln!("=== End Failures ===");

    // CST should still be parseable (resilient parsing)
    let cst = cst_opt.expect("CST should be present even with syntax errors");

    // Verify the CST structure
    let tree = cst.to_serializable();

    // Set up insta snapshot
    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/missing_end");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();

    // Snapshot the CST to ensure it's reasonable
    insta::assert_yaml_snapshot!("missing_end_sub_cst", tree);

    // Snapshot the failures to document current error reporting behavior
    let failure_messages: Vec<String> = failures.iter().map(|f| format!("{f:?}")).collect();
    assert_failure_count_matches_snapshot("missing_end", "missing_end_sub_failures", failures.len());
    insta::assert_yaml_snapshot!("missing_end_sub_failures", failure_messages);
}

/// Test missing End Sub when another Sub declaration starts.
#[test]
fn missing_end_sub_before_next_sub() {
    let source = r#"
Sub FirstSub()
    Dim x As Integer
    x = 10
Public Sub SecondSub()
    MsgBox "ok"
End Sub
"#;

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    eprintln!("=== Failures for missing_end_sub_before_next_sub ===");
    eprintln!("Number of failures: {}", failures.len());
    for failure in &failures {
        failure.eprint();
    }
    eprintln!("=== End Failures ===");

    let cst = cst_opt.expect("CST should be present even with syntax errors");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/missing_end");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();

    insta::assert_yaml_snapshot!("missing_end_sub_before_next_sub_cst", tree);

    let failure_messages: Vec<String> = failures.iter().map(|f| format!("{f:?}")).collect();
    assert_failure_count_matches_snapshot(
        "missing_end",
        "missing_end_sub_before_next_sub_failures",
        failures.len(),
    );
    insta::assert_yaml_snapshot!("missing_end_sub_before_next_sub_failures", failure_messages);
}

/// Test missing End Function statement
#[test]
fn missing_end_function() {
    let source = r"
Function Calculate(x As Integer) As Integer
    Calculate = x * 2
";

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    eprintln!("=== Failures for missing_end_function ===");
    eprintln!("Number of failures: {}", failures.len());
    for failure in &failures {
        failure.eprint();
    }
    eprintln!("=== End Failures ===");

    let cst = cst_opt.expect("CST should be present even with syntax errors");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/missing_end");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();

    insta::assert_yaml_snapshot!("missing_end_function_cst", tree);

    let failure_messages: Vec<String> = failures.iter().map(|f| format!("{f:?}")).collect();
    assert_failure_count_matches_snapshot("missing_end", "missing_end_function_failures", failures.len());
    insta::assert_yaml_snapshot!("missing_end_function_failures", failure_messages);
}

/// Test missing End Function when another Function declaration starts.
#[test]
fn missing_end_function_before_next_function() {
    let source = r#"
Function FirstFunction(x As Integer) As Integer
    FirstFunction = x * 2
Public Function SecondFunction(y As Integer) As Integer
    SecondFunction = y + 1
End Function
"#;

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    eprintln!("=== Failures for missing_end_function_before_next_function ===");
    eprintln!("Number of failures: {}", failures.len());
    for failure in &failures {
        failure.eprint();
    }
    eprintln!("=== End Failures ===");

    let cst = cst_opt.expect("CST should be present even with syntax errors");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/missing_end");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();

    insta::assert_yaml_snapshot!("missing_end_function_before_next_function_cst", tree);

    let failure_messages: Vec<String> = failures.iter().map(|f| format!("{f:?}")).collect();
    assert_failure_count_matches_snapshot(
        "missing_end",
        "missing_end_function_before_next_function_failures",
        failures.len(),
    );
    insta::assert_yaml_snapshot!(
        "missing_end_function_before_next_function_failures",
        failure_messages
    );
}

/// Test missing End Property statement
#[test]
fn missing_end_property() {
    let source = r"
Property Get Name() As String
    Name = m_name
";

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    eprintln!("=== Failures for missing_end_property ===");
    eprintln!("Number of failures: {}", failures.len());
    for failure in &failures {
        failure.eprint();
    }
    eprintln!("=== End Failures ===");

    let cst = cst_opt.expect("CST should be present even with syntax errors");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/missing_end");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();

    insta::assert_yaml_snapshot!("missing_end_property_cst", tree);

    let failure_messages: Vec<String> = failures.iter().map(|f| format!("{f:?}")).collect();
    assert_failure_count_matches_snapshot("missing_end", "missing_end_property_failures", failures.len());
    insta::assert_yaml_snapshot!("missing_end_property_failures", failure_messages);
}

/// Test missing End If statement
#[test]
fn missing_end_if() {
    let source = r"
Sub Test()
    If x > 0 Then
        Debug.Print x
End Sub
";

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    eprintln!("=== Failures for missing_end_if ===");
    eprintln!("Number of failures: {}", failures.len());
    for failure in &failures {
        failure.eprint();
    }
    eprintln!("=== End Failures ===");

    let cst = cst_opt.expect("CST should be present even with syntax errors");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/missing_end");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();

    insta::assert_yaml_snapshot!("missing_end_if_cst", tree);

    let failure_messages: Vec<String> = failures.iter().map(|f| format!("{f:?}")).collect();
    assert_failure_count_matches_snapshot("missing_end", "missing_end_if_failures", failures.len());
    insta::assert_yaml_snapshot!("missing_end_if_failures", failure_messages);
}

/// Test missing End Type statement
#[test]
fn missing_end_type() {
    let source = r"
Type Point
    X As Long
    Y As Long
";

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    eprintln!("=== Failures for missing_end_type ===");
    eprintln!("Number of failures: {}", failures.len());
    for failure in &failures {
        failure.eprint();
    }
    eprintln!("=== End Failures ===");

    let cst = cst_opt.expect("CST should be present even with syntax errors");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/missing_end");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();

    insta::assert_yaml_snapshot!("missing_end_type_cst", tree);

    let failure_messages: Vec<String> = failures.iter().map(|f| format!("{f:?}")).collect();
    assert_failure_count_matches_snapshot("missing_end", "missing_end_type_failures", failures.len());
    insta::assert_yaml_snapshot!("missing_end_type_failures", failure_messages);
}

/// Test missing End Select statement
#[test]
fn missing_end_select() {
    let source = r#"
Sub Test()
    Select Case x
        Case 1
            Debug.Print "One"
        Case 2
            Debug.Print "Two"
End Sub
"#;

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    eprintln!("=== Failures for missing_end_select ===");
    eprintln!("Number of failures: {}", failures.len());
    for failure in &failures {
        failure.eprint();
    }
    eprintln!("=== End Failures ===");

    let cst = cst_opt.expect("CST should be present even with syntax errors");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/missing_end");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();

    insta::assert_yaml_snapshot!("missing_end_select_cst", tree);

    let failure_messages: Vec<String> = failures.iter().map(|f| format!("{f:?}")).collect();
    assert_failure_count_matches_snapshot("missing_end", "missing_end_select_failures", failures.len());
    insta::assert_yaml_snapshot!("missing_end_select_failures", failure_messages);
}

/// Test nested missing End statements
#[test]
fn nested_missing_ends() {
    let source = r"
Sub Test()
    If x > 0 Then
        For i = 1 To 10
            Debug.Print i
    ' Missing Next, End If, and End Sub
";

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    eprintln!("=== Failures for nested_missing_ends ===");
    eprintln!("Number of failures: {}", failures.len());
    for failure in &failures {
        failure.eprint();
    }
    eprintln!("=== End Failures ===");

    let cst = cst_opt.expect("CST should be present even with syntax errors");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/missing_end");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();

    insta::assert_yaml_snapshot!("nested_missing_ends_cst", tree);

    let failure_messages: Vec<String> = failures.iter().map(|f| format!("{f:?}")).collect();
    assert_failure_count_matches_snapshot("missing_end", "nested_missing_ends_failures", failures.len());
    insta::assert_yaml_snapshot!("nested_missing_ends_failures", failure_messages);
}
