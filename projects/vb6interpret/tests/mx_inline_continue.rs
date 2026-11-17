use vb6interpret::run_source;
use vb6parse::parsers::{SyntaxKind, cst::ConcreteSyntaxTree};

fn has_kind(source: &str, kind: SyntaxKind) -> bool {
    let (cst, failures) = ConcreteSyntaxTree::from_text("inline.bas", source).unpack();
    assert!(failures.is_empty(), "source should parse cleanly");
    cst.expect("CST should be available")
        .to_root_node()
        .find(kind)
        .is_some()
}
#[test]
fn mx_continue_for_is_executable_in_single_line_if() {
    let source = "Attribute VB_Name = \"M\"\n\
        Option VB6RExtensions On\n\
        Sub Main()\n\
            Dim i As Long\n\
            For i = 1 To 4\n\
                If i = 2 Then Continue For\n\
                Debug.Print i\n\
            Next i\n\
        End Sub\n";

    assert!(has_kind(source, SyntaxKind::ContinueStatement));
    assert_eq!(
        run_source(source).expect("inline Continue For should execute"),
        vec!["1", "3", "4"]
    );
}

#[test]
fn mx_continue_do_is_executable_in_single_line_if() {
    let source = "Attribute VB_Name = \"M\"\n\
        Option VB6RExtensions On\n\
        Sub Main()\n\
            Dim i As Long\n\
            i = 0\n\
            Do\n\
                i = i + 1\n\
                If i < 3 Then Continue Do\n\
                Debug.Print i\n\
            Loop While i < 4\n\
        End Sub\n";

    assert!(has_kind(source, SyntaxKind::ContinueStatement));
    assert_eq!(
        run_source(source).expect("inline Continue Do should execute"),
        vec!["3", "4"]
    );
}

#[test]
fn inline_continue_remains_a_legacy_procedure_call_without_opt_in() {
    let source = "Attribute VB_Name = \"M\"\n\
        Sub Main()\n\
            Dim i As Long\n\
            For i = 1 To 2\n\
                If i = 1 Then Continue For\n\
            Next i\n\
        End Sub\n";

    assert!(has_kind(source, SyntaxKind::CallStatement));
    assert!(!has_kind(source, SyntaxKind::ContinueStatement));
    let error = run_source(source).expect_err("legacy Continue must not enable MX semantics");
    assert_eq!(error.error.number, 35);
}
