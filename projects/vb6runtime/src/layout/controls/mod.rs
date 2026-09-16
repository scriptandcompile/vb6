//! Per-control style builder dispatch.
//!
//! Exports `build_style_for_control` which delegates to individual control modules
//! based on the control kind. Each control module implements a `build_*_style` function
//! that consumes the control's properties and produces a [`LayoutStyle`].
//!
//! # Modules
//! - [`button`] — CommandButton style builder
//! - [`label`] — Label style builder
//! - [`textbox`] — TextBox style builder
//! - [`frame`] — Frame style builder
//! - [`picturebox`] — PictureBox style builder
//! - [`image`] — Image style builder
//! - [`checkbox`] — CheckBox / OptionButton style builder
//! - [`combobox`] — ComboBox style builder
//! - [`listbox`] — ListBox style builder
//! - [`scrollbar`] — HScrollBar / VScrollBar style builder
//! - [`shape`] — Shape style builder
//! - [`line`] — Line style builder
//! - [`timer`] — Timer style builder (returns empty style)

pub mod button;
pub mod checkbox;
pub mod combobox;
pub mod frame;
pub mod image;
pub mod label;
pub mod line;
pub mod listbox;
pub mod picturebox;
pub mod scrollbar;
pub mod shape;
pub mod textbox;
pub mod timer;

use vb6parse::language::ControlKind;

use super::LayoutConfig;
use super::model::style::LayoutStyle;

/// Build CSS style properties for a control based on its [`ControlKind`].
///
/// Delegates to individual control modules. Timer and Data controls return
/// default (empty) style since they have no visual output. Custom/Ole controls
/// should be rejected before this function is called.
pub fn build_style_for_control(kind: &ControlKind, config: &LayoutConfig) -> LayoutStyle {
    match kind {
        ControlKind::CommandButton { properties, .. } => {
            button::build_button_style(properties, config)
        }
        ControlKind::Label { properties, .. } => label::build_label_style(properties, config),
        ControlKind::TextBox { properties, .. } => textbox::build_textbox_style(properties, config),
        ControlKind::Frame { properties, .. } => frame::build_frame_style(properties, config),
        ControlKind::PictureBox { properties, .. } => {
            picturebox::build_picturebox_style(properties, config)
        }
        ControlKind::Image { properties, .. } => image::build_image_style(properties, config),
        ControlKind::CheckBox { properties, .. } => {
            checkbox::build_checkbox_style(properties, config)
        }
        ControlKind::OptionButton { properties, .. } => {
            checkbox::build_optionbutton_style(properties, config)
        }
        ControlKind::ComboBox { properties, .. } => {
            combobox::build_combobox_style(properties, config)
        }
        ControlKind::ListBox { properties, .. } => listbox::build_listbox_style(properties, config),
        ControlKind::HScrollBar { properties, .. } | ControlKind::VScrollBar { properties, .. } => {
            scrollbar::build_scrollbar_style(properties, config)
        }
        ControlKind::Shape { properties, .. } => shape::build_shape_style(properties, config),
        ControlKind::Line { properties, .. } => line::build_line_style(properties, config),
        ControlKind::Timer { .. }
        | ControlKind::Data { .. }
        | ControlKind::DriveListBox { .. }
        | ControlKind::DirListBox { .. }
        | ControlKind::FileListBox { .. } => LayoutStyle::default(),
        ControlKind::Ole { .. } | ControlKind::Custom { .. } | ControlKind::Menu { .. } => {
            unreachable!("Custom/Ole/Menu controls must be rejected before this point")
        }
    }
}
