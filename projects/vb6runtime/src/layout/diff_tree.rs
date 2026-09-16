//! Diff types for incremental layout rendering.
//!
//! This module defines the types used to describe changes between two versions
//! of a layout tree, enabling incremental (diff-aware) rendering.
//!
//! The diff is a **flat** list of leaf-level changes. Containers whose children
//! change are re-rendered in full — move detection is intentionally omitted for
//! Phase 1 simplicity.

use super::model::{LayoutNode, NodeId};

/// Describes the type of change for a node in the layout tree.
///
/// `Same` means the node is identical to what's currently in the DOM.
/// `Inserted` / `Removed` describe structural changes to the tree.
/// `ValueChanged`, `VisibilityChanged`, `EnabledChanged` describe
/// mutations on existing nodes.
#[derive(Clone, Debug, PartialEq)]
pub enum DiffKind {
    /// No change — the node is identical to what's currently rendered.
    Same,

    /// Control's writable value changed (text, caption, checked state).
    ValueChanged {
        /// Previous value.
        old_value: Option<String>,
        /// New value.
        new_value: Option<String>,
    },

    /// Visibility changed.
    VisibilityChanged {
        /// Previous visibility.
        old_visible: bool,
        /// New visibility.
        new_visible: bool,
    },

    /// Enabled state changed.
    EnabledChanged {
        /// Previous enabled state.
        old_enabled: bool,
        /// New enabled state.
        new_enabled: bool,
    },

    /// A new child appeared in the tree.
    Inserted {
        /// The newly inserted node.
        node: Box<LayoutNode>,
    },

    /// A child was removed from the tree.
    Removed,

    /// Multiple children in sequence changed (used for container-level diffs).
    /// Each pair is (position, new node) for inserts.
    ChildrenChanged {
        /// Positions and nodes of newly inserted children.
        inserts: Vec<(usize, LayoutNode)>,
    },
}

/// A single diff change for one node in the tree.
#[derive(Clone, Debug, PartialEq)]
pub struct DiffChange {
    /// Unique identity of the changed node.
    pub id: NodeId,
    /// What kind of change occurred.
    pub kind: DiffKind,
    /// Diff changes for children of this node.
    pub child_diff: DiffTree,
}

/// Result of diffing two layout tree states.
///
/// Contains a flat list of leaf-level changes and a list of container IDs
/// that were confirmed unchanged (skippable during incremental rendering).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DiffTree {
    /// Leaf-level changes — each describes what changed on one control.
    pub leaf_changes: Vec<DiffChange>,
    /// Container IDs whose entire subtree was confirmed unchanged.
    pub unchanged_containers: Vec<NodeId>,
}

impl DiffTree {
    /// Create an empty diff tree.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a leaf change to this diff tree.
    pub fn push(&mut self, change: DiffChange) {
        self.leaf_changes.push(change);
    }

    /// Return `true` if this diff tree has no changes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.leaf_changes.is_empty() && self.unchanged_containers.is_empty()
    }

    /// Look up a diff change by node ID.
    ///
    /// Returns `None` if no change is recorded for the given ID.
    #[must_use]
    pub fn get_change(&self, id: &NodeId) -> Option<&DiffChange> {
        self.leaf_changes.iter().find(|c| &c.id == id)
    }
}

