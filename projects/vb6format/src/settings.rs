pub struct FmtSettings {
    pub indent_size: usize,
    pub blank_lines_around_directives: bool,
    pub blank_lines_inside_directives: bool,
}

impl Default for FmtSettings {
    fn default() -> Self {
        Self {
            indent_size: 4,
            blank_lines_around_directives: false,
            blank_lines_inside_directives: false,
        }
    }
}
