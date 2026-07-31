use vb6format::FmtSettings;

mod common;

#[test]
fn empty_source() {
    common::assert_stable("");
}

#[test]
fn simple_no_indent() {
    common::assert_stable(
        "\
Dim x As Integer
x = 42
",
    );
}

#[test]
fn sub_body_indent() {
    common::assert_fmt(
        "\
Public Sub Foo()
Dim x As Integer
End Sub
",
        "\
Public Sub Foo()
    Dim x As Integer
End Sub
",
    );
}

#[test]
fn function_body_indent() {
    common::assert_fmt(
        "\
Public Function Add(a, b)
Add = a + b
End Function
",
        "\
Public Function Add(a, b)
    Add = a + b
End Function
",
    );
}

#[test]
fn property_get() {
    common::assert_fmt(
        "\
Property Get Name() As String
Name = m_Name
End Property
",
        "\
Property Get Name() As String
    Name = m_Name
End Property
",
    );
}

#[test]
fn nested_sub_indent() {
    let expected = "\
Public Sub Outer()
    Dim x As Integer

    Public Sub Inner()
        Dim y As Integer
    End Sub
End Sub
";
    common::assert_fmt(
        "\
Public Sub Outer()
Dim x As Integer

Public Sub Inner()
Dim y As Integer
End Sub
End Sub
",
        expected,
    );
}

#[test]
fn custom_indent_size() {
    let settings = FmtSettings {
        indent_size: 2,
        ..FmtSettings::default()
    };
    common::assert_fmt_with(
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
        &settings,
    );
}

#[test]
fn idempotent_batch() {
    let cases = [
        "",
        "\
Dim x As Integer
",
        "\
Sub Foo()
    x = 1
End Sub
",
    ];
    for src in &cases {
        common::assert_stable(src);
    }
}
