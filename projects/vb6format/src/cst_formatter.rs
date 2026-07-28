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
        }
    }

    pub(super) fn walk_node(&mut self, node: &CstNode) {
        if node.is_token() {
            self.emit_token(node);
            return;
        }

        let is_stmt_list = node.kind() == SyntaxKind::StatementList;
        if is_stmt_list {
            self.indent_level += 1;
        }

        for child in node.children() {
            self.walk_node(child);
        }

        if is_stmt_list {
            self.indent_level = self.indent_level.saturating_sub(1);
        }
    }

    fn emit_token(&mut self, token: &CstNode) {
        match token.kind() {
            SyntaxKind::Newline => {
                self.output.push_str(self.line_ending);
                self.pending_indent = true;
            }
            SyntaxKind::Whitespace => {
                if !self.pending_indent {
                    self.output.push(' ');
                }
            }
            SyntaxKind::ErrorExpectedTokens | SyntaxKind::ErrorMissingTokens => {}
            _ => {
                if self.pending_indent {
                    let indent = self.indent_level * self.settings.indent_size;
                    for _ in 0..indent {
                        self.output.push(' ');
                    }
                    self.pending_indent = false;
                }
                self.output.push_str(token.text());
            }
        }
    }
}
