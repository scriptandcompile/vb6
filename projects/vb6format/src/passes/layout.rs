use crate::context::Context;
use crate::passes::{FormatPass, TokenBuffer};
use crate::settings::FmtSettings;
use vb6parse::SyntaxKind;
use vb6parse::parsers::CstNode;

pub struct LayoutPass<'a> {
    settings: &'a FmtSettings,
}

impl<'a> LayoutPass<'a> {
    pub fn new(settings: &'a FmtSettings) -> Self {
        Self { settings }
    }
}

impl FormatPass for LayoutPass<'_> {
    fn on_token(&self, token: &CstNode, context: &mut Context, buffer: &mut TokenBuffer) {
        match token.kind() {
            SyntaxKind::Whitespace => {
                if context.pending_indent {
                    buffer.emit = false;
                } else {
                    buffer.text = " ".to_string();
                    buffer.emit = true;
                }
            }
            SyntaxKind::ErrorExpectedTokens | SyntaxKind::ErrorMissingTokens => {
                buffer.emit = false;
            }
            SyntaxKind::Newline => {}
            SyntaxKind::EndOfLineComment | SyntaxKind::RemComment => {
                if context.pending_indent {
                    let indent = context.indent_level * self.settings.indent_size;
                    for _ in 0..indent {
                        buffer.prefix.push(' ');
                    }
                    context.pending_indent = false;
                }
                context.last_was_blank = false;
                context.line_has_content = true;
            }
            _ => {
                if context.pending_indent {
                    let indent = context.indent_level * self.settings.indent_size;
                    for _ in 0..indent {
                        buffer.prefix.push(' ');
                    }
                    context.pending_indent = false;
                }
                context.last_was_blank = false;
                context.line_has_content = true;
            }
        }
    }
}
