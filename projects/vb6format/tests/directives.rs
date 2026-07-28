use vb6format::FmtSettings;

mod common;

#[test]
fn directive_simple() {
    common::assert_fmt(
        "Sub Foo()\n#If DEBUG Then\nDebug.Print \"hi\"\n#End If\nEnd Sub\n",
        "Sub Foo()\n    #If DEBUG Then\n        Debug.Print \"hi\"\n    #End If\nEnd Sub\n",
    );
}

#[test]
fn directive_with_else() {
    common::assert_fmt(
        "Sub Foo()\n#If A Then\nx = 1\n#ElseIf B Then\nx = 2\n#Else\nx = 3\n#End If\nEnd Sub\n",
        "Sub Foo()\n    #If A Then\n        x = 1\n    #ElseIf B Then\n        x = 2\n    #Else\n        x = 3\n    #End If\nEnd Sub\n",
    );
}

#[test]
fn directive_nested() {
    common::assert_fmt(
        "Sub Foo()\n#If A Then\nx = 1\n#If B Then\ny = 2\n#End If\nz = 3\n#End If\nEnd Sub\n",
        "Sub Foo()\n    #If A Then\n        x = 1\n        #If B Then\n            y = 2\n        #End If\n        z = 3\n    #End If\nEnd Sub\n",
    );
}

#[test]
fn directive_top_level() {
    common::assert_fmt(
        "#If Win64 Then\nPtrSafe\n#End If\n",
        "#If Win64 Then\n    PtrSafe\n#End If\n",
    );
}

#[test]
fn directive_blank_lines_around() {
    let settings = FmtSettings {
        blank_lines_around_directives: true,
        ..FmtSettings::default()
    };
    let input = "Sub Foo()\n#If DEBUG Then\nDebug.Print \"hi\"\n#End If\nEnd Sub\n";
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
    let input = "Sub Foo()\n#If DEBUG Then\nDebug.Print \"hi\"\n#End If\nEnd Sub\n";
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
    let cases = ["#If DEBUG Then\n    x = 1\n#End If\n"];
    for src in &cases {
        common::assert_stable(src);
    }
}