/// Collect all leaf changes from a diff tree recursively.
///
/// Walks both the top-level `leaf_changes` and any `child_diff` subtrees
/// to produce a flat list of all changes in the tree.
pub fn collect_all_changes(tree: &DiffTree) -> Vec<&DiffChange> {
    let mut result = Vec::new();
    for change in &tree.leaf_changes {
        result.push(change);
        result.extend(collect_all_changes(&change.child_diff));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::super::model::{LayoutControlType, LayoutLeaf};
    use super::*;

    fn test_node_id() -> NodeId {
        NodeId {
            name: "lbl1".into(),
            kind: LayoutControlType::Label,
            index: 0,
        }
    }

    #[test]
    fn diff_kind_same() {
        assert!(matches!(DiffKind::Same, DiffKind::Same));
    }

    #[test]
    fn diff_kind_value_changed() {
        let kind = DiffKind::ValueChanged {
            old_value: Some("Old".into()),
            new_value: Some("New".into()),
        };
        assert!(matches!(kind, DiffKind::ValueChanged { .. }));
    }

    #[test]
    fn diff_kind_visibility_changed() {
        let kind = DiffKind::VisibilityChanged {
            old_visible: true,
            new_visible: false,
        };
        assert!(matches!(kind, DiffKind::VisibilityChanged { .. }));
    }

    #[test]
    fn diff_kind_enabled_changed() {
        let kind = DiffKind::EnabledChanged {
            old_enabled: true,
            new_enabled: false,
        };
        assert!(matches!(kind, DiffKind::EnabledChanged { .. }));
    }

    #[test]
    fn diff_kind_inserted() {
        let leaf = Box::new(LayoutNode::Leaf(LayoutLeaf {
            name: "new".into(),
            control_type: LayoutControlType::Label,
            value: Some("Hello".into()),
            ..Default::default()
        }));
        let kind = DiffKind::Inserted { node: leaf };
        assert!(matches!(kind, DiffKind::Inserted { .. }));
    }

    #[test]
    fn diff_kind_removed() {
        assert!(matches!(DiffKind::Removed, DiffKind::Removed));
    }

    #[test]
    fn diff_kind_children_changed() {
        let kind = DiffKind::ChildrenChanged { inserts: vec![] };
        assert!(matches!(kind, DiffKind::ChildrenChanged { .. }));
    }

    #[test]
    fn diff_tree_empty() {
        let tree = DiffTree::default();
        assert!(tree.is_empty());
        assert!(tree.leaf_changes.is_empty());
        assert!(tree.unchanged_containers.is_empty());
    }

    #[test]
    fn diff_tree_new() {
        let tree = DiffTree::new();
        assert!(tree.is_empty());
    }

    #[test]
    fn diff_tree_push() {
        let mut tree = DiffTree::new();
        tree.push(DiffChange {
            id: test_node_id(),
            kind: DiffKind::ValueChanged {
                old_value: None,
                new_value: Some("Hello".into()),
            },
            child_diff: DiffTree::default(),
        });
        assert!(!tree.is_empty());
        assert_eq!(tree.leaf_changes.len(), 1);
    }

    #[test]
    fn diff_tree_get_change_found() {
        let mut tree = DiffTree::new();
        let id = test_node_id();
        tree.push(DiffChange {
            id: id.clone(),
            kind: DiffKind::Same,
            child_diff: DiffTree::default(),
        });
        assert!(tree.get_change(&id).is_some());
    }

    #[test]
    fn diff_tree_get_change_not_found() {
        let tree = DiffTree::new();
        let id = NodeId {
            name: "nonexistent".into(),
            kind: LayoutControlType::Label,
            index: 0,
        };
        assert!(tree.get_change(&id).is_none());
    }

    #[test]
    fn diff_change_value_changed() {
        let change = DiffChange {
            id: test_node_id(),
            kind: DiffKind::ValueChanged {
                old_value: Some("Old".into()),
                new_value: Some("New".into()),
            },
            child_diff: DiffTree::default(),
        };
        assert!(matches!(change.kind, DiffKind::ValueChanged { .. }));
    }

    #[test]
    fn collect_all_changes_flat() {
        let mut tree = DiffTree::new();
        tree.push(DiffChange {
            id: NodeId {
                name: "a".into(),
                kind: LayoutControlType::Label,
                index: 0,
            },
            kind: DiffKind::Same,
            child_diff: DiffTree::default(),
        });
        tree.push(DiffChange {
            id: NodeId {
                name: "b".into(),
                kind: LayoutControlType::TextBox,
                index: 0,
            },
            kind: DiffKind::VisibilityChanged {
                old_visible: true,
                new_visible: false,
            },
            child_diff: DiffTree::default(),
        });
        let all = collect_all_changes(&tree);
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn collect_all_changes_nested() {
        let mut child_tree = DiffTree::new();
        child_tree.push(DiffChange {
            id: NodeId {
                name: "child".into(),
                kind: LayoutControlType::Label,
                index: 0,
            },
            kind: DiffKind::Same,
            child_diff: DiffTree::default(),
        });
        let mut tree = DiffTree::new();
        tree.push(DiffChange {
            id: NodeId {
                name: "parent".into(),
                kind: LayoutControlType::Frame,
                index: 0,
            },
            kind: DiffKind::Same,
            child_diff: child_tree,
        });
        let all = collect_all_changes(&tree);
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn diff_kind_equality() {
        let a = DiffKind::Same;
        let b = DiffKind::Same;
        assert_eq!(a, b);

        let c = DiffKind::Removed;
        let d = DiffKind::Removed;
        assert_eq!(c, d);
    }

    #[test]
    fn diff_change_equality() {
        let id = test_node_id();
        let change_a = DiffChange {
            id: id.clone(),
            kind: DiffKind::Same,
            child_diff: DiffTree::default(),
        };
        let change_b = DiffChange {
            id,
            kind: DiffKind::Same,
            child_diff: DiffTree::default(),
        };
        assert_eq!(change_a, change_b);
    }
}
