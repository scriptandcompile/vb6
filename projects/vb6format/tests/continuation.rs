mod common;

#[test]
fn continuation() {
    common::assert_fmt(
        "Sub Foo()\nx = 1 + _\n        2 + _\n        3\nEnd Sub\n",
        "Sub Foo()\n    x = 1 + _\n    2 + _\n    3\nEnd Sub\n",
    );
}

#[test]
fn idempotent_batch() {
    let cases = ["Sub Foo()\n    x = 1 + _\n    2\nEnd Sub\n"];
    for src in &cases {
        common::assert_stable(src);
    }
}
