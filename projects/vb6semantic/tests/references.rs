//! Integration tests for the usage/definition/references query layer.
//!
//! These assert that the analyzer records precise, resolvable identifier
//! occurrences: definitions at declaration sites, usages (including forward
//! references) at call sites, and type references in `As` clauses.

use std::fs;

use tempfile::tempdir;
use vb6parse::files::{ModuleFile, ProjectFile};
use vb6parse::io::SourceFile;
use vb6semantic::query::{QueryIndex, ReferenceKind};
use vb6semantic::{SemanticAnalyzer, scope::ScopeKind};

const MODULE: &str = "Attribute VB_Name = \"MathUtils\"
Option Explicit

Public Type Point
    X As Double
End Type

Public Function Square(x As Double) As Double
    Square = x * x
End Function

Public Sub Report()
    Dim origin As Point
    Dim value As Double
    value = Square(4)
    Debug.Print Square(value)
End Sub
";

const GREETER: &str = "Attribute VB_Name = \"Greeter\"
Option Explicit
Public Function Greet(who As String) As String
    Greet = \"Hi \" & who
End Function
";

const RUNNER: &str = "Attribute VB_Name = \"Runner\"
Option Explicit
Public Sub Run()
    Dim msg As String
    msg = Greet(\"World\")
End Sub
";

fn analyze_module(code: &str) -> SemanticAnalyzer {
    let source = SourceFile::from_string("MathUtils.bas", code);
    let (module_opt, failures) = ModuleFile::parse(&source).unpack();
    assert!(failures.is_empty(), "parse failures: {:?}", failures);
    let module = module_opt.expect("module should parse");

    let mut analyzer = SemanticAnalyzer::new();
    analyzer
        .analyze_module(&module)
        .expect("analysis should succeed");
    analyzer
}

/// The scope id of the module whose scope name is `name`.
fn module_scope_id(analyzer: &SemanticAnalyzer, name: &str) -> usize {
    analyzer
        .scope_manager()
        .get_scopes_by_kind(ScopeKind::Global)
        .into_iter()
        .find(|scope| scope.name == name)
        .expect("module scope")
        .id
}

/// The procedure scope of the procedure named `name`.
fn procedure_scope_id(analyzer: &SemanticAnalyzer, name: &str) -> usize {
    analyzer
        .scope_manager()
        .get_scopes_by_kind(ScopeKind::Procedure)
        .into_iter()
        .find(|scope| scope.name == name)
        .expect("procedure scope")
        .id
}

/// The (line, column) of every occurrence of the given symbol.
fn occurrences(index: &QueryIndex, scope_id: usize, name: &str) -> Vec<(usize, usize)> {
    index
        .references_for(scope_id, name)
        .map(|refs| {
            refs.iter()
                .map(|r| (r.location.line, r.location.column))
                .collect()
        })
        .unwrap_or_default()
}

#[test]
fn module_records_definitions_usages_and_type_references() {
    let analyzer = analyze_module(MODULE);
    assert!(
        analyzer.errors().is_empty(),
        "errors: {:?}",
        analyzer.errors()
    );

    let index = analyzer.query_index();
    let module_scope = module_scope_id(&analyzer, "MathUtils");

    // Square: one definition, one self-assignment, and two call sites.
    assert_eq!(
        occurrences(index, module_scope, "square"),
        vec![(8, 17), (9, 5), (15, 13), (16, 17)]
    );
    let square_refs = index.references_for(module_scope, "square").unwrap();
    assert_eq!(
        square_refs
            .iter()
            .filter(|r| r.kind == ReferenceKind::Definition)
            .count(),
        1
    );

    // Point: the declaration plus the `As Point` in Report.
    assert_eq!(
        occurrences(index, module_scope, "point"),
        vec![(4, 13), (13, 19)]
    );
    let point_refs = index.references_for(module_scope, "point").unwrap();
    assert_eq!(point_refs[1].kind, ReferenceKind::TypeReference);

    // Parameter `x`: definition plus both operands of `x * x`.
    let square_proc = procedure_scope_id(&analyzer, "Square");
    assert_eq!(
        occurrences(index, square_proc, "x"),
        vec![(8, 24), (9, 14), (9, 18)]
    );
}

#[test]
fn procedure_local_dims_are_not_collected() {
    let analyzer = analyze_module(MODULE);
    let index = analyzer.query_index();
    let report_scope = procedure_scope_id(&analyzer, "Report");

    // `origin` and `value` are procedure-local and intentionally out of v1 scope.
    assert!(index.references_for(report_scope, "origin").is_none());
    assert!(index.references_for(report_scope, "value").is_none());
}

#[test]
fn symbol_at_resolves_cursor_to_symbol() {
    let analyzer = analyze_module(MODULE);
    let index = analyzer.query_index();
    let module_scope = module_scope_id(&analyzer, "MathUtils");

    // Inside the `Square = x * x` line: the identifier spans columns 5..10.
    let key = index.symbol_at("MathUtils", 9, 5).expect("start");
    assert_eq!((key.scope_id, key.name.as_str()), (module_scope, "square"));
    assert!(index.symbol_at("MathUtils", 9, 10).is_some());
    assert!(index.symbol_at("MathUtils", 9, 11).is_none());
    assert!(index.symbol_at("MathUtils", 9, 4).is_none());

    // references_at / definition_at resolve through the cursor.
    assert_eq!(index.references_at("MathUtils", 9, 5).unwrap().len(), 4);
    let definition = index.definition_at("MathUtils", 9, 5).expect("definition");
    assert_eq!(definition.location.line, 8);
}

#[test]
fn cross_module_usage_resolves_across_files() {
    let temp_dir = tempdir().expect("temporary directory");
    let greeter_path = temp_dir.path().join("Greeter.bas");
    fs::write(&greeter_path, GREETER).unwrap();
    let runner_path = temp_dir.path().join("Runner.bas");
    fs::write(&runner_path, RUNNER).unwrap();

    let project_source = format!(
        "Type=Exe\n\
         Module=Greeter; {}\n\
         Module=Runner; {}\n",
        greeter_path.display(),
        runner_path.display()
    );
    let source = SourceFile::from_string("Project1.vbp", project_source);
    let (project_opt, failures) = ProjectFile::parse(&source).unpack();
    assert!(failures.is_empty(), "parse failures: {:?}", failures);
    let project = project_opt.expect("project should parse");

    let mut analyzer = SemanticAnalyzer::new();
    analyzer
        .analyze_project(&project)
        .expect("analysis should succeed");
    assert!(
        analyzer.errors().is_empty(),
        "errors: {:?}",
        analyzer.errors()
    );

    let index = analyzer.query_index();
    let greeter_scope = module_scope_id(&analyzer, "Greeter");

    // The call site in Runner.bas resolves back to Greeter's definition.
    assert_eq!(
        occurrences(index, greeter_scope, "greet"),
        vec![(3, 17), (4, 5), (5, 11)]
    );

    // The definition and both usages all point at the same symbol.
    let greet_refs = index.references_for(greeter_scope, "greet").unwrap();
    assert_eq!(greet_refs.len(), 3);
    assert_eq!(
        greet_refs
            .iter()
            .map(|r| r.location.file.as_str())
            .collect::<Vec<_>>(),
        vec!["Greeter", "Greeter", "Runner"]
    );

    // The parameter `who` is used inside its own body.
    let greet_proc = procedure_scope_id(&analyzer, "Greet");
    assert_eq!(
        occurrences(index, greet_proc, "who"),
        vec![(3, 23), (4, 21)]
    );
}
