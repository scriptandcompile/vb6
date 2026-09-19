//! Container node hierarchy: `LayoutContainer`, `LayoutLeaf`, `LayoutNode`.

use std::fmt;

use super::style::LayoutStyle;
use super::types::{LayoutControlType, LayoutPosition, LayoutSize, NodeId};

/// Links a layout node to the VB6 procedure that handles its events.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EventProcedure {
    /// The node ID of the control in the layout tree.
    pub node_id: NodeId,
    /// The VB6 control name (e.g. "cmdOK").
    pub control: String,
    /// The VB6 event name (e.g. "Click").
    pub event: String,
    /// The full procedure name (e.g. "cmdOK_Click").
    pub procedure: String,
}

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
            tooltip: None,
            tabindex: None,
            is_default: false,
            is_cancel: false,
            is_locked: false,
            max_length: None,
            password_char: None,
            hide_selection: false,
            scroll_bars: None,
            stretch: false,
            range_min: None,
            range_max: None,
            range_step: None,
            combo_style: None,
            image_src: None,
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

/// Top-level form container that wraps the root node of a VB6 form.
///
/// This struct holds form-level metadata (name, position, size, style) alongside
/// a `root_node` that contains all child controls. Runtime-writable state
/// (caption, visible, enabled, current_value) is stored here.
///
/// Diffing support: [`snapshot`] captures the form state after each render for
/// incremental diff-based updates; [`render_id`] increments on each render call.
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutForm {
    /// The name of the form (e.g. "Form1").
    pub name: String,
    /// The type of control (always `Form` for this struct).
    pub control_type: LayoutControlType,
    /// The index in a control array (0 for non-indexed forms).
    pub index: i32,
    /// Position of the form, in pixels.
    pub position: LayoutPosition,
    /// Size of the form, in pixels.
    pub size: LayoutSize,
    /// Computed CSS-compatible style properties.
    pub style: LayoutStyle,
    /// The root node containing all child controls.
    pub root_node: LayoutNode,
    /// Form caption / title text.
    pub caption: String,
    /// Whether the form is visible.
    pub visible: bool,
    /// Whether the form is enabled.
    pub enabled: bool,
    /// Runtime-writable current value (e.g. form caption).
    pub current_value: Option<String>,
    /// Snapshot of the form state captured after the last render, used for
    /// incremental diff-based rendering. `None` before the first render.
    pub snapshot: Option<super::super::snapshot::SnapshotNode>,
    /// Monotonically increasing render counter. Increments on each call to
    /// [`capture_snapshot`][super::super::capture_snapshot].
    pub render_id: u64,
    /// Event procedures for controls on this form.
    pub event_procedures: Vec<EventProcedure>,
}

impl Default for LayoutForm {
    fn default() -> Self {
        Self {
            name: String::new(),
            control_type: LayoutControlType::Form,
            index: 0,
            position: LayoutPosition::default(),
            size: LayoutSize::default(),
            style: LayoutStyle::default(),
            root_node: LayoutNode::Container(LayoutContainer::default()),
            caption: String::new(),
            visible: true,
            enabled: true,
            current_value: None,
            snapshot: None,
            render_id: 0,
            event_procedures: vec![],
        }
    }
}

impl LayoutForm {
    /// Returns the node ID for this form.
    #[must_use]
    pub fn node_id(&self) -> NodeId {
        NodeId {
            name: self.name.clone(),
            kind: self.control_type,
            index: self.index,
        }
    }

