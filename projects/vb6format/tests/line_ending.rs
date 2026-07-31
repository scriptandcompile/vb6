mod common;

#[test]
fn crlf_preserved() {
    common::assert_fmt(
        concat!("Sub Foo()\r\n", "Dim x As Integer\r\n", "End Sub\r\n"),
        concat!("Sub Foo()\r\n", "    Dim x As Integer\r\n", "End Sub\r\n"),
    );
}