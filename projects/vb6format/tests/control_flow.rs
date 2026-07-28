mod common;

#[test]
fn if_block() {
    common::assert_fmt(
        "Sub Foo()\nIf True Then\nx = 1\nEnd If\nEnd Sub\n",
        "Sub Foo()\n    If True Then\n        x = 1\n    End If\nEnd Sub\n",
    );
}

#[test]
fn if_else() {
    common::assert_fmt(
        "Sub Foo()\nIf a Then\nx = 1\nElse\nx = 2\nEnd If\nEnd Sub\n",
        "Sub Foo()\n    If a Then\n        x = 1\n    Else\n        x = 2\n    End If\nEnd Sub\n",
    );
}

#[test]
fn if_elseif_else() {
    common::assert_fmt(
        "Sub Foo()\nIf a Then\nx = 1\nElseIf b Then\nx = 2\nElse\nx = 3\nEnd If\nEnd Sub\n",
        "Sub Foo()\n    If a Then\n        x = 1\n    ElseIf b Then\n        x = 2\n    Else\n        x = 3\n    End If\nEnd Sub\n",
    );
}

#[test]
fn for_loop() {
    common::assert_fmt(
        "Sub Foo()\nFor i = 1 To 10\nTotal = Total + i\nNext\nEnd Sub\n",
        "Sub Foo()\n    For i = 1 To 10\n        Total = Total + i\n    Next\nEnd Sub\n",
    );
}

#[test]
fn do_loop() {
    common::assert_fmt(
        "Sub Foo()\nDo While True\nx = x + 1\nLoop\nEnd Sub\n",
        "Sub Foo()\n    Do While True\n        x = x + 1\n    Loop\nEnd Sub\n",
    );
}

#[test]
fn while_wend() {
    common::assert_fmt(
        "Sub Foo()\nWhile x < 10\nx = x + 1\nWend\nEnd Sub\n",
        "Sub Foo()\n    While x < 10\n        x = x + 1\n    Wend\nEnd Sub\n",
    );
}

#[test]
fn with_block() {
    common::assert_fmt(
        "Sub Foo()\nWith obj\n.Name = \"bar\"\nEnd With\nEnd Sub\n",
        "Sub Foo()\n    With obj\n        .Name = \"bar\"\n    End With\nEnd Sub\n",
    );
}

#[test]
fn select_case() {
    common::assert_fmt(
        "Sub Foo()\nSelect Case x\nCase 1\nDoOne\nCase 2\nDoTwo\nCase Else\nDoDefault\nEnd Select\nEnd Sub\n",
        "Sub Foo()\n    Select Case x\n    Case 1\n        DoOne\n    Case 2\n        DoTwo\n    Case Else\n        DoDefault\n    End Select\nEnd Sub\n",
    );
}

#[test]
fn idempotent_batch() {
    let cases = ["Sub Foo()\n    If True Then\n        x = 1\n    End If\nEnd Sub\n"];
    for src in &cases {
        common::assert_stable(src);
    }
}
