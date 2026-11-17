use vb6parse::files::ModuleFile;
use vb6parse::io::SourceFile;
use vb6semantic::{SemanticAnalyzer, SemanticError, TypeChecker, TypeInfo, VBType};

fn analyze(code: &str) -> vb6semantic::HirAnalysis {
    let source = SourceFile::from_string("MxTyping.bas", code);
    let (module, failures) = ModuleFile::parse(&source).unpack();
    assert!(failures.is_empty(), "parse failures: {failures:?}");
    SemanticAnalyzer::new()
        .analyze_module_hir(&module.expect("module should parse"))
        .expect("semantic analysis should complete")
}

const MX: &str = "Attribute VB_Name = \"MxTyping\"\n\
                  Option VB6RExtensions On\n";

#[test]
fn auto_infers_supported_literal_initializers_in_hir() {
    let result = analyze(&format!(
        "{MX}\
         Public Sub Work()\n\
             Dim count As Auto = 12\n\
             Dim longCount As Auto = 40000\n\
             Dim ratio As Auto = 1.5\n\
             Dim message As Auto = \"hello\"\n\
             Dim enabled As Auto = True\n\
             Dim objectValue As Auto = Nothing\n\
         End Sub\n"
    ));
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let program = result
        .program
        .expect("supported Auto initializers produce HIR");
    let body = &program.procedures[0].body;
    for (index, expected) in [
        VBType::Integer,
        VBType::Long,
        VBType::Double,
        VBType::String,
        VBType::Boolean,
        VBType::Nothing,
    ]
    .into_iter()
    .enumerate()
    {
        let vb6semantic::StatementKind::Declaration(declaration) = &body[index].kind else {
            panic!("expected declaration")
        };
        assert_eq!(declaration.info.ty.as_ref().unwrap().kind, expected);
    }
}

#[test]
fn auto_diagnoses_missing_unsupported_and_parameter_initializers() {
    let result = analyze(&format!(
        "{MX}\
         Public Sub Work(value As Auto)\n\
             Dim missing As Auto\n\
             Dim dynamic As Auto = BuildValue()\n\
             Dim nullValue As Auto = Null\n\
             Dim emptyValue As Auto = Empty\n\
         End Sub\n"
    ));
    assert!(result.program.is_none());
    let diagnostics: Vec<_> = result
        .errors
        .iter()
        .filter_map(|error| match error {
            SemanticError::UnsupportedFeature {
                feature, reason, ..
            } if feature == "MX S04 As Auto" => Some(reason.as_str()),
            _ => None,
        })
        .collect();
    assert_eq!(diagnostics.len(), 5, "{:?}", result.errors);
    assert!(
        diagnostics
            .iter()
            .any(|reason| reason.contains("parameter"))
    );
    assert!(
        diagnostics
            .iter()
            .filter(|reason| reason.contains("initializer"))
            .count()
            >= 4
    );
}

