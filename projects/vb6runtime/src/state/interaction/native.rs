//! Native implementation of user interaction operations.
//!
//! Uses real process environment and OS primitives. Suitable for
//! Windows, Linux, macOS, and (when a host installs it) wasm32.
//!
//! `MsgBox` rendering is platform-specific:
//!
//! - **Windows**: the Win32 `MessageBoxW` API (exact VB6 parity).
//! - **macOS**: `osascript` driving `display dialog`.
//! - **Linux**: `zenity`, when installed.
//! - **wasm32**: the browser's modal `alert`/`confirm` dialogs.
//! - **Anything else / headless**: the request is logged to stderr and the
//!   default button is returned, so programs remain runnable without a GUI.

use crate::error::VBResult;

use super::backend::InteractionBackend;
use super::msgbox::{MsgBoxButton, MsgBoxRequest};

/// Native interaction backend using real OS facilities.
pub struct NativeBackend;

impl NativeBackend {
    /// Create a new native backend.
    pub fn new() -> Self {
        Self
    }
}

impl Default for NativeBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl InteractionBackend for NativeBackend {
    fn command_args(&self) -> Vec<String> {
        std::env::args().skip(1).collect()
    }

    fn do_events(&self) -> i16 {
        std::thread::yield_now();
        0
    }

    fn beep(&self) {
        // Write the terminal bell character to stderr — works on all
        // major terminal emulators across Windows, Linux, and macOS.
        use std::io::Write;
        let _ = std::io::stderr().write_all(b"\x07");
    }

    fn stop(&self) {
        // No interactive debugger is attached for native batch runs; the
        // interpreter falls back to the compiled-`.exe` behavior
        // (`Stop` acts like `End`).
    }

