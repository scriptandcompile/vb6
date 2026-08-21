//! Message-box request model shared by every interaction backend.
//!
//! `MsgBox` packs five groups of flags into a single numeric `buttons`
//! argument (button set, icon, default button, modality, and miscellany).
//! [`MsgBoxRequest`] decodes and validates that value once — raising VB6
//! error 5 ("Invalid procedure call or argument") for impossible
//! combinations — so backends only ever see a well-formed request.

use crate::error::{err_number, VBError, VBResult};

// ---------------------------------------------------------------------------
// Buttons
// ---------------------------------------------------------------------------

/// A button that can appear in a message box.
///
/// The discriminant values are the VB6 `VbMsgBoxResult` constants that
/// `MsgBox` returns (`vbOK` = 1 ... `vbNo` = 7).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MsgBoxButton {
    /// The OK button was clicked (`vbOK`, 1).
    Ok,
    /// The Cancel button was clicked (`vbCancel`, 2).
    Cancel,
    /// The Abort button was clicked (`vbAbort`, 3).
    Abort,
    /// The Retry button was clicked (`vbRetry`, 4).
    Retry,
    /// The Ignore button was clicked (`vbIgnore`, 5).
    Ignore,
    /// The Yes button was clicked (`vbYes`, 6).
    Yes,
    /// The No button was clicked (`vbNo`, 7).
    No,
}

impl MsgBoxButton {
    /// Every button, in `VbMsgBoxResult` order.
    pub const ALL: [MsgBoxButton; 7] = [
        MsgBoxButton::Ok,
        MsgBoxButton::Cancel,
        MsgBoxButton::Abort,
        MsgBoxButton::Retry,
        MsgBoxButton::Ignore,
        MsgBoxButton::Yes,
        MsgBoxButton::No,
    ];

    /// The `VbMsgBoxResult` integer this button maps to.
    pub fn id(self) -> i16 {
        match self {
            MsgBoxButton::Ok => 1,
            MsgBoxButton::Cancel => 2,
            MsgBoxButton::Abort => 3,
            MsgBoxButton::Retry => 4,
            MsgBoxButton::Ignore => 5,
            MsgBoxButton::Yes => 6,
            MsgBoxButton::No => 7,
        }
    }

    /// The caption displayed on the button's face.
    pub fn name(self) -> &'static str {
        match self {
            MsgBoxButton::Ok => "OK",
            MsgBoxButton::Cancel => "Cancel",
            MsgBoxButton::Abort => "Abort",
            MsgBoxButton::Retry => "Retry",
            MsgBoxButton::Ignore => "Ignore",
            MsgBoxButton::Yes => "Yes",
            MsgBoxButton::No => "No",
        }
    }

    /// Look a button up by its `VbMsgBoxResult` value.
    pub fn from_id(id: i16) -> Option<Self> {
        Self::ALL.into_iter().find(|button| button.id() == id)
    }

    /// Look a button up by caption, case-insensitively.
    ///
    /// Hosts that script responses by label (the memory backend) use this.
    pub fn from_name(name: &str) -> Option<Self> {
        let name = name.trim();
        Self::ALL
            .into_iter()
            .find(|b| b.name().eq_ignore_ascii_case(name))
    }
}

/// Which set of buttons the dialog displays (first group of the `buttons`
/// argument, bits `0x0F`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MsgBoxButtonSet {
    /// OK only (`vbOKOnly`, 0) — the default.
    OkOnly,
    /// OK and Cancel (`vbOKCancel`, 1).
    OkCancel,
    /// Abort, Retry, and Ignore (`vbAbortRetryIgnore`, 2).
    AbortRetryIgnore,
    /// Yes, No, and Cancel (`vbYesNoCancel`, 3).
    YesNoCancel,
    /// Yes and No (`vbYesNo`, 4).
    YesNo,
    /// Retry and Cancel (`vbRetryCancel`, 5).
    RetryCancel,
}