    /// Returns an iterator over the visible children of the root node.
    pub fn visible_children(&self) -> impl Iterator<Item = &LayoutNode> {
        self.root_node.visible_children()
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

    /// Returns a reference to the control type of this node.
    #[must_use]
    pub fn control_type(&self) -> &LayoutControlType {
        match self {
            Self::Container(c) => &c.control_type,
            Self::Leaf(l) => &l.control_type,
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
    /// HTML title attribute text (tooltip).
    pub tooltip: Option<String>,
    /// HTML tabindex attribute value.
    pub tabindex: Option<i32>,
    /// Whether this is the default button (autofocus).
    pub is_default: bool,
    /// Whether this is the cancel button (Escape key handler).
    pub is_cancel: bool,
    /// Whether the control is read-only (like HTML `readonly`).
    pub is_locked: bool,
    /// Maximum input length (HTML `maxlength`).
    pub max_length: Option<i32>,
    /// Password character for password fields (HTML `type="password"`).
    pub password_char: Option<char>,
    /// Whether selection is hidden when control loses focus.
    pub hide_selection: bool,
    /// Scroll bar configuration (overflow-x / overflow-y).
    pub scroll_bars: Option<String>,
    /// Whether the Image control stretches its picture to fill bounds.
    pub stretch: bool,
    /// Min value for ScrollBar controls (HTML `min`).
    pub range_min: Option<i32>,
    /// Max value for ScrollBar controls (HTML `max`).
    pub range_max: Option<i32>,
    /// Step value for ScrollBar controls (HTML `step`).
    pub range_step: Option<i32>,
    /// ComboBox style: "dropdown" (editable), "dropdown-readonly" (non-editable), "simple" (always-visible list).
    pub combo_style: Option<String>,

    /// Base64-encoded image data URL for Image controls (e.g. "data:image/png;base64,...").
    pub image_src: Option<String>,
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
            ..Default::default()
        }
    }

    fn default_container(name: &str) -> LayoutContainer {
        LayoutContainer {
            name: name.into(),
            control_type: LayoutControlType::Form,
            index: 0,
            position: LayoutPosition::default(),
            size: LayoutSize {
                width: 400.0,
                height: 300.0,
            },
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
            ..Default::default()
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
            size: LayoutSize {
                width: 400.0,
                height: 300.0,
            },
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
            children: vec![LayoutNode::Leaf(default_leaf("lblVisible")), child],
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

    #[test]
    fn layout_form_default() {
        let form = LayoutForm::default();
        assert_eq!(form.name, "");
        assert_eq!(form.control_type, LayoutControlType::Form);
        assert_eq!(form.index, 0);
        assert!(form.visible);
        assert!(form.enabled);
        assert!(form.root_node.visible());
    }

    #[test]
    fn layout_form_node_id() {
        let form = LayoutForm {
            name: "Form1".into(),
            index: 2,
            ..LayoutForm::default()
        };
        let id = form.node_id();
        assert_eq!(id.name, "Form1");
        assert_eq!(id.kind, LayoutControlType::Form);
        assert_eq!(id.index, 2);
    }

    #[test]
    fn layout_form_visible_children() {
        let child = LayoutNode::Leaf(LayoutLeaf {
            name: "cmdOK".into(),
            control_type: LayoutControlType::CommandButton,
            value: Some("OK".into()),
            visible: true,
            ..LayoutLeaf::default()
        });
        let hidden = LayoutNode::Leaf(LayoutLeaf {
            name: "lblHidden".into(),
            control_type: LayoutControlType::Label,
            visible: false,
            ..LayoutLeaf::default()
        });
        let form = LayoutForm {
            name: "Form1".into(),
            root_node: LayoutNode::Container(LayoutContainer {
                children: vec![child, hidden],
                ..LayoutContainer::default()
            }),
            ..LayoutForm::default()
        };
        let visible: Vec<_> = form.visible_children().collect();
        assert_eq!(visible.len(), 1);
        assert_eq!(visible[0].node_id().name, "cmdOK");
    }

    #[test]
    fn layout_form_has_root_container() {
        let form = LayoutForm::default();
        match &form.root_node {
            LayoutNode::Container(c) => {
                assert_eq!(c.children.len(), 0);
                assert!(c.visible);
            }
            LayoutNode::Leaf(_) => panic!("root_node should be a Container"),
        }
    }

    #[test]
    fn layout_form_snapshot_default_none() {
        let form = LayoutForm::default();
        assert!(form.snapshot.is_none());
    }

    #[test]
    fn layout_form_render_id_default_zero() {
        let form = LayoutForm::default();
        assert_eq!(form.render_id, 0);
    }

    #[test]
    fn layout_form_snapshot_set_and_clone() {
        let mut form = LayoutForm {
            name: "Form1".into(),
            caption: "Form1".into(),
            ..LayoutForm::default()
        };
        // Update the root container name so the snapshot captures it
        if let LayoutNode::Container(ref mut rc) = form.root_node {
            rc.name = "Form1".into();
        }
        let snap = form.root_node.to_snapshot();
        form.snapshot = Some(snap);
        assert!(form.snapshot.is_some());
        assert_eq!(form.snapshot.as_ref().unwrap().id.name, "Form1");

        let cloned = form.clone();
        assert!(cloned.snapshot.is_some());
        assert_eq!(cloned.snapshot.unwrap().id.name, "Form1");
    }

    #[test]
    fn layout_form_default_has_empty_event_procedures() {
        let form = LayoutForm::default();
        assert!(form.event_procedures.is_empty());
    }

    #[test]
    fn layout_form_clone_preserves_empty_event_procedures() {
        let form = LayoutForm {
            name: "Form1".into(),
            event_procedures: vec![EventProcedure {
                node_id: NodeId {
                    name: String::new(),
                    kind: LayoutControlType::Form,
                    index: 0,
                },
                control: "cmdOK".into(),
                event: "Click".into(),
                procedure: "cmdOK_Click".into(),
            }],
            ..LayoutForm::default()
        };
        let cloned = form.clone();
        assert_eq!(cloned.event_procedures.len(), 1);
        assert_eq!(cloned.event_procedures[0].control, "cmdOK");
    }
}
