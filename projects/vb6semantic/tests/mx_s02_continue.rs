use vb6parse::files::ModuleFile;
use vb6parse::io::SourceFile;
use vb6semantic::{ContinueTarget, SemanticAnalyzer, SemanticError, StatementKind};

fn analyze(code: &str) -> vb6semantic::HirAnalysis {
    let source = SourceFile::from_string("MxContinue.bas", code);
    let (module, failures) = ModuleFile::parse(&source).unpack();
    assert!(failures.is_empty(), "parse failures: {failures:?}");
    SemanticAnalyzer::new()
        .analyze_module_hir(&module.expect("module should parse"))
        .expect("semantic analysis should complete")
}

const MX: &str = "Attribute VB_Name = \"MxContinue\"\n\
                  Option VB6RExtensions On\n";

#[test]
fn continue_for_and_do_are_lowered_inside_matching_loops() {
    let result = analyze(&format!(
        "{MX}\
         Public Sub Work()\n\
             Dim i As Integer\n\
             For i = 1 To 3\n\
                 Continue For\n\
             Next i\n\
             Do While True\n\
                 Continue Do\n\
             Loop\n\
         End Sub\n"
    ));
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let program = result
        .program
        .expect("matching continues should produce HIR");
    assert!(matches!(
        program.procedures[0].body[1].kind,
        StatementKind::For { .. }
    ));
    assert!(matches!(
        program.procedures[0].body[2].kind,
        StatementKind::Loop { .. }
    ));
    let StatementKind::For { body, .. } = &program.procedures[0].body[1].kind else {
        panic!("expected For loop")
    };
    assert!(matches!(
        body[0].kind,
        StatementKind::Continue(ContinueTarget::For)
    ));
}

#[test]
fn continue_requires_an_enclosing_loop_of_the_requested_kind() {
    let result = analyze(&format!(
        "{MX}\
         Public Sub Work()\n\
             Dim i As Integer\n\
             For i = 1 To 3\n\
                 Continue Do\n\
             Next i\n\
         End Sub\n"
    ));
    assert!(result.program.is_none());
    assert!(result.errors.iter().any(|error| matches!(
        error,
        SemanticError::InvalidOperation { message, .. }
            if message.contains("Continue Do") && message.contains("matching loop")
    )));
}

#[test]
fn continue_inside_nested_other_loop_can_target_matching_outer_loop() {
    let result = analyze(&format!(
        "{MX}\
         Public Sub Work()\n\
             Dim i As Integer\n\
             Do While True\n\
                 For i = 1 To 3\n\
                     Continue Do\n\
                 Next i\n\
             Loop\n\
         End Sub\n"
    ));
    assert!(result.errors.is_empty(), "{:?}", result.errors);
}

#[test]
fn continue_do_does_not_target_a_while_loop() {
    let result = analyze(&format!(
        "{MX}\
         Public Sub Work()\n\
             While True\n\
                 Continue Do\n\
             Wend\n\
         End Sub\n"
    ));
    assert!(result.program.is_none());
    assert!(result.errors.iter().any(|error| matches!(
        error,
        SemanticError::InvalidOperation { message, .. }
            if message.contains("Continue Do") && message.contains("matching loop")
    )));
}

#[test]
fn continue_do_skips_an_inner_while_for_an_outer_do() {
    let result = analyze(&format!(
        "{MX}\
         Public Sub Work()\n\
             Do While True\n\
                 While True\n\
                     Continue Do\n\
                 Wend\n\
             Loop\n\
         End Sub\n"
    ));
    assert!(result.errors.is_empty(), "{:?}", result.errors);
}
