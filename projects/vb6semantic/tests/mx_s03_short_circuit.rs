use vb6parse::files::ModuleFile;
use vb6parse::io::SourceFile;
use vb6semantic::{ExpressionKind, SemanticAnalyzer, SemanticError, StatementKind, VBType};

fn analyze(code: &str) -> vb6semantic::HirAnalysis {
    let source = SourceFile::from_string("MxShortCircuit.bas", code);
    let (module, failures) = ModuleFile::parse(&source).unpack();
    assert!(failures.is_empty(), "parse failures: {failures:?}");
    SemanticAnalyzer::new()
        .analyze_module_hir(&module.expect("module should parse"))
        .expect("semantic analysis should complete")
}

const MX: &str = "Attribute VB_Name = \"MxShortCircuit\"\n\
                  Option VB6RExtensions On\n";

#[test]
fn short_circuit_hir_preserves_lazy_operator_and_boolean_result() {
    let result = analyze(&format!(
        "{MX}\
         Public Sub Work()\n\
             Dim ready As Boolean\n\
             If ready AndAlso True Then\n\
             End If\n\
         End Sub\n"
    ));
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let program = result
        .program
        .expect("valid short-circuit should produce HIR");
    let StatementKind::If { condition, .. } = &program.procedures[0].body[1].kind else {
        panic!("expected If")
    };
    assert_eq!(condition.info.ty.as_ref().unwrap().kind, VBType::Boolean);
    assert!(matches!(
        condition.kind,
        ExpressionKind::ShortCircuit { .. }
    ));
}

#[test]
fn short_circuit_operands_use_boolean_conversions_without_merging_null_states() {
    let result = analyze(&format!(
        "{MX}\
         Public Sub Work()\n\
             If Null AndAlso Empty Then\n\
             End If\n\
         End Sub\n"
    ));
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let program = result
        .program
        .expect("VB sentinel conversions should lower");
    let StatementKind::If { condition, .. } = &program.procedures[0].body[0].kind else {
        panic!("expected If")
    };
    let ExpressionKind::ShortCircuit { left, right, .. } = &condition.kind else {
        panic!("expected lazy short-circuit node")
    };
    let ExpressionKind::Conversion {
        expression: left_value,
        ..
    } = &left.kind
    else {
        panic!("Null operand should retain a Boolean conversion")
    };
    let ExpressionKind::Conversion {
        expression: right_value,
        ..
    } = &right.kind
    else {
        panic!("Empty operand should retain a Boolean conversion")
    };
    assert_eq!(left_value.info.ty.as_ref().unwrap().kind, VBType::Null);
    assert_eq!(right_value.info.ty.as_ref().unwrap().kind, VBType::Empty);
}

#[test]
fn short_circuit_rejects_object_operands_statically() {
    let result = analyze(&format!(
        "{MX}\
         Public Sub Work()\n\
             Dim target As Object\n\
             If target AndAlso True Then\n\
             End If\n\
         End Sub\n"
    ));
    assert!(result.program.is_none());
    assert!(result.errors.iter().any(|error| matches!(
        error,
        SemanticError::InvalidOperation { message, .. }
            if message.contains("Boolean conversion") && message.contains("Object")
    )));
}
