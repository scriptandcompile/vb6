use vb6format::FmtSettings;

mod common;

#[test]
fn empty_source() {
    common::assert_stable("");
}

#[test]
fn simple_no_indent() {
    common::assert_stable("Dim x As Integer\nx = 42\n");
}

#[test]
fn sub_body_indent() {
    common::assert_fmt(
        "Public Sub Foo()\nDim x As Integer\nEnd Sub\n",
        "Public Sub Foo()\n    Dim x As Integer\nEnd Sub\n",
    );
}

#[test]
fn function_body_indent() {
    common::assert_fmt(
        "Public Function Add(a, b)\nAdd = a + b\nEnd Function\n",
        "Public Function Add(a, b)\n    Add = a + b\nEnd Function\n",
    );
}

#[test]
fn property_get() {
    common::assert_fmt(
        "Property Get Name() As String\nName = m_Name\nEnd Property\n",
        "Property Get Name() As String\n    Name = m_Name\nEnd Property\n",
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
        "Public Sub Outer()\nDim x As Integer\n\nPublic Sub Inner()\nDim y As Integer\nEnd Sub\nEnd Sub\n",
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
        "Sub Foo()\nx = 1\nEnd Sub\n",
        "Sub Foo()\n  x = 1\nEnd Sub\n",
        &settings,
    );
}

#[test]
fn idempotent_batch() {
    let cases = ["", "Dim x As Integer\n", "Sub Foo()\n    x = 1\nEnd Sub\n"];
    for src in &cases {
        common::assert_stable(src);
    }
}