impl MsgBoxButtonSet {
    /// Decode the first group of the raw `buttons` value.
    fn from_raw(raw: u32) -> VBResult<Self> {
        let set = match raw & 0x0F {
            0x0 => Self::OkOnly,
            0x1 => Self::OkCancel,
            0x2 => Self::AbortRetryIgnore,
            0x3 => Self::YesNoCancel,
            0x4 => Self::YesNo,
            0x5 => Self::RetryCancel,
            other => return Err(invalid_buttons(raw, other)),
        };
        Ok(set)
    }

    /// The buttons this set displays, left to right.
    pub fn buttons(self) -> &'static [MsgBoxButton] {
        match self {
            Self::OkOnly => &[MsgBoxButton::Ok],
            Self::OkCancel => &[MsgBoxButton::Ok, MsgBoxButton::Cancel],
            Self::AbortRetryIgnore => &[
                MsgBoxButton::Abort,
                MsgBoxButton::Retry,
                MsgBoxButton::Ignore,
            ],
            Self::YesNoCancel => &[MsgBoxButton::Yes, MsgBoxButton::No, MsgBoxButton::Cancel],
            Self::YesNo => &[MsgBoxButton::Yes, MsgBoxButton::No],
            Self::RetryCancel => &[MsgBoxButton::Retry, MsgBoxButton::Cancel],
        }
    }
}

/// Which icon the dialog displays (second group, bits `0xF0`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MsgBoxIcon {
    /// No icon (`0`) — the default.
    None,
    /// Critical Message / red X (`vbCritical`, 16).
    Critical,
    /// Warning Query / question mark (`vbQuestion`, 32).
    Question,
    /// Warning Message / exclamation point (`vbExclamation`, 48).
    Exclamation,
    /// Information Message / lowercase i (`vbInformation`, 64).
    Information,
}

impl MsgBoxIcon {
    /// Decode the second group of the raw `buttons` value.
    fn from_raw(raw: u32) -> VBResult<Self> {
        let icon = match raw & 0xF0 {
            0x00 => Self::None,
            0x10 => Self::Critical,
            0x20 => Self::Question,
            0x30 => Self::Exclamation,
            0x40 => Self::Information,
            other => return Err(invalid_buttons(raw, other)),
        };
        Ok(icon)
    }
}

/// Dialog modality (fourth group, bits `0x3000`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MsgBoxModality {
    /// Application modal (`vbApplicationModal`, 0) — the default.
    Application,
    /// System modal (`vbSystemModal`, 4096).
    System,
}

impl MsgBoxModality {
    /// Decode the fourth group of the raw `buttons` value.
    fn from_raw(raw: u32) -> VBResult<Self> {
        match raw & 0x3000 {
            0x0000 => Ok(Self::Application),
            0x1000 => Ok(Self::System),
            other => Err(invalid_buttons(raw, other)),
        }
    }
}

// ---------------------------------------------------------------------------
// Request
// ---------------------------------------------------------------------------

/// A fully decoded, validated `MsgBox` invocation.
///
/// Backends receive this instead of the raw flag soup: everything is already
/// checked, so a backend only has to render it and report which button the
/// user chose.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MsgBoxRequest {
    /// The message text.
    pub prompt: String,
    /// Title-bar text; `None` means "use the application name".
    pub title: Option<String>,
    /// Which buttons are displayed.
    pub button_set: MsgBoxButtonSet,
    /// Which icon is displayed.
    pub icon: MsgBoxIcon,
    /// 1-based index (into [`offered_buttons`](Self::offered_buttons)) of the
    /// button activated by Enter.
    pub default_button: usize,
    /// Dialog modality.
    pub modality: MsgBoxModality,
    /// Whether a Help button is appended (`vbMsgBoxHelpButton`).
    pub help_button: bool,
    /// Whether the dialog takes the foreground (`vbMsgBoxSetForeground`).
    pub set_foreground: bool,
    /// Whether text is right-aligned (`vbMsgBoxRight`).
    pub right_aligned: bool,
    /// Whether text reads right-to-left (`vbMsgBoxRtlReading`).
    pub rtl_reading: bool,
    /// The original `buttons` value, preserved for hosts that care about
    /// details the parsed model does not model.
    pub raw_buttons: u32,
}

