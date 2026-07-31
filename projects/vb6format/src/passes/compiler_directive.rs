use crate::context::{Context, DirectivePhase};
use crate::passes::{FormatPass, TokenBuffer};
use crate::settings::FmtSettings;
use vb6parse::SyntaxKind;
use vb6parse::parsers::CstNode;

pub struct CompilerDirectivePass<'a> {
    settings: &'a FmtSettings,
}

impl<'a> CompilerDirectivePass<'a> {
    pub fn new(settings: &'a FmtSettings) -> Self {
        Self { settings }
    }

    fn is_compiler_directive(node: &CstNode) -> bool {
        node.kind() == SyntaxKind::CompilerDirective
    }
}

impl FormatPass for CompilerDirectivePass<'_> {
    fn on_node_enter(&self, node: &CstNode, context: &mut Context) {
        if Self::is_compiler_directive(node) {
            if self.settings.blank_lines_around_directives {
                context.pending_blank = true;
                context.directive_phase = Some(DirectivePhase::BeforeDirective);
            }
            context.compiler_directive_depth = context.compiler_directive_depth.saturating_add(1);
            return;
        }

        if node.kind() == SyntaxKind::StatementList
            && context.compiler_directive_depth > 0
            && self.settings.blank_lines_inside_directives
        {
            context.pending_blank = true;
            context.directive_phase = Some(DirectivePhase::BeforeBody);
            return;
        }

        if node.kind() == SyntaxKind::CompilerEndIfClause
            && context.compiler_directive_depth > 0
            && self.settings.blank_lines_inside_directives
        {
            context.pending_blank = true;
            context.directive_phase = Some(DirectivePhase::BeforeBody);
        }
    }

    fn on_node_exit(&self, node: &CstNode, context: &mut Context) {
        if !Self::is_compiler_directive(node) {
            return;
        }

        context.compiler_directive_depth = context.compiler_directive_depth.saturating_sub(1);

        if self.settings.blank_lines_around_directives {
            context.pending_blank = true;
            context.directive_phase = Some(DirectivePhase::AfterDirective);
        }
    }

    fn on_token(&self, token: &CstNode, context: &mut Context, buffer: &mut TokenBuffer) {
        match token.kind() {
            SyntaxKind::Whitespace
            | SyntaxKind::Newline
            | SyntaxKind::ErrorExpectedTokens
            | SyntaxKind::ErrorMissingTokens => return,
            _ => {}
        }

        if context.pending_blank {
            if context.directive_phase.is_some() {
                buffer.prefix.push_str(context.line_ending());
                buffer.prefix.push_str(context.line_ending());
            } else {
                buffer.prefix.push_str(context.line_ending());
            }
            context.pending_blank = false;
            context.directive_phase = None;
        }
    }
}
