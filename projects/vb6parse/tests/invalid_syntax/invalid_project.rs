// Invalid project file syntax tests
//
// These tests verify that the project file parser handles invalid syntax gracefully,
// producing reasonable error messages for malformed project files.

use vb6parse::{ProjectFile, SourceFile};

fn assert_failure_count_matches_snapshot(snapshot_dir: &str, snapshot_name: &str, actual: usize) {
    let primary_path =
        format!("snapshots/tests/invalid_syntax/{snapshot_dir}/{snapshot_name}.snap");
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
        actual, expected,
        "Unexpected failure count for snapshot {snapshot_name}"
    );
}

#[test]
fn unterminated_section_header() {
    let source = r"Type=Exe
[MS Transaction Server
AutoRefresh=1
";

    let source_file = SourceFile::from_string("test.vbp", source);
    let (_project_opt, failures) = ProjectFile::parse(&source_file).unpack();

    assert!(!failures.is_empty(), "Expected parsing failures");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_project");
    settings.bind(|| {
        let failure_messages: Vec<String> = failures.iter().map(|f| format!("{f:?}")).collect();
        assert_failure_count_matches_snapshot(
            "invalid_project",
            "unterminated_section_header_failures",
            failures.len(),
        );
        insta::assert_yaml_snapshot!("unterminated_section_header_failures", failure_messages);
    });
}

#[test]
fn missing_property_name() {
    let source = r"Type=Exe
=SomeValue
";

    let source_file = SourceFile::from_string("test.vbp", source);
    let (_project_opt, failures) = ProjectFile::parse(&source_file).unpack();

    assert!(!failures.is_empty(), "Expected parsing failures");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_project");
    settings.bind(|| {
        let failure_messages: Vec<String> = failures.iter().map(|f| format!("{f:?}")).collect();
        assert_failure_count_matches_snapshot(
            "invalid_project",
            "missing_property_name_failures",
            failures.len(),
        );
        insta::assert_yaml_snapshot!("missing_property_name_failures", failure_messages);
    });
}

#[test]
fn invalid_project_type() {
    let source = r"Type=InvalidType
";

    let source_file = SourceFile::from_string("test.vbp", source);
    let (_project_opt, failures) = ProjectFile::parse(&source_file).unpack();

    assert!(!failures.is_empty(), "Expected parsing failures");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_project");
    settings.bind(|| {
        let failure_messages: Vec<String> = failures.iter().map(|f| format!("{f:?}")).collect();
        assert_failure_count_matches_snapshot(
            "invalid_project",
            "invalid_project_type_failures",
            failures.len(),
        );
        insta::assert_yaml_snapshot!("invalid_project_type_failures", failure_messages);
    });
}

#[test]
fn reference_compiled_missing_closing_brace() {
    let source = r"Type=Exe
Reference=*\G{00020430-0000-0000-C000-000000000046#2.0#0#C:\Windows\System32\stdole2.tlb#OLE Automation
";

    let source_file = SourceFile::from_string("test.vbp", source);
    let (_project_opt, failures) = ProjectFile::parse(&source_file).unpack();

    assert!(!failures.is_empty(), "Expected parsing failures");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_project");
    settings.bind(|| {
        let failure_messages: Vec<String> = failures.iter().map(|f| format!("{f:?}")).collect();
        assert_failure_count_matches_snapshot(
            "invalid_project",
            "reference_compiled_missing_closing_brace_failures",
            failures.len(),
        );
        insta::assert_yaml_snapshot!(
            "reference_compiled_missing_closing_brace_failures",
            failure_messages
        );
    });
}

#[test]
fn reference_compiled_invalid_uuid() {
    let source = r"Type=Exe
Reference=*\G{not-a-valid-uuid}#2.0#0#C:\Windows\System32\stdole2.tlb#OLE Automation
";

    let source_file = SourceFile::from_string("test.vbp", source);
    let (_project_opt, failures) = ProjectFile::parse(&source_file).unpack();

    assert!(!failures.is_empty(), "Expected parsing failures");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_project");
    settings.bind(|| {
        let failure_messages: Vec<String> = failures.iter().map(|f| format!("{f:?}")).collect();
        assert_failure_count_matches_snapshot(
            "invalid_project",
            "reference_compiled_invalid_uuid_failures",
            failures.len(),
        );
        insta::assert_yaml_snapshot!("reference_compiled_invalid_uuid_failures", failure_messages);
    });
}

#[test]
fn reference_compiled_missing_unknown1() {
    let source = r"Type=Exe
Reference=*\G{00020430-0000-0000-C000-000000000046}#
";

    let source_file = SourceFile::from_string("test.vbp", source);
    let (_project_opt, failures) = ProjectFile::parse(&source_file).unpack();

    assert!(!failures.is_empty(), "Expected parsing failures");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_project");
    settings.bind(|| {
        let failure_messages: Vec<String> = failures.iter().map(|f| format!("{f:?}")).collect();
        assert_failure_count_matches_snapshot(
            "invalid_project",
            "reference_compiled_missing_unknown1_failures",
            failures.len(),
        );
        insta::assert_yaml_snapshot!(
            "reference_compiled_missing_unknown1_failures",
            failure_messages
        );
    });
}

#[test]
fn reference_compiled_missing_path() {
    let source = r"Type=Exe
Reference=*\G{00020430-0000-0000-C000-000000000046}#2.0#0#
";

    let source_file = SourceFile::from_string("test.vbp", source);
    let (_project_opt, failures) = ProjectFile::parse(&source_file).unpack();

    assert!(!failures.is_empty(), "Expected parsing failures");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_project");
    settings.bind(|| {
        let failure_messages: Vec<String> = failures.iter().map(|f| format!("{f:?}")).collect();
        assert_failure_count_matches_snapshot(
            "invalid_project",
            "reference_compiled_missing_path_failures",
            failures.len(),
        );
        insta::assert_yaml_snapshot!("reference_compiled_missing_path_failures", failure_messages);
    });
}

#[test]
fn reference_project_missing_path() {
    let source = r"Type=Exe
Reference=
";

    let source_file = SourceFile::from_string("test.vbp", source);
    let (_project_opt, failures) = ProjectFile::parse(&source_file).unpack();

    assert!(!failures.is_empty(), "Expected parsing failures");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_project");
    settings.bind(|| {
        let failure_messages: Vec<String> = failures.iter().map(|f| format!("{f:?}")).collect();
        assert_failure_count_matches_snapshot(
            "invalid_project",
            "reference_project_missing_path_failures",
            failures.len(),
        );
        insta::assert_yaml_snapshot!("reference_project_missing_path_failures", failure_messages);
    });
}

#[test]
fn reference_project_invalid_path() {
    let source = r#"Type=Exe
Reference="C:\InvalidPath\Project.vbp"
"#;

    let source_file = SourceFile::from_string("test.vbp", source);
    let (_project_opt, failures) = ProjectFile::parse(&source_file).unpack();

    assert!(!failures.is_empty(), "Expected parsing failures");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_project");
    settings.bind(|| {
        let failure_messages: Vec<String> = failures.iter().map(|f| format!("{f:?}")).collect();
        assert_failure_count_matches_snapshot(
            "invalid_project",
            "reference_project_invalid_path_failures",
            failures.len(),
        );
        insta::assert_yaml_snapshot!("reference_project_invalid_path_failures", failure_messages);
    });
}

#[test]
fn object_compiled_missing_opening_brace() {
    let source = r"Type=Exe
Object=00020430-0000-0000-C000-000000000046}#2.0#0; stdole2.tlb
";

    let source_file = SourceFile::from_string("test.vbp", source);
    let (_project_opt, failures) = ProjectFile::parse(&source_file).unpack();

    assert!(!failures.is_empty(), "Expected parsing failures");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_project");
    settings.bind(|| {
        let failure_messages: Vec<String> = failures.iter().map(|f| format!("{f:?}")).collect();
        assert_failure_count_matches_snapshot(
            "invalid_project",
            "object_compiled_missing_opening_brace_failures",
            failures.len(),
        );
        insta::assert_yaml_snapshot!(
            "object_compiled_missing_opening_brace_failures",
            failure_messages
        );
    });
}

#[test]
fn object_compiled_missing_closing_brace() {
    let source = r"Type=Exe
Object={00020430-0000-0000-C000-000000000046#2.0#0; stdole2.tlb
";

    let source_file = SourceFile::from_string("test.vbp", source);
    let (_project_opt, failures) = ProjectFile::parse(&source_file).unpack();

    assert!(!failures.is_empty(), "Expected parsing failures");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_project");
    settings.bind(|| {
        let failure_messages: Vec<String> = failures.iter().map(|f| format!("{f:?}")).collect();
        assert_failure_count_matches_snapshot(
            "invalid_project",
            "object_compiled_missing_closing_brace_failures",
            failures.len(),
        );
        insta::assert_yaml_snapshot!(
            "object_compiled_missing_closing_brace_failures",
            failure_messages
        );
    });
}

#[test]
fn object_compiled_invalid_uuid() {
    let source = r"Type=Exe
Object={invalid-uuid}#2.0#0; stdole2.tlb
";

    let source_file = SourceFile::from_string("test.vbp", source);
    let (_project_opt, failures) = ProjectFile::parse(&source_file).unpack();

    assert!(!failures.is_empty(), "Expected parsing failures");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_project");
    settings.bind(|| {
        let failure_messages: Vec<String> = failures.iter().map(|f| format!("{f:?}")).collect();
        assert_failure_count_matches_snapshot(
            "invalid_project",
            "object_compiled_invalid_uuid_failures",
            failures.len(),
        );
        insta::assert_yaml_snapshot!("object_compiled_invalid_uuid_failures", failure_messages);
    });
}

#[test]
fn object_compiled_missing_version() {
    let source = r"Type=Exe
Object={00020430-0000-0000-C000-000000000046}
";

    let source_file = SourceFile::from_string("test.vbp", source);
    let (_project_opt, failures) = ProjectFile::parse(&source_file).unpack();

    assert!(!failures.is_empty(), "Expected parsing failures");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_project");
    settings.bind(|| {
        let failure_messages: Vec<String> = failures.iter().map(|f| format!("{f:?}")).collect();
        assert_failure_count_matches_snapshot(
            "invalid_project",
            "object_compiled_missing_version_failures",
            failures.len(),
        );
        insta::assert_yaml_snapshot!("object_compiled_missing_version_failures", failure_messages);
    });
}

#[test]
fn object_compiled_invalid_version() {
    let source = r"Type=Exe
Object={00020430-0000-0000-C000-000000000046}#invalid#0; stdole2.tlb
";

    let source_file = SourceFile::from_string("test.vbp", source);
    let (_project_opt, failures) = ProjectFile::parse(&source_file).unpack();

    assert!(!failures.is_empty(), "Expected parsing failures");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_project");
    settings.bind(|| {
        let failure_messages: Vec<String> = failures.iter().map(|f| format!("{f:?}")).collect();
        assert_failure_count_matches_snapshot(
            "invalid_project",
            "object_compiled_invalid_version_failures",
            failures.len(),
        );
        insta::assert_yaml_snapshot!("object_compiled_invalid_version_failures", failure_messages);
    });
}

#[test]
fn object_compiled_missing_filename() {
    let source = r"Type=Exe
Object={00020430-0000-0000-C000-000000000046}#2.0#0; 
";

    let source_file = SourceFile::from_string("test.vbp", source);
    let (_project_opt, failures) = ProjectFile::parse(&source_file).unpack();

    assert!(!failures.is_empty(), "Expected parsing failures");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_project");
    settings.bind(|| {
        let failure_messages: Vec<String> = failures.iter().map(|f| format!("{f:?}")).collect();
        assert_failure_count_matches_snapshot(
            "invalid_project",
            "object_compiled_missing_filename_failures",
            failures.len(),
        );
        insta::assert_yaml_snapshot!(
            "object_compiled_missing_filename_failures",
            failure_messages
        );
    });
}

#[test]
fn module_missing_filename() {
    let source = r"Type=Exe
Module=Module1; 
";

    let source_file = SourceFile::from_string("test.vbp", source);
    let (_project_opt, failures) = ProjectFile::parse(&source_file).unpack();

    assert!(!failures.is_empty(), "Expected parsing failures");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_project");
    settings.bind(|| {
        let failure_messages: Vec<String> = failures.iter().map(|f| format!("{f:?}")).collect();
        assert_failure_count_matches_snapshot(
            "invalid_project",
            "module_missing_filename_failures",
            failures.len(),
        );
        insta::assert_yaml_snapshot!("module_missing_filename_failures", failure_messages);
    });
}

#[test]
fn class_missing_filename() {
    let source = r"Type=Exe
Class=Class1; 
";

    let source_file = SourceFile::from_string("test.vbp", source);
    let (_project_opt, failures) = ProjectFile::parse(&source_file).unpack();

    assert!(!failures.is_empty(), "Expected parsing failures");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_project");
    settings.bind(|| {
        let failure_messages: Vec<String> = failures.iter().map(|f| format!("{f:?}")).collect();
        assert_failure_count_matches_snapshot(
            "invalid_project",
            "class_missing_filename_failures",
            failures.len(),
        );
        insta::assert_yaml_snapshot!("class_missing_filename_failures", failure_messages);
    });
}

#[test]
fn designer_missing_path() {
    let source = r"Type=Exe
Designer=
";

    let source_file = SourceFile::from_string("test.vbp", source);
    let (_project_opt, failures) = ProjectFile::parse(&source_file).unpack();

    assert!(!failures.is_empty(), "Expected parsing failures");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_project");
    settings.bind(|| {
        let failure_messages: Vec<String> = failures.iter().map(|f| format!("{f:?}")).collect();
        assert_failure_count_matches_snapshot(
            "invalid_project",
            "designer_missing_path_failures",
            failures.len(),
        );
        insta::assert_yaml_snapshot!("designer_missing_path_failures", failure_messages);
    });
}

#[test]
fn form_missing_path() {
    let source = r"Type=Exe
Form=
";

    let source_file = SourceFile::from_string("test.vbp", source);
    let (_project_opt, failures) = ProjectFile::parse(&source_file).unpack();

    assert!(!failures.is_empty(), "Expected parsing failures");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_project");
    settings.bind(|| {
        let failure_messages: Vec<String> = failures.iter().map(|f| format!("{f:?}")).collect();
        assert_failure_count_matches_snapshot(
            "invalid_project",
            "form_missing_path_failures",
            failures.len(),
        );
        insta::assert_yaml_snapshot!("form_missing_path_failures", failure_messages);
    });
}

#[test]
fn usercontrol_missing_path() {
    let source = r"Type=Exe
UserControl=
";

    let source_file = SourceFile::from_string("test.vbp", source);
    let (_project_opt, failures) = ProjectFile::parse(&source_file).unpack();

    assert!(!failures.is_empty(), "Expected parsing failures");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_project");
    settings.bind(|| {
        let failure_messages: Vec<String> = failures.iter().map(|f| format!("{f:?}")).collect();
        assert_failure_count_matches_snapshot(
            "invalid_project",
            "usercontrol_missing_path_failures",
            failures.len(),
        );
        insta::assert_yaml_snapshot!("usercontrol_missing_path_failures", failure_messages);
    });
}

#[test]
fn parameter_missing_opening_quote() {
    let source = r#"Type=Exe
Title=MyProject"`
"#;

    let source_file = SourceFile::from_string("test.vbp", source);
    let (_project_opt, failures) = ProjectFile::parse(&source_file).unpack();

    assert!(!failures.is_empty(), "Expected parsing failures");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_project");
    settings.bind(|| {
        let failure_messages: Vec<String> = failures.iter().map(|f| format!("{f:?}")).collect();
        assert_failure_count_matches_snapshot(
            "invalid_project",
            "parameter_missing_opening_quote_failures",
            failures.len(),
        );
        insta::assert_yaml_snapshot!("parameter_missing_opening_quote_failures", failure_messages);
    });
}

#[test]
fn parameter_missing_closing_quote() {
    let source = r#"Type=Exe
Title="MyProject
"#;

    let source_file = SourceFile::from_string("test.vbp", source);
    let (_project_opt, failures) = ProjectFile::parse(&source_file).unpack();

    assert!(!failures.is_empty(), "Expected parsing failures");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_project");
    settings.bind(|| {
        let failure_messages: Vec<String> = failures.iter().map(|f| format!("{f:?}")).collect();
        assert_failure_count_matches_snapshot(
            "invalid_project",
            "parameter_missing_closing_quote_failures",
            failures.len(),
        );
        insta::assert_yaml_snapshot!("parameter_missing_closing_quote_failures", failure_messages);
    });
}

#[test]
fn parameter_missing_both_quotes() {
    let source = r"Type=Exe
Title=MyProject
";

    let source_file = SourceFile::from_string("test.vbp", source);
    let (_project_opt, failures) = ProjectFile::parse(&source_file).unpack();

    assert!(!failures.is_empty(), "Expected parsing failures");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_project");
    settings.bind(|| {
        let failure_messages: Vec<String> = failures.iter().map(|f| format!("{f:?}")).collect();
        assert_failure_count_matches_snapshot(
            "invalid_project",
            "parameter_missing_both_quotes_failures",
            failures.len(),
        );
        insta::assert_yaml_snapshot!("parameter_missing_both_quotes_failures", failure_messages);
    });
}

#[test]
fn parameter_invalid_enum_value() {
    let source = r#"Type=Exe
Retained="5"
"#;

    let source_file = SourceFile::from_string("test.vbp", source);
    let (_project_opt, failures) = ProjectFile::parse(&source_file).unpack();

    assert!(!failures.is_empty(), "Expected parsing failures");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_project");
    settings.bind(|| {
        let failure_messages: Vec<String> = failures.iter().map(|f| format!("{f:?}")).collect();
        assert_failure_count_matches_snapshot(
            "invalid_project",
            "parameter_invalid_enum_value_failures",
            failures.len(),
        );
        insta::assert_yaml_snapshot!("parameter_invalid_enum_value_failures", failure_messages);
    });
}

#[test]
fn dllbaseaddress_missing_value() {
    let source = r"Type=OleDll
DllBaseAddress=
";

    let source_file = SourceFile::from_string("test.vbp", source);
    let (_project_opt, failures) = ProjectFile::parse(&source_file).unpack();

    assert!(!failures.is_empty(), "Expected parsing failures");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_project");
    settings.bind(|| {
        let failure_messages: Vec<String> = failures.iter().map(|f| format!("{f:?}")).collect();
        assert_failure_count_matches_snapshot(
            "invalid_project",
            "dllbaseaddress_missing_value_failures",
            failures.len(),
        );
        insta::assert_yaml_snapshot!("dllbaseaddress_missing_value_failures", failure_messages);
    });
}

#[test]
fn dllbaseaddress_missing_hex_prefix() {
    let source = r"Type=OleDll
DllBaseAddress=11000000
";

    let source_file = SourceFile::from_string("test.vbp", source);
    let (_project_opt, failures) = ProjectFile::parse(&source_file).unpack();

    assert!(!failures.is_empty(), "Expected parsing failures");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_project");
    settings.bind(|| {
        let failure_messages: Vec<String> = failures.iter().map(|f| format!("{f:?}")).collect();
        assert_failure_count_matches_snapshot(
            "invalid_project",
            "dllbaseaddress_missing_hex_prefix_failures",
            failures.len(),
        );
        insta::assert_yaml_snapshot!(
            "dllbaseaddress_missing_hex_prefix_failures",
            failure_messages
        );
    });
}

#[test]
fn dllbaseaddress_invalid_hex() {
    let source = r"Type=OleDll
DllBaseAddress=&hGGGGGGGG
";

    let source_file = SourceFile::from_string("test.vbp", source);
    let (_project_opt, failures) = ProjectFile::parse(&source_file).unpack();

    assert!(!failures.is_empty(), "Expected parsing failures");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_project");
    settings.bind(|| {
        let failure_messages: Vec<String> = failures.iter().map(|f| format!("{f:?}")).collect();
        assert_failure_count_matches_snapshot(
            "invalid_project",
            "dllbaseaddress_invalid_hex_failures",
            failures.len(),
        );
        insta::assert_yaml_snapshot!("dllbaseaddress_invalid_hex_failures", failure_messages);
    });
}

#[test]
fn dllbaseaddress_empty_hex() {
    let source = r"Type=OleDll
DllBaseAddress=&h
";

    let source_file = SourceFile::from_string("test.vbp", source);
    let (_project_opt, failures) = ProjectFile::parse(&source_file).unpack();

    assert!(!failures.is_empty(), "Expected parsing failures");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_project");
    settings.bind(|| {
        let failure_messages: Vec<String> = failures.iter().map(|f| format!("{f:?}")).collect();
        assert_failure_count_matches_snapshot(
            "invalid_project",
            "dllbaseaddress_empty_hex_failures",
            failures.len(),
        );
        insta::assert_yaml_snapshot!("dllbaseaddress_empty_hex_failures", failure_messages);
    });
}

#[test]
fn multiple_errors_in_one_file() {
    let source = r"Type=InvalidType
Reference=*\G{invalid-uuid}#2.0#0#path#desc
Object={00020430}#2.0#0; file.ocx
Module=; Module1.bas
Class=Class1; 
Form=
Title=NoQuotes
DllBaseAddress=11000000
";

    let source_file = SourceFile::from_string("test.vbp", source);
    let (_project_opt, failures) = ProjectFile::parse(&source_file).unpack();

    // Should have multiple failures
    assert!(
        failures.len() > 5,
        "Expected multiple parsing failures, got {}",
        failures.len()
    );

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("../../snapshots/tests/invalid_syntax/invalid_project");
    settings.bind(|| {
        let failure_messages: Vec<String> = failures.iter().map(|f| format!("{f:?}")).collect();
        assert_failure_count_matches_snapshot(
            "invalid_project",
            "multiple_errors_in_one_file_failures",
            failures.len(),
        );
        insta::assert_yaml_snapshot!("multiple_errors_in_one_file_failures", failure_messages);
    });
}