impl MsgBoxRequest {
    /// Decode and validate a `buttons` value into a request for `prompt`.
    ///
    /// Mirrors VB6 validation: any group holding a value outside the
    /// documented constants raises error 5, as does a default-button index
    /// past the last displayed button.
    pub fn parse(prompt: impl Into<String>, buttons: i64) -> VBResult<Self> {
        if !(0..=u32::MAX as i64).contains(&buttons) {
            return Err(VBError::with_description(
                err_number::INVALID_PROCEDURE_CALL,
                format!(
                    "Invalid procedure call or argument: buttons value {buttons} is out of range"
                ),
            ));
        }
        let raw = buttons as u32;
        let button_set = MsgBoxButtonSet::from_raw(raw)?;
        let icon = MsgBoxIcon::from_raw(raw)?;
        let modality = MsgBoxModality::from_raw(raw)?;

        let offered = button_set.buttons();
        let default_button = ((raw & 0x0F00) >> 8) as usize + 1;
        if default_button > offered.len() {
            return Err(VBError::with_description(
                err_number::INVALID_PROCEDURE_CALL,
                format!(
                    "Invalid procedure call or argument: buttons value {raw} selects default \
                     button {default_button}, but the dialog only shows {} button(s)",
                    offered.len()
                ),
            ));
        }

        Ok(Self {
            prompt: prompt.into(),
            title: None,
            button_set,
            icon,
            default_button,
            modality,
            help_button: raw & 0x4000 != 0,
            set_foreground: raw & 0x1_0000 != 0,
            right_aligned: raw & 0x8_0000 != 0,
            rtl_reading: raw & 0x10_0000 != 0,
            raw_buttons: raw,
        })
    }

    /// Set the title-bar text (`None` restores the application-name default).
    pub fn with_title(mut self, title: Option<String>) -> Self {
        self.title = title;
        self
    }

    /// The buttons the dialog offers, left to right.
    pub fn offered_buttons(&self) -> &'static [MsgBoxButton] {
        self.button_set.buttons()
    }

    /// The button activated by pressing Enter.
    pub fn default_button_value(&self) -> MsgBoxButton {
        self.offered_buttons()[self.default_button - 1]
    }
}

/// Build the error 5 raised for an unusable `buttons` value.
fn invalid_buttons(requested: u32, offending: u32) -> VBError {
    VBError::with_description(
        err_number::INVALID_PROCEDURE_CALL,
        format!(
            "Invalid procedure call or argument: buttons value {requested} has invalid \
             component {offending}"
        ),
    )
}

/// One recorded `MsgBox` invocation — what the dialog showed, kept so hosts
/// and tests can assert on the requests a program made.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MsgBoxRecord {
    /// The message text shown.
    pub prompt: String,
    /// The title-bar text shown (`None` = application name).
    pub title: Option<String>,
    /// The buttons offered, left to right.
    pub offered_buttons: Vec<MsgBoxButton>,
    /// The button Enter activates.
    pub default_button: MsgBoxButton,
}

