mod common;

use vb6format::FmtSettings;

#[test]
fn comments_preserved() {
    common::assert_stable(
        "' this is a comment\nSub Foo()\n    ' inside comment\n    x = 1\nEnd Sub\n' trailing\n",
    );
}

#[test]
fn comment_only_lines() {
    common::assert_stable("' just a comment\n' another one\n");
}

#[test]
fn idempotent_batch() {
    let cases = ["' comment\nSub Foo()\n    x = 1\nEnd Sub\n' comment\n"];
    for src in &cases {
        common::assert_stable(src);
    }
}

#[test]
fn blank_lines_around_top_level_only_between_constructs() {
    let source = "Option Explicit\nSub Foo()\nEnd Sub\nSub Bar()\nEnd Sub\n";
    let expected = "Option Explicit\nSub Foo()\nEnd Sub\n\nSub Bar()\nEnd Sub\n";
    let settings = FmtSettings {
        blank_lines_around_top_level: true,
        ..FmtSettings::default()
    };

    common::assert_fmt_with(source, expected, &settings);
}

#[test]
fn blank_lines_around_top_level_keeps_comments_with_next_construct() {
    let source = "Option Explicit\nSub Foo()\nEnd Sub\n' docs\nSub Bar()\nEnd Sub\n";
    let expected = "Option Explicit\nSub Foo()\nEnd Sub\n\n' docs\nSub Bar()\nEnd Sub\n";
    let settings = FmtSettings {
        blank_lines_around_top_level: true,
        ..FmtSettings::default()
    };

    common::assert_fmt_with(source, expected, &settings);
}

#[test]
fn blank_lines_around_top_level_preserves_existing_spacing() {
    let source = "Option Explicit\nSub Foo()\nEnd Sub\n\nSub Bar()\nEnd Sub\n";
    let expected = "Option Explicit\nSub Foo()\nEnd Sub\n\nSub Bar()\nEnd Sub\n";
    let settings = FmtSettings {
        blank_lines_around_top_level: true,
        ..FmtSettings::default()
    };

    common::assert_fmt_with(source, expected, &settings);
}
