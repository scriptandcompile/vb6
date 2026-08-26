use vb6parse::ConcreteSyntaxTree;

#[test]
fn resume_simple() {
    let source = r"
Sub Test()
    On Error GoTo ErrorHandler
    Exit Sub
ErrorHandler:
    Resume
End Sub
";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../../snapshots/syntax/statements/control_flow/resume");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn resume_next() {
    let source = r"
Sub Test()
    On Error GoTo ErrorHandler
    x = 1 / 0
    Exit Sub
ErrorHandler:
    Resume Next
End Sub
";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../../snapshots/syntax/statements/control_flow/resume");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn resume_with_label() {
    let source = r#"
Sub Test()
    On Error GoTo ErrorHandler
    Exit Sub
ErrorHandler:
    Resume CleanUp
CleanUp:
    MsgBox "Cleanup"
End Sub
"#;
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../../snapshots/syntax/statements/control_flow/resume");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn resume_with_line_number() {
    let source = r"
Sub Test()
    On Error GoTo ErrorHandler
    Exit Sub
ErrorHandler:
    Resume 100
End Sub
";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../../snapshots/syntax/statements/control_flow/resume");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn resume_in_error_handler() {
    let source = r#"
Sub ProcessFile()
    On Error GoTo FileError
    Open "test.txt" For Input As #1
    Exit Sub
FileError:
    If Err.Number = 53 Then
        MsgBox "File not found"
        Resume Next
    Else
        Resume
    End If
End Sub
"#;
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../../snapshots/syntax/statements/control_flow/resume");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn resume_with_comment() {
    let source = r"
Sub Test()
    On Error GoTo ErrorHandler
    Exit Sub
ErrorHandler:
    Resume Next ' Continue after error
End Sub
";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../../snapshots/syntax/statements/control_flow/resume");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn resume_preserves_whitespace() {
    let source = "    Resume    Next    \n";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../../snapshots/syntax/statements/control_flow/resume");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn resume_in_nested_error_handler() {
    let source = r"
Sub Test()
    On Error GoTo ErrorHandler
    Call SubProcedure
    Exit Sub
ErrorHandler:
    If Err.Number = 5 Then
        Resume Next
    Else
        Resume CleanUp
    End If
CleanUp:
    ' Cleanup code
End Sub
";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../../snapshots/syntax/statements/control_flow/resume");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn resume_at_module_level() {
    let source = "Resume Next\n";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../../snapshots/syntax/statements/control_flow/resume");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn resume_with_select_case() {
    let source = r"
Sub Test()
    On Error GoTo ErrorHandler
    Exit Sub
ErrorHandler:
    Select Case Err.Number
        Case 5
            Resume Next
        Case 11
            Resume
        Case Else
            Resume CleanUp
    End Select
CleanUp:
    ' Cleanup
End Sub
";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../../snapshots/syntax/statements/control_flow/resume");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn resume_in_loop() {
    let source = r"
Sub ProcessFiles()
    On Error GoTo ErrorHandler
    For i = 1 To 10
        ProcessFile i
    Next i
    Exit Sub
ErrorHandler:
    Resume Next
End Sub
";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../../snapshots/syntax/statements/control_flow/resume");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn resume_complex_error_handling() {
    let source = r#"
Function OpenDatabase() As Boolean
    On Error GoTo DBError
    ' Database opening code
    OpenDatabase = True
    Exit Function
DBError:
    Select Case Err.Number
        Case 3024
            MsgBox "Database locked"
            Resume Retry
        Case 3044
            MsgBox "Path not found"
            Resume Next
        Case Else
            MsgBox "Unknown error"
            Resume ExitPoint
    End Select
Retry:
    ' Retry logic
    Resume
ExitPoint:
    OpenDatabase = False
End Function
"#;
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../../snapshots/syntax/statements/control_flow/resume");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn resume_with_do_loop() {
    let source = r"
Sub Test()
    On Error GoTo ErrorHandler
    Do
        ' Code
    Loop
    Exit Sub
ErrorHandler:
    Resume Next
End Sub
";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../../snapshots/syntax/statements/control_flow/resume");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn multiple_resume_statements() {
    let source = r"
Sub Test()
    On Error GoTo ErrorHandler
    Exit Sub
ErrorHandler:
    If Err.Number = 5 Then
        Resume
    End If
    If Err.Number = 11 Then
        Resume Next
    End If
    Resume CleanUp
CleanUp:
    ' Cleanup
End Sub
";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../../snapshots/syntax/statements/control_flow/resume");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn resume_with_on_error_resume_next() {
    let source = r#"
Sub Test()
    On Error Resume Next
    x = 1 / 0
    If Err.Number <> 0 Then
        MsgBox "Error occurred"
    End If
End Sub
"#;
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../../snapshots/syntax/statements/control_flow/resume");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn resume_in_class_module() {
    let source = r"
Private Sub Class_Initialize()
    On Error GoTo InitError
    ' Initialization code
    Exit Sub
InitError:
    Resume Next
End Sub
";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.cls", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../../snapshots/syntax/statements/control_flow/resume");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn resume_with_error_number_check() {
    let source = r"
Sub Test()
    On Error GoTo ErrorHandler
    Exit Sub
ErrorHandler:
    If Err.Number = 53 Then
        Resume Next
    ElseIf Err.Number = 5 Then
        Resume
    Else
        Resume ExitSub
    End If
ExitSub:
End Sub
";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../../snapshots/syntax/statements/control_flow/resume");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn resume_in_function() {
    let source = r"
Function Calculate() As Double
    On Error GoTo CalcError
    Calculate = x / y
    Exit Function
CalcError:
    Calculate = 0
    Resume Next
End Function
";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../../snapshots/syntax/statements/control_flow/resume");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn resume_with_line_continuation() {
    let source = r"
Sub Test()
    On Error GoTo ErrorHandler
    Exit Sub
ErrorHandler:
    Resume _
        Next
End Sub
";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../../snapshots/syntax/statements/control_flow/resume");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn resume_inline_if() {
    let source = r"
Sub Test()
    On Error GoTo ErrorHandler
    Exit Sub
ErrorHandler:
    If Err.Number = 5 Then Resume Next
End Sub
";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../../snapshots/syntax/statements/control_flow/resume");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn resume_with_transaction() {
    let source = r"
Sub ProcessTransaction()
    On Error GoTo TransError
    BeginTrans
    ' Transaction code
    CommitTrans
    Exit Sub
TransError:
    Rollback
    Resume Next
End Sub
";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../../snapshots/syntax/statements/control_flow/resume");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn resume_with_goto_label() {
    let source = r"
Sub Test()
    On Error GoTo ErrorHandler
    GoTo ProcessData
ProcessData:
    ' Code
    Exit Sub
ErrorHandler:
    Resume ProcessData
End Sub
";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../../snapshots/syntax/statements/control_flow/resume");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn resume_with_exit_statement() {
    let source = r"
Sub Test()
    On Error GoTo ErrorHandler
    For i = 1 To 10
        If i = 5 Then Exit For
    Next i
    Exit Sub
ErrorHandler:
    Resume Next
End Sub
";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../../snapshots/syntax/statements/control_flow/resume");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn resume_file_operations() {
    let source = r#"
Sub ReadFile()
    On Error GoTo FileError
    Open "data.txt" For Input As #1
    Line Input #1, dataLine
    Close #1
    Exit Sub
FileError:
    If Err.Number = 53 Then
        Resume CreateFile
    Else
        Resume Next
    End If
CreateFile:
    ' Create file logic
End Sub
"#;
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../../snapshots/syntax/statements/control_flow/resume");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn resume_database_operations() {
    let source = r#"
Sub QueryDatabase()
    On Error GoTo DBError
    rs.Open "SELECT * FROM Users"
    Exit Sub
DBError:
    If Err.Number = 3021 Then
        Resume Next
    Else
        Resume CleanUp
    End If
CleanUp:
    rs.Close
End Sub
"#;
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");

    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../../snapshots/syntax/statements/control_flow/resume");
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}
