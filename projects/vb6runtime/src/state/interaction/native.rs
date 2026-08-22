//! Native implementation of user interaction operations.
//!
//! Uses real process environment and OS primitives. Suitable for
//! Windows, Linux, macOS, and wasm32 (where it is the default backend).
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
//!
//! `AppActivate` follows it too:
//!
//! - **Windows**: `EnumWindows` matching captions by prefix (then suffix,
//!   per VB6 rules; numeric titles additionally try Shell task IDs via
//!   process id), raised with `SetForegroundWindow`.
//! - **macOS**: `osascript` driving System Events (`frontmost`, window
//!   raise).
//! - **Linux (X11)**: `wmctrl -l -a`.
//! - **Anything else / headless**: the request is logged to stderr and the
//!   call succeeds, so programs remain runnable without a GUI.
//!
//! On every platform that *does* have a working window facility but no
//! matching window, the statement raises VB6 error 5 ("Invalid procedure
//! call or argument"), exactly as real VB6 does.
//!
//! `Shell` follows the same philosophy:
//!
//! - **Windows**: `CreateProcessW` passing the command line through intact,
//!   with the requested window style applied via `STARTF_USESHOWWINDOW`
//!   (`SW_HIDE` ... `SW_SHOWMINNOACTIVE`) — exact VB6 parity. The returned
//!   task ID is the new process id, which is also what `AppActivate`'s
//!   numeric form matches against.
//! - **Linux/macOS**: the command line is split into program + arguments
//!   (double quotes honored), then spawned detached in its own process
//!   group with its standard streams disconnected; the window style has no
//!   equivalent and is ignored.
//! - **wasm32**: browsers cannot start processes, so the request is logged
//!   to stderr and a synthetic task ID is returned so programs remain
//!   runnable.
//!
//! A spawn failure raises VB6 error 53 ("File not found") when the program
//! does not exist and error 70 ("Permission denied") when it cannot be
//! executed, matching VB6's trappable errors.

use crate::error::{err_number, VBError, VBResult};

use super::appactivate::AppActivateRequest;
use super::backend::InteractionBackend;
use super::inputbox::InputBoxRequest;
use super::msgbox::{MsgBoxButton, MsgBoxRequest};
use super::shell::ShellRequest;

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

    fn app_activate(&self, request: &AppActivateRequest) -> VBResult<()> {
        activate_window(request)
    }

    fn shell(&self, request: &ShellRequest) -> VBResult<f64> {
        launch(request).map_err(|err| {
            // The OS error alone ("No such file or directory (os error 2)")
            // never names the program; real VB6's error 53 is understood to
            // be about the pathname that was passed, so include it.
            let mapped = VBError::from(err);
            VBError::with_description(
                mapped.number,
                format!("\"{}\": {}", request.pathname, mapped.description),
            )
        })
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

/// Activate a matching window where the platform provides one; log-and-
/// succeed everywhere else. Only a platform with a working window facility
/// that cannot find the window raises VB6 error 5 — a headless machine has
/// no windows to find, so failing every call would abort otherwise runnable
/// programs.
fn activate_window(request: &AppActivateRequest) -> VBResult<()> {
    #[cfg(target_os = "windows")]
    {
        if windows::activate_window(request) {
            Ok(())
        } else {
            Err(no_such_window(&request.title))
        }
    }
    #[cfg(target_os = "macos")]
    {
        match macos::activate_window(request) {
            Some(true) => Ok(()),
            Some(false) => Err(no_such_window(&request.title)),
            None => {
                activate_fallback(request);
                Ok(())
            }
        }
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        match linux::activate_window(request) {
            Some(true) => Ok(()),
            Some(false) => Err(no_such_window(&request.title)),
            None => {
                activate_fallback(request);
                Ok(())
            }
        }
    }
    #[cfg(not(any(unix, windows)))]
    {
        // Browsers (wasm32) and exotic targets expose no OS windows.
        let _ = request;
        activate_fallback(request);
        Ok(())
    }
}

/// Build the error 5 raised when a platform with real windows has none
/// matching `title`.
#[cfg_attr(
    not(any(windows, unix)),
    allow(dead_code) // exercised by tests; consumed by the platform backends
)]
fn no_such_window(title: &str) -> VBError {
    VBError::with_description(
        err_number::INVALID_PROCEDURE_CALL,
        format!(
            "Invalid procedure call or argument: AppActivate found no window titled \
             \"{title}\""
        ),
    )
}

