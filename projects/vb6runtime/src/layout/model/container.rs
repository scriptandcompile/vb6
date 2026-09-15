//! Container node hierarchy: `LayoutContainer`, `LayoutLeaf`, `LayoutNode`.

use std::fmt;

use super::style::LayoutStyle;
use super::types::{LayoutControlType, LayoutPosition, LayoutSize, NodeId};

impl Default for LayoutLeaf {
    fn default() -> Self {
        Self {
            name: String::new(),
            control_type: LayoutControlType::Label,
            index: 0,
            position: LayoutPosition::default(),
            size: LayoutSize::default(),
            style: LayoutStyle::default(),
            value: None,
            visible: true,
            enabled: true,
        }
    }
}

impl Default for LayoutContainer {
    fn default() -> Self {
        Self {
            name: String::new(),
            control_type: LayoutControlType::Form,
            index: 0,
            position: LayoutPosition::default(),
            size: LayoutSize::default(),
            style: LayoutStyle::default(),
            children: vec![],
            caption: None,
            visible: true,
            enabled: true,
            current_value: None,
        }
    }
}

/// A node in the layout tree — either a container with children or a leaf control.
#[derive(Debug, Clone, PartialEq)]
pub enum LayoutNode {
    /// A container control that can have child controls.
    Container(LayoutContainer),
    /// A leaf control with no children.
    Leaf(LayoutLeaf),
}

impl LayoutNode {
    /// Returns the node ID for this node.
    #[must_use]
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::Container(c) => c.node_id(),
            Self::Leaf(l) => l.node_id(),
        }
    }

    /// Returns `true` if this node is visible.
    #[must_use]
    pub fn visible(&self) -> bool {
        match self {
            Self::Container(c) => c.visible,
            Self::Leaf(l) => l.visible,
        }
    }

    /// Returns `true` if this node is enabled.
    #[must_use]
    pub fn enabled(&self) -> bool {
        match self {
            Self::Container(c) => c.enabled,
            Self::Leaf(l) => l.enabled,
        }
    }

    /// Returns an iterator over the visible children of this node (if it is a container).
    pub fn visible_children(&self) -> Box<dyn Iterator<Item = &LayoutNode> + '_> {
        match self {
            Self::Container(c) => Box::new(c.children.iter().filter(|child| child.visible())),
            Self::Leaf(_) => Box::new(std::iter::empty()),
        }
    }

    /// Returns a reference to the visible children of this node (if it is a container).
    #[must_use]
    pub fn children(&self) -> &[LayoutNode] {
        match self {
            Self::Container(c) => &c.children,
            Self::Leaf(_) => &[],
        }
    }
}

/// A container control that can have child controls (Form, Frame, PictureBox).
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutContainer {
    /// The name of the control.
    pub name: String,
    /// The type of control.
    pub control_type: LayoutControlType,
    /// The index in a control array (0 for non-indexed controls).
    pub index: i32,
    /// Position within the parent container, in pixels.
    pub position: LayoutPosition,
    /// Size of the control, in pixels.
    pub size: LayoutSize,
    /// Computed CSS-compatible style properties.
    pub style: LayoutStyle,
    /// Child controls.
    pub children: Vec<LayoutNode>,
    /// Caption / title text (may be None for controls without captions).
    pub caption: Option<String>,
    /// Whether the control is visible.
    pub visible: bool,
    /// Whether the control is enabled.
    pub enabled: bool,
    /// Runtime-writable current value (e.g. form caption).
    pub current_value: Option<String>,
}

impl LayoutContainer {
    /// Returns the node ID for this container.
    #[must_use]
    pub fn node_id(&self) -> NodeId {
        NodeId {
            name: self.name.clone(),
            kind: self.control_type,
            index: self.index,
        }
    }

    /// Returns an iterator over the visible children.
    pub fn visible_children(&self) -> impl Iterator<Item = &LayoutNode> {
        self.children.iter().filter(|child| child.visible())
    }
}

