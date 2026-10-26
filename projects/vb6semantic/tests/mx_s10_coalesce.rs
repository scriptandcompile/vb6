use vb6parse::files::ModuleFile;
use vb6parse::io::SourceFile;
use vb6semantic::{ExpressionKind, SemanticAnalyzer, StatementKind, VBType};

fn analyze(code: &str) -> vb6semantic::HirAnalysis {
    let source = SourceFile::from_string("MxCoalesce.bas", code);
    let (module, failures) = ModuleFile::parse(&source).unpack();
    assert!(failures.is_empty(), "parse failures: {failures:?}");
    SemanticAnalyzer::new()
        .analyze_module_hir(&module.expect("module should parse"))
        .expect("semantic analysis should complete")
}

const MX: &str = "Attribute VB_Name = \"MxCoalesce\"\n\
                  Option VB6RExtensions On\n";

fn returned_expression(result: &vb6semantic::HirAnalysis) -> &vb6semantic::Expression {
    let program = result
        .program
        .as_ref()
        .expect("valid source should produce HIR");
    program.procedures[0]
        .body
        .iter()
        .find_map(|statement| match &statement.kind {
            StatementKind::Return(value) => Some(value),
            _ => None,
        })
        .expect("expected Return expression")
}

#[test]
fn nothing_coalescing_infers_the_fallback_type_and_keeps_lazy_node() {
    let result = analyze(&format!(
        "{MX}\
         Public Function Result() As Long\n\
             Return Nothing ?? 12\n\
         End Function\n"
    ));
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let value = returned_expression(&result);
    let expression = match &value.kind {
        ExpressionKind::Conversion { expression, .. } => expression.as_ref(),
        _ => value,
    };
    assert_eq!(expression.info.ty.as_ref().unwrap().kind, VBType::Integer);
    assert!(matches!(expression.kind, ExpressionKind::Coalesce { .. }));
}

#[test]
fn null_and_empty_coalescing_retain_their_sentinel_types() {
    for sentinel in [("Null", VBType::Null), ("Empty", VBType::Empty)] {
        let result = analyze(&format!(
            "{MX}\
             Public Function Result() As Variant\n\
                 Return {} ?? 12\n\
             End Function\n",
            sentinel.0
        ));
        assert!(result.errors.is_empty(), "{:?}", result.errors);
        let value = returned_expression(&result);
        let ExpressionKind::Conversion { expression, .. } = &value.kind else {
            panic!("sentinel return to Variant should be explicit")
        };
        assert_eq!(expression.info.ty.as_ref().unwrap().kind, sentinel.1);
        let ExpressionKind::Coalesce { left, right } = &expression.kind else {
            panic!("expected lazy coalescing node")
        };
        assert_eq!(left.info.ty.as_ref().unwrap().kind, sentinel.1);
        assert_eq!(right.info.ty.as_ref().unwrap().kind, VBType::Integer);
    }
}

#[test]
fn variant_coalescing_keeps_a_dynamic_result_type() {
    let result = analyze(&format!(
        "{MX}\
         Public Function Result() As Variant\n\
             Dim source As Variant\n\
             Return source ?? 12\n\
         End Function\n"
    ));
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let value = returned_expression(&result);
    let ExpressionKind::Coalesce { left, right } = &value.kind else {
        panic!("expected lazy coalescing node")
    };
    assert_eq!(value.info.ty.as_ref().unwrap().kind, VBType::Variant);
    assert_eq!(left.info.ty.as_ref().unwrap().kind, VBType::Variant);
    assert_eq!(right.info.ty.as_ref().unwrap().kind, VBType::Integer);
}
