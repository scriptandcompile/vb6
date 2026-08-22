//! Shell request model shared by every interaction backend.
//!
//! `Shell` takes the program to run plus an optional window style
//! (`VbAppWinStyle`). [`ShellRequest`] validates that style once — raising
//! VB6 error 5 ("Invalid procedure call or argument") for values outside
//! the six documented constants — so backends only ever see a well-formed
//! request.

use crate::error::{err_number, VBError, VBResult};

// ---------------------------------------------------------------------------
// Window styles
// ---------------------------------------------------------------------------

/// The window style a shelled program starts in (`VbAppWinStyle`).
///
/// The discriminant values are the VB6 constants (`vbHide` = 0 ...
/// `vbMinimizedNoFocus` = 6); note that 5 is not defined by VB6.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WindowStyle {
    /// Window is hidden and focus is passed to the hidden window
    /// (`vbHide`, 0). This is the default when the argument is omitted.
    Hide,
    /// Window has focus and is restored to its original size and position
    /// (`vbNormalFocus`, 1).
    NormalFocus,
    /// Window is displayed as an icon with focus (`vbMinimizedFocus`, 2).
    MinimizedFocus,
    /// Window is maximized with focus (`vbMaximizedFocus`, 3).
    MaximizedFocus,
    /// Window is restored to its most recent size and position; the
    /// currently active window remains active (`vbNormalNoFocus`, 4).
    NormalNoFocus,
    /// Window is displayed as an icon; the currently active window remains
    /// active (`vbMinimizedNoFocus`, 6).
    MinimizedNoFocus,
}

impl WindowStyle {
    /// Every style, in `VbAppWinStyle` order.
    pub const ALL: [WindowStyle; 6] = [
        WindowStyle::Hide,
        WindowStyle::NormalFocus,
        WindowStyle::MinimizedFocus,
        WindowStyle::MaximizedFocus,
        WindowStyle::NormalNoFocus,
        WindowStyle::MinimizedNoFocus,
    ];

    /// The `VbAppWinStyle` integer this style maps to.
    pub fn id(self) -> i64 {
        match self {
            WindowStyle::Hide => 0,
            WindowStyle::NormalFocus => 1,
            WindowStyle::MinimizedFocus => 2,
            WindowStyle::MaximizedFocus => 3,
            WindowStyle::NormalNoFocus => 4,
            WindowStyle::MinimizedNoFocus => 6,
        }
    }

    /// The VB6 constant name for this style, e.g. `"vbNormalFocus"`.
    pub fn name(self) -> &'static str {
        match self {
            WindowStyle::Hide => "vbHide",
            WindowStyle::NormalFocus => "vbNormalFocus",
            WindowStyle::MinimizedFocus => "vbMinimizedFocus",
            WindowStyle::MaximizedFocus => "vbMaximizedFocus",
            WindowStyle::NormalNoFocus => "vbNormalNoFocus",
            WindowStyle::MinimizedNoFocus => "vbMinimizedNoFocus",
        }
    }

    /// Look a style up by its `VbAppWinStyle` value.
    ///
    /// Returns `None` for anything outside the six documented constants
    /// (including 5, which VB6 does not define).
    pub fn from_id(id: i64) -> Option<Self> {
        Self::ALL.into_iter().find(|style| style.id() == id)
    }

    /// Look a style up by its constant name, case-insensitively.
    ///
    /// Hosts that script responses by name (the memory backend) use this;
    /// the `vb` prefix is optional.
    pub fn from_name(name: &str) -> Option<Self> {
        let name = name.trim();
        Self::ALL.into_iter().find(|s| {
            let full = s.name();
            full.eq_ignore_ascii_case(name)
                || full.trim_start_matches("vb").eq_ignore_ascii_case(name)
        })
    }
}

impl Default for WindowStyle {
    /// VB6 starts a program minimized with focus when `windowstyle` is
    /// omitted.
    fn default() -> Self {
        WindowStyle::MinimizedFocus
    }
}

// ---------------------------------------------------------------------------
// Requests and records
// ---------------------------------------------------------------------------

/// A validated `Shell` request: what to run and how its window should look.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellRequest {
    /// The program to execute, with any command-line arguments. May include
    /// a directory path; names containing spaces are expected to be quoted
    /// by the caller, exactly as in VB6.
    pub pathname: String,
    /// The window style the program should start in.
    pub window_style: WindowStyle,
}

impl ShellRequest {
    /// Build a request for `pathname` with VB6's omitted-argument default
    /// (`vbMinimizedFocus`).
    pub fn new(pathname: impl Into<String>) -> Self {
        Self {
            pathname: pathname.into(),
            window_style: WindowStyle::default(),
        }
    }

