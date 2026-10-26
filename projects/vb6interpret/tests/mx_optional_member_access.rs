use vb6interpret::run_source;

fn module(body: &str) -> String {
    format!(
        "Attribute VB_Name = \"M\"\n\
         Option VB6RExtensions On\n\
         Sub Main()\n{body}\
         End Sub\n"
    )
}

#[test]
fn optional_member_access_propagates_nothing_through_a_chain() {
    let source = module("    Debug.Print Nothing?.First?.Second\n");
    assert_eq!(
        run_source(&source).expect("Nothing should propagate"),
        vec!["Nothing"]
    );
}

#[test]
fn optional_member_access_preserves_null_error_94() {
    let source = module("    Debug.Print Null?.Member\n");
    let error = run_source(&source).expect_err("Null is not Nothing");
    assert_eq!(error.error.number, 94);
}

#[test]
fn optional_member_access_rejects_non_nothing_receivers() {
    let source = module("    Debug.Print 1?.Member\n");
    let error = run_source(&source).expect_err("member access is unsupported");
    assert_eq!(error.error.number, 5);
    assert!(
        error
            .error
            .description
            .contains("member access is not supported")
    );
}

#[test]
fn optional_member_access_requires_mx_opt_in() {
    let source = "Attribute VB_Name = \"M\"\n\
        Sub Main()\n\
            Debug.Print Nothing?.Member\n\
        End Sub\n";
    let error = run_source(source).expect_err("optional access must require opt-in");
    assert_eq!(error.error.number, 35);
}
