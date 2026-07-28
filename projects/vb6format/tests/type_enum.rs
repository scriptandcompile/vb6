mod common;

#[test]
fn type_body() {
    common::assert_fmt(
        "Public Type MyType\nx As Integer\ny As String\nEnd Type\n",
        "Public Type MyType\n    x As Integer\n    y As String\nEnd Type\n",
    );
}

#[test]
fn private_type() {
    common::assert_fmt(
        "Private Type MyType\nx As Integer\nEnd Type\n",
        "Private Type MyType\n    x As Integer\nEnd Type\n",
    );
}

#[test]
fn enum_body() {
    common::assert_fmt(
        "Enum MyEnum\na = 1\nb = 2\nEnd Enum\n",
        "Enum MyEnum\n    a = 1\n    b = 2\nEnd Enum\n",
    );
}

#[test]
fn idempotent_batch() {
    let cases = [
        "Type T\n    x As Integer\nEnd Type\n",
        "Enum E\n    A\n    B\nEnd Enum\n",
    ];
    for src in &cases {
        common::assert_stable(src);
    }
}