    /// Validate the raw `windowstyle` argument (error 5 when it is not one
    /// of the six documented `VbAppWinStyle` values) and build a request.
    pub fn parse(pathname: impl Into<String>, window_style: i64) -> VBResult<Self> {
        let Some(style) = WindowStyle::from_id(window_style) else {
            return Err(invalid_window_style(window_style));
        };
        Ok(Self {
            pathname: pathname.into(),
            window_style: style,
        })
    }

    /// Override the window style.
    pub fn with_window_style(mut self, window_style: WindowStyle) -> Self {
        self.window_style = window_style;
        self
    }
}

/// A recorded `Shell` request, as captured by the memory backend.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellRecord {
    /// The requested command line.
    pub pathname: String,
    /// The requested window style.
    pub window_style: WindowStyle,
}

impl ShellRecord {
    /// Capture a request.
    pub fn of(request: &ShellRequest) -> Self {
        Self {
            pathname: request.pathname.clone(),
            window_style: request.window_style,
        }
    }
}

/// Build the error 5 raised when `windowstyle` is not a `VbAppWinStyle`
/// value, listing the valid ones.
fn invalid_window_style(raw: i64) -> VBError {
    VBError::with_description(
        err_number::INVALID_PROCEDURE_CALL,
        format!(
            "Invalid procedure call or argument: {} is not a valid window style \
             (expected one of {})",
            raw,
            WindowStyle::ALL
                .iter()
                .map(|s| s.name())
                .collect::<Vec<_>>()
                .join(", "),
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_match_vb6_constants() {
        assert_eq!(WindowStyle::Hide.id(), 0);
        assert_eq!(WindowStyle::NormalFocus.id(), 1);
        assert_eq!(WindowStyle::MinimizedFocus.id(), 2);
        assert_eq!(WindowStyle::MaximizedFocus.id(), 3);
        assert_eq!(WindowStyle::NormalNoFocus.id(), 4);
        assert_eq!(WindowStyle::MinimizedNoFocus.id(), 6);
    }

    #[test]
    fn from_id_round_trips_every_style() {
        for style in WindowStyle::ALL {
            assert_eq!(WindowStyle::from_id(style.id()), Some(style));
        }
    }

    #[test]
    fn five_is_not_a_defined_style() {
        assert_eq!(WindowStyle::from_id(5), None);
        assert_eq!(WindowStyle::from_id(-1), None);
        assert_eq!(WindowStyle::from_id(7), None);
    }

    #[test]
    fn from_name_matches_constant_names_case_insensitively() {
        assert_eq!(
            WindowStyle::from_name("vbNormalFocus"),
            Some(WindowStyle::NormalFocus)
        );
        assert_eq!(WindowStyle::from_name("VBHIDE"), Some(WindowStyle::Hide));
        assert_eq!(
            WindowStyle::from_name("minimizednofocus"),
            Some(WindowStyle::MinimizedNoFocus)
        );
        assert_eq!(WindowStyle::from_name("nonsense"), None);
    }

    #[test]
    fn new_defaults_to_minimized_with_focus() {
        // VB6's documented default for an omitted windowstyle argument.
        let request = ShellRequest::new("notepad.exe");
        assert_eq!(request.window_style, WindowStyle::MinimizedFocus);
    }

    #[test]
    fn parse_accepts_documented_styles() {
        let request = ShellRequest::parse("calc.exe", 3).unwrap();
        assert_eq!(request.pathname, "calc.exe");
        assert_eq!(request.window_style, WindowStyle::MaximizedFocus);
    }

    #[test]
    fn parse_rejects_undefined_styles_with_error_5() {
        let err = ShellRequest::parse("calc.exe", 5).unwrap_err();
        assert_eq!(err.number, err_number::INVALID_PROCEDURE_CALL);
        assert!(
            err.description.contains("vbMinimizedNoFocus"),
            "{}",
            err.description
        );
    }

    #[test]
    fn with_window_style_overrides_the_default() {
        let request = ShellRequest::new("backup.bat").with_window_style(WindowStyle::Hide);
        assert_eq!(request.window_style, WindowStyle::Hide);
    }

    #[test]
    fn record_captures_the_request() {
        let request = ShellRequest::new(r#""C:\Program Files\App.exe" /flag"#)
            .with_window_style(WindowStyle::NormalNoFocus);
        let record = ShellRecord::of(&request);
        assert_eq!(record.pathname, r#""C:\Program Files\App.exe" /flag"#);
        assert_eq!(record.window_style, WindowStyle::NormalNoFocus);
    }
}
