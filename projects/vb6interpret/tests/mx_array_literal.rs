use vb6interpret::run_source;

#[test]
fn non_empty_array_literal_is_zero_based_and_widens_numeric_elements() {
    let source = "Attribute VB_Name = \"M\"\n\
        Option VB6RExtensions On\n\
        Sub Main()\n\
            Dim values As Variant\n\
            values = [1, 2.5, 3]\n\
            Debug.Print values(0)\n\
            Debug.Print values(1)\n\
            Debug.Print values(2)\n\
        End Sub\n";

    assert_eq!(
        run_source(source).expect("array literal should execute"),
        vec!["1", "2.5", "3"]
    );
}

#[test]
fn empty_array_literal_reports_missing_typed_context() {
    let source = "Attribute VB_Name = \"M\"\n\
        Option VB6RExtensions On\n\
        Sub Main()\n\
            Dim values As Variant\n\
            values = []\n\
        End Sub\n";

    let error = run_source(source).expect_err("empty literal must not fabricate a value");
    assert_eq!(error.error.number, 5);
    assert!(
        error
            .error
            .description
            .contains("Empty array literal requires a typed context")
    );
}

#[test]
fn bracketed_identifier_is_not_reinterpreted_without_mx_opt_in() {
    let (cst, failures) =
        vb6parse::ConcreteSyntaxTree::from_text("legacy.bas", "value = [value with spaces]\n")
            .unpack();
    assert!(failures.is_empty());
    let tree = format!(
        "{:?}",
        cst.expect("legacy expression should parse")
            .to_serializable()
    );
    assert!(tree.contains("value with spaces"));
    assert!(!tree.contains("ArrayLiteralExpression"));
}

#[test]
fn bracketed_identifier_stays_an_identifier_with_mx_opt_in() {
    let source = vb6parse::io::SourceFile::from_string(
        "mx.bas",
        "Attribute VB_Name = \"M\"\n\
         Option VB6RExtensions On\n\
         Sub Main()\n\
             Debug.Print [value with spaces]\n\
         End Sub\n",
    );
    let (module, failures) = vb6parse::files::ModuleFile::parse(&source).unpack();
    assert!(failures.is_empty());
    let tree = format!(
        "{:?}",
        module.expect("MX module should parse").cst.to_root_node()
    );
    assert!(tree.contains("value with spaces"));
    assert!(!tree.contains("ArrayLiteralExpression"));
}
