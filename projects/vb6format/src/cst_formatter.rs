use vb6parse::ConcreteSyntaxTree;
use vb6parse::parsers::{CstNode, SyntaxKind};

use crate::context::Context;
use crate::passes::PassManager;
use crate::settings::FmtSettings;

pub(super) struct CstFormatter<'a> {
    cst: ConcreteSyntaxTree,
    output: String,
    context: Context,
    passes: PassManager<'a>,
}

impl<'a> CstFormatter<'a> {
    pub(super) fn new(cst: ConcreteSyntaxTree, settings: &'a FmtSettings) -> Self {
        let root = cst.to_root_node();
        let output = String::with_capacity(root.text().len());

        Self {
            cst,
            output,
            context: Context::new(0),
            passes: PassManager::new(settings),
        }
    }

    pub(super) fn format(mut self) -> String {
        let root = self.cst.to_root_node();
        self.walk_node(&root);
        self.output
    }

    fn walk_node(&mut self, node: &CstNode) {
        if node.is_token() {
            self.emit_token(node);
            return;
        }

        self.passes.on_node_enter(node, &mut self.context);

        match node.kind() {
            SyntaxKind::StatementList => {
                self.context.indent_level += 1;
                for child in node.children() {
                    self.walk_node(child);
                }
                self.context.indent_level = self.context.indent_level.saturating_sub(1);
            }
            _ => {
                for child in node.children() {
                    self.walk_node(child);
                }
            }
        }

        self.passes.on_node_exit(node, &mut self.context);
    }

    fn emit_token(&mut self, token: &CstNode) {
        let buffer = self.passes.on_token(token, &mut self.context);
        if buffer.emit {
            self.output.push_str(&buffer.prefix);
            self.output.push_str(&buffer.text);
        }
    }
}
