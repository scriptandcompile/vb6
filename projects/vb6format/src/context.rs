pub struct Context {
    pub indent_level: usize,
    pub line_ending: &'static str,
}

impl Context {
    pub fn new(indent_level: usize, line_ending: &'static str) -> Self {
        Self {
            indent_level,
            line_ending,
        }
    }

    /// Returns the number of spaces for the current indentation level.
    pub fn get_indent(&self) -> usize {
        self.indent_level * 4 // Defaulting to 4 as a placeholder until config is integrated
    }

    /// Returns the newline character used by the system.
    pub fn line_ending(&self) -> &str {
        self.line_ending
    }
}
