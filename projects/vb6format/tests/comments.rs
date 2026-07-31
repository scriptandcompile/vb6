mod common;

#[test]
fn comments_preserved() {
    common::assert_stable(
        "\
' this is a comment
Sub Foo()
    ' inside comment
    x = 1
End Sub
' trailing
",
    );
}

#[test]
fn comment_only_lines() {
    common::assert_stable(
        "\
' just a comment
' another one
",
    );
}

#[test]
fn idempotent_batch() {
    let cases = ["\
' comment
Sub Foo()
    x = 1
End Sub
' comment
"];
    for src in &cases {
        common::assert_stable(src);
    }
}
