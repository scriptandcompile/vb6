//! Query index over resolved identifier occurrences.
//!
//! The analyzer records every identifier it can resolve as a [`Reference`],
//! keyed by the scope and (case-insensitive) name of the symbol it resolves
//! to. [`QueryIndex`] answers the editor-oriented questions behind
//! go-to-definition and find-references:
//!
//! - `references_for(scope_id, name)` — every occurrence of a symbol
//!   (including its definition).
//! - `symbol_at(file, line, column)` — which symbol an identifier at a
//!   position resolves to.
//! - `references_at(file, line, column)` — all occurrences of the symbol
//!   under the cursor.
//!
//! References are recorded during analysis with precise byte offsets and
//! 1-based positions (see [`crate::location::LineIndex`]).

use crate::error::SourceLocation;
use std::collections::{BTreeMap, HashSet};

/// The role an identifier occurrence plays for its symbol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReferenceKind {
    /// The declaration that defines the symbol.
    Definition,
    /// A use of the symbol as a value or call target.
    Usage,
    /// A use of the symbol as a type name (`As` clause, `New` target).
    TypeReference,
}

/// A single occurrence of a symbol in the source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reference {
    /// The role this occurrence plays for its symbol.
    pub kind: ReferenceKind,
    /// 1-based start position of the identifier.
    pub location: SourceLocation,
    /// Inclusive start byte offset of the identifier.
    pub start_offset: u32,
    /// Exclusive end byte offset of the identifier.
    pub end_offset: u32,
    /// 1-based exclusive column of the identifier's end (identifier text is
    /// ASCII, so `width = end_offset - start_offset` in characters).
    pub end_column: usize,
}

impl Reference {
    /// Create a reference from a start position and byte range.
    pub fn new(
        kind: ReferenceKind,
        location: SourceLocation,
        start_offset: u32,
        end_offset: u32,
    ) -> Self {
        let width = (end_offset - start_offset) as usize;
        Self {
            kind,
            end_column: location.column + width,
            location,
            start_offset,
            end_offset,
        }
    }

    /// The 1-based exclusive column of the identifier's end.
    pub fn end_column(&self) -> usize {
        self.end_column
    }
}

/// Identity of a symbol inside a query index: unique within its scope.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SymbolKey {
    /// Scope the symbol lives in.
    pub scope_id: usize,
    /// Lowercased symbol name (VB6 names are case-insensitive).
    pub name: String,
}

/// A single occurrence plus the key of its symbol, kept sorted by
/// `(file, line, start_column)` for position queries.
#[derive(Debug, Clone, PartialEq, Eq)]
struct PositionedReference {
    file: String,
    line: usize,
    start_column: usize,
    end_column: usize,
    key: SymbolKey,
}

/// Bidirectional index: symbol → occurrences and position → symbol.
#[derive(Debug, Clone, Default)]
pub struct QueryIndex {
    /// Symbol (`scope_id`, lowercase name) → its occurrences, in record order.
    by_symbol: BTreeMap<SymbolKey, Vec<Reference>>,
    /// Byte ranges of every recorded occurrence, for deduplication during
    /// usage collection (`file`, `start_offset`, `end_offset`).
    recorded_ranges: HashSet<(String, u32, u32)>,
    /// All occurrences sorted by position, built by [`QueryIndex::finalize`].
    by_position: Vec<PositionedReference>,
}

impl QueryIndex {
    /// Create an empty index.
    pub fn new() -> Self {
        Self::default()
    }

    /// Record an occurrence of a symbol.
    pub fn record(&mut self, scope_id: usize, name: &str, reference: Reference) {
        self.recorded_ranges.insert((
            reference.location.file.clone(),
            reference.start_offset,
            reference.end_offset,
        ));
        let key = SymbolKey {
            scope_id,
            name: name.to_lowercase(),
        };
        self.by_symbol
            .entry(key.clone())
            .or_default()
            .push(reference.clone());
        self.by_position.push(PositionedReference {
            file: reference.location.file,
            line: reference.location.line,
            start_column: reference.location.column,
            end_column: reference.end_column,
            key,
        });
    }

    /// Whether an identifier byte range has already been recorded in `file`
    /// (used to avoid double-counting occurrences during collection).
    pub fn is_recorded(&self, file: &str, start: u32, end: u32) -> bool {
        self.recorded_ranges
            .contains(&(file.to_string(), start, end))
    }

    /// Sort occurrences by position so position queries can binary-search.
    /// Call once after collection completes.
    pub fn finalize(&mut self) {
        self.by_position.sort_by(|a, b| {
            a.file
                .cmp(&b.file)
                .then(a.line.cmp(&b.line))
                .then(a.start_column.cmp(&b.start_column))
        });
    }

