use vb6parse::*;

const SNAPSHOT_PATH: &str = "../../snapshots/parsers/cst/attributes/attribute_statements";

#[test]
fn parse_attribute_statement() {
    let source = "Attribute VB_Name = \"modTest\"\n";
    let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
    assert_eq!(failures.len(), 0, "Expected no parse failures.");
    let cst = cst_opt.expect("CST should be parsed");
    let tree = cst.to_serializable();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(SNAPSHOT_PATH);
    settings.set_prepend_module_to_snapshot(false);
    let _guard = settings.bind_to_scope();
    insta::assert_yaml_snapshot!(tree);
}
