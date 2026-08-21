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
//!
//! `InputBox` follows the same pattern:
//!
//! - **Windows**: a modal `DialogBoxIndirectParamW` dialog built from an
//!   in-memory template (label, edit box seeded with the default response,
//!   OK/Cancel buttons); honors `xpos`/`ypos`.
//! - **macOS**: `osascript display dialog ... default answer`.
//! - **Linux**: `zenity --entry`.
//! - **wasm32**: the browser's modal `prompt` (title prepended to the
//!   message; browsers cannot position dialogs, so `xpos`/`ypos` are
//!   ignored).
//! - **Anything else / headless**: the request is logged to stderr and the
//!   default response is returned, so programs remain runnable without a
//!   GUI. Cancel always yields `""`, matching VB6.

use crate::error::VBResult;

use super::backend::InteractionBackend;
use super::inputbox::InputBoxRequest;
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

    fn input_box(&self, request: &InputBoxRequest) -> VBResult<String> {
        show_input_dialog(request)
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

/// Show a real input dialog where the platform provides one; log-and-default
/// everywhere else. Never fails: a missing dialog tool degrades to the
/// fallback so an `InputBox` cannot abort an otherwise runnable program.
fn show_input_dialog(request: &InputBoxRequest) -> VBResult<String> {
    #[cfg(target_os = "windows")]
    {
        Ok(windows::input_dialog(request).unwrap_or_else(|| input_fallback(request)))
    }
    #[cfg(target_os = "macos")]
    {
        Ok(macos::input_dialog(request).unwrap_or_else(|| input_fallback(request)))
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        Ok(linux::entry_dialog(request).unwrap_or_else(|| input_fallback(request)))
    }
    #[cfg(target_arch = "wasm32")]
    {
        // Browser prompt is modal and always answers (None = Cancel).
        Ok(wasm::prompt_dialog(request).unwrap_or_default())
    }
    #[cfg(not(any(windows, unix, target_arch = "wasm32")))]
    {
        let _ = request;
        Ok(input_fallback(request))
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

/// Log the request to stderr and answer with its default response.
///
/// Used when no dialog facility exists (headless Linux without zenity,
/// CI machines) so batch runs keep going instead of hanging or failing.
#[cfg_attr(target_arch = "wasm32", allow(dead_code))] // still exercised by tests
fn input_fallback(request: &InputBoxRequest) -> String {
    let title = request.title.as_deref().unwrap_or("InputBox");
    eprintln!(
        "[InputBox] {title}: {} [{}]",
        request.prompt, request.default_response
    );
    request.default_response.clone()
}

#[cfg(target_os = "windows")]
mod windows {
    use std::cell::RefCell;
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        DialogBoxIndirectParamW, EndDialog, GetDialogBaseUnits, GetDlgItemTextW, MessageBoxW,
        SetDlgItemTextW, BS_DEFPUSHBUTTON, BS_PUSHBUTTON, DS_CENTER, DS_MODALFRAME, ES_AUTOHSCROLL,
        IDCANCEL, IDOK, MB_ABORTRETRYIGNORE, MB_DEFBUTTON1, MB_DEFBUTTON2, MB_DEFBUTTON3,
        MB_DEFBUTTON4, MB_HELP, MB_ICONERROR, MB_ICONINFORMATION, MB_ICONQUESTION, MB_ICONWARNING,
        MB_OK, MB_OKCANCEL, MB_RETRYCANCEL, MB_RIGHT, MB_RTLREADING, MB_SETFOREGROUND,
        MB_SYSTEMMODAL, MB_YESNO, MB_YESNOCANCEL, WM_COMMAND, WM_INITDIALOG, WS_BORDER, WS_CAPTION,
        WS_CHILD, WS_GROUP, WS_POPUP, WS_SYSMENU, WS_TABSTOP, WS_VISIBLE,
    };

    use super::super::inputbox::InputBoxRequest;
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

    // ---- InputBox ----

    thread_local! {
        /// Answer collected by the dialog procedure for the current modal
        /// run; `None` means the user cancelled or the dialog failed.
        static INPUT_ANSWER: RefCell<Option<String>> = const { RefCell::new(None) };
    }

    /// Control id of the edit box inside the input dialog template.
    const ID_EDIT: i32 = 1001;

    /// Predefined window-class atoms for dialog items.
    const CLASS_BUTTON: u16 = 0x0080;
    const CLASS_EDIT: u16 = 0x0081;
    const CLASS_STATIC: u16 = 0x0082;

    /// Show a modal input box via `DialogBoxIndirectParamW`.
    ///
    /// The dialog is built from an in-memory template (prompt label, single
    /// line edit seeded with the default response, OK and Cancel buttons).
    /// Returns `None` when the dialog cannot be created, letting the caller
    /// fall back to logging. Enter accepts; Esc and Cancel both yield
    /// `""` at the caller, mirroring VB6.
    pub(super) fn input_dialog(request: &InputBoxRequest) -> Option<String> {
        let template = build_template(request);
        INPUT_ANSWER.with(|slot| *slot.borrow_mut() = None);
        let failed = unsafe {
            DialogBoxIndirectParamW(
                std::ptr::null_mut(), // template references no resources
                template.as_ptr().cast(),
                std::ptr::null_mut(),
                Some(input_dialog_proc),
                request as *const InputBoxRequest as isize,
            ) == -1
        };
        if failed {
            return None;
        }
        INPUT_ANSWER.with(|slot| slot.borrow_mut().take())
    }

    /// Assemble the raw `DLGTEMPLATE` for an input box.
    ///
    /// When the request carries a position, the dialog's origin is placed at
    /// that screen location (twips converted through the system dialog base
    /// units); otherwise the dialog centers on the screen.
    fn build_template(request: &InputBoxRequest) -> Vec<u16> {
        let mut style =
            WS_POPUP | WS_CAPTION | WS_SYSMENU | DS_MODALFRAME as u32 | DS_CENTER as u32;
        let (mut x, mut y) = (0i16, 0i16);
        if let (Some(xpos), Some(ypos)) = (request.xpos, request.ypos) {
            (x, y) = position_in_dialog_units(xpos, ypos);
            style &= !(DS_CENTER as u32);
        }

        let mut b = TemplateBuilder::dialog(style, x, y, 210, 94, request.title.as_deref());
        // Prompt label (style 0 == SS_LEFT), then edit box, then buttons.
        b.item(
            WS_CHILD | WS_VISIBLE | WS_GROUP,
            10,
            8,
            190,
            44,
            0,
            CLASS_STATIC,
            &request.prompt,
        );
        b.item(
            WS_CHILD | WS_VISIBLE | WS_BORDER | WS_TABSTOP | ES_AUTOHSCROLL as u32,
            10,
            56,
            190,
            13,
            ID_EDIT as u16,
            CLASS_EDIT,
            "",
        );
        b.item(
            WS_CHILD | WS_VISIBLE | WS_TABSTOP | BS_DEFPUSHBUTTON as u32,
            92,
            76,
            50,
            14,
            IDOK as u16,
            CLASS_BUTTON,
            "OK",
        );
        b.item(
            WS_CHILD | WS_VISIBLE | WS_TABSTOP | BS_PUSHBUTTON as u32,
            148,
            76,
            50,
            14,
            IDCANCEL as u16,
            CLASS_BUTTON,
            "Cancel",
        );
        b.finish()
    }

    /// Convert a requested twips offset into dialog units at 96 DPI.
    fn position_in_dialog_units(xpos: i32, ypos: i32) -> (i16, i16) {
        // LOWORD: average character width; HIWORD: average character height.
        let base = unsafe { GetDialogBaseUnits() };
        let base_x = (base & 0xFFFF).max(1) as i32;
        let base_y = ((base >> 16) & 0xFFFF).max(1) as i32;
        // 1440 twips per inch at 96 dpi => 15 twips per pixel; pixels =>
        // dialog units by the standard base-unit ratios (4 and 8).
        let dlu_x = (xpos / 15) * 4 / base_x;
        let dlu_y = (ypos / 15) * 8 / base_y;
        (
            dlu_x.clamp(i16::MIN as i32, i16::MAX as i32) as i16,
            dlu_y.clamp(i16::MIN as i32, i16::MAX as i32) as i16,
        )
    }

    /// The modal dialog procedure backing [`input_dialog`].
    ///
    /// The request rides in `dwInitParam`; the accepted answer travels back
    /// out through [`INPUT_ANSWER`] because the modal loop runs on the
    /// calling thread. Cancel/Esc end the dialog without touching it.
    unsafe extern "system" fn input_dialog_proc(
        hwnd: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        match message {
            WM_INITDIALOG => {
                let request = &*(lparam as *const InputBoxRequest);
                let default_text = wide(&request.default_response);
                SetDlgItemTextW(hwnd, ID_EDIT, default_text.as_ptr());
                0 // let the system focus the first tab stop (the edit box)
            }
            WM_COMMAND => match wparam & 0xFFFF {
                id if id == IDOK as usize => {
                    let text = read_edit_text(hwnd);
                    INPUT_ANSWER.with(|slot| *slot.borrow_mut() = Some(text));
                    EndDialog(hwnd, 1);
                    1
                }
                id if id == IDCANCEL as usize => {
                    EndDialog(hwnd, 0);
                    0
                }
                _ => 0,
            },
            _ => 0,
        }
    }

    /// Read the current contents of the edit box, growing until it fits.
    unsafe fn read_edit_text(hwnd: HWND) -> String {
        let mut capacity = 260usize;
        loop {
            let mut buffer = vec![0u16; capacity];
            let copied =
                GetDlgItemTextW(hwnd, ID_EDIT, buffer.as_mut_ptr(), capacity as i32) as usize;
            if copied + 1 < capacity {
                buffer.truncate(copied);
                return String::from_utf16_lossy(&buffer);
            }
            capacity *= 2;
        }
    }

    /// Builder for a NUL-free sequence of words forming a `DLGTEMPLATE`
    /// plus its items, keeping every field DWORD-aligned as required.
    struct TemplateBuilder {
        words: Vec<u16>,
    }

    impl TemplateBuilder {
        /// Start a dialog header with the given geometry and title.
        fn dialog(style: u32, x: i16, y: i16, cx: i16, cy: i16, title: Option<&str>) -> Self {
            let mut b = Self { words: Vec::new() };
            b.dword(style);
            b.dword(0); // dwExtendedStyle
            b.word(0); // cdit, patched by finish()
            b.word(x as u16);
            b.word(y as u16);
            b.word(cx as u16);
            b.word(cy as u16);
            b.word(0); // menu: none
            b.word(0); // class: system dialog class
            b.text(&title.unwrap_or("Input"));
            b
        }

        /// Append one control item.
        #[allow(clippy::too_many_arguments)]
        fn item(
            &mut self,
            style: u32,
            x: i16,
            y: i16,
            cx: i16,
            cy: i16,
            id: u16,
            class_atom: u16,
            text: &str,
        ) {
            self.align_dword();
            self.dword(style);
            self.dword(0); // dwExtendedStyle
            self.word(x as u16);
            self.word(y as u16);
            self.word(cx as u16);
            self.word(cy as u16);
            self.word(id);
            self.word(0xFFFF);
            self.word(class_atom);
            self.text(text);
            self.word(0); // no creation data
        }

        fn word(&mut self, value: u16) {
            self.words.push(value);
        }

        fn dword(&mut self, value: u32) {
            self.words.push(value as u16);
            self.words.push((value >> 16) as u16);
        }

        fn align_dword(&mut self) {
            if self.words.len() % 2 != 0 {
                self.word(0);
            }
        }

        fn text(&mut self, value: &str) {
            self.words.extend(OsStr::new(value).encode_wide());
            self.word(0);
        }

        /// Patch in the item count and terminate on a DWORD boundary.
        fn finish(mut self) -> Vec<u16> {
            self.align_dword();
            self.words[6] = 4; // cdit: prompt, edit, OK, Cancel
            self.words
        }
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

    /// Show an input box via `osascript display dialog`.
    ///
    /// The edit box is seeded with `default answer`; osascript reports the
    /// accepted text back as `text returned:...`. Esc and Cancel exit with a
    /// failure status, which VB6 reads as the empty string. Returns `None`
    /// when osascript is unavailable or cannot run (headless machines),
    /// letting the caller fall back to logging.
    pub(super) fn input_dialog(request: &InputBoxRequest) -> Option<String> {
        let mut script = format!(
            "display dialog \"{}\" default answer \"{}\" \
             buttons {{\"OK\", \"Cancel\"}} default button \"OK\"",
            escape(&request.prompt),
            escape(&request.default_response),
        );
        if let Some(title) = &request.title {
            script.push_str(&format!(" with title \"{}\"", escape(title)));
        }

        let output = Command::new("osascript")
            .arg("-e")
            .arg(&script)
            .output()
            .ok()?;
        if !output.status.success() {
            return Some(String::new());
        }

        // Success output looks like "button returned:OK, text returned:hi".
        // rsplit keeps us honest if the typed text itself contains a
        // ", text returned:" lookalike; strip only the trailing newline.
        let stdout = String::from_utf8_lossy(&output.stdout);
        let (_, value) = stdout.rsplit_once("text returned:")?;
        Some(value.strip_suffix('\n').unwrap_or(value).to_string())
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
mod linux {
    use std::process::Command;

    use super::super::inputbox::InputBoxRequest;
    use super::super::msgbox::{MsgBoxButton, MsgBoxIcon, MsgBoxRequest};

    /// Whether no display server is reachable, so GUI dialogs would either
    /// fail slowly or hang; headless runs (CI, SSH sessions) go straight to
    /// the logging fallback.
    fn headless() -> bool {
        std::env::var_os("DISPLAY").is_none_or(|v| v.is_empty())
            && std::env::var_os("WAYLAND_DISPLAY").is_none_or(|v| v.is_empty())
    }

    /// Show the dialog via `zenity`.
    ///
    /// Zenity has no first-class multi-button message box, so the offered
    /// buttons are mapped onto its ok/cancel/extra-button slots: exit
    /// status 0 means the first (ok-label) button, any other status means
    /// the second (cancel-label) button — mirroring Esc-as-Cancel — and a
    /// pressed extra button prints its label on stdout. Returns `None`
    /// when zenity is not installed or cannot show the dialog.
    pub(super) fn zenity_dialog(request: &MsgBoxRequest) -> Option<MsgBoxButton> {
        if headless() {
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

    /// Show an input box via `zenity --entry`.
    ///
    /// The entry field is seeded with `--entry-text`; zenity prints the
    /// accepted text on stdout, and a non-zero exit (Cancel/Esc) reads as
    /// the empty string, mirroring VB6. Position arguments have no zenity
    /// equivalent and are ignored. Returns `None` when zenity is not
    /// installed or no display server is reachable.
    pub(super) fn entry_dialog(request: &InputBoxRequest) -> Option<String> {
        if headless() {
            return None;
        }

        let mut command = Command::new("zenity");
        command.arg("--entry");
        command.arg("--text").arg(&request.prompt);
        command.arg("--entry-text").arg(&request.default_response);
        if let Some(title) = &request.title {
            command.arg("--title").arg(title);
        }

        let output = command.output().ok()?;
        if !output.status.success() {
            return Some(String::new());
        }
        // Zenity appends exactly one newline to the typed text; keep any
        // trailing spaces the user actually entered.
        let mut text = String::from_utf8_lossy(&output.stdout).into_owned();
        if text.ends_with('\n') {
            text.pop();
        }
        Some(text)
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

    use super::super::inputbox::InputBoxRequest;
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

        /// The browser's modal text-input dialog (`None` = Cancel).
        #[wasm_bindgen(js_namespace = window, js_name = prompt)]
        fn window_prompt(message: &str, default_value: &str) -> Option<String>;
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

    /// Show the input box with the browser's `prompt` primitive.
    ///
    /// Browsers have no title bar, so the title is prepended to the message;
    /// they also cannot position dialogs, so `xpos`/`ypos` are ignored.
    /// Cancel (`null` from `prompt`) reads as the empty string, matching
    /// VB6.
    pub(super) fn prompt_dialog(request: &InputBoxRequest) -> Option<String> {
        let message = browser_message(request.title.as_deref(), &request.prompt);
        window_prompt(&message, &request.default_response)
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
    fn input_fallback_answers_default_response() {
        let request = InputBoxRequest::new("headless?").with_default("42");
        assert_eq!(input_fallback(&request), "42");
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