    /// All occurrences of the symbol in `scope_id` named `name`
    /// (case-insensitive), including its definition, if any.
    pub fn references_for(&self, scope_id: usize, name: &str) -> Option<&Vec<Reference>> {
        let key = SymbolKey {
            scope_id,
            name: name.to_lowercase(),
        };
        self.by_symbol.get(&key)
    }

    /// The symbol whose identifier covers `(file, line, column)`, if any.
    pub fn symbol_at(&self, file: &str, line: usize, column: usize) -> Option<&SymbolKey> {
        let start = self.by_position.partition_point(|p| {
            p.file.as_str() < file || (p.file.as_str() == file && p.line < line)
        });
        for positioned in &self.by_position[start..] {
            if positioned.file != file || positioned.line != line {
                break;
            }
            if column >= positioned.start_column && column < positioned.end_column {
                return Some(&positioned.key);
            }
        }
        None
    }

    /// All occurrences of the symbol under the cursor (definition included).
    pub fn references_at(&self, file: &str, line: usize, column: usize) -> Option<&Vec<Reference>> {
        let key = self.symbol_at(file, line, column)?;
        self.by_symbol.get(key)
    }

    /// The definition of the symbol under the cursor, if any.
    pub fn definition_at(&self, file: &str, line: usize, column: usize) -> Option<&Reference> {
        self.references_at(file, line, column)
            .and_then(|references| {
                references
                    .iter()
                    .find(|r| r.kind == ReferenceKind::Definition)
            })
    }

    /// Iterate over every symbol and its occurrences.
    pub fn iter(&self) -> impl Iterator<Item = (&SymbolKey, &Vec<Reference>)> {
        self.by_symbol.iter()
    }

    /// Number of recorded occurrences.
    pub fn len(&self) -> usize {
        self.by_position.len()
    }

    /// Whether the index holds no occurrences.
    pub fn is_empty(&self) -> bool {
        self.by_position.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn location(file: &str, line: usize, column: usize) -> SourceLocation {
        SourceLocation {
            file: file.to_string(),
            line,
            column,
        }
    }

    fn record(index: &mut QueryIndex, scope: usize, name: &str, kind: ReferenceKind, line: usize, col: usize) {
        let reference = Reference::new(kind, location("M.bas", line, col), 10, 14);
        index.record(scope, name, reference);
    }

    #[test]
    fn references_for_is_case_insensitive() {
        let mut index = QueryIndex::new();
        record(&mut index, 1, "Counter", ReferenceKind::Definition, 1, 1);
        record(&mut index, 1, "Counter", ReferenceKind::Usage, 5, 3);
        assert_eq!(index.references_for(1, "counter").unwrap().len(), 2);
        assert_eq!(index.references_for(1, "COUNTER").unwrap().len(), 2);
        assert!(index.references_for(2, "Counter").is_none());
    }

    #[test]
    fn symbol_at_matches_identifier_span() {
        let mut index = QueryIndex::new();
        record(&mut index, 1, "Foo", ReferenceKind::Definition, 3, 7);
        index.finalize();

        let key = index.symbol_at("M.bas", 3, 7).expect("start column");
        assert_eq!(key.name, "foo");
        let key = index.symbol_at("M.bas", 3, 10).expect("end column");
        assert_eq!(key.name, "foo");
        assert!(index.symbol_at("M.bas", 3, 6).is_none());
        assert!(index.symbol_at("M.bas", 3, 11).is_none());
        assert!(index.symbol_at("M.bas", 4, 7).is_none());
        assert!(index.symbol_at("other.bas", 3, 7).is_none());
    }

    #[test]
    fn references_at_and_definition_at() {
        let mut index = QueryIndex::new();
        record(&mut index, 1, "Helper", ReferenceKind::Definition, 2, 5);
        record(&mut index, 1, "Helper", ReferenceKind::Usage, 9, 2);
        index.finalize();

        let refs = index.references_at("M.bas", 9, 2).unwrap();
        assert_eq!(refs.len(), 2);
        assert_eq!(refs[1].kind, ReferenceKind::Usage);

        let def = index.definition_at("M.bas", 9, 2).unwrap();
        assert_eq!(def.kind, ReferenceKind::Definition);
        assert_eq!(def.location.line, 2);
    }

    #[test]
    fn is_recorded_tracks_recorded_ranges() {
        let mut index = QueryIndex::new();
        record(&mut index, 1, "Foo", ReferenceKind::Definition, 1, 1);
        record(&mut index, 1, "Foo", ReferenceKind::Usage, 5, 3);
        assert!(index.is_recorded("M.bas", 10, 14));
        assert!(!index.is_recorded("M.bas", 10, 15));
        assert!(!index.is_recorded("N.bas", 10, 14));
    }
}
