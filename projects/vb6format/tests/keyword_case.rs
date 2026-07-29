use vb6format::FmtSettings;

mod common;

#[test]
fn keyword_case_upper() {
    let settings = FmtSettings {
        keyword_case: "upper".to_string(),
        ..FmtSettings::default()
    };

    common::assert_fmt_with(
        "public sub foo()\nif x then\nend if\nend sub\n",
        "PUBLIC SUB foo()\n    IF x THEN\n    END IF\nEND SUB\n",
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
        "PUBLIC SUB Foo()\nIF x THEN\nEND IF\nEND SUB\n",
        "public sub Foo()\n    if x then\n    end if\nend sub\n",
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
        "public sub foo()\nelseif x then\nend if\nend sub\n",
        "Public Sub foo()\n    ElseIf x Then\n    End If\nEnd Sub\n",
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
        "PUBLIC SUB Foo()\nELSEIF x THEN\nEND IF\nEND SUB\n",
        "Public Sub Foo()\n    Elseif x Then\n    End If\nEnd Sub\n",
        &settings,
    );
}