#[test]
fn option_strict_enforces_known_typed_assignments_only_with_mx_opt_in() {
    let good = analyze(&format!(
        "{MX}Option Strict On\n\
         Public Sub Work()\n\
             Dim count As Long\n\
             Dim source As Long\n\
             count = source\n\
         End Sub\n"
    ));
    assert!(good.errors.is_empty(), "{:?}", good.errors);
    assert!(good.program.is_some());

    let widening = analyze(&format!(
        "{MX}Option Strict On\n\
         Public Sub Work()\n\
             Dim target As Double\n\
             Dim source As Long\n\
             target = source\n\
         End Sub\n"
    ));
    assert!(widening.errors.is_empty(), "{:?}", widening.errors);
    assert!(widening.program.is_some());

    let bad = analyze(&format!(
        "{MX}Option Strict On\n\
         Public Sub Work()\n\
             Dim count As Integer\n\
             Dim source As Long\n\
             count = source\n\
         End Sub\n"
    ));
    assert!(bad.program.is_none());
    assert!(bad.errors.iter().any(|error| matches!(
        error,
        SemanticError::TypeMismatch { expected, found, .. }
            if expected == "Integer" && found == "Long"
    )));

    TypeChecker::new()
        .check_strict_assignment(
            &TypeInfo::variant(),
            &TypeInfo::new(VBType::Long),
            &vb6semantic::SourceLocation {
                file: "test.bas".to_string(),
                line: 1,
                column: 1,
            },
        )
        .expect("assignment to Variant is allowed");

    let variant_to_long = TypeChecker::new()
        .check_strict_assignment(
            &TypeInfo::new(VBType::Long),
            &TypeInfo::variant(),
            &vb6semantic::SourceLocation {
                file: "test.bas".to_string(),
                line: 1,
                column: 1,
            },
        )
        .unwrap_err();
    assert!(matches!(
        variant_to_long,
        SemanticError::TypeMismatch { expected, found, .. }
            if expected == "Long" && found == "Variant"
    ));

    let unknown = TypeChecker::new()
        .check_strict_assignment(
            &TypeInfo::new(VBType::Long),
            &TypeInfo::unknown(),
            &vb6semantic::SourceLocation {
                file: "test.bas".to_string(),
                line: 1,
                column: 1,
            },
        )
        .unwrap_err();
    assert!(matches!(
        unknown,
        SemanticError::UnsupportedFeature { feature, reason, .. }
            if feature == "MX S08 Option Strict" && reason.contains("unknown")
    ));
}

#[test]
fn strict_mode_and_auto_do_not_change_legacy_variant_declarations_or_conversions() {
    let legacy = analyze(
        "Attribute VB_Name = \"Legacy\"\n\
         Public Sub Work()\n\
             Dim value\n\
             Dim count As Long\n\
             count = 12\n\
         End Sub\n",
    );
    assert!(legacy.errors.is_empty(), "{:?}", legacy.errors);
    let program = legacy.program.expect("legacy assignment remains accepted");
    let vb6semantic::StatementKind::Declaration(declaration) = &program.procedures[0].body[0].kind
    else {
        panic!("expected declaration")
    };
    assert_eq!(declaration.info.ty.as_ref().unwrap().kind, VBType::Variant);

    let strict_without_mx = analyze(
        "Attribute VB_Name = \"Legacy\"\n\
         ' Option VB6RExtensions On\n\
         Option Strict On\n\
         Public Sub Work()\n\
             Dim count As Long\n\
             Dim source As Integer\n\
             count = source\n\
         End Sub\n",
    );
    assert!(
        strict_without_mx.program.is_some(),
        "{:?}",
        strict_without_mx.errors
    );
    assert!(
        !strict_without_mx.errors.iter().any(|error| matches!(
            error,
            SemanticError::UnsupportedFeature { feature, .. }
                if feature == "MX S08 Option Strict"
        )),
        "{:?}",
        strict_without_mx.errors
    );
}

#[test]
fn as_auto_is_rejected_without_mx_opt_in() {
    let result = analyze(
        "Attribute VB_Name = \"Legacy\"\n\
         Public Sub Work(value As Auto)\n\
             Dim inferred As Auto = 12\n\
         End Sub\n",
    );
    assert!(result.program.is_none());
    assert_eq!(
        result
            .errors
            .iter()
            .filter(|error| matches!(
                error,
                SemanticError::UnsupportedFeature {
                    feature,
                    reason,
                    ..
                } if feature == "MX S04 As Auto" && reason.contains("requires Option VB6RExtensions On")
            ))
            .count(),
        2
    );
}

#[test]
fn null_empty_and_nothing_keep_distinct_hir_types() {
    let kinds = [VBType::Nothing, VBType::Null, VBType::Empty];
    assert_ne!(kinds[0], kinds[1]);
    assert_ne!(kinds[1], kinds[2]);
    assert_ne!(kinds[0], kinds[2]);
}