/// A leaf control with no children (Label, Button, TextBox, etc.).
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutLeaf {
    /// The name of the control.
    pub name: String,
    /// The type of control.
    pub control_type: LayoutControlType,
    /// The index in a control array (0 for non-indexed controls).
    pub index: i32,
    /// Position within the parent container, in pixels.
    pub position: LayoutPosition,
    /// Size of the control, in pixels.
    pub size: LayoutSize,
    /// Computed CSS-compatible style properties.
    pub style: LayoutStyle,
    /// Runtime-writable value (text, caption, checked state, etc.).
    pub value: Option<String>,
    /// Whether the control is visible.
    pub visible: bool,
    /// Whether the control is enabled.
    pub enabled: bool,
}

impl LayoutLeaf {
    /// Returns the node ID for this leaf.
    #[must_use]
    pub fn node_id(&self) -> NodeId {
        NodeId {
            name: self.name.clone(),
            kind: self.control_type,
            index: self.index,
        }
    }
}

// Implement Display so the container node can be debugged nicely.
impl fmt::Display for LayoutContainer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}[{}] ({}x{} @ {}/{} {} children)",
            self.name,
            self.index,
            self.size.width,
            self.size.height,
            self.position.left,
            self.position.top,
            self.children.len()
        )
    }
}

impl fmt::Display for LayoutLeaf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}[{}] ({}x{} @ {}/{})",
            self.name,
            self.index,
            self.size.width,
            self.size.height,
            self.position.left,
            self.position.top
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_leaf(name: &str) -> LayoutLeaf {
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

    fn default_container(name: &str) -> LayoutContainer {
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
    fn leaf_has_no_children() {
        let leaf = LayoutLeaf {
            name: "Label1".into(),
            control_type: LayoutControlType::Label,
            index: 0,
            position: LayoutPosition::default(),
            size: LayoutSize::default(),
            style: LayoutStyle::default(),
            value: Some("Hello".into()),
            visible: true,
            enabled: true,
        };
        assert!(matches!(LayoutNode::Leaf(leaf), LayoutNode::Leaf(_)));
    }

    #[test]
    fn container_with_children() {
        let container = LayoutContainer {
            name: "Form1".into(),
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
        };
        assert_eq!(container.children.len(), 0);
    }

    #[test]
    fn visible_children_filters_invisible() {
        let child = LayoutNode::Leaf(LayoutLeaf {
            visible: false,
            ..default_leaf("lblHidden")
        });
        let container = LayoutContainer {
            children: vec![
                LayoutNode::Leaf(default_leaf("lblVisible")),
                child,
            ],
            ..default_container("frmParent")
        };
        let visible: Vec<_> = container.visible_children().collect();
        assert_eq!(visible.len(), 1);
    }

    #[test]
    fn leaf_visible_children_empty() {
        let leaf = LayoutNode::Leaf(default_leaf("lblOnly"));
        let visible: Vec<_> = leaf.visible_children().collect();
        assert!(visible.is_empty());
    }

    #[test]
    fn node_id_from_leaf() {
        let leaf = LayoutNode::Leaf(LayoutLeaf {
            name: "cmdOK".into(),
            control_type: LayoutControlType::CommandButton,
            index: 1,
            ..LayoutLeaf::default()
        });
        let id = leaf.node_id();
        assert_eq!(id.name, "cmdOK");
        assert_eq!(id.kind, LayoutControlType::CommandButton);
        assert_eq!(id.index, 1);
    }

    #[test]
    fn node_id_from_container() {
        let container = LayoutNode::Container(LayoutContainer {
            name: "Frame1".into(),
            control_type: LayoutControlType::Frame,
            index: 0,
            ..LayoutContainer::default()
        });
        let id = container.node_id();
        assert_eq!(id.name, "Frame1");
        assert_eq!(id.kind, LayoutControlType::Frame);
    }
}
