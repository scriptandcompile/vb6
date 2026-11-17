use vb6interpret::run_source;

#[test]
fn return_expression_is_converted_to_the_declared_function_type() {
    let source = "Attribute VB_Name = \"M\"\n\
        Option VB6RExtensions On\n\
        Function AsString() As String\n\
            Return 42\n\
        End Function\n\
        Sub Main()\n\
            Debug.Print TypeName(AsString())\n\
        End Sub\n";

    assert_eq!(
        run_source(source).expect("integer should convert to String"),
        vec!["String"]
    );
}

#[test]
fn return_expression_conversion_propagates_type_mismatch() {
    let source = "Attribute VB_Name = \"M\"\n\
        Option VB6RExtensions On\n\
        Function AsInteger() As Integer\n\
            Return \"not a number\"\n\
        End Function\n\
        Sub Main()\n\
            Debug.Print AsInteger()\n\
        End Sub\n";

    let error = run_source(source).expect_err("invalid conversion must fail");
    assert_eq!(error.error.number, 13);
}

#[test]
fn return_expression_conversion_checks_overflow() {
    let source = "Attribute VB_Name = \"M\"\n\
        Option VB6RExtensions On\n\
        Function AsByte() As Byte\n\
            Return 256\n\
        End Function\n\
        Sub Main()\n\
            Debug.Print AsByte()\n\
        End Sub\n";

    let error = run_source(source).expect_err("Byte return conversion must overflow");
    assert_eq!(error.error.number, 6);
}
