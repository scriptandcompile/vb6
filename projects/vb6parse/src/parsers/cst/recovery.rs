use super::{ConcreteSyntaxTree, VB6Language};
use crate::errors::Span;
use crate::language::Token;
use crate::parsers::SyntaxKind;

/// Strategy used by parser recovery when creating `ErrorRecovery` nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryStrategy {
    /// Recover by consuming exactly one unexpected token.
    SingleToken,
    /// Recover by consuming tokens until the end of the current line.
    ToNewline,
    /// Recover from a mismatched `End <block>` terminator.
    ProcedureTerminator,
}

/// A single parser recovery event captured during CST construction.
#[derive(Debug, Clone)]
pub struct RecoveryEvent {
    /// Monotonic event id in parse order.
    pub id: usize,
    /// Human-readable expectations for this parser location.
    pub expected: Vec<String>,
    /// Tokens consumed during recovery.
    pub found: Vec<Token>,
    /// Recovery strategy that was used.
    pub strategy: RecoveryStrategy,
    /// Span at which recovery started.
    pub span: Span,
}

/// Byte range for a CST node in source content.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodeRange {
    /// Inclusive start byte offset.
    pub start: u32,
    /// Exclusive end byte offset.
    pub end: u32,
}

impl ConcreteSyntaxTree {
    /// Returns byte ranges for each `ErrorRecovery` node in the CST.
    #[must_use]
    pub fn error_recovery_ranges(&self) -> Vec<NodeRange> {
        let syntax_node = rowan::SyntaxNode::<VB6Language>::new_root(self.root.clone());

        syntax_node
            .descendants()
            .filter(|node| node.kind() == SyntaxKind::ErrorRecovery)
            .map(|node| {
                let range = node.text_range();
                NodeRange {
                    start: range.start().into(),
                    end: range.end().into(),
                }
            })
            .collect()
    }
}
