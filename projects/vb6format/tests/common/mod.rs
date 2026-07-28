use vb6format::{FmtSettings, fmt_source};

pub fn assert_fmt(source: &str, expected: &str) {
    assert_fmt_with(source, expected, &FmtSettings::default());
}

pub fn assert_fmt_with(source: &str, expected: &str, settings: &FmtSettings) {
    let once = fmt_source(source, settings).unwrap();
    assert_eq!(once, expected, "first format mismatch");
    let twice = fmt_source(&once, settings).unwrap();
    assert_eq!(once, twice, "format is not idempotent on:\n{once:?}");
}

#[allow(dead_code)]
pub fn assert_stable(source: &str) {
    assert_fmt(source, source);
}
