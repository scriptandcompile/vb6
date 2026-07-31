mod common;

#[test]
fn type_body() {
    common::assert_fmt(
        "\
Public Type MyType
x As Integer
y As String
End Type
",
        "\
Public Type MyType
    x As Integer
    y As String
End Type
",
    );
}

#[test]
fn private_type() {
    common::assert_fmt(
        "\
Private Type MyType
x As Integer
End Type
",
        "\
Private Type MyType
    x As Integer
End Type
",
    );
}

#[test]
fn enum_body() {
    common::assert_fmt(
        "\
Enum MyEnum
a = 1
b = 2
End Enum
",
        "\
Enum MyEnum
    a = 1
    b = 2
End Enum
",
    );
}

#[test]
fn idempotent_batch() {
    let cases = [
        "\
Type T
    x As Integer
End Type
",
        "\
Enum E
    A
    B
End Enum
",
    ];
    for src in &cases {
        common::assert_stable(src);
    }
}
