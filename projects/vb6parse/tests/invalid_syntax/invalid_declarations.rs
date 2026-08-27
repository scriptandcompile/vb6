use vb6parse::parsers::cst::ConcreteSyntaxTree;
const SNAPSHOT_PATH: &str = "../../snapshots/parsers/cst/invalid_syntax/invalid_declarations";

/// Test Dim statement with missing identifier
#[test]
fn dim_missing_identifier() {
    let source = r"
Sub Test()
    Dim As Integer
End Sub
";

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    let cst = cst_opt.expect("CST should be present even with syntax errors");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();

    insta::assert_yaml_snapshot!("dim_missing_identifier_cst", tree);

    insta::assert_yaml_snapshot!(
        "dim_missing_identifier_failures",
        failures
            .iter()
            .map(|f| format!("{f:?}"))
            .collect::<Vec<_>>()
    );
}

/// Test Dim statement with missing type annotation
#[test]
fn dim_missing_type() {
    let source = r"
Sub Test()
    Dim x As
End Sub
";

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    let cst = cst_opt.expect("CST should be present even with syntax errors");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();

    insta::assert_yaml_snapshot!("dim_missing_type_cst", tree);

    insta::assert_yaml_snapshot!(
        "dim_missing_type_failures",
        failures
            .iter()
            .map(|f| format!("{f:?}"))
            .collect::<Vec<_>>()
    );
}

/// Test Function with missing return type
#[test]
fn function_missing_return_type() {
    let source = r"
Function Calculate(x As Integer) As
    Calculate = x * 2
End Function
";

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    let cst = cst_opt.expect("CST should be present even with syntax errors");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();

    insta::assert_yaml_snapshot!("function_missing_return_type_cst", tree);

    insta::assert_yaml_snapshot!(
        "function_missing_return_type_failures",
        failures
            .iter()
            .map(|f| format!("{f:?}"))
            .collect::<Vec<_>>()
    );
}

/// Test Sub with missing identifier (empty parentheses)
#[test]
fn sub_missing_name() {
    let source = r#"
Sub ()
    Debug.Print "Test"
End Sub
"#;

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    let cst = cst_opt.expect("CST should be present even with syntax errors");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();

    insta::assert_yaml_snapshot!("sub_missing_name_cst", tree);

    insta::assert_yaml_snapshot!(
        "sub_missing_name_failures",
        failures
            .iter()
            .map(|f| format!("{f:?}"))
            .collect::<Vec<_>>()
    );
}

/// Test duplicate Public modifiers
#[test]
fn duplicate_public_modifier() {
    let source = r#"
Public Public Sub Test()
    Debug.Print "Duplicate modifier"
End Sub
"#;

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    let cst = cst_opt.expect("CST should be present even with syntax errors");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();

    insta::assert_yaml_snapshot!("duplicate_public_modifier_cst", tree);

    insta::assert_yaml_snapshot!(
        "duplicate_public_modifier_failures",
        failures
            .iter()
            .map(|f| format!("{f:?}"))
            .collect::<Vec<_>>()
    );
}

/// Test conflicting visibility modifiers
#[test]
fn conflicting_visibility_modifiers() {
    let source = r#"
Public Private Sub Test()
    Debug.Print "Conflicting modifiers"
End Sub
"#;

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    let cst = cst_opt.expect("CST should be present even with syntax errors");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();

    insta::assert_yaml_snapshot!("conflicting_visibility_modifiers_cst", tree);

    insta::assert_yaml_snapshot!(
        "conflicting_visibility_modifiers_failures",
        failures
            .iter()
            .map(|f| format!("{f:?}"))
            .collect::<Vec<_>>()
    );
}

/// Test array declaration with missing bounds
#[test]
fn array_missing_bounds() {
    let source = r"
Sub Test()
    Dim arr() As Integer
End Sub
";

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    let cst = cst_opt.expect("CST should be present even with syntax errors");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();

    insta::assert_yaml_snapshot!("array_missing_bounds_cst", tree);

    insta::assert_yaml_snapshot!(
        "array_missing_bounds_failures",
        failures
            .iter()
            .map(|f| format!("{f:?}"))
            .collect::<Vec<_>>()
    );
}

/// Test Const without value assignment
#[test]
fn const_missing_value() {
    let source = r"
Sub Test()
    Const MAX_VALUE As Integer
End Sub
";

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    let cst = cst_opt.expect("CST should be present even with syntax errors");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();

    insta::assert_yaml_snapshot!("const_missing_value_cst", tree);

    insta::assert_yaml_snapshot!(
        "const_missing_value_failures",
        failures
            .iter()
            .map(|f| format!("{f:?}"))
            .collect::<Vec<_>>()
    );
}

/// Test Type with missing member name
#[test]
fn type_missing_member_name() {
    let source = r"
Type Point
    As Long
    Y As Long
End Type
";

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    let cst = cst_opt.expect("CST should be present even with syntax errors");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();

    insta::assert_yaml_snapshot!("type_missing_member_name_cst", tree);

    insta::assert_yaml_snapshot!(
        "type_missing_member_name_failures",
        failures
            .iter()
            .map(|f| format!("{f:?}"))
            .collect::<Vec<_>>()
    );
}

/// Test parameter with missing name
#[test]
fn parameter_missing_name() {
    let source = r"
Sub Test(As Integer, y As String)
    Debug.Print y
End Sub
";

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    let cst = cst_opt.expect("CST should be present even with syntax errors");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();

    insta::assert_yaml_snapshot!("parameter_missing_name_cst", tree);

    insta::assert_yaml_snapshot!(
        "parameter_missing_name_failures",
        failures
            .iter()
            .map(|f| format!("{f:?}"))
            .collect::<Vec<_>>()
    );
}

/// Test Optional parameter without default value
#[test]
fn optional_parameter_missing_default() {
    let source = r"
Sub Test(Optional x As Integer =)
    Debug.Print x
End Sub
";

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    let cst = cst_opt.expect("CST should be present even with syntax errors");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();

    insta::assert_yaml_snapshot!("optional_parameter_missing_default_cst", tree);

    insta::assert_yaml_snapshot!(
        "optional_parameter_missing_default_failures",
        failures
            .iter()
            .map(|f| format!("{f:?}"))
            .collect::<Vec<_>>()
    );
}

/// Test duplicate Static modifiers on variables
#[test]
fn duplicate_static_modifier() {
    let source = r"
Sub Test()
    Static Static x As Integer
    x = 10
End Sub
";

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    let cst = cst_opt.expect("CST should be present even with syntax errors");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();

    insta::assert_yaml_snapshot!("duplicate_static_modifier_cst", tree);

    insta::assert_yaml_snapshot!(
        "duplicate_static_modifier_failures",
        failures
            .iter()
            .map(|f| format!("{f:?}"))
            .collect::<Vec<_>>()
    );
}

/// Test Enum with missing member value after equals
#[test]
fn enum_missing_member_value() {
    let source = r"
Enum Colors
    Red = 1
    Green =
    Blue = 3
End Enum
";

    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();

    let cst = cst_opt.expect("CST should be present even with syntax errors");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();

    insta::assert_yaml_snapshot!("enum_missing_member_value_cst", tree);

    insta::assert_yaml_snapshot!(
        "enum_missing_member_value_failures",
        failures
            .iter()
            .map(|f| format!("{f:?}"))
            .collect::<Vec<_>>()
    );
}
