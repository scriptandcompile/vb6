use std::cell::Cell;

use crate::context::Context;
use crate::passes::{FormatPass, TokenBuffer};
use vb6parse::parsers::CstNode;

pub struct DeduplicateBlankLinesPass {
    newline_streak: Cell<usize>,
}

impl DeduplicateBlankLinesPass {
    pub fn new() -> Self {
        Self {
            newline_streak: Cell::new(0),
        }
    }
}

impl FormatPass for DeduplicateBlankLinesPass {
    fn on_token(&self, _token: &CstNode, context: &mut Context, buffer: &mut TokenBuffer) {
        let line_ending = context.line_ending();
        if !buffer.emit {
            return;
        }

        buffer.prefix = sanitize_chunk(&buffer.prefix, line_ending, &self.newline_streak);
        buffer.text = sanitize_chunk(&buffer.text, line_ending, &self.newline_streak);

        if buffer.prefix.is_empty() && buffer.text.is_empty() {
            buffer.emit = false;
        }
    }
}

fn sanitize_chunk(chunk: &str, line_ending: &str, streak: &Cell<usize>) -> String {
    let mut out = String::with_capacity(chunk.len());
    let mut index = 0;

    while index < chunk.len() {
        let tail = &chunk[index..];
        if tail.starts_with(line_ending) {
            if streak.get() < 2 {
                out.push_str(line_ending);
            }
            streak.set(streak.get().saturating_add(1));
            index += line_ending.len();
            continue;
        }

        let mut chars = tail.chars();
        if let Some(ch) = chars.next() {
            out.push(ch);
            if ch != '\n' && ch != '\r' {
                streak.set(0);
            }
            index += ch.len_utf8();
        } else {
            break;
        }
    }

    out
}
