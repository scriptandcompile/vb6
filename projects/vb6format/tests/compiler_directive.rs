use vb6format::FmtSettings;

mod common;

#[test]
fn directive_simple() {
    common::assert_fmt(
        "\
Sub Foo()
#If DEBUG Then
Debug.Print \"hi\"
#End If
End Sub
",
        "\
Sub Foo()
    #If DEBUG Then
        Debug.Print \"hi\"
    #End If
End Sub
",
    );
}

#[test]
fn directive_with_else() {
    common::assert_fmt(
        "\
Sub Foo()
#If A Then
x = 1
#ElseIf B Then
x = 2
#Else
x = 3
#End If
End Sub
",
        "\
Sub Foo()
    #If A Then
        x = 1
    #ElseIf B Then
        x = 2
    #Else
        x = 3
    #End If
End Sub
",
    );
}

#[test]
fn directive_nested() {
    common::assert_fmt(
        "\
Sub Foo()
#If A Then
x = 1
#If B Then
y = 2
#End If
z = 3
#End If
End Sub
",
        "\
Sub Foo()
    #If A Then
        x = 1
        #If B Then
            y = 2
        #End If
        z = 3
    #End If
End Sub
",
    );
}

#[test]
fn directive_top_level() {
    common::assert_fmt(
        "\
#If Win64 Then
PtrSafe
#End If
",
        "\
#If Win64 Then
    PtrSafe
#End If
",
    );
}

#[test]
fn directive_blank_lines_around() {
    let settings = FmtSettings {
        blank_lines_around_directives: true,
        ..FmtSettings::default()
    };
    let input = "\
Sub Foo()
#If DEBUG Then
Debug.Print \"hi\"
#End If
End Sub
";
    let expected = "\
Sub Foo()

    #If DEBUG Then
        Debug.Print \"hi\"
    #End If

End Sub
";
    common::assert_fmt_with(input, expected, &settings);
}

#[test]
fn directive_blank_lines_inside() {
    let settings = FmtSettings {
        blank_lines_inside_directives: true,
        ..FmtSettings::default()
    };
    let input = "\
Sub Foo()
#If DEBUG Then
Debug.Print \"hi\"
#End If
End Sub
";
    let expected = "\
Sub Foo()
    #If DEBUG Then

        Debug.Print \"hi\"

    #End If
End Sub
";
    common::assert_fmt_with(input, expected, &settings);
}

#[test]
fn idempotent_batch() {
    let cases = ["\
#If DEBUG Then
    x = 1
#End If
"];
    for src in &cases {
        common::assert_stable(src);
    }
}
