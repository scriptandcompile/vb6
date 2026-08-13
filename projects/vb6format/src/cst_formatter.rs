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

        if is_designer_node(node.kind()) {
            self.emit_verbatim(node);
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

    /// Copies a subtree to the output exactly as it was read, without running
    /// any pass over it.
    fn emit_verbatim(&mut self, node: &CstNode) {
        let text = node.text();
        let ends_line = text.ends_with('\n');

        self.output.push_str(text);

        // Leave the layout state as if the copied text had been emitted token
        // by token, so that whatever follows the block is still formatted.
        self.context.pending_indent = ends_line;
        self.context.line_has_content = !ends_line;
        self.context.last_was_blank = false;
    }
}

/// The designer section of a `.frm`, `.ctl` or `.cls` file: the `VERSION`
/// header and the `Begin ... End` block that the VB6 IDE itself writes and
/// reads.
///
/// That section is not source code. Reformatting it fights the IDE, which
/// rewrites the block in its own layout every time the form is saved, so any
/// change here comes straight back as diff noise. It is also where a form's
/// control geometry lives, which makes silently rewriting it the riskiest
/// thing a formatter could do to a VB6 project. It is copied through
/// unchanged.
fn is_designer_node(kind: SyntaxKind) -> bool {
    matches!(
        kind,
        SyntaxKind::VersionStatement | SyntaxKind::PropertiesBlock
    )
}
