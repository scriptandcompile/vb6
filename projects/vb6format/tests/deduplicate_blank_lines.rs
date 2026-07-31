mod common;

#[test]
fn collapses_multiple_blank_lines_between_statements() {
    common::assert_fmt(
        "\
Sub Foo()



x = 1



End Sub
",
        "\
Sub Foo()

    x = 1

End Sub
",
    );
}

#[test]
fn collapses_multiple_blank_lines_crlf() {
    common::assert_fmt(
        concat!(
            "Sub Foo()\r\n",
            "\r\n",
            "\r\n",
            "\r\n",
            "Dim x As Integer\r\n",
            "\r\n",
            "\r\n",
            "\r\n",
            "End Sub\r\n"
        ),
        concat!(
            "Sub Foo()\r\n",
            "\r\n",
            "    Dim x As Integer\r\n",
            "\r\n",
            "End Sub\r\n"
        ),
    );
}