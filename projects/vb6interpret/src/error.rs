//! Interpreter runtime errors.
//!
//! An interpretation error carries the underlying VB6 error (mirroring the
//! `Err` object) together with optional source location context.

use std::fmt;

use ariadne::{Config, Label, Report, ReportKind, Source};
use vb6core::error::VBError;

/// An error raised during interpretation of VB6 code.
#[derive(Debug, Clone)]
pub struct RunError {
    /// The underlying VB6 error (`Err.Number` / `Err.Description`).
    pub error: Box<VBError>,
    /// Whether this is an internal debugger pause rather than a runtime fault.
    pub is_debug_pause: bool,
    /// The 1-based source line where the error occurred, when known.
    pub line: Option<usize>,
    /// The name of the procedure that was executing, when known.
    pub procedure: Option<String>,
}

impl RunError {
    /// Create an error without source context.
    pub fn new(error: VBError) -> Self {
        Self {
            error: Box::new(error),
            is_debug_pause: false,
            line: None,
            procedure: None,
        }
    }

    /// Create an internal pause signal used by debugger-style stepping.
    pub fn debug_pause() -> Self {
        Self {
            error: Box::new(VBError::new(0)),
            is_debug_pause: true,
            line: None,
            procedure: None,
        }
    }

    /// Attach the 1-based source line.
    pub fn at_line(mut self, line: usize) -> Self {
        self.line = Some(line);
        self
    }

    /// Attach the executing procedure name.
    pub fn in_procedure(mut self, name: &str) -> Self {
        self.procedure = Some(name.to_string());
        self
    }

    /// Build from a VB6 error number.
    pub fn err_number(number: i32) -> Self {
        Self::new(VBError::new(number))
    }

    /// Error 5: Invalid procedure call or argument.
    pub fn invalid_procedure_call() -> Self {
        Self::new(VBError::invalid_procedure_call())
    }

    /// Error 13: Type mismatch.
    pub fn type_mismatch() -> Self {
        Self::new(VBError::type_mismatch())
    }

    /// Error 35: Sub or Function not defined.
    pub fn sub_or_function_not_defined() -> Self {
        Self::new(VBError::new(35))
    }

    /// Whether this error represents a debugger pause.
    pub fn is_debug_pause(&self) -> bool {
        self.is_debug_pause
    }
}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_debug_pause {
            if let Some(line) = self.line {
                return write!(f, "paused at line {line}");
            }
            return f.write_str("paused before next statement");
        }

        if let Some(line) = self.line {
            write!(f, "line {line}: {}", self.error)
        } else {
            write!(f, "{}", self.error)
        }
    }
}

impl std::error::Error for RunError {}

impl From<VBError> for RunError {
    fn from(error: VBError) -> Self {
        Self::new(error)
    }
}

impl From<RunError> for VBError {
    fn from(run_error: RunError) -> Self {
        *run_error.error
    }
}

/// Render an ariadne report pointing at the source line where `error` occurred.
///
/// `source` is the original source text and `source_name` its display name.
/// `line_offset` is the number of header lines stripped from the module CST
/// (see `ModuleFile::line_offset`); it is added to `error.line` to map the
/// body-relative line number onto the original source.
///
/// Returns `None` when the error carries no line, is a debugger pause, or the
/// line lies outside the source text.
pub fn render_error_report(
    source_name: &str,
    source: &str,
    error: &RunError,
    line_offset: usize,
) -> Option<String> {
    if error.is_debug_pause {
        return None;
    }
    let line = error.line? + line_offset;
    render_report_at_line(source_name, source, line, &error.error.to_string())
}

/// Render an ariadne report pointing at the 1-based source `line` with `message`
/// as the report header. Used by callers that carry their own message rather
/// than a VB6 `Err` value (e.g. parse diagnostics).
///
/// Returns `None` when the line lies outside the source text.
pub fn render_report_at_line(
    source_name: &str,
    source: &str,
    line: usize,
    message: &str,
) -> Option<String> {
    let (span_start, span_end) = line_byte_span(source, line)?;

    let cache = (source_name.to_string(), Source::from(source));
    let mut buf = Vec::new();
    let report = Report::build(
        ReportKind::Error,
        (source_name.to_string(), span_start..=span_end),
    )
    .with_message(message)
    .with_label(
        Label::new((source_name.to_string(), span_start..=span_end))
            .with_message("error here"),
    )
    .with_config(Config::new().with_color(false));
    report.finish().write(cache, &mut buf).ok()?;
    String::from_utf8(buf).ok()
}

/// Byte offsets of the 1-based line `line` in `source`, with the trailing
/// newline trimmed. The returned range always covers at least one byte.
fn line_byte_span(source: &str, line: usize) -> Option<(usize, usize)> {
    let mut start = 0usize;
    for (index, part) in source.split_inclusive('\n').enumerate() {
        let line_no = index + 1;
        if line_no == line {
            let trimmed = part.trim_end_matches(['\r', '\n']);
            let end = start + trimmed.len();
            return Some((start, end.max(start + 1)));
        }
        start += part.len();
    }
    None
}

/// Convenience alias for interpreter results.
pub type RunResult<T> = Result<T, RunError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_points_at_the_offending_line() {
        let source = "Attribute VB_Name = \"M\"\n\
Sub Main()\n\
    Dim x As Double\n\
    x = 1 / 0\n\
End Sub\n";
        // Body-relative line 3 (`x = 1 / 0`) plus the 1 stripped header line.
        let error = RunError::new(VBError::new(11))
            .at_line(3)
            .in_procedure("Main");
        let report = render_error_report("scratch.bas", source, &error, 1).unwrap();
        assert!(report.contains("Runtime error 11") || report.contains("Error 11"));
        assert!(report.contains("scratch.bas:4"));
        assert!(report.contains("x = 1 / 0"));
    }

    #[test]
    fn report_includes_error_number_and_description() {
        let source = "Sub Main()\n    Debug.Print Missing()\nEnd Sub\n";
        let error = RunError::new(VBError::new(450)).at_line(2);
        let report = render_error_report("m.bas", source, &error, 0).unwrap();
        assert!(report.contains("450"));
        assert!(report.contains("Wrong number of arguments"));
    }

    #[test]
    fn report_is_none_for_debug_pause_or_missing_line() {
        let source = "Sub Main()\nEnd Sub\n";
        let pause = RunError::debug_pause();
        assert!(render_error_report("m.bas", source, &pause, 0).is_none());

        let no_line = RunError::new(VBError::new(13));
        assert!(render_error_report("m.bas", source, &no_line, 0).is_none());
    }

    #[test]
    fn report_is_none_when_line_is_out_of_range() {
        let source = "Sub Main()\nEnd Sub\n";
        let error = RunError::new(VBError::new(13)).at_line(99);
        assert!(render_error_report("m.bas", source, &error, 0).is_none());
    }

    #[test]
    fn report_at_line_uses_provided_message() {
        let source = "Dim x As ?\nSub Main()\nEnd Sub\n";
        let report = render_report_at_line("m.bas", source, 1, "Unknown token '?'").unwrap();
        assert!(report.contains("Unknown token '?'"));
        assert!(report.contains("m.bas:1"));
        assert!(report.contains("Dim x As ?"));
        assert!(!report.contains("(Error "), "parse reports should not carry Err numbers");
    }
}

