use vb6parse::parsers::{CstNode, SyntaxKind};

use crate::settings::FmtSettings;

pub(super) fn detect_line_ending(source: &str) -> &'static str {
    if source.contains("\r\n") {
        "\r\n"
    } else {
        "\n"
    }
}

pub(super) struct CstFormatter<'a> {
    output: &'a mut String,
    indent_level: usize,
    pending_indent: bool,
    line_ending: &'a str,
    settings: &'a FmtSettings,
    last_was_blank: bool,
    line_has_content: bool,
    pending_blank: bool,
}

impl<'a> CstFormatter<'a> {
    pub(super) fn new(
        output: &'a mut String,
        line_ending: &'a str,
        settings: &'a FmtSettings,
    ) -> Self {
        Self {
            output,
            indent_level: 0,
            pending_indent: true,
            line_ending,
            settings,
            last_was_blank: true,
            line_has_content: false,
            pending_blank: false,
        }
    }

    pub(super) fn walk_node(&mut self, node: &CstNode) {
        if node.is_token() {
            self.emit_token(node);
            return;
        }

        match node.kind() {
            SyntaxKind::StatementList => {
                self.indent_level += 1;
                for child in node.children() {
                    self.walk_node(child);
                }
                self.indent_level = self.indent_level.saturating_sub(1);
            }
            SyntaxKind::CompilerDirective => {
                self.walk_compiler_directive(node);
            }
            _ => {
                for child in node.children() {
                    self.walk_node(child);
                }
            }
        }
    }

    fn walk_compiler_directive(&mut self, node: &CstNode) {
        let base_indent = self.indent_level;

        if self.settings.blank_lines_around_directives && !self.last_was_blank {
            self.pending_blank = true;
        }

        for child in node.children() {
            match child.kind() {
                SyntaxKind::CompilerIfClause
                | SyntaxKind::CompilerElseIfClause
                | SyntaxKind::CompilerElseClause
                | SyntaxKind::CompilerEndIfClause => {
                    if matches!(
                        child.kind(),
                        SyntaxKind::CompilerElseIfClause | SyntaxKind::CompilerElseClause
                    ) {
                        if self.settings.blank_lines_inside_directives && !self.last_was_blank {
                            self.output.push_str(self.line_ending);
                            self.last_was_blank = true;
                        }
                    }

                    let saved = self.indent_level;
                    self.indent_level = base_indent;
                    for token in child.children() {
                        self.walk_node(token);
                    }
                    self.indent_level = saved;
                }
                SyntaxKind::StatementList => {
                    if self.settings.blank_lines_inside_directives && !self.last_was_blank {
                        self.output.push_str(self.line_ending);
                        self.last_was_blank = true;
                    }

                    self.indent_level = base_indent + 1;
                    for stmt in child.children() {
                        self.walk_node(stmt);
                    }
                    self.indent_level = base_indent;

                    if self.settings.blank_lines_inside_directives && !self.last_was_blank {
                        self.output.push_str(self.line_ending);
                        self.last_was_blank = true;
                    }
                }
                _ => {
                    self.walk_node(child);
                }
            }
        }

        if self.settings.blank_lines_around_directives {
            self.pending_blank = true;
        }
    }

    fn emit_token(&mut self, token: &CstNode) {
        match token.kind() {
            SyntaxKind::Newline => {
                if !self.line_has_content {
                    self.last_was_blank = true;
                    self.pending_blank = false;
                }
                self.output.push_str(self.line_ending);
                self.pending_indent = true;
                self.line_has_content = false;
            }
            SyntaxKind::Whitespace => {
                if !self.pending_indent {
                    self.output.push(' ');
                }
            }
            SyntaxKind::EndOfLineComment | SyntaxKind::RemComment => {
                if self.pending_indent {
                    let indent = self.indent_level * self.settings.indent_size;
                    for _ in 0..indent {
                        self.output.push(' ');
                    }
                    self.pending_indent = false;
                }
                self.output.push_str(token.text());
                self.last_was_blank = false;
                self.line_has_content = true;
            }
            SyntaxKind::ErrorExpectedTokens | SyntaxKind::ErrorMissingTokens => {}
            _ => {
                if self.pending_blank && !self.last_was_blank {
                    self.output.push_str(self.line_ending);
                    self.pending_blank = false;
                }
                if self.pending_indent {
                    let indent = self.indent_level * self.settings.indent_size;
                    for _ in 0..indent {
                        self.output.push(' ');
                    }
                    self.pending_indent = false;
                }
                self.output.push_str(token.text());
                self.last_was_blank = false;
                self.line_has_content = true;
            }
        }
    }
}
