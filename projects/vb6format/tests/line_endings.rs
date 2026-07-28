mod common;

#[test]
fn crlf_preserved() {
    common::assert_fmt(
        "Sub Foo()\r\nDim x As Integer\r\nEnd Sub\r\n",
        "Sub Foo()\r\n    Dim x As Integer\r\nEnd Sub\r\n",
    );
}