/// Log the request to stderr and report success.
///
/// Used when no window facility exists (headless machines, wasm32) so
/// batch runs keep going instead of failing.
#[cfg_attr(target_arch = "wasm32", allow(dead_code))] // still exercised by tests
fn activate_fallback(request: &AppActivateRequest) {
    if request.wait {
        eprintln!("[AppActivate] {} [wait]", request.title);
    } else {
        eprintln!("[AppActivate] {}", request.title);
    }
}

// ---- Shell ----

/// Source of task IDs on platforms that cannot start processes at all;
/// monotonically increasing so successive launches stay distinguishable
/// (and truthy — VB6 uses 0/absence to mean failure).
#[cfg(not(any(windows, unix)))]
static SYNTHETIC_TASK_IDS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// Next synthetic task ID for platforms without process creation.
#[cfg(not(any(windows, unix)))]
fn next_synthetic_task_id() -> f64 {
    use std::sync::atomic::Ordering;
    1.0 + SYNTHETIC_TASK_IDS.fetch_add(1, Ordering::Relaxed) as f64
}

/// Start `request`'s program without waiting for it and report its task ID.
///
/// Platform dispatch for `Shell`: real process creation where the OS
/// provides one, log-and-synthesize elsewhere so a browser run cannot crash
/// a program that merely shells out. The returned task ID is the child's
/// process id, which is what `AppActivate`'s numeric form matches against.
fn launch(request: &ShellRequest) -> std::io::Result<f64> {
    #[cfg(target_os = "windows")]
    {
        windows::spawn_process(request)
    }
    #[cfg(unix)]
    {
        posix::spawn_process(request)
    }
    #[cfg(not(any(windows, unix)))]
    {
        let _ = request;
        eprintln!("[Shell] {}", request.pathname);
        Ok(next_synthetic_task_id())
    }
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

    use super::super::appactivate::AppActivateRequest;
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

    // ---- AppActivate ----

    /// One top-level window seen by [`activate_window`]'s enumeration.
    struct WindowInfo {
        hwnd: isize,
        title: String,
        pid: u32,
    }

    /// Bring the window matching `request` to the foreground.
    ///
    /// Mirrors VB6 matching: numeric titles are tried as Shell task IDs
    /// (process ids) first; otherwise an exact caption match wins over a
    /// case-insensitive prefix match, which in turn wins over a
    /// case-insensitive suffix match. Returns whether a window was found
    /// and focused.
    pub(super) fn activate_window(request: &AppActivateRequest) -> bool {
        use windows_sys::Win32::UI::WindowsAndMessaging::{
            EnumWindows, IsIconic, SetForegroundWindow, ShowWindow, SW_RESTORE,
        };

        unsafe extern "system" fn enum_proc(hwnd: HWND, lparam: LPARAM) -> windows_sys::core::BOOL {
            use windows_sys::Win32::UI::WindowsAndMessaging::{
                GetWindowTextLengthW, GetWindowTextW, GetWindowThreadProcessId, IsWindowVisible,
            };

            let windows = &mut *(lparam as *mut Vec<WindowInfo>);
            if IsWindowVisible(hwnd) != 0 {
                let mut title = String::new();
                let len = GetWindowTextLengthW(hwnd);
                if len > 0 {
                    let mut buffer = vec![0u16; len as usize + 1];
                    let copied =
                        GetWindowTextW(hwnd, buffer.as_mut_ptr(), buffer.len() as i32) as usize;
                    buffer.truncate(copied);
                    title = String::from_utf16_lossy(&buffer);
                }
                let mut pid: u32 = 0;
                GetWindowThreadProcessId(hwnd, &mut pid);
                windows.push(WindowInfo {
                    hwnd: hwnd as isize,
                    title,
                    pid,
                });
            }
            1 // keep enumerating
        }

        let mut windows: Vec<WindowInfo> = Vec::new();
        unsafe {
            EnumWindows(Some(enum_proc), &mut windows as *mut _ as LPARAM);
        }

        let needle = request.title.to_lowercase();

        // Task-id form (`AppActivate Shell(...)`) matches by process id;
        // string form walks VB6's exact → prefix → suffix ladder.
        let target = if let Some(task_id) = request.as_task_id() {
            windows
                .iter()
                .find(|w| w.pid == task_id as u32)
                .map(|w| w.hwnd)
                .or_else(|| find_string_match(&windows, &needle))
        } else {
            find_string_match(&windows, &needle)
        };

        let Some(hwnd) = target else {
            return false;
        };

        unsafe {
            let hwnd = hwnd as HWND;
            if IsIconic(hwnd) != 0 {
                ShowWindow(hwnd, SW_RESTORE);
            }
            SetForegroundWindow(hwnd);
        }
        true
    }

    /// Walk the exact → prefix → suffix caption ladder for `needle`.
    ///
    /// Comparisons fold case, mirroring VB6's case-insensitive title
    /// matching.
    fn find_string_match(windows: &[WindowInfo], needle: &str) -> Option<isize> {
        let folded: Vec<(isize, String)> = windows
            .iter()
            .map(|w| (w.hwnd, w.title.to_lowercase()))
            .collect();
        folded
            .iter()
            .find(|(_, title)| title == needle)
            .or_else(|| folded.iter().find(|(_, title)| title.starts_with(needle)))
            .or_else(|| folded.iter().find(|(_, title)| title.ends_with(needle)))
            .map(|(hwnd, _)| *hwnd)
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

    // ---- Shell ----

    /// Spawn `request.pathname` via `CreateProcessW` and return the new
    /// process id as its task ID.
    ///
    /// The command line is passed through intact — VB6 lets `Shell` carry
    /// arguments, so Windows' own parser splits it. The requested window
    /// style rides in `STARTUPINFOW` (`STARTF_USESHOWWINDOW`), giving the
    /// child exactly the show state VB6 promises; a failed spawn surfaces
    /// the OS error so callers map it onto error 53/70.
    pub(super) fn spawn_process(request: &ShellRequest) -> std::io::Result<f64> {
        use windows_sys::Win32::Foundation::CloseHandle;
        use windows_sys::Win32::System::Threading::{
            CreateProcessW, CREATE_UNICODE_ENVIRONMENT, PROCESS_INFORMATION, STARTF_USESHOWWINDOW,
            STARTUPINFOW,
        };

        // NUL-terminated UTF-16 command line; CreateProcessW may write back
        // a normalized form into this buffer, hence mutability.
        let mut command_line: Vec<u16> = std::ffi::OsStr::new(&request.pathname)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        let mut startup: STARTUPINFOW = unsafe { std::mem::zeroed() };
        startup.cb = std::mem::size_of::<STARTUPINFOW>() as u32;
        startup.dwFlags = STARTF_USESHOWWINDOW;
        startup.wShowWindow = show_window_flag(request.window_style);
        let mut process: PROCESS_INFORMATION = unsafe { std::mem::zeroed() };

        let started = unsafe {
            CreateProcessW(
                std::ptr::null(), // derive application name from the command line
                command_line.as_mut_ptr(),
                std::ptr::null(),
                std::ptr::null(),
                0, // no handle inheritance: Shell shares nothing with the child
                CREATE_UNICODE_ENVIRONMENT,
                std::ptr::null(), // inherit our environment block
                std::ptr::null(), // inherit our current directory
                &startup,
                &mut process,
            )
        };
        if started == 0 {
            return Err(std::io::Error::last_os_error());
        }

        // The task ID outlives these handles; dropping them releases our
        // interest without disturbing the running child.
        unsafe {
            CloseHandle(process.hThread);
            CloseHandle(process.hProcess);
        }
        Ok(f64::from(process.dwProcessId))
    }

    /// Map a VB6 window style onto the Win32 `SW_*` constant that requests
    /// the same initial show state.
    fn show_window_flag(style: super::super::shell::WindowStyle) -> u16 {
        use super::super::shell::WindowStyle;
        match style {
            WindowStyle::Hide => 0,             // SW_HIDE
            WindowStyle::NormalFocus => 1,      // SW_SHOWNORMAL
            WindowStyle::MinimizedFocus => 2,   // SW_SHOWMINIMIZED
            WindowStyle::MaximizedFocus => 3,   // SW_SHOWMAXIMIZED
            WindowStyle::NormalNoFocus => 4,    // SW_SHOWNOACTIVATE
            WindowStyle::MinimizedNoFocus => 7, // SW_SHOWMINNOACTIVE
        }
    }
}

#[cfg(target_os = "macos")]
mod macos {
    use std::process::Command;

    use super::super::appactivate::AppActivateRequest;
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

    // ---- AppActivate ----

    /// Bring a window whose title matches to the foreground via
    /// `osascript` + System Events.
    ///
    /// Two passes mirror VB6 matching: process/window names that begin
    /// with the requested title win first, ones that end with it second;
    /// comparisons are case-insensitive. Returns:
    ///
    /// - `Some(true)` / `Some(false)` when osascript ran (match found or
    ///   not),
    /// - `None` when osascript is unavailable or refuses to run (headless
    ///   machines, automation permissions), letting the caller fall back.
    pub(super) fn activate_window(request: &AppActivateRequest) -> Option<bool> {
        let title = escape(&request.title);

        for comparison in ["begins with", "ends with"] {
            let script = format!(
                "tell application \"System Events\"\n\
                 \x20 repeat with p in (every application process whose visible is true)\n\
                 \x20   if name of p {comparison} \"{title}\" then\n\
                 \x20     set frontmost of p to true\n\
                 \x20     return \"activated\"\n\
                 \x20   end if\n\
                 \x20   try\n\
                 \x20     repeat with w in (every window of p)\n\
                 \x20       if name of w {comparison} \"{title}\" then\n\
                 \x20         perform action \"AXRaise\" of w\n\
                 \x20         set frontmost of p to true\n\
                 \x20         return \"activated\"\n\
                 \x20       end if\n\
                 \x20     end repeat\n\
                 \x20   end try\n\
                 \x20 end repeat\n\
                 end tell\n\
                 return \"missing\""
            );

            let output = Command::new("osascript")
                .arg("-e")
                .arg(&script)
                .output()
                .ok()?;
            if !output.status.success() {
                return None;
            }
            if String::from_utf8_lossy(&output.stdout).trim() == "activated" {
                return Some(true);
            }
        }
        Some(false)
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
mod linux {
    use std::process::Command;

    use super::super::appactivate::AppActivateRequest;
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

    /// Bring a window whose title matches to the foreground via `wmctrl`.
    ///
    /// The window list from `wmctrl -l` is matched in Rust so VB6's rules
    /// apply exactly: case-insensitive prefix first, suffix second (exact
    /// matches win inside both). Returns:
    ///
    /// - `Some(true)` / `Some(false)` when wmctrl ran (match found and
    ///   activated, or no match),
    /// - `None` when wmctrl is unavailable or cannot reach a display
    ///   (headless machines, Wayland sessions without XWayland), letting
    ///   the caller fall back.
    pub(super) fn activate_window(request: &AppActivateRequest) -> Option<bool> {
        let listing = Command::new("wmctrl").arg("-l").output().ok()?;
        if !listing.status.success() {
            return None;
        }

        let needle = request.title.to_lowercase();
        let windows: Vec<(String, String)> = String::from_utf8_lossy(&listing.stdout)
            .lines()
            .filter_map(|line| {
                let mut parts = line.splitn(4, char::is_whitespace);
                let id = parts.next()?.to_string();
                parts.next()?; // desktop number
                parts.next()?; // hostname
                Some((id, parts.next()?.to_lowercase()))
            })
            .collect();

        let find = |predicate: &dyn Fn(&str) -> bool| -> Option<String> {
            windows
                .iter()
                .find(|(_, title)| predicate(title))
                .map(|(id, _)| id.clone())
        };
        let target = find(&|t| t == needle)
            .or_else(|| find(&|t| t.starts_with(&needle)))
            .or_else(|| find(&|t| t.ends_with(&needle)))?;

        Command::new("wmctrl")
            .args(["-i", "-a"])
            .arg(target)
            .status()
            .ok()
            .map(|status| status.success())
    }
}

/// Shell process creation shared by Linux and macOS.
///
/// POSIX has no single-command-line spawn: the program and its arguments
/// are separate. VB6 programs are written against Windows' command-line
/// conventions, so this module splits the line itself (double quotes
/// honored) before spawning.
#[cfg(unix)]
mod posix {
    use std::process::{Command, Stdio};

    use super::super::shell::ShellRequest;

    /// Spawn `request.pathname` detached and return its process id.
    ///
    /// The child runs in its own process group so it outlives our signal
    /// delivery; its standard streams are disconnected because VB6's Shell
    /// offers no way to capture them anyway; and a background thread reaps
    /// the exit status so short-lived programs do not linger as zombies
    /// while a host keeps running. The requested window style has no POSIX
    /// equivalent and is ignored, mirroring how `InputBox` ignores
    /// unsupported positioning.
    pub(super) fn spawn_process(request: &ShellRequest) -> std::io::Result<f64> {
        let (program, arguments) = split_command_line(&request.pathname);
        let mut command = Command::new(program);
        command.args(arguments);
        command
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        use std::os::unix::process::CommandExt;
        command.process_group(0);

        let mut child = command.spawn()?;
        let pid = child.id();

        // Shell never waits (VB6 launches asynchronously), but an unreaped
        // child would hold a zombie entry until the interpreter exits.
        std::thread::spawn(move || {
            let _ = child.wait();
        });
        Ok(f64::from(pid))
    }

    /// Split a command line into its program and arguments.
    ///
    /// Whitespace separates tokens unless enclosed in double quotes; quote
    /// characters themselves do not reach the argument values. An empty or
    /// all-whitespace line yields an empty program name, which the spawn
    /// then reports as "file not found" — the same error real VB6 raises
    /// for `Shell ""`.
    pub(super) fn split_command_line(line: &str) -> (String, Vec<String>) {
        let mut tokens: Vec<String> = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;
        let mut token_started = false;
        for ch in line.chars() {
            match ch {
                '"' => {
                    in_quotes = !in_quotes;
                    token_started = true;
                }
                c if c.is_whitespace() && !in_quotes => {
                    if token_started {
                        tokens.push(std::mem::take(&mut current));
                        token_started = false;
                    }
                }
                c => {
                    current.push(c);
                    token_started = true;
                }
            }
        }
        if token_started {
            tokens.push(current);
        }
        if tokens.is_empty() {
            return (String::new(), Vec::new());
        }

        let program = tokens.drain(..1).next().unwrap_or_default();
        (program, tokens)
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
    fn no_such_window_is_error_5_describing_the_title() {
        let err = no_such_window("Ghost Window");
        assert_eq!(err.number, err_number::INVALID_PROCEDURE_CALL);
        assert!(
            err.description.contains("Ghost Window"),
            "{}",
            err.description
        );
    }

    #[test]
    fn app_activate_does_not_panic() {
        // Exercises whatever path this platform provides (real window
        // facility, logging fallback) without asserting on OS state.
        let backend = NativeBackend::new();
        let _ = backend.app_activate(&AppActivateRequest::new(
            "definitely-not-a-real-window-title-42",
        ));
    }

    #[test]
    #[cfg(unix)]
    fn shell_spawns_a_program_and_reports_its_task_id() {
        // `true` exists on every Unix CI runner and exits immediately.
        let task_id = launch(&ShellRequest::new("true")).unwrap();
        assert!(task_id > 0.0);
    }

    #[test]
    #[cfg(unix)]
    fn shell_missing_program_is_file_not_found() {
        let err = NativeBackend::new()
            .shell(&ShellRequest::new("definitely-not-a-program-42"))
            .unwrap_err();
        assert_eq!(err.number, err_number::FILE_NOT_FOUND);
        assert!(err.description.contains("definitely-not-a-program-42"));
    }

    #[test]
    #[cfg(unix)]
    fn command_line_splitting_honors_double_quotes() {
        let (program, args) = posix::split_command_line(
            r#""C:\Program Files\App.exe" /flag "my file.txt"   trailing"#,
        );
        assert_eq!(program, r"C:\Program Files\App.exe");
        assert_eq!(args, vec!["/flag", "my file.txt", "trailing"]);
    }

    #[test]
    #[cfg(unix)]
    fn empty_command_lines_split_to_an_empty_program() {
        let (program, args) = posix::split_command_line("   ");
        assert_eq!(program, "");
        assert!(args.is_empty());
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
