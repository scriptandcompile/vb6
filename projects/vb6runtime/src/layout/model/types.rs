//! Core types for the layout model.
//!
//! This module defines the fundamental types used throughout the layout engine:
//! - [`LayoutControlType`] — maps to [`vb6parse::language::ControlKind`] variants
//! - [`LayoutPosition`] — pixel coordinates within a parent container
//! - [`LayoutSize`] — dimensions in pixels
//! - [`NodeId`] — unique identifier combining name, kind, and index

/// Type of a VB6 control in the layout model.
///
/// Maps directly to `ControlKind` variants from vb6parse but simplified
/// for layout computation purposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LayoutControlType {
    /// A Form control.
    Form,
    /// An MDI Form control.
    MDIForm,
    /// A Label control.
    Label,
    /// A TextBox control.
    TextBox,
    /// A CommandButton control.
    CommandButton,
    /// A Frame control.
    Frame,
    /// A PictureBox control.
    PictureBox,
    /// An Image control.
    Image,
    /// A CheckBox control.
    CheckBox,
    /// An OptionButton (radio button) control.
    OptionButton,
    /// A ComboBox control.
    ComboBox,
    /// A ListBox control.
    ListBox,
    /// A horizontal ScrollBar control.
    HScrollBar,
    /// A vertical ScrollBar control.
    VScrollBar,
    /// A Timer control (no visual output).
    Timer,
    /// A Shape control (Rectangle, Oval, etc.).
    Shape,
    /// A Line control (x1,y1,x2,y2).
    Line,
    /// A DriveListBox control.
    DriveListBox,
    /// A DirListBox control.
    DirListBox,
    /// A FileListBox control.
    FileListBox,
    /// A Data control (no visual output).
    Data,
    /// A Custom/OLE control (third-party or UserControl).
    Custom,
}

impl LayoutControlType {
    /// Returns a string name suitable for CSS class generation.
    #[must_use]
    pub fn css_class(&self) -> &'static str {
        match self {
            Self::Form => "form",
            Self::MDIForm => "mdiform",
            Self::Label => "label",
            Self::TextBox => "textbox",
            Self::CommandButton => "commandbutton",
            Self::Frame => "frame",
            Self::PictureBox => "picturebox",
            Self::Image => "image",
            Self::CheckBox => "checkbox",
            Self::OptionButton => "optionbutton",
            Self::ComboBox => "combobox",
            Self::ListBox => "listbox",
            Self::HScrollBar => "hscrollbar",
            Self::VScrollBar => "vscrollbar",
            Self::Timer => "timer",
            Self::Shape => "shape",
            Self::Line => "line",
            Self::DriveListBox => "drivelistbox",
            Self::DirListBox => "dirlistbox",
            Self::FileListBox => "filelistbox",
            Self::Data => "data",
            Self::Custom => "custom",
        }
    }
}

/// Position of a control within its parent container, in pixels.
///
/// Values are relative to the parent's client area origin.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct LayoutPosition {
    /// Distance from the left edge of the parent's client area.
    pub left: f32,
    /// Distance from the top edge of the parent's client area.
    pub top: f32,
}

/// Size of a control in pixels.
///
/// Width and height are always non-negative.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct LayoutSize {
    /// Width in pixels.
    pub width: f32,
    /// Height in pixels.
    pub height: f32,
}

/// Unique identifier for a layout node.
///
/// Combines the control's name, type, and index (for indexed control arrays)
/// to form a unique key. Useful for lookup and debugging.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeId {
    /// The name of the control.
    pub name: String,
    /// The type of control.
    pub kind: LayoutControlType,
    /// The index in a control array (0 for non-indexed controls).
    pub index: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_id_equality() {
        let a = NodeId {
            name: "cmdOK".into(),
            kind: LayoutControlType::CommandButton,
            index: 0,
        };
        let b = NodeId {
            name: "cmdOK".into(),
            kind: LayoutControlType::CommandButton,
            index: 0,
        };
        let c = NodeId {
            name: "cmdOK".into(),
            kind: LayoutControlType::CommandButton,
            index: 1,
        };
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn position_default() {
        let p = LayoutPosition::default();
        assert_eq!(p.left, 0.0);
        assert_eq!(p.top, 0.0);
    }

    #[test]
    fn size_default() {
        let s = LayoutSize::default();
        assert_eq!(s.width, 0.0);
        assert_eq!(s.height, 0.0);
    }

    #[test]
    fn layout_control_type_variants() {
        let _ = LayoutControlType::Form;
        let _ = LayoutControlType::MDIForm;
        let _ = LayoutControlType::Label;
        let _ = LayoutControlType::TextBox;
        let _ = LayoutControlType::CommandButton;
        let _ = LayoutControlType::Frame;
        let _ = LayoutControlType::PictureBox;
        let _ = LayoutControlType::Image;
        let _ = LayoutControlType::CheckBox;
        let _ = LayoutControlType::OptionButton;
        let _ = LayoutControlType::ComboBox;
        let _ = LayoutControlType::ListBox;
        let _ = LayoutControlType::HScrollBar;
        let _ = LayoutControlType::VScrollBar;
        let _ = LayoutControlType::Timer;
        let _ = LayoutControlType::Shape;
        let _ = LayoutControlType::Line;
        let _ = LayoutControlType::DriveListBox;
        let _ = LayoutControlType::DirListBox;
        let _ = LayoutControlType::FileListBox;
        let _ = LayoutControlType::Data;
        let _ = LayoutControlType::Custom;
    }

    #[test]
    fn css_class_names() {
        assert_eq!(LayoutControlType::Form.css_class(), "form");
        assert_eq!(LayoutControlType::MDIForm.css_class(), "mdiform");
        assert_eq!(LayoutControlType::Label.css_class(), "label");
        assert_eq!(LayoutControlType::TextBox.css_class(), "textbox");
        assert_eq!(LayoutControlType::CommandButton.css_class(), "commandbutton");
        assert_eq!(LayoutControlType::Frame.css_class(), "frame");
        assert_eq!(LayoutControlType::PictureBox.css_class(), "picturebox");
        assert_eq!(LayoutControlType::Image.css_class(), "image");
        assert_eq!(LayoutControlType::CheckBox.css_class(), "checkbox");
        assert_eq!(LayoutControlType::OptionButton.css_class(), "optionbutton");
        assert_eq!(LayoutControlType::ComboBox.css_class(), "combobox");
        assert_eq!(LayoutControlType::ListBox.css_class(), "listbox");
        assert_eq!(LayoutControlType::HScrollBar.css_class(), "hscrollbar");
        assert_eq!(LayoutControlType::VScrollBar.css_class(), "vscrollbar");
        assert_eq!(LayoutControlType::Timer.css_class(), "timer");
        assert_eq!(LayoutControlType::Shape.css_class(), "shape");
        assert_eq!(LayoutControlType::Line.css_class(), "line");
        assert_eq!(LayoutControlType::DriveListBox.css_class(), "drivelistbox");
        assert_eq!(LayoutControlType::DirListBox.css_class(), "dirlistbox");
        assert_eq!(LayoutControlType::FileListBox.css_class(), "filelistbox");
        assert_eq!(LayoutControlType::Data.css_class(), "data");
        assert_eq!(LayoutControlType::Custom.css_class(), "custom");
    }
}
