use vb6parse::parsers::cst::ConcreteSyntaxTree;

/// Test Exit Sub outside of a subroutine
#[test]
fn exit_sub_outside_sub() {
    let source = r"
Exit Sub
";

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    let cst = cst_opt.expect("CST should be present even with syntax errors");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_control_flow");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();

    insta::assert_yaml_snapshot!("exit_sub_outside_sub_cst", tree);
    insta::assert_yaml_snapshot!("exit_sub_outside_sub_failures", failures.iter().map(|f| format!("{f:?}")).collect::<Vec<_>>());
}

/// Test Exit Function outside of a function
#[test]
fn exit_function_outside_function() {
    let source = r"
Sub Test()
    Exit Function
End Sub
";

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    let cst = cst_opt.expect("CST should be present even with syntax errors");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_control_flow");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();

    insta::assert_yaml_snapshot!("exit_function_outside_function_cst", tree);
    insta::assert_yaml_snapshot!("exit_function_outside_function_failures", failures.iter().map(|f| format!("{f:?}")).collect::<Vec<_>>());
}

/// Test Exit Property outside of a property
#[test]
fn exit_property_outside_property() {
    let source = r"
Function Calculate() As Integer
    Exit Property
    Calculate = 0
End Function
";

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    let cst = cst_opt.expect("CST should be present even with syntax errors");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_control_flow");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();

    insta::assert_yaml_snapshot!("exit_property_outside_property_cst", tree);
    insta::assert_yaml_snapshot!("exit_property_outside_property_failures", failures.iter().map(|f| format!("{f:?}")).collect::<Vec<_>>());
}

/// Test Exit For outside of a For loop
#[test]
fn exit_for_outside_loop() {
    let source = r"
Sub Test()
    Exit For
    Dim i As Integer
    i = 5
End Sub
";

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    let cst = cst_opt.expect("CST should be present even with syntax errors");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_control_flow");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();

    insta::assert_yaml_snapshot!("exit_for_outside_loop_cst", tree);
    insta::assert_yaml_snapshot!("exit_for_outside_loop_failures", failures.iter().map(|f| format!("{f:?}")).collect::<Vec<_>>());
}

/// Test Exit Do outside of a Do loop
#[test]
fn exit_do_outside_loop() {
    let source = r"
Sub Test()
    While True
        Exit Do
    Wend
End Sub
";

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    let cst = cst_opt.expect("CST should be present even with syntax errors");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_control_flow");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();

    insta::assert_yaml_snapshot!("exit_do_outside_loop_cst", tree);
    insta::assert_yaml_snapshot!("exit_do_outside_loop_failures", failures.iter().map(|f| format!("{f:?}")).collect::<Vec<_>>());
}

/// Test `GoTo` with missing label
#[test]
fn goto_missing_label() {
    let source = r#"
Sub Test()
    GoTo
    Debug.Print "Test"
End Sub
"#;

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    let cst = cst_opt.expect("CST should be present even with syntax errors");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_control_flow");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();

    insta::assert_yaml_snapshot!("goto_missing_label_cst", tree);
    insta::assert_yaml_snapshot!("goto_missing_label_failures", failures.iter().map(|f| format!("{f:?}")).collect::<Vec<_>>());
}

/// Test `GoSub` with missing label
#[test]
fn gosub_missing_label() {
    let source = r"
Sub Test()
    GoSub
    Exit Sub
ErrorHandler:
    Resume Next
End Sub
";

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    let cst = cst_opt.expect("CST should be present even with syntax errors");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_control_flow");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();

    insta::assert_yaml_snapshot!("gosub_missing_label_cst", tree);
    insta::assert_yaml_snapshot!("gosub_missing_label_failures", failures.iter().map(|f| format!("{f:?}")).collect::<Vec<_>>());
}

/// Test `On Error` with missing destination
#[test]
fn on_error_missing_destination() {
    let source = r"
Sub Test()
    On Error
    Dim x As Integer
End Sub
";

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    let cst = cst_opt.expect("CST should be present even with syntax errors");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_control_flow");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();

    insta::assert_yaml_snapshot!("on_error_missing_destination_cst", tree);
    insta::assert_yaml_snapshot!("on_error_missing_destination_failures", failures.iter().map(|f| format!("{f:?}")).collect::<Vec<_>>());
}

/// Test Resume without On Error context
#[test]
fn resume_without_on_error() {
    let source = r"
Sub Test()
    Resume Next
End Sub
";

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    let cst = cst_opt.expect("CST should be present even with syntax errors");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_control_flow");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();

    insta::assert_yaml_snapshot!("resume_without_on_error_cst", tree);
    insta::assert_yaml_snapshot!("resume_without_on_error_failures", failures.iter().map(|f| format!("{f:?}")).collect::<Vec<_>>());
}

/// Test nested Exit statements in wrong context
#[test]
fn nested_exit_wrong_context() {
    let source = r"
Sub Test()
    For i = 1 To 10
        Do While i < 5
            Exit Sub
        Loop
    Next i
End Sub
";

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    let cst = cst_opt.expect("CST should be present even with syntax errors");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_control_flow");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();

    insta::assert_yaml_snapshot!("nested_exit_wrong_context_cst", tree);
    insta::assert_yaml_snapshot!("nested_exit_wrong_context_failures", failures.iter().map(|f| format!("{f:?}")).collect::<Vec<_>>());
}

/// Test Return statement in module (only valid in classes)
#[test]
fn return_in_module() {
    let source = r"
Sub Test()
    Return
End Sub
";

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    let cst = cst_opt.expect("CST should be present even with syntax errors");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_control_flow");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();

    insta::assert_yaml_snapshot!("return_in_module_cst", tree);
    insta::assert_yaml_snapshot!("return_in_module_failures", failures.iter().map(|f| format!("{f:?}")).collect::<Vec<_>>());
}

/// Test `On Error GoTo` with missing line number/label
#[test]
fn on_error_goto_missing_target() {
    let source = r"
Sub Test()
    On Error GoTo
    Dim x As Integer
End Sub
";

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    let cst = cst_opt.expect("CST should be present even with syntax errors");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_control_flow");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();

    insta::assert_yaml_snapshot!("on_error_goto_missing_target_cst", tree);
    insta::assert_yaml_snapshot!("on_error_goto_missing_target_failures", failures.iter().map(|f| format!("{f:?}")).collect::<Vec<_>>());
}

/// Test Stop statement with arguments (should have none)
#[test]
fn stop_with_arguments() {
    let source = r"
Sub Test()
    Stop 123
End Sub
";

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    let cst = cst_opt.expect("CST should be present even with syntax errors");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_control_flow");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();

    insta::assert_yaml_snapshot!("stop_with_arguments_cst", tree);
    insta::assert_yaml_snapshot!("stop_with_arguments_failures", failures.iter().map(|f| format!("{f:?}")).collect::<Vec<_>>());
}

/// Test Resume with invalid keyword combination
#[test]
fn resume_invalid_combination() {
    let source = r"
Sub Test()
    On Error Resume 0
End Sub
";

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    let cst = cst_opt.expect("CST should be present even with syntax errors");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_control_flow");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();

    insta::assert_yaml_snapshot!("resume_invalid_combination_cst", tree);
    insta::assert_yaml_snapshot!("resume_invalid_combination_failures", failures.iter().map(|f| format!("{f:?}")).collect::<Vec<_>>());
}
