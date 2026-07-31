use vb6parse::parsers::CstNode;
use crate::context::Context;

/// Represents the possible outcomes of a rewrite operation.
pub enum RewriteAction {
    /// The node is consumed (removed from the output).
    Consume,
    /// The node is replaced by the provided string.
    Replace(String),
}

/// A result type for rewrite operations, combining success actions and errors.
pub type RewriteResult = Result<RewriteAction, String>;

/// The trait that every formatting rule must implement.
pub trait Rewrite {
    fn rewrite(&self, node: &CstNode, context: &Context) -> RewriteResult;
}
