use std::borrow::Cow;
use vb6parse::parsers::{CstNode, SyntaxKind};

use crate::LineEnding;
use crate::context::Context;
use crate::settings::FmtSettings;

pub(super) fn detect_line_ending(source: &str) -> LineEnding {
    if source.contains("\r\n") {
        LineEnding::CrLf
    } else {
        LineEnding::Lf
    }
}

pub(super) struct CstFormatter<'a> {
    output: &'a mut String,
    context: Context,
    settings: &'a FmtSettings,
}

impl<'a> CstFormatter<'a> {
    pub(super) fn new(
        output: &'a mut String,
        line_ending: LineEnding,
        settings: &'a FmtSettings,
    ) -> Self {
        Self {
            output,
            context: Context::new(0, line_ending),
            settings,
        }
    }

    pub(super) fn walk_node(&mut self, node: &CstNode) {
        if node.is_token() {
            self.emit_token(node);
            return;
        }

        match node.kind() {
            SyntaxKind::StatementList => {
                self.context.indent_level += 1;
                for child in node.children() {
                    self.walk_node(child);
                }
                self.context.indent_level = self.context.indent_level.saturating_sub(1);
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
        let base_indent = self.context.indent_level;

        if self.settings.blank_lines_around_directives && !self.context.last_was_blank {
            self.context.pending_blank = true;
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
                    ) && self.settings.blank_lines_inside_directives
                        && !self.context.last_was_blank
                    {
                        self.output.push_str(self.context.line_ending());
                        self.context.last_was_blank = true;
                    }

                    let saved = self.context.indent_level;
                    self.context.indent_level = base_indent;
                    for token in child.children() {
                        self.walk_node(token);
                    }
                    self.context.indent_level = saved;
                }
                SyntaxKind::StatementList => {
                    if self.settings.blank_lines_inside_directives && !self.context.last_was_blank {
                        self.output.push_str(self.context.line_ending());
                        self.context.last_was_blank = true;
                    }

                    self.context.indent_level = base_indent + 1;
                    for stmt in child.children() {
                        self.walk_node(stmt);
                    }
                    self.context.indent_level = base_indent;

                    if self.settings.blank_lines_inside_directives && !self.context.last_was_blank {
                        self.output.push_str(self.context.line_ending());
                        self.context.last_was_blank = true;
                    }
                }
                _ => {
                    self.walk_node(child);
                }
            }
        }

        if self.settings.blank_lines_around_directives {
            self.context.pending_blank = true;
        }
    }

    fn emit_token(&mut self, token: &CstNode) {
        match token.kind() {
            SyntaxKind::Newline => {
                if !self.context.line_has_content {
                    self.context.last_was_blank = true;
                    self.context.pending_blank = false;
                }
                self.output.push_str(self.context.line_ending());
                self.context.pending_indent = true;
                self.context.line_has_content = false;
            }
            SyntaxKind::Whitespace => {
                if !self.context.pending_indent {
                    self.output.push(' ');
                }
            }
            SyntaxKind::EndOfLineComment | SyntaxKind::RemComment => {
                if self.context.pending_indent {
                    let indent = self.context.indent_level * self.settings.indent_size;
                    for _ in 0..indent {
                        self.output.push(' ');
                    }
                    self.context.pending_indent = false;
                }
                self.output.push_str(token.text());
                self.context.last_was_blank = false;
                self.context.line_has_content = true;
            }
            SyntaxKind::ErrorExpectedTokens | SyntaxKind::ErrorMissingTokens => {}
            _ => {
                if self.context.pending_blank && !self.context.last_was_blank {
                    self.output.push_str(self.context.line_ending());
                    self.context.pending_blank = false;
                }
                if self.context.pending_indent {
                    let indent = self.context.indent_level * self.settings.indent_size;
                    for _ in 0..indent {
                        self.output.push(' ');
                    }
                    self.context.pending_indent = false;
                }
                let text = format_token_text(token, self.settings.keyword_case.as_str());
                self.output.push_str(&text);
                self.context.last_was_blank = false;
                self.context.line_has_content = true;
            }
        }
    }
}

fn format_token_text<'a>(token: &'a CstNode, keyword_case: &str) -> Cow<'a, str> {
    let original = token.text();
    let Some(canonical_keyword) = keyword_camel_text(token.kind()) else {
        return Cow::Borrowed(original);
    };

    match keyword_case {
        "upper" => Cow::Owned(canonical_keyword.to_ascii_uppercase()),
        "lower" => Cow::Owned(canonical_keyword.to_ascii_lowercase()),
        "camel" => Cow::Owned(canonical_keyword),
        "first" => {
            let mut chars = canonical_keyword.chars();
            let Some(first) = chars.next() else {
                return Cow::Owned(canonical_keyword);
            };
            let mut out = String::with_capacity(canonical_keyword.len());
            out.extend(first.to_uppercase());
            out.push_str(&chars.as_str().to_ascii_lowercase());
            Cow::Owned(out)
        }
        _ => Cow::Borrowed(original),
    }
}

fn keyword_camel_text(kind: SyntaxKind) -> Option<String> {
    if !kind.is_keyword() {
        return None;
    }

    let kind_name = kind.to_string();
    kind_name
        .strip_suffix("Keyword")
        .map(std::string::ToString::to_string)
}
