use vb6interpret::run_source;

#[test]
fn spread_expands_rank_one_array_in_index_order_and_evaluates_source_once() {
    let source = "Attribute VB_Name = \"M\"\n\
        Option VB6RExtensions On\n\
        Dim evaluations As Integer\n\
        Function MakeValues() As Variant\n\
            evaluations = evaluations + 1\n\
            MakeValues = [20, 10]\n\
        End Function\n\
        Sub Consume(ByVal first, ByVal second)\n\
            Debug.Print first, second\n\
        End Sub\n\
        Sub Main()\n\
            Consume(...MakeValues())\n\
            Debug.Print evaluations\n\
        End Sub\n";
    assert_eq!(
        run_source(source).expect("spread call should execute"),
        vec!["20\t10", "1"]
    );
}

#[test]
fn debug_print_does_not_accept_call_argument_spread() {
    let source = "Attribute VB_Name = \"M\"\n\
        Option VB6RExtensions On\n\
        Sub Main()\n\
            Debug.Print(...[1, 2])\n\
        End Sub\n";

    let error = run_source(source).expect_err("Debug.Print is not a procedure call");
    assert_eq!(error.error.number, 5);
    assert!(error.error.description.contains("only in procedure call"));
}

#[test]
fn spread_is_not_enabled_without_mx_opt_in() {
    let source = "Attribute VB_Name = \"M\"\n\
        Sub Main()\n\
            Consume ...values\n\
        End Sub\n";

    assert!(
        run_source(source).is_err(),
        "legacy source must not accept the MX spread syntax"
    );
}

#[test]
fn spread_rejects_non_array_source() {
    let source = "Attribute VB_Name = \"M\"\n\
        Option VB6RExtensions On\n\
        Sub Consume(ByVal value As Variant)\n\
        End Sub\n\
        Sub Main()\n\
            Consume(...5)\n\
        End Sub\n";

    let error = run_source(source).expect_err("scalar spread source must fail");
    assert_eq!(error.error.number, 13);
    assert!(error.error.description.contains("rank-1 array"));
}

#[test]
fn spread_values_cannot_bind_to_byref_formals() {
    let source = "Attribute VB_Name = \"M\"\n\
        Option VB6RExtensions On\n\
        Sub Consume(ByRef value As Variant)\n\
        End Sub\n\
        Sub Main()\n\
            Consume(...[1])\n\
        End Sub\n";

    let error = run_source(source).expect_err("spread must not fabricate ByRef aliasing");
    assert_eq!(error.error.number, 5);
    assert!(error.error.description.contains("ByRef"));
}

#[test]
fn every_expanded_value_is_checked_against_its_formal() {
    let source = "Attribute VB_Name = \"M\"\n\
        Option VB6RExtensions On\n\
        Sub Consume(ByVal first, ByRef second, ByVal third)\n\
        End Sub\n\
        Sub Main()\n\
            Consume(0, ...[1, 2])\n\
        End Sub\n";

    let error = run_source(source).expect_err("the second spread value binds ByRef");
    assert_eq!(error.error.number, 5);
    assert!(error.error.description.contains("ByRef"));
}
