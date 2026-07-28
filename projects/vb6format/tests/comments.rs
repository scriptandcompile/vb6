mod common;

#[test]
fn comments_preserved() {
    common::assert_stable(
        "' this is a comment\nSub Foo()\n    ' inside comment\n    x = 1\nEnd Sub\n' trailing\n",
    );
}

#[test]
fn comment_only_lines() {
    common::assert_stable("' just a comment\n' another one\n");
}

#[test]
fn idempotent_batch() {
    let cases = ["' comment\nSub Foo()\n    x = 1\nEnd Sub\n' comment\n"];
    for src in &cases {
        common::assert_stable(src);
    }
}
