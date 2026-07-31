use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LineEnding {
    CrLf,
    Lf,
}

impl Display for LineEnding {
    #[allow(clippy::write_with_newline)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineEnding::CrLf => write!(f, "\r\n"),
            LineEnding::Lf => write!(f, "\n"),
        }
    }
}

pub(crate) struct Context {
    pub indent_level: usize,
    pub line_ending: LineEnding,
    pub pending_indent: bool,
    pub last_was_blank: bool,
    pub line_has_content: bool,
    pub pending_blank: bool,
    pub directive_phase: Option<DirectivePhase>,
    pub compiler_directive_depth: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectivePhase {
    BeforeDirective,
    BeforeBody,
    AfterDirective,
}

impl Context {
    pub fn new(indent_level: usize) -> Self {
        Self {
            indent_level,
            line_ending: LineEnding::Lf,
            pending_indent: true,
            last_was_blank: true,
            line_has_content: false,
            pending_blank: false,
            directive_phase: None,
            compiler_directive_depth: 0,
        }
    }

    /// Returns the newline character used by the system.
    pub fn line_ending(&self) -> &str {
        match self.line_ending {
            LineEnding::CrLf => "\r\n",
            LineEnding::Lf => "\n",
        }
    }
}
