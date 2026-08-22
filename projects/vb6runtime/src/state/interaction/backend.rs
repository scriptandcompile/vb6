//! Trait abstracting over platform-specific user interaction operations.
//!
//! VB6 interaction functions (`Command$`, `DoEvents`, `Beep`, `MsgBox`,
//! `InputBox`, `AppActivate`) need a pluggable backend to support both
//! native platforms and WASM environments:
//!
//! - [`NativeBackend`](super::native::NativeBackend) (the default) uses the
//!   real process environment and OS primitives, showing the browser's
//!   modal `alert`/`confirm` dialogs on wasm32.
//! - [`MemoryBackend`](super::memory::MemoryBackend) provides injectable,
//!   deterministic behavior for tests.

use crate::error::VBResult;

use super::appactivate::AppActivateRequest;
use super::inputbox::InputBoxRequest;
use super::msgbox::{MsgBoxButton, MsgBoxRequest};
use super::sendkeys::SendKeysRequest;
use super::shell::ShellRequest;

/// Abstraction over user interaction operations.
///
/// Implementations must be `Send` so the backend can be shared across
/// threads via a `Mutex<Box<dyn InteractionBackend>>`.
pub trait InteractionBackend: Send {
    /// Get the command-line arguments passed to the program.
    ///
    /// Returns individual arguments (not including the program name).
    /// On native platforms this reads `std::env::args()`. On WASM it
    /// returns whatever the host injected.
    fn command_args(&self) -> Vec<String>;

    /// Yield execution to the operating system.
    ///
    /// Returns the number of open forms (VB6 stand-alone only; always 0
    /// for our interpreter). On native platforms this yields the thread.
    /// On WASM this is typically a no-op.
    fn do_events(&self) -> i16;

    /// Play the system beep sound.
    ///
    /// On native platforms this writes the terminal bell character (`\x07`)
    /// to stderr. On WASM this is a no-op.
    fn beep(&self);

    /// Signal a `Stop` statement break request.
    ///
    /// In the VB6 development environment `Stop` suspends execution and
    /// enters break mode; hosts that can honor that (a debugger, the WASM
    /// playground) observe this signal. Platforms with no debugger attached
    /// treat it as a no-op — the interpreter then applies the compiled
    /// `.exe` behavior instead (`Stop` acts like `End`).
    fn stop(&self);

    /// Show a modal message box and report which button was clicked.
    ///
    /// The `request` arrives fully decoded and validated (see
    /// [`MsgBoxRequest::parse`](super::msgbox::MsgBoxRequest::parse));
    /// implementations render it with whatever dialog facility the platform
    /// offers and return the chosen button. Implementations that cannot show
    /// a real dialog (headless machines, unsupported platforms) should log
    /// the request and return the request's default button rather than
    /// erroring, so programs stay runnable.
    fn msg_box(&self, request: &MsgBoxRequest) -> VBResult<MsgBoxButton>;

    /// Show a modal input box and report the text the user entered.
    ///
    /// The `request` arrives fully assembled (see [`InputBoxRequest`]);
    /// implementations display the prompt with a single-line edit box seeded
    /// from `default_response`, and return the entered text. Accepting an
    /// untouched box returns the default; Cancel (or Esc) returns `""`.
    /// Arguments the platform cannot honor — `xpos`/`ypos` on browsers and
    /// most window managers — are ignored rather than rejected.
    /// Implementations that cannot collect input at all (headless machines)
    /// should log the request and return `default_response` so programs stay
    /// runnable.
    fn input_box(&self, request: &InputBoxRequest) -> VBResult<String>;

    /// Bring an application window to the foreground.
    ///
    /// The `request` arrives fully decoded (see
    /// [`AppActivateRequest`](super::appactivate::AppActivateRequest));
    /// implementations locate the matching window — by caption prefix,
    /// then suffix, per VB6 rules — and give it the focus. Implementations
    /// must raise VB6 error 5 ("Invalid procedure call or argument") when
    /// no window matches but a window facility exists; where there is no
    /// window facility at all (headless machines) they should log the
    /// request and succeed so programs stay runnable.
    fn app_activate(&self, request: &AppActivateRequest) -> VBResult<()>;

    /// Deliver keystrokes to the active window as if typed at the keyboard.
    ///
    /// The `request` arrives fully decoded (see
    /// [`SendKeysRequest`](super::sendkeys::SendKeysRequest)): its
    /// `strokes` list is the expanded key sequence, ready for synthesis.
    /// Implementations feed each stroke to the platform's input injector —
    /// `SendInput` on Windows, `xdotool` on Linux, System Events on macOS.
    /// Platforms with no keyboard facility at all (headless machines,
    /// browsers) should log the request and succeed so programs stay
    /// runnable; there is deliberately no error path beyond malformed
    /// strings, which [`SendKeysRequest::parse`] rejects up front.
    fn send_keys(&self, request: &SendKeysRequest) -> VBResult<()>;

    /// Run an executable program asynchronously and report its task ID.
    ///
    /// The `request` arrives fully validated (see [`ShellRequest`]);
    /// implementations start the program without waiting for it to finish
    /// (VB6's `Shell` is asynchronous) and return a task ID — the process id
    /// on native platforms — usable with `AppActivate`. Implementations must
    /// raise VB6 error 53 ("File not found") when the program cannot be
    /// started; window styles the platform cannot honor are ignored rather
    /// than rejected.
    fn shell(&self, request: &ShellRequest) -> VBResult<f64>;

    /// Access the concrete backend type for downcasting.
    ///
    /// Hosts and tests install a backend they control and later need its
    /// type-specific API back (e.g. the memory backend's scripted-response
    /// queue); this makes the boxed trait object recoverable.
    fn as_any(&self) -> &dyn std::any::Any;
}
