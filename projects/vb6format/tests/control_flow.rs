mod common;

#[test]
fn if_block() {
    common::assert_fmt(
        "\
Sub Foo()
If True Then
x = 1
End If
End Sub
",
        "\
Sub Foo()
    If True Then
        x = 1
    End If
End Sub
",
    );
}

#[test]
fn if_else() {
    common::assert_fmt(
        "\
Sub Foo()
If a Then
x = 1
Else
x = 2
End If
End Sub
",
        "\
Sub Foo()
    If a Then
        x = 1
    Else
        x = 2
    End If
End Sub
",
    );
}

#[test]
fn if_elseif_else() {
    common::assert_fmt(
        "\
Sub Foo()
If a Then
x = 1
ElseIf b Then
x = 2
Else
x = 3
End If
End Sub
",
        "\
Sub Foo()
    If a Then
        x = 1
    ElseIf b Then
        x = 2
    Else
        x = 3
    End If
End Sub
",
    );
}

#[test]
fn for_loop() {
    common::assert_fmt(
        "\
Sub Foo()
For i = 1 To 10
Total = Total + i
Next
End Sub
",
        "\
Sub Foo()
    For i = 1 To 10
        Total = Total + i
    Next
End Sub
",
    );
}

#[test]
fn do_loop() {
    common::assert_fmt(
        "\
Sub Foo()
Do While True
x = x + 1
Loop
End Sub
",
        "\
Sub Foo()
    Do While True
        x = x + 1
    Loop
End Sub
",
    );
}

#[test]
fn while_wend() {
    common::assert_fmt(
        "\
Sub Foo()
While x < 10
x = x + 1
Wend
End Sub
",
        "\
Sub Foo()
    While x < 10
        x = x + 1
    Wend
End Sub
",
    );
}

#[test]
fn with_block() {
    common::assert_fmt(
        "\
Sub Foo()
With obj
.Name = \"bar\"
End With
End Sub
",
        "\
Sub Foo()
    With obj
        .Name = \"bar\"
    End With
End Sub
",
    );
}

#[test]
fn select_case() {
    common::assert_fmt(
        "\
Sub Foo()
Select Case x
Case 1
DoOne
Case 2
DoTwo
Case Else
DoDefault
End Select
End Sub
",
        "\
Sub Foo()
    Select Case x
    Case 1
        DoOne
    Case 2
        DoTwo
    Case Else
        DoDefault
    End Select
End Sub
",
    );
}

#[test]
fn idempotent_batch() {
    let cases = ["\
Sub Foo()
    If True Then
        x = 1
    End If
End Sub
"];
    for src in &cases {
        common::assert_stable(src);
    }
}
