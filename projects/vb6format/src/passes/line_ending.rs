use std::cell::Cell;

use crate::context::{Context, LineEnding};
use crate::passes::{FormatPass, TokenBuffer};
use vb6parse::SyntaxKind;
use vb6parse::parsers::CstNode;

pub struct LineEndingPass {
    initialized: Cell<bool>,
}

impl LineEndingPass {
    pub fn new() -> Self {
        Self {
            initialized: Cell::new(false),
        }
    }
}

impl FormatPass for LineEndingPass {
    fn on_node_enter(&self, node: &CstNode, context: &mut Context) {
        if self.initialized.get() {
            return;
        }
        self.initialized.set(true);

        context.line_ending = if node.text().contains("\r\n") {
            LineEnding::CrLf
        } else {
            LineEnding::Lf
        };
    }

    fn on_token(&self, token: &CstNode, context: &mut Context, buffer: &mut TokenBuffer) {
        if token.kind() != SyntaxKind::Newline {
            return;
        }

        if !context.line_has_content {
            context.last_was_blank = true;
        }
        context.pending_indent = true;
        context.line_has_content = false;
        buffer.text = context.line_ending().to_string();
        buffer.emit = true;
    }
}
