use crate::LineEnding;

pub struct Context {
    pub indent_level: usize,
    pub line_ending: LineEnding,
    pub pending_indent: bool,
    pub last_was_blank: bool,
    pub line_has_content: bool,
    pub pending_blank: bool,
}

impl Context {
    pub fn new(indent_level: usize, line_ending: LineEnding) -> Self {
        Self {
            indent_level,
            line_ending,
            pending_indent: true,
            last_was_blank: true,
            line_has_content: false,
            pending_blank: false,
        }
    }

    /// Returns the number of spaces for the current indentation level.
    pub fn get_indent(&self) -> usize {
        self.indent_level * 4 // Defaulting to 4 as a placeholder until config is integrated
    }

    /// Returns the newline character used by the system.
    pub fn line_ending(&self) -> &str {
        match self.line_ending {
            LineEnding::CrLf => "\r\n",
            LineEnding::Lf => "\n",
        }
    }
}