    fn msg_box(&self, request: &MsgBoxRequest) -> VBResult<MsgBoxButton> {
        show_dialog(request)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Show a real dialog where the platform provides one; log-and-default
/// everywhere else. Never fails: a missing dialog tool degrades to the
/// fallback so a `MsgBox` cannot abort an otherwise runnable program.
fn show_dialog(request: &MsgBoxRequest) -> VBResult<MsgBoxButton> {
    #[cfg(target_os = "windows")]
    {
        // Win32 MessageBoxW always answers with a VbMsgBoxResult value.
        Ok(windows::message_box(request))
    }
    #[cfg(target_os = "macos")]
    {
        Ok(macos::display_dialog(request).unwrap_or_else(|| fallback(request)))
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        Ok(linux::zenity_dialog(request).unwrap_or_else(|| fallback(request)))
    }
    #[cfg(target_arch = "wasm32")]
    {
        // Browser alert/confirm are modal and always answer, so this
        // cannot fail either.
        Ok(wasm::display_dialog(request))
    }
    #[cfg(not(any(windows, unix, target_arch = "wasm32")))]
    {
        let _ = request;
        Ok(fallback(request))
    }
}

/// Log the request to stderr and answer with its default button.
///
/// Used when no dialog facility exists (headless Linux without zenity,
/// CI machines) so batch runs keep going instead of hanging or failing.
#[cfg_attr(target_arch = "wasm32", allow(dead_code))] // still exercised by tests
fn fallback(request: &MsgBoxRequest) -> MsgBoxButton {
    let title = request.title.as_deref().unwrap_or("MsgBox");
    let buttons = request
        .offered_buttons()
        .iter()
        .map(|b| b.name())
        .collect::<Vec<_>>()
        .join("|");
    eprintln!("[MsgBox] {title}: {} [{buttons}]", request.prompt);
    request.default_button_value()
}

#[cfg(target_os = "windows")]
mod windows {
    use std::os::windows::ffi::OsStrExt;

    use windows_sys::Win32::UI::WindowsAndMessaging::{
        MessageBoxW, MB_ABORTRETRYIGNORE, MB_DEFBUTTON1, MB_DEFBUTTON2, MB_DEFBUTTON3,
        MB_DEFBUTTON4, MB_HELP, MB_ICONERROR, MB_ICONINFORMATION, MB_ICONQUESTION, MB_ICONWARNING,
        MB_OK, MB_OKCANCEL, MB_RETRYCANCEL, MB_RIGHT, MB_RTLREADING, MB_SETFOREGROUND,
        MB_SYSTEMMODAL, MB_YESNO, MB_YESNOCANCEL,
    };

    use super::super::msgbox::{
        MsgBoxButton, MsgBoxButtonSet, MsgBoxIcon, MsgBoxModality, MsgBoxRequest,
    };

    /// Encode a Rust string as a NUL-terminated UTF-16 buffer.
    fn wide(s: &str) -> Vec<u16> {
        use std::iter::once;
        std::ffi::OsStr::new(s)
            .encode_wide()
            .chain(once(0))
            .collect()
    }

    /// Show the dialog with `MessageBoxW` and map the result back.
    ///
    /// If the call fails (returns 0) or the user picks Help, the request's
    /// default button is returned so callers always get a valid answer.
    pub(super) fn message_box(request: &MsgBoxRequest) -> MsgBoxButton {
        let text = wide(&request.prompt);
        let caption = wide(request.title.as_deref().unwrap_or(""));

        let mut flags = match request.button_set {
            MsgBoxButtonSet::OkOnly => MB_OK,
            MsgBoxButtonSet::OkCancel => MB_OKCANCEL,
            MsgBoxButtonSet::AbortRetryIgnore => MB_ABORTRETRYIGNORE,
            MsgBoxButtonSet::YesNoCancel => MB_YESNOCANCEL,
            MsgBoxButtonSet::YesNo => MB_YESNO,
            MsgBoxButtonSet::RetryCancel => MB_RETRYCANCEL,
        } | match request.icon {
            MsgBoxIcon::None => 0,
            MsgBoxIcon::Critical => MB_ICONERROR,
            MsgBoxIcon::Question => MB_ICONQUESTION,
            MsgBoxIcon::Exclamation => MB_ICONWARNING,
            MsgBoxIcon::Information => MB_ICONINFORMATION,
        } | match request.default_button {
            1 => MB_DEFBUTTON1,
            2 => MB_DEFBUTTON2,
            3 => MB_DEFBUTTON3,
            _ => MB_DEFBUTTON4,
        };
        flags |= match request.modality {
            MsgBoxModality::Application => 0,
            MsgBoxModality::System => MB_SYSTEMMODAL,
        };
        if request.help_button {
            flags |= MB_HELP;
        }
        if request.set_foreground {
            flags |= MB_SETFOREGROUND;
        }
        if request.right_aligned {
            flags |= MB_RIGHT;
        }
        if request.rtl_reading {
            flags |= MB_RTLREADING;
        }

        let hwnd: *mut core::ffi::c_void = std::ptr::null_mut();
        let result = unsafe { MessageBoxW(hwnd, text.as_ptr(), caption.as_ptr(), flags) };

        MsgBoxButton::from_id(result as i16).unwrap_or_else(|| request.default_button_value())
    }
}

#[cfg(target_os = "macos")]
mod macos {
    use std::process::Command;

    use super::super::msgbox::{MsgBoxButton, MsgBoxIcon, MsgBoxRequest};

    /// Escape a string for embedding in a double-quoted AppleScript literal.
    fn escape(s: &str) -> String {
        s.replace('\\', "\\\\").replace('"', "\\\"")
    }

    /// Show the dialog via `osascript display dialog`.
    ///
    /// Returns `None` when osascript is unavailable or fails (headless
    /// machines), letting the caller fall back to logging.
    pub(super) fn display_dialog(request: &MsgBoxRequest) -> Option<MsgBoxButton> {
        let offered = request.offered_buttons();
        let labels = offered
            .iter()
            .map(|b| format!("\"{}\"", escape(b.name())))
            .collect::<Vec<_>>()
            .join(", ");
        let default_label = request.default_button_value().name();

        let mut script = format!(
            "display dialog \"{}\" buttons {{{labels}}} default button \"{}\"",
            escape(&request.prompt),
            escape(default_label),
        );
        if let Some(title) = &request.title {
            script.push_str(&format!(" with title \"{}\"", escape(title)));
        }
        script.push_str(match request.icon {
            MsgBoxIcon::Critical => " with icon stop",
            MsgBoxIcon::Question | MsgBoxIcon::Exclamation => " with icon caution",
            MsgBoxIcon::Information => " with icon note",
            MsgBoxIcon::None => "",
        });

        let output = Command::new("osascript")
            .arg("-e")
            .arg(&script)
            .output()
            .ok()?;
        if !output.status.success() {
            // Esc maps to Cancel when that button exists, mirroring VB6.
            if offered.contains(&MsgBoxButton::Cancel) {
                return Some(MsgBoxButton::Cancel);
            }
            return None;
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        stdout
            .trim()
            .strip_prefix("button returned:")
            .and_then(MsgBoxButton::from_name)
            .or(Some(request.default_button_value()))
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
mod linux {
    use std::process::Command;

    use super::super::msgbox::{MsgBoxButton, MsgBoxIcon, MsgBoxRequest};

    /// Show the dialog via `zenity`.
    ///
    /// Zenity has no first-class multi-button message box, so the offered
    /// buttons are mapped onto its ok/cancel/extra-button slots: exit
    /// status 0 means the first (ok-label) button, any other status means
    /// the second (cancel-label) button — mirroring Esc-as-Cancel — and a
    /// pressed extra button prints its label on stdout. Returns `None`
    /// when zenity is not installed or cannot show the dialog.
    pub(super) fn zenity_dialog(request: &MsgBoxRequest) -> Option<MsgBoxButton> {
        // Without a display server zenity would either fail slowly or hang;
        // headless runs (CI, SSH sessions) go straight to the fallback.
        if std::env::var_os("DISPLAY").is_none_or(|v| v.is_empty())
            && std::env::var_os("WAYLAND_DISPLAY").is_none_or(|v| v.is_empty())
        {
            return None;
        }

        let offered = request.offered_buttons();

        let mut command = Command::new("zenity");
        command.arg(match request.icon {
            MsgBoxIcon::Critical => "--error",
            MsgBoxIcon::Question => "--question",
            MsgBoxIcon::Exclamation => "--warning",
            MsgBoxIcon::Information | MsgBoxIcon::None => "--info",
        });
        command.arg("--no-wrap");
        command.arg("--text").arg(&request.prompt);
        if let Some(title) = &request.title {
            command.arg("--title").arg(title);
        }

        let first = offered[0];
        let second = offered.get(1).copied();
        let third = offered.get(2).copied();
        command.arg("--ok-label").arg(first.name());
        if let Some(second) = second {
            command.arg("--cancel-label").arg(second.name());
        }
        if let Some(third) = third {
            command.arg("--extra-button").arg(third.name());
        }

        let output = command.output().ok()?;

        if let Some(third) = third {
            let label = String::from_utf8_lossy(&output.stdout);
            if label.trim().eq_ignore_ascii_case(third.name()) {
                return Some(third);
            }
        }
        if output.status.success() {
            Some(first)
        } else {
            second
        }
    }
}

/// Compose the message for a one-button (`alert`) or first-step
/// (`confirm`) browser dialog: browsers have no title bar, so the title is
/// prepended to the prompt.
#[cfg_attr(
    not(target_arch = "wasm32"),
    allow(dead_code) // exercised by tests; consumed by the wasm32 backend
)]
fn browser_message(title: Option<&str>, prompt: &str) -> String {
    match title {
        Some(title) => format!("{title}\n\n{prompt}"),
        None => prompt.to_string(),
    }
}

/// Compose the message for a three-button dialog's second `confirm` step.
///
/// Browsers only offer OK/Cancel, so a tri-state VB6 dialog (Yes/No/Cancel,
/// Abort/Retry/Ignore) becomes two confirms; this step's text spells out
/// which choice maps to which button so the flow stays understandable.
#[cfg_attr(
    not(target_arch = "wasm32"),
    allow(dead_code) // exercised by tests; consumed by the wasm32 backend
)]
fn browser_secondary_message(
    title: Option<&str>,
    prompt: &str,
    second: &str,
    third: &str,
) -> String {
    format!(
        "{}\n\n(OK = {second}, Cancel = {third})",
        browser_message(title, prompt)
    )
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use wasm_bindgen::prelude::*;

    use super::super::msgbox::{MsgBoxButton, MsgBoxRequest};
    use super::{browser_message, browser_secondary_message};

    #[wasm_bindgen]
    extern "C" {
        /// The browser's modal message dialog (OK button only).
        #[wasm_bindgen(js_namespace = window)]
        fn alert(message: &str);

        /// The browser's modal OK/Cancel question dialog.
        #[wasm_bindgen(js_namespace = window, js_name = confirm)]
        fn window_confirm(message: &str) -> bool;
    }

    /// Show the dialog with browser primitives and map the answer back.
    ///
    /// One offered button maps to `alert`; two map onto `confirm`'s
    /// OK/Cancel; three run a chained pair of confirms (first button vs.
    /// the rest, then second vs. third). Icons, default-button placement,
    /// and modality have no browser equivalent and are ignored.
    pub(super) fn display_dialog(request: &MsgBoxRequest) -> MsgBoxButton {
        let title = request.title.as_deref();
        let prompt = request.prompt.as_str();
        let offered = request.offered_buttons();

        match offered {
            [only] => {
                alert(&browser_message(title, prompt));
                *only
            }
            [first, second] => {
                if window_confirm(&browser_message(title, prompt)) {
                    *first
                } else {
                    *second
                }
            }
            [first, second, third] => {
                if window_confirm(&browser_message(title, prompt)) {
                    *first
                } else if window_confirm(&browser_secondary_message(
                    title,
                    prompt,
                    second.name(),
                    third.name(),
                )) {
                    *second
                } else {
                    *third
                }
            }
            _ => request.default_button_value(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_args_skips_program_name() {
        // We can't control std::env::args() in a unit test, but we can
        // verify the method doesn't panic and returns a Vec.
        let backend = NativeBackend::new();
        let _args = backend.command_args();
    }

    #[test]
    fn do_events_returns_zero() {
        let backend = NativeBackend::new();
        assert_eq!(backend.do_events(), 0);
    }

    #[test]
    fn fallback_answers_default_button() {
        let request = MsgBoxRequest::parse("headless?", 4 + 32 + 256).unwrap();
        assert_eq!(fallback(&request), MsgBoxButton::No);
    }

    #[test]
    fn browser_message_prepends_the_title() {
        assert_eq!(browser_message(None, "hi"), "hi");
        assert_eq!(browser_message(Some("App"), "hi"), "App\n\nhi");
    }

    #[test]
    fn browser_secondary_message_labels_both_choices() {
        let message = browser_secondary_message(Some("App"), "Overwrite?", "No", "Cancel");
        assert!(message.contains("App\n\nOverwrite?"));
        assert!(message.contains("(OK = No, Cancel = Cancel)"));
    }
}
