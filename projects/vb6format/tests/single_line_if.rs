mod common;

#[test]
fn single_line_if() {
    common::assert_stable("Sub Foo()\n    If True Then x = 1\nEnd Sub\n");
}
