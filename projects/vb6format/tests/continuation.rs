mod common;

#[test]
fn continuation() {
    common::assert_fmt(
        "\
Sub Foo()
x = 1 + _
        2 + _
        3
End Sub
",
        "\
Sub Foo()
    x = 1 + _
    2 + _
    3
End Sub
",
    );
}

#[test]
fn idempotent_batch() {
    let cases = ["\
Sub Foo()
    x = 1 + _
    2
End Sub
"];
    for src in &cases {
        common::assert_stable(src);
    }
}
