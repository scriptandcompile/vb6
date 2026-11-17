use vb6parse::files::{ClassFile, ModuleFile};
use vb6parse::io::SourceFile;
use vb6semantic::{SemanticAnalyzer, StatementKind};

fn analyze(code: &str) -> vb6semantic::HirAnalysis {
    let source = SourceFile::from_string("Typed.bas", code);
    let (module, failures) = ModuleFile::parse(&source).unpack();
    assert!(failures.is_empty(), "parse failures: {failures:?}");
    SemanticAnalyzer::new()
        .analyze_module_hir(&module.expect("module should parse"))
        .expect("module analysis should complete")
}

#[test]
fn constructs_typed_hir_with_procedure_parameters_and_spans() {
    let result = analyze(
        "Attribute VB_Name = \"Typed\"\n\
         Public Sub Work(ByVal amount As Long, Optional ByRef label As String)\n\
             Dim total As Long\n\
             total = amount + 1\n\
         End Sub\n",
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let program = result.program.expect("valid VB6 should produce HIR");
    assert!(program.compilable);
    let procedure = &program.procedures[0];
    assert_eq!(procedure.name, "Work");
    assert_eq!(procedure.signature.parameters.len(), 2);
    assert!(procedure.signature.parameters[0].info.symbol.is_some());
    assert_eq!(procedure.body.len(), 2);
    assert!(matches!(
        procedure.body[0].kind,
        StatementKind::Declaration(_)
    ));
    let StatementKind::Assignment { value, .. } = &procedure.body[1].kind else {
        panic!("expected typed assignment")
    };
    assert_eq!(
        value.info.ty.as_ref().unwrap().kind,
        vb6semantic::VBType::Long
    );
    let vb6semantic::ExpressionKind::Binary { right, .. } = &value.kind else {
        panic!("expected binary expression")
    };
    assert!(matches!(
        right.kind,
        vb6semantic::ExpressionKind::Conversion { .. }
    ));
    assert!(procedure.info.span.end > procedure.info.span.start);
    assert!(value.info.span.end > value.info.span.start);
}

#[test]
fn associates_native_kinds_with_source_expression_forms() {
    let result = analyze(
        "Attribute VB_Name = \"Typed\"\n\
         Public Function Helper(ByVal value As Long) As Long\n\
             Helper = value\n\
         End Function\n\
         Public Function Work(ByVal value As Long) As Long\n\
             Work = (Helper(value + 1))\n\
         End Function\n",
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let program = result.program.expect("valid VB6 should produce HIR");
    let StatementKind::Assignment { value, .. } = &program.procedures[1].body[0].kind else {
        panic!("expected function-result assignment")
    };
    assert_eq!(value.native_ast.kind, Some(0x65));
    let vb6semantic::ExpressionKind::Parenthesized(call) = &value.kind else {
        panic!("expected parenthesized function call")
    };
    assert_eq!(call.native_ast.kind, Some(0x61));
    let vb6semantic::ExpressionKind::Call { callee, arguments } = &call.kind else {
        panic!("expected function call")
    };
    assert_eq!(callee.native_ast.kind, Some(0x0f));
    let vb6semantic::ExpressionKind::Binary { right, .. } = &arguments[0].kind else {
        panic!("expected binary argument")
    };
    assert_eq!(right.native_ast.kind, Some(0x0e));
}

#[test]
fn lowers_user_type_members_and_array_indexing() {
    let result = analyze(
        "Attribute VB_Name = \"Typed\"\n\
         Private Type Point\n\
             X As Long\n\
         End Type\n\
         Public Function Work(ByVal index As Long) As Long\n\
             Dim point As Point\n\
             Dim values(0 To 3) As Long\n\
             point.X = values(index)\n\
             Work = point.X\n\
         End Function\n",
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let program = result.program.expect("valid VB6 should produce HIR");
    let StatementKind::Assignment { target, value } = &program.procedures[0].body[2].kind else {
        panic!("expected member assignment")
    };
    assert_eq!(target.native_ast.kind, Some(0x68));
    assert_eq!(value.native_ast.kind, Some(0x6a));
    assert_eq!(
        value.info.ty.as_ref().unwrap().kind,
        vb6semantic::VBType::Long
    );
    let StatementKind::Assignment { value, .. } = &program.procedures[0].body[3].kind else {
        panic!("expected function-result assignment")
    };
    assert_eq!(value.native_ast.kind, Some(0x68));
}

#[test]
fn symbol_binding_is_case_insensitive() {
    let result = analyze(
        "Attribute VB_Name = \"Typed\"\n\
         Public Sub Work()\n\
             Dim Amount As Long\n\
             amount = 1\n\
         End Sub\n",
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let program = result
        .program
        .expect("case-insensitive binding should succeed");
    let StatementKind::Assignment { target, .. } = &program.procedures[0].body[1].kind else {
        panic!("expected assignment")
    };
    assert_eq!(target.info.symbol.as_ref().unwrap().name, "amount");
}

#[test]
fn invalid_assignment_and_unsupported_control_flow_do_not_produce_compilable_hir() {
    let invalid = analyze(
        "Attribute VB_Name = \"Typed\"\n\
         Public Sub Work()\n\
             Dim amount As Long\n\
             amount = \"not numeric\"\n\
         End Sub\n",
    );
    assert!(invalid.program.is_none(), "{invalid:?}");
    assert!(!invalid.errors.is_empty());

    let unsupported = analyze(
        "Attribute VB_Name = \"Typed\"\n\
         Public Sub Work()\n\
             Dim amount As Long\n\
             Dim values As Variant\n\
             For Each amount In values\n\
                 amount = amount + 1\n\
             Next amount\n\
         End Sub\n",
    );
    assert!(unsupported.program.is_none());
    assert!(!unsupported.errors.is_empty());
}

#[test]
fn lowers_counted_for_loop_bounds() {
    let result = analyze(
        "Attribute VB_Name = \"Typed\"\n\
         Public Sub Work()\n\
             Dim amount As Long\n\
             For amount = 1 To 3 Step 1\n\
                 amount = amount + 1\n\
             Next amount\n\
         End Sub\n",
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let program = result.program.expect("counted For should produce HIR");
    let StatementKind::For {
        variable,
        start,
        end,
        step,
        body,
    } = &program.procedures[0].body[1].kind
    else {
        panic!("expected counted For")
    };
    assert_eq!(
        variable.info.ty.as_ref().unwrap().kind,
        vb6semantic::VBType::Long
    );
    assert_eq!(
        start.info.ty.as_ref().unwrap().kind,
        vb6semantic::VBType::Long
    );
    assert_eq!(
        end.info.ty.as_ref().unwrap().kind,
        vb6semantic::VBType::Long
    );
    assert!(step.is_some());
    assert_eq!(body.len(), 1);
}

#[test]
fn lowers_structured_if_branches() {
    let result = analyze(
        "Attribute VB_Name = \"Typed\"\n\
         Public Sub Work()\n\
             Dim amount As Long\n\
             If True Then\n\
                 amount = 1\n\
             Else\n\
                 amount = 2\n\
             End If\n\
         End Sub\n",
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let program = result.program.expect("supported If should produce HIR");
    let StatementKind::If {
        then_body,
        else_body,
        ..
    } = &program.procedures[0].body[1].kind
    else {
        panic!("expected structured If statement")
    };
    assert_eq!(then_body.len(), 1);
    assert_eq!(else_body.len(), 1);
}

#[test]
fn lowers_while_loop_with_condition_and_body() {
    let result = analyze(
        "Attribute VB_Name = \"Typed\"\n\
         Public Sub Work()\n\
             Dim amount As Long\n\
             While amount < 3\n\
                 amount = amount + 1\n\
             Wend\n\
         End Sub\n",
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let program = result.program.expect("While should produce typed loop HIR");
    let StatementKind::Loop {
        condition,
        post_test,
        until,
        body,
        ..
    } = &program.procedures[0].body[1].kind
    else {
        panic!("expected loop")
    };
    assert!(condition.is_some());
    assert!(!post_test);
    assert!(!until);
    assert_eq!(body.len(), 1);
}

#[test]
fn constructs_class_hir_and_retains_property_accessor_scopes() {
    let source = SourceFile::from_string(
        "Person.cls",
        "VERSION 1.0 CLASS\n\
         BEGIN\n\
           MultiUse = -1\n\
           Persistable = 0\n\
           DataBindingBehavior = 0\n\
           DataSourceBehavior = 0\n\
           MTSTransactionMode = 0\n\
         END\n\
         Attribute VB_Name = \"Person\"\n\
         Attribute VB_GlobalNameSpace = False\n\
         Attribute VB_Creatable = True\n\
         Attribute VB_PredeclaredId = False\n\
         Attribute VB_Exposed = False\n\
         Private m_Name As String\n\
         Public Property Get DisplayName() As String\n\
             DisplayName = m_Name\n\
         End Property\n\
         Public Property Let DisplayName(ByVal value As String)\n\
             m_Name = value\n\
         End Property\n\
         Public Function GetInfo() As String\n\
             GetInfo = m_Name & \" is \" & m_Name\n\
         End Function\n",
    );
    let (class, failures) = ClassFile::parse(&source).unpack();
    assert!(failures.is_empty(), "parse failures: {failures:?}");
    let result = SemanticAnalyzer::new()
        .analyze_class_hir(&class.expect("class should parse"))
        .expect("class analysis should complete");
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let program = result.program.expect("class should produce typed HIR");
    assert_eq!(program.unit_kind, vb6semantic::UnitKind::Class);
    assert!(
        program
            .procedures
            .iter()
            .any(|procedure| procedure.name == "DisplayName")
    );
}

#[test]
fn binds_calls_and_checks_argument_arity() {
    let valid = analyze(
        "Attribute VB_Name = \"Typed\"\n\
         Public Function Fetch(ByVal index As Long) As Long\n\
         End Function\n\
         Public Sub Work()\n\
             Dim amount As Long\n\
             amount = Fetch(1)\n\
         End Sub\n",
    );
    assert!(valid.errors.is_empty(), "{:?}", valid.errors);
    let program = valid.program.expect("valid function call should lower");
    let StatementKind::Assignment { value, .. } = &program.procedures[1].body[1].kind else {
        panic!("expected assignment")
    };
    assert!(matches!(
        value.kind,
        vb6semantic::ExpressionKind::Call { .. }
    ));

    let invalid = analyze(
        "Attribute VB_Name = \"Typed\"\n\
         Public Function Fetch(ByVal index As Long) As Long\n\
         End Function\n\
         Public Sub Work()\n\
             Dim amount As Long\n\
             amount = Fetch()\n\
         End Sub\n",
    );
    assert!(invalid.program.is_none(), "{invalid:?}");
    assert!(!invalid.errors.is_empty());
}
