mod compiler_directive;
mod deduplicate_blank_lines;
mod keyword;
mod layout;
mod line_ending;
mod top_level_spacing;

use crate::context::Context;
use crate::settings::FmtSettings;
use vb6parse::parsers::CstNode;

use compiler_directive::CompilerDirectivePass;
use deduplicate_blank_lines::DeduplicateBlankLinesPass;
use keyword::KeywordCasePass;
use layout::LayoutPass;
use line_ending::LineEndingPass;
use top_level_spacing::TopLevelSpacingPass;

pub struct TokenBuffer {
    pub prefix: String,
    pub text: String,
    pub emit: bool,
}

impl TokenBuffer {
    fn from_token(token: &CstNode) -> Self {
        Self {
            prefix: String::new(),
            text: token.text().to_string(),
            emit: true,
        }
    }
}

pub trait FormatPass {
    fn on_node_enter(&self, _node: &CstNode, _context: &mut Context) {}

    fn on_node_exit(&self, _node: &CstNode, _context: &mut Context) {}

    fn on_token(&self, _token: &CstNode, _context: &mut Context, _buffer: &mut TokenBuffer) {}
}

pub struct PassManager<'a> {
    passes: Vec<Box<dyn FormatPass + 'a>>,
}

impl<'a> PassManager<'a> {
    pub fn new(settings: &'a FmtSettings) -> Self {
        let passes: Vec<Box<dyn FormatPass + 'a>> = vec![
            Box::new(LineEndingPass::new()),
            Box::new(CompilerDirectivePass::new(settings)),
            Box::new(KeywordCasePass::new(settings)),
            Box::new(LayoutPass::new(settings)),
            Box::new(TopLevelSpacingPass::new(settings)),
            Box::new(DeduplicateBlankLinesPass::new()),
        ];

        Self { passes }
    }

    pub fn on_node_enter(&self, node: &CstNode, context: &mut Context) {
        for pass in &self.passes {
            pass.on_node_enter(node, context);
        }
    }

    pub fn on_node_exit(&self, node: &CstNode, context: &mut Context) {
        for pass in &self.passes {
            pass.on_node_exit(node, context);
        }
    }

    pub fn on_token(&self, token: &CstNode, context: &mut Context) -> TokenBuffer {
        let mut buffer = TokenBuffer::from_token(token);
        for pass in &self.passes {
            pass.on_token(token, context, &mut buffer);
        }
        buffer
    }
}
