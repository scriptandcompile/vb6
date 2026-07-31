use vb6format::FmtSettings;

mod common;

#[test]
fn keyword_case_upper() {
    let settings = FmtSettings {
        keyword_case: "upper".to_string(),
        ..FmtSettings::default()
    };

    common::assert_fmt_with(
        "\
public sub foo()
if x then
end if
end sub
",
        "\
PUBLIC SUB foo()
    IF x THEN
    END IF
END SUB
",
        &settings,
    );
}

#[test]
fn keyword_case_lower() {
    let settings = FmtSettings {
        keyword_case: "lower".to_string(),
        ..FmtSettings::default()
    };

    common::assert_fmt_with(
        "\
PUBLIC SUB Foo()
IF x THEN
END IF
END SUB
",
        "\
public sub Foo()
    if x then
    end if
end sub
",
        &settings,
    );
}

#[test]
fn keyword_case_camel() {
    let settings = FmtSettings {
        keyword_case: "camel".to_string(),
        ..FmtSettings::default()
    };

    common::assert_fmt_with(
        "\
public sub foo()
elseif x then
end if
end sub
",
        "\
Public Sub foo()
    ElseIf x Then
    End If
End Sub
",
        &settings,
    );
}

#[test]
fn keyword_case_first() {
    let settings = FmtSettings {
        keyword_case: "first".to_string(),
        ..FmtSettings::default()
    };

    common::assert_fmt_with(
        "\
PUBLIC SUB Foo()
ELSEIF x THEN
END IF
END SUB
",
        "\
Public Sub Foo()
    Elseif x Then
    End If
End Sub
",
        &settings,
    );
}
