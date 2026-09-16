//! Snapshot types for incremental layout diffing.
//!
//! A [`SnapshotNode`] is a lightweight, serializable representation of a [`LayoutNode`]
//! that captures only the fields used for change detection. Snapshots are captured
//! after each render and compared against the live model to produce a [`DiffTree`].
//!
//! This module provides:
//! - [`SnapshotNode`] — a lightweight node representation for diff comparison
//! - [`LayoutNode::to_snapshot()`] — conversion from the live model to a snapshot

use super::model::{LayoutNode, LayoutStyle, NodeId};

/// A lightweight, serializable representation of a [`LayoutNode`] for diff comparison.
///
/// Captures only the fields that affect rendering output: identity, style, value,
/// visibility, enabled state, and child IDs. Used to build [`DiffTree`] instances
/// by comparing against the live model.
#[derive(Clone, Debug, PartialEq)]
pub struct SnapshotNode {
    /// Unique identity of the node.
    pub id: NodeId,
    /// Computed CSS-compatible style properties at the time of snapshot.
    pub style: LayoutStyle,
    /// Runtime-writable value (text, caption, checked state, etc.).
    pub value: Option<String>,
    /// Whether the node was visible at the time of snapshot.
    pub visible: bool,
    /// Whether the node was enabled at the time of snapshot.
    pub enabled: bool,
    /// IDs of child nodes in order. Empty for leaf controls.
    pub child_ids: Vec<NodeId>,
}

impl SnapshotNode {
    /// Create a new snapshot from a node ID and style.
    #[must_use]
    pub fn new(id: NodeId, style: LayoutStyle) -> Self {
        Self {
            id,
            style,
            value: None,
            visible: true,
            enabled: true,
            child_ids: Vec::new(),
        }
    }

    /// Create a leaf snapshot with no children.
    #[must_use]
    pub fn leaf(id: NodeId, style: LayoutStyle) -> Self {
        Self {
            id,
            style,
            value: None,
            visible: true,
            enabled: true,
            child_ids: Vec::new(),
        }
    }

    /// Create a container snapshot with child IDs.
    #[must_use]
    pub fn container(id: NodeId, style: LayoutStyle, child_ids: Vec<NodeId>) -> Self {
        Self {
            id,
            style,
            value: None,
            visible: true,
            enabled: true,
            child_ids,
        }
    }

    /// Set the value field.
    #[must_use]
    pub fn with_value(mut self, value: Option<String>) -> Self {
        self.value = value;
        self
    }

    /// Set the visible field.
    #[must_use]
    pub fn with_visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Set the enabled field.
    #[must_use]
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set the child IDs field.
    #[must_use]
    pub fn with_child_ids(mut self, child_ids: Vec<NodeId>) -> Self {
        self.child_ids = child_ids;
        self
    }
}

