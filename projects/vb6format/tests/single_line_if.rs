mod common;

#[test]
fn single_line_if() {
    common::assert_stable(
        "\
Sub Foo()
    If True Then x = 1
End Sub
",
    );
}
