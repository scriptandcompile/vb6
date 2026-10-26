use vb6parse::files::ModuleFile;
use vb6parse::io::SourceFile;
use vb6semantic::{ExpressionKind, SemanticAnalyzer, SemanticError, StatementKind, VBType};

fn analyze(code: &str) -> vb6semantic::HirAnalysis {
    let source = SourceFile::from_string("MxReturn.bas", code);
    let (module, failures) = ModuleFile::parse(&source).unpack();
    assert!(failures.is_empty(), "parse failures: {failures:?}");
    SemanticAnalyzer::new()
        .analyze_module_hir(&module.expect("module should parse"))
        .expect("semantic analysis should complete")
}

const MX: &str = "Attribute VB_Name = \"MxReturn\"\n\
                  Option VB6RExtensions On\n";

#[test]
fn return_expression_is_typed_and_coerced_in_the_hir() {
    let result = analyze(&format!(
        "{MX}\
         Public Function Result() As Long\n\
             Dim source As Integer\n\
             Return source\n\
         End Function\n"
    ));
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let program = result.program.expect("valid return should produce HIR");
    let StatementKind::Return(value) = &program.procedures[0].body[1].kind else {
        panic!("expected return expression")
    };
    let ExpressionKind::Conversion { target_type, .. } = &value.kind else {
        panic!("Integer to Long return should carry a conversion")
    };
    assert_eq!(target_type.kind, VBType::Long);
}

#[test]
fn return_expression_rejects_incompatible_static_types() {
    let result = analyze(&format!(
        "{MX}\
         Public Function Result() As Long\n\
             Return \"text\"\n\
         End Function\n"
    ));
    assert!(result.program.is_none());
    assert!(result.errors.iter().any(|error| matches!(
        error,
        SemanticError::TypeMismatch { expected, .. } if expected == "Long"
    )));
}

#[test]
fn variant_return_expression_keeps_a_runtime_conversion() {
    let result = analyze(&format!(
        "{MX}\
         Public Function Result() As Long\n\
             Dim source As Variant\n\
             Return source\n\
         End Function\n"
    ));
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let program = result
        .program
        .expect("Variant conversion should be retained");
    let StatementKind::Return(value) = &program.procedures[0].body[1].kind else {
        panic!("expected return expression")
    };
    let ExpressionKind::Conversion { target_type, .. } = &value.kind else {
        panic!("Variant result conversion must remain explicit")
    };
    assert_eq!(target_type.kind, VBType::Long);
}

#[test]
fn return_expression_is_rejected_in_a_sub() {
    let result = analyze(&format!(
        "{MX}\
         Public Sub Work()\n\
             Return 1\n\
         End Sub\n"
    ));
    assert!(result.program.is_none());
    assert!(result.errors.iter().any(|error| matches!(
        error,
        SemanticError::InvalidOperation { message, .. }
            if message.contains("only in a Function")
    )));
}