impl LayoutNode {
    /// Convert this node and its subtree into a snapshot tree.
    ///
    /// Walks the layout node tree recursively, producing a [`SnapshotNode`] that
    /// captures all fields needed for diff comparison. The `child_ids` field of
    /// parent snapshots references their direct children by [`NodeId`].
    ///
    /// # Example
    /// ```
    /// use vb6runtime::layout::model::{LayoutNode, LayoutLeaf, LayoutControlType};
    /// use vb6runtime::layout::model::types::NodeId;
    ///
    /// let leaf = LayoutNode::Leaf(LayoutLeaf {
    ///     name: "Label1".into(),
    ///     control_type: LayoutControlType::Label,
    ///     value: Some("Hello".into()),
    ///     visible: true,
    ///     enabled: true,
    ///     ..Default::default()
    /// });
    /// let snap = leaf.to_snapshot();
    /// assert_eq!(snap.id.name, "Label1");
    /// assert_eq!(snap.value, Some("Hello".into()));
    /// assert!(snap.child_ids.is_empty());
    /// ```
    #[must_use]
    pub fn to_snapshot(&self) -> SnapshotNode {
        match self {
            Self::Leaf(leaf) => {
                SnapshotNode::leaf(leaf.node_id(), leaf.style.clone())
                    .with_value(leaf.value.clone())
                    .with_visible(leaf.visible)
                    .with_enabled(leaf.enabled)
            }
            Self::Container(container) => {
                let child_ids: Vec<NodeId> = container.children.iter().map(|c| c.node_id()).collect();
                SnapshotNode::container(container.node_id(), container.style.clone(), child_ids)
                    .with_visible(container.visible)
                    .with_enabled(container.enabled)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::model::{LayoutContainer, LayoutControlType, LayoutLeaf, LayoutPosition, LayoutSize, LayoutStyle};

    fn test_leaf(name: &str) -> LayoutLeaf {
        LayoutLeaf {
            name: name.into(),
            control_type: LayoutControlType::Label,
            index: 0,
            position: LayoutPosition::default(),
            size: LayoutSize::default(),
            style: LayoutStyle::default(),
            value: Some("Hello".into()),
            visible: true,
            enabled: true,
        }
    }

    fn test_container(name: &str) -> LayoutContainer {
        LayoutContainer {
            name: name.into(),
            control_type: LayoutControlType::Form,
            index: 0,
            position: LayoutPosition::default(),
            size: LayoutSize { width: 400.0, height: 300.0 },
            style: LayoutStyle::default(),
            children: vec![],
            caption: Some("My Form".into()),
            visible: true,
            enabled: true,
            current_value: None,
        }
    }

    #[test]
    fn leaf_to_snapshot() {
        let leaf = LayoutNode::Leaf(test_leaf("lbl1"));
        let snap = leaf.to_snapshot();
        assert_eq!(snap.id.name, "lbl1");
        assert_eq!(snap.id.kind, LayoutControlType::Label);
        assert_eq!(snap.value, Some("Hello".into()));
        assert!(snap.visible);
        assert!(snap.enabled);
        assert!(snap.child_ids.is_empty());
    }

    #[test]
    fn leaf_to_snapshot_invisible() {
        let leaf = LayoutNode::Leaf(LayoutLeaf {
            name: "lblHidden".into(),
            visible: false,
            ..test_leaf("lblHidden")
        });
        let snap = leaf.to_snapshot();
        assert!(!snap.visible);
    }

    #[test]
    fn leaf_to_snapshot_disabled() {
        let leaf = LayoutNode::Leaf(LayoutLeaf {
            name: "cmdDisabled".into(),
            enabled: false,
            ..test_leaf("cmdDisabled")
        });
        let snap = leaf.to_snapshot();
        assert!(!snap.enabled);
    }

    #[test]
    fn container_to_snapshot_with_children() {
        let mut container = test_container("frm1");
        container.children.push(LayoutNode::Leaf(test_leaf("child1")));
        container.children.push(LayoutNode::Leaf(test_leaf("child2")));
        let node = LayoutNode::Container(container);
        let snap = node.to_snapshot();
        assert_eq!(snap.id.name, "frm1");
        assert_eq!(snap.id.kind, LayoutControlType::Form);
        assert_eq!(snap.child_ids.len(), 2);
        assert_eq!(snap.child_ids[0].name, "child1");
        assert_eq!(snap.child_ids[1].name, "child2");
    }

    #[test]
    fn container_to_snapshot_no_children() {
        let container = LayoutNode::Container(test_container("frmEmpty"));
        let snap = container.to_snapshot();
        assert_eq!(snap.id.name, "frmEmpty");
        assert!(snap.child_ids.is_empty());
    }

    #[test]
    fn snapshot_node_creation() {
        let id = NodeId {
            name: "test".into(),
            kind: LayoutControlType::TextBox,
            index: 0,
        };
        let style = LayoutStyle::default();
        let snap = SnapshotNode::new(id.clone(), style.clone());
        assert_eq!(snap.id, id);
        assert!(snap.child_ids.is_empty());
    }

    #[test]
    fn snapshot_builder_fluent() {
        let id = NodeId {
            name: "btn".into(),
            kind: LayoutControlType::CommandButton,
            index: 0,
        };
        let style = LayoutStyle::default();
        let snap = SnapshotNode::leaf(id, style)
            .with_value(Some("Click me".into()))
            .with_visible(true)
            .with_enabled(false);
        assert_eq!(snap.value, Some("Click me".into()));
        assert!(snap.visible);
        assert!(!snap.enabled);
    }

    #[test]
    fn snapshot_container_builder() {
        let id = NodeId {
            name: "fraGroup".into(),
            kind: LayoutControlType::Frame,
            index: 0,
        };
        let style = LayoutStyle::default();
        let children = vec![
            NodeId { name: "lblInside".into(), kind: LayoutControlType::Label, index: 0 },
            NodeId { name: "txtInside".into(), kind: LayoutControlType::TextBox, index: 0 },
        ];
        let snap = SnapshotNode::container(id, style, children.clone());
        assert_eq!(snap.child_ids, children);
    }

    #[test]
    fn snapshot_clone() {
        let leaf = LayoutNode::Leaf(test_leaf("lblClone"));
        let snap1 = leaf.to_snapshot();
        let snap2 = snap1.clone();
        assert_eq!(snap1, snap2);
        assert!(std::ptr::eq(&snap1.id as *const _, &snap1.id as *const _));
    }

    #[test]
    fn snapshot_equality() {
        let leaf1 = LayoutNode::Leaf(test_leaf("lblEq"));
        let leaf2 = LayoutNode::Leaf(test_leaf("lblEq"));
        let snap1 = leaf1.to_snapshot();
        let snap2 = leaf2.to_snapshot();
        assert_eq!(snap1, snap2);
    }

    #[test]
    fn snapshot_inequality_different_values() {
        let leaf1 = LayoutNode::Leaf(LayoutLeaf {
            name: "lblDiff".into(),
            value: Some("A".into()),
            ..test_leaf("lblDiff")
        });
        let leaf2 = LayoutNode::Leaf(LayoutLeaf {
            name: "lblDiff".into(),
            value: Some("B".into()),
            ..test_leaf("lblDiff")
        });
        let snap1 = leaf1.to_snapshot();
        let snap2 = leaf2.to_snapshot();
        assert_ne!(snap1, snap2);
    }

    #[test]
    fn snapshot_inequality_different_visibility() {
        let leaf1 = LayoutNode::Leaf(test_leaf("lblVis"));
        let leaf2 = LayoutNode::Leaf(LayoutLeaf {
            name: "lblVis".into(),
            visible: false,
            ..test_leaf("lblVis")
        });
        let snap1 = leaf1.to_snapshot();
        let snap2 = leaf2.to_snapshot();
        assert_ne!(snap1, snap2);
    }

    #[test]
    fn nested_container_snapshot() {
        let mut inner = test_container("inner");
        inner.children.push(LayoutNode::Leaf(test_leaf("grandchild")));
        let mut outer = test_container("outer");
        outer.children.push(LayoutNode::Container(inner));
        let node = LayoutNode::Container(outer);
        let snap = node.to_snapshot();

        // Outer snapshot has one child (the inner container)
        assert_eq!(snap.child_ids.len(), 1);
        assert_eq!(snap.child_ids[0].name, "inner");
        assert_eq!(snap.child_ids[0].kind, LayoutControlType::Form);
    }

    #[test]
    fn snapshot_preserves_control_type() {
        for (name, control_type) in [
            ("cmd", LayoutControlType::CommandButton),
            ("txt", LayoutControlType::TextBox),
            ("fra", LayoutControlType::Frame),
            ("pic", LayoutControlType::PictureBox),
            ("img", LayoutControlType::Image),
            ("chk", LayoutControlType::CheckBox),
            ("opt", LayoutControlType::OptionButton),
            ("cmb", LayoutControlType::ComboBox),
            ("lst", LayoutControlType::ListBox),
            ("scr", LayoutControlType::HScrollBar),
            ("shp", LayoutControlType::Shape),
            ("ln", LayoutControlType::Line),
        ] {
            let leaf = LayoutNode::Leaf(LayoutLeaf {
                name: name.into(),
                control_type,
                index: 0,
                ..test_leaf(name)
            });
            let snap = leaf.to_snapshot();
            assert_eq!(snap.id.kind, control_type, "control type mismatch for {name}");
        }
    }

    #[test]
    fn snapshot_child_ids_preserve_order() {
        let mut container = test_container("parent");
        container.children = (0..5)
            .map(|i| LayoutNode::Leaf(LayoutLeaf {
                name: format!("child{i}"),
                control_type: LayoutControlType::Label,
                index: i,
                ..test_leaf(&format!("child{i}"))
            }))
            .collect();
        let node = LayoutNode::Container(container);
        let snap = node.to_snapshot();
        for (i, child_id) in snap.child_ids.iter().enumerate() {
            assert_eq!(child_id.name, format!("child{i}"));
            assert_eq!(child_id.index, i as i32);
        }
    }
}
