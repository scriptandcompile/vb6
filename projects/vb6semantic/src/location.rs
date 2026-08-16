//! Byte-offset → (line, column) mapping for CST nodes.
//!
//! The CST stores every token with its byte range in the source, but the
//! analyzer reports positions as 1-based line/column pairs. [`LineIndex`]
//! precomputes the byte offset where each line starts from the CST's
//! `Newline` tokens, turning any token offset into a precise position in
//! O(log n) — no token-walking required at query time.

use vb6parse::parsers::SyntaxKind;
use vb6parse::parsers::cst::CstNode;

/// Maps byte offsets in a CST to 1-based `(line, column)` positions.
#[derive(Debug, Clone, Default)]
pub struct LineIndex {
    /// Byte offset of the start of each line. Line *i* (1-based) starts at
    /// `line_starts[i - 1]`; `line_starts[0]` is always `0`.
    line_starts: Vec<u32>,
}

impl LineIndex {
    /// Build a line index from a CST root by recording every `Newline` token's
    /// end offset as the start of the following line.
    pub fn from_cst_root(root: &CstNode) -> Self {
        let mut line_starts = vec![0];
        for node in root.descendants() {
            if node.is_token() && node.kind() == SyntaxKind::Newline {
                line_starts.push(node.end_offset());
            }
        }
        Self { line_starts }
    }

    /// The number of lines covered by this index.
    pub fn line_count(&self) -> usize {
        self.line_starts.len()
    }

    /// The 1-based `(line, column)` of a byte offset. Offsets past the final
    /// newline are clamped to the last line.
    pub fn position(&self, offset: u32) -> (usize, usize) {
        let lo = self.line_starts.partition_point(|start| *start <= offset);
        let line_index = lo.saturating_sub(1);
        let line = line_index + 1;
        let column = (offset - self.line_starts[line_index] + 1) as usize;
        (line, column)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vb6parse::parsers::cst::ConcreteSyntaxTree;

    fn index_for(source: &str) -> LineIndex {
        let (cst_opt, failures) = ConcreteSyntaxTree::from_text("test.bas", source).unpack();
        assert!(failures.is_empty(), "parse failures: {failures:?}");
        let cst = cst_opt.expect("CST should parse");
        LineIndex::from_cst_root(&cst.to_root_node())
    }

    #[test]
    fn single_line_positions() {
        let index = index_for("Foo = 1");
        assert_eq!(index.position(0), (1, 1));
        assert_eq!(index.position(4), (1, 5));
        assert_eq!(index.position(7), (1, 8));
    }

    #[test]
    fn positions_across_newlines() {
        // Line 1: "Sub Foo()"     (offsets 0..9, \n at 9)
        // Line 2: "  x = 1"       (offsets 10..17, \n at 17)
        // Line 3: "End Sub"       (offsets 18..25, \n at 25)
        let index = index_for("Sub Foo()\n  x = 1\nEnd Sub\n");
        assert_eq!(index.position(9), (1, 10));
        assert_eq!(index.position(10), (2, 1));
        assert_eq!(index.position(12), (2, 3));
        assert_eq!(index.position(19), (3, 2));
        assert_eq!(index.position(24), (3, 7));
        assert_eq!(index.position(26), (4, 1));
    }

    #[test]
    fn offset_past_last_newline_clamps_to_last_line() {
        // "a = 1" is offsets 0..4, the \n is at offset 5 (end 6), so the
        // position of the newline itself is still reported on line 1.
        let index = index_for("a = 1\n");
        assert_eq!(index.position(5), (1, 6));
        assert_eq!(index.position(100), (2, 95));
    }

    #[test]
    fn empty_source_has_single_line() {
        let index = index_for("");
        assert_eq!(index.line_count(), 1);
        assert_eq!(index.position(0), (1, 1));
    }

    #[test]
    fn crlf_newlines_advance_one_line() {
        // \r and \n are separate tokens; the Newline token spans the \n only,
        // so the next line starts right after it.
        let index = index_for("a = 1\r\nb = 2\r\n");
        assert_eq!(index.position(6), (1, 7));
        assert_eq!(index.position(7), (2, 1));
        assert_eq!(index.position(13), (2, 7));
        assert_eq!(index.position(14), (3, 1));
    }
}
