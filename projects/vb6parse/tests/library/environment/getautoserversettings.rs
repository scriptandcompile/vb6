use vb6parse::ConcreteSyntaxTree;

#[test]
fn getautoserversettings_basic() {
    let source = r#"settings = GetAutoServerSettings("MyServer.Application", "{12345678-1234-1234-1234-123456789012}", "SERVER01")"#;
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(
        "../../../snapshots/parsers/cst/library/environment/getautoserversettings",
    );
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn getautoserversettings_with_variables() {
    let source = r"result = GetAutoServerSettings(progID, clsID, serverName)";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(
        "../../../snapshots/parsers/cst/library/environment/getautoserversettings",
    );
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn getautoserversettings_in_if() {
    let source =
        r#"If GetAutoServerSettings(progID, clsID, "SERVER01") <> 0 Then MsgBox "Available""#;
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(
        "../../../snapshots/parsers/cst/library/environment/getautoserversettings",
    );
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn getautoserversettings_in_function() {
    let source = r"Function ValidateServer() As Boolean
    ValidateServer = (GetAutoServerSettings(m_ProgID, m_CLSID, m_Server) <> 0)
End Function";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(
        "../../../snapshots/parsers/cst/library/environment/getautoserversettings",
    );
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn getautoserversettings_assignment() {
    let source = r#"Dim settings As Long
settings = GetAutoServerSettings("App.Server", "{AAAABBBB-CCCC-DDDD-EEEE-FFFF00001111}", "REMOTE-PC")"#;
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(
        "../../../snapshots/parsers/cst/library/environment/getautoserversettings",
    );
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn getautoserversettings_comparison() {
    let source = r#"If GetAutoServerSettings(progID, clsID, server1) = GetAutoServerSettings(progID, clsID, server2) Then
    Debug.Print "Same settings"
End If"#;
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(
        "../../../snapshots/parsers/cst/library/environment/getautoserversettings",
    );
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn getautoserversettings_for_loop() {
    let source = r"For i = 1 To serverCount
    settings = GetAutoServerSettings(progID, clsID, servers(i))
Next i";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(
        "../../../snapshots/parsers/cst/library/environment/getautoserversettings",
    );
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn getautoserversettings_select_case() {
    let source = r#"Select Case GetAutoServerSettings(progID, clsID, serverName)
    Case Is > 0
        Debug.Print "Configured"
End Select"#;
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(
        "../../../snapshots/parsers/cst/library/environment/getautoserversettings",
    );
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn getautoserversettings_error_handling() {
    let source = r"On Error GoTo ErrorHandler
settings = GetAutoServerSettings(progID, clsID, serverName)";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(
        "../../../snapshots/parsers/cst/library/environment/getautoserversettings",
    );
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn getautoserversettings_debug_print() {
    let source = r#"Debug.Print "Settings: " & GetAutoServerSettings(progID, clsID, serverName)"#;
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(
        "../../../snapshots/parsers/cst/library/environment/getautoserversettings",
    );
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn getautoserversettings_array_element() {
    let source = r"settings(i) = GetAutoServerSettings(progID, clsID, servers(i))";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(
        "../../../snapshots/parsers/cst/library/environment/getautoserversettings",
    );
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn getautoserversettings_class_member() {
    let source = r"m_Settings = GetAutoServerSettings(m_ProgID, m_CLSID, m_ServerName)";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(
        "../../../snapshots/parsers/cst/library/environment/getautoserversettings",
    );
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn getautoserversettings_type_field() {
    let source = r"serverInfo.Settings = GetAutoServerSettings(progID, clsID, serverInfo.Name)";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(
        "../../../snapshots/parsers/cst/library/environment/getautoserversettings",
    );
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn getautoserversettings_do_loop() {
    let source = r"Do While retry < maxRetries
    settings = GetAutoServerSettings(progID, clsID, serverName)
    If settings <> 0 Then Exit Do
Loop";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(
        "../../../snapshots/parsers/cst/library/environment/getautoserversettings",
    );
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn getautoserversettings_local_server() {
    let source = r#"localSettings = GetAutoServerSettings(progID, clsID, ".")"#;
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(
        "../../../snapshots/parsers/cst/library/environment/getautoserversettings",
    );
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn getautoserversettings_concatenation() {
    let source =
        r#"result = "Settings: " & CStr(GetAutoServerSettings(progID, clsID, serverName))"#;
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(
        "../../../snapshots/parsers/cst/library/environment/getautoserversettings",
    );
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn getautoserversettings_msgbox() {
    let source = r#"MsgBox "Settings: " & GetAutoServerSettings(progID, clsID, serverName)"#;
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(
        "../../../snapshots/parsers/cst/library/environment/getautoserversettings",
    );
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn getautoserversettings_collection() {
    let source = r"results.Add GetAutoServerSettings(progID, clsID, serverName)";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(
        "../../../snapshots/parsers/cst/library/environment/getautoserversettings",
    );
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn getautoserversettings_file_output() {
    let source = r#"Print #fileNum, "Server: " & serverName & " Settings: " & GetAutoServerSettings(progID, clsID, serverName)"#;
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(
        "../../../snapshots/parsers/cst/library/environment/getautoserversettings",
    );
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn getautoserversettings_iif() {
    let source = r#"status = IIf(GetAutoServerSettings(progID, clsID, serverName) <> 0, "Online", "Offline")"#;
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(
        "../../../snapshots/parsers/cst/library/environment/getautoserversettings",
    );
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn getautoserversettings_with_statement() {
    let source = r"With serverConfig
    .Settings = GetAutoServerSettings(.ProgID, .CLSID, .ServerName)
End With";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(
        "../../../snapshots/parsers/cst/library/environment/getautoserversettings",
    );
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn getautoserversettings_on_error_resume() {
    let source = r"On Error Resume Next
settings = GetAutoServerSettings(progID, clsID, serverName)
On Error GoTo 0";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(
        "../../../snapshots/parsers/cst/library/environment/getautoserversettings",
    );
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn getautoserversettings_min_comparison() {
    let source = r"If GetAutoServerSettings(progID, clsID, serverName) < minSettings Then
    minSettings = GetAutoServerSettings(progID, clsID, serverName)
End If";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(
        "../../../snapshots/parsers/cst/library/environment/getautoserversettings",
    );
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn getautoserversettings_for_each() {
    let source = r"For Each server In servers
    settings = GetAutoServerSettings(progID, clsID, CStr(server))
Next server";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(
        "../../../snapshots/parsers/cst/library/environment/getautoserversettings",
    );
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn getautoserversettings_property() {
    let source = r"Property Get ServerSettings() As Long
    ServerSettings = GetAutoServerSettings(m_ProgID, m_CLSID, m_ServerName)
End Property";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(
        "../../../snapshots/parsers/cst/library/environment/getautoserversettings",
    );
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn getautoserversettings_exit_condition() {
    let source = r"If GetAutoServerSettings(progID, clsID, serverName) = 0 Then Exit Function";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(
        "../../../snapshots/parsers/cst/library/environment/getautoserversettings",
    );
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}

#[test]
fn getautoserversettings_listbox() {
    let source = r#"lstServers.AddItem serverName & " - " & GetAutoServerSettings(progID, clsID, serverName)"#;
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(
        "../../../snapshots/parsers/cst/library/environment/getautoserversettings",
    );
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}