impl MsgBoxRecord {
    /// Capture the display-relevant parts of `request`.
    pub fn of(request: &MsgBoxRequest) -> Self {
        Self {
            prompt: request.prompt.clone(),
            title: request.title.clone(),
            offered_buttons: request.offered_buttons().to_vec(),
            default_button: request.default_button_value(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn button_ids_match_vbmsgboxresult() {
        assert_eq!(MsgBoxButton::from_id(1), Some(MsgBoxButton::Ok));
        assert_eq!(MsgBoxButton::from_id(7), Some(MsgBoxButton::No));
        assert_eq!(MsgBoxButton::from_id(8), None);
        assert_eq!(MsgBoxButton::Yes.id(), 6);
    }

    #[test]
    fn button_names_round_trip_case_insensitively() {
        for button in MsgBoxButton::ALL {
            assert_eq!(MsgBoxButton::from_name(button.name()), Some(button));
        }
        assert_eq!(
            MsgBoxButton::from_name("cancel"),
            Some(MsgBoxButton::Cancel)
        );
        assert_eq!(MsgBoxButton::from_name("nope"), None);
    }

    #[test]
    fn zero_is_the_plain_ok_dialog() {
        let request = MsgBoxRequest::parse("hi", 0).unwrap();
        assert_eq!(request.button_set, MsgBoxButtonSet::OkOnly);
        assert_eq!(request.icon, MsgBoxIcon::None);
        assert_eq!(request.default_button, 1);
        assert_eq!(request.modality, MsgBoxModality::Application);
        assert!(!request.help_button);
        assert_eq!(request.offered_buttons(), &[MsgBoxButton::Ok]);
    }

    #[test]
    fn combined_flags_decode() {
        // vbYesNo + vbQuestion + vbDefaultButton2 + vbSystemModal
        let request = MsgBoxRequest::parse("save?", 4 + 32 + 256 + 4096).unwrap();
        assert_eq!(request.button_set, MsgBoxButtonSet::YesNo);
        assert_eq!(request.icon, MsgBoxIcon::Question);
        assert_eq!(request.default_button, 2);
        assert_eq!(request.modality, MsgBoxModality::System);
        assert_eq!(
            request.offered_buttons(),
            &[MsgBoxButton::Yes, MsgBoxButton::No]
        );
        assert_eq!(request.default_button_value(), MsgBoxButton::No);
    }

    #[test]
    fn misc_flags_decode() {
        let raw: i64 = 16384 | 65536 | 524288 | 1048576; // Help|Foreground|Right|RtlReading
        let request = MsgBoxRequest::parse("x", raw).unwrap();
        assert!(request.help_button);
        assert!(request.set_foreground);
        assert!(request.right_aligned);
        assert!(request.rtl_reading);
        assert_eq!(request.raw_buttons, raw as u32);
    }

    #[test]
    fn unknown_button_set_is_error_5() {
        let err = MsgBoxRequest::parse("x", 6).unwrap_err();
        assert_eq!(err.number, err_number::INVALID_PROCEDURE_CALL);
        assert!(err.description.contains('6'));
    }

    #[test]
    fn unknown_icon_is_error_5() {
        // Icon nibble 0x80 is not one of 0/16/32/48/64.
        assert!(MsgBoxRequest::parse("x", 0x80).is_err());
    }

    #[test]
    fn unknown_modality_is_error_5() {
        assert!(MsgBoxRequest::parse("x", 0x2000).is_err());
    }

    #[test]
    fn default_button_past_last_displayed_is_error_5() {
        // vbOKOnly + vbDefaultButton2: no second button to make default.
        let err = MsgBoxRequest::parse("x", 256).unwrap_err();
        assert_eq!(err.number, err_number::INVALID_PROCEDURE_CALL);
        // vbDefaultButton3 on a Yes/No/Cancel dialog is fine.
        assert!(MsgBoxRequest::parse("x", 3 + 512).is_ok());
    }

    #[test]
    fn negative_and_oversized_values_are_error_5() {
        assert!(MsgBoxRequest::parse("x", -1).is_err());
        assert!(MsgBoxRequest::parse("x", (u32::MAX as i64) + 1).is_err());
    }

    #[test]
    fn record_captures_display_relevant_parts() {
        let request = MsgBoxRequest::parse("Save changes?", 3 + 32)
            .unwrap()
            .with_title(Some("App".into()));
        let record = MsgBoxRecord::of(&request);
        assert_eq!(record.prompt, "Save changes?");
        assert_eq!(record.title.as_deref(), Some("App"));
        assert_eq!(
            record.offered_buttons,
            vec![MsgBoxButton::Yes, MsgBoxButton::No, MsgBoxButton::Cancel]
        );
        assert_eq!(record.default_button, MsgBoxButton::Yes);
    }
}
