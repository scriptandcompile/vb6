/// Configuration options for VB6 source code formatting.
pub struct FmtSettings {
    /// Number of spaces per indentation level.
    pub indent_size: usize,
    /// Case style for keywords ("camel", "pascal", "snake", etc.).
    pub keyword_case: String,
    /// Insert blank lines before and after compiler directive blocks.
    pub blank_lines_around_directives: bool,
    /// Insert blank lines between directives inside a directive block.
    pub blank_lines_inside_directives: bool,
    /// Insert blank lines between top-level declarations.
    pub blank_lines_around_top_level: bool,
}

impl Default for FmtSettings {
    fn default() -> Self {
        Self {
            indent_size: 4,
            keyword_case: "camel".to_string(),
            blank_lines_around_directives: false,
            blank_lines_inside_directives: false,
            blank_lines_around_top_level: true,
        }
    }
}
