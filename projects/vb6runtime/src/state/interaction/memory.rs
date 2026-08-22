//! In-memory interaction backend for WASM and tests.
//!
//! Stores injectable command-line arguments, records every `MsgBox`,
//! `InputBox`, `AppActivate`, `Shell`, and `SendKeys` request a program
//! makes, and answers those requests from scripted response lists — no OS
//! side effects anywhere.

use std::cell::{Cell, RefCell};
use std::collections::VecDeque;

use crate::error::{err_number, VBError, VBResult};

use super::appactivate::{AppActivateRecord, AppActivateRequest};
use super::backend::InteractionBackend;
use super::inputbox::{InputBoxRecord, InputBoxRequest};
use super::msgbox::{MsgBoxButton, MsgBoxRecord, MsgBoxRequest};
use super::sendkeys::{SendKeysRecord, SendKeysRequest};
use super::shell::{ShellRecord, ShellRequest};

/// In-memory interaction backend.
///
/// Command-line arguments are stored in a `Vec` and can be set via
/// [`set_command_args`]. `Beep` and `DoEvents` are silent no-ops; `Stop`
/// records its break request for hosts to observe.
///
/// `MsgBox` is fully scripted: each request is appended to the request log
/// (read it with [`msgbox_requests`](Self::msgbox_requests)), then answered
/// by popping the next queued response (see
/// [`push_msgbox_response`](Self::push_msgbox_response)). With an empty
/// queue the dialog's default button is returned so programs keep running;
/// a queued response that the dialog does not offer is reported as error 5
/// instead of being silently coerced.
///
/// `InputBox` works the same way: requests land in the log (see
/// [`inputbox_requests`](Self::inputbox_requests)) and are answered from the
/// scripted list fed by [`push_input_response`](Self::push_input_response).
/// An empty queue returns the dialog's default text; remember Cancel also
/// reads as the empty string in VB6, so queueing `""` scripts a cancel.
///
/// `AppActivate` is scripted too: requests land in the log (see
/// [`appactivate_requests`](Self::appactivate_requests)) and are answered
/// from the success flags queued by
/// [`push_activate_response`](Self::push_activate_response) — `true` means a
/// matching window was found and activated, `false` fails the statement with
/// VB6 error 5, exactly as when no window matches. An empty queue succeeds,
/// so programs keep running.
///
/// `Shell` follows suit without touching the OS: requests land in the log
/// (see [`shell_requests`](Self::shell_requests)) and each call pops the
/// next task ID from the list fed by
/// [`push_shell_response`](Self::push_shell_response) — "the value responses
/// are stored in a list and returned as requested". An empty queue hands out
/// synthetic IDs (1.0, 2.0, ...) so non-interactive runs proceed.
///
/// `SendKeys` is the same dummy: nothing reaches a real keyboard. Every
/// invocation is appended to the log (see
/// [`sendkeys_requests`](Self::sendkeys_requests)) — the sent keystrokes are
/// the stored values, returned to whoever asks — and, like `AppActivate`,
/// delivery can be scripted: outcomes queued via
/// [`push_sendkeys_response`](Self::push_sendkeys_response) decide whether
/// the statement succeeds or fails with VB6 error 5, and an empty queue
/// always succeeds.
pub struct MemoryBackend {
    /// The injected command-line arguments.
    command_args: Vec<String>,
    /// Whether a `Stop` statement requested a break.
    break_requested: Cell<bool>,
    /// Scripted `MsgBox` answers, consumed first-in first-out.
    msgbox_responses: RefCell<VecDeque<MsgBoxButton>>,
    /// Every `MsgBox` request this backend has seen, in order.
    msgbox_requests: RefCell<Vec<MsgBoxRecord>>,
    /// Scripted `InputBox` answers, consumed first-in first-out.
    input_responses: RefCell<VecDeque<String>>,
    /// Every `InputBox` request this backend has seen, in order.
    input_requests: RefCell<Vec<InputBoxRecord>>,
    /// Scripted `AppActivate` outcomes (`true` = activated), consumed
    /// first-in first-out.
    activate_responses: RefCell<VecDeque<bool>>,
    /// Every `AppActivate` request this backend has seen, in order.
    activate_requests: RefCell<Vec<AppActivateRecord>>,
    /// Scripted `Shell` task IDs, consumed first-in first-out.
    shell_responses: RefCell<VecDeque<f64>>,
    /// Every `Shell` request this backend has seen, in order.
    shell_requests: RefCell<Vec<ShellRecord>>,
    /// Scripted `SendKeys` outcomes (`true` = delivered), consumed
    /// first-in first-out.
    sendkeys_responses: RefCell<VecDeque<bool>>,
    /// Every `SendKeys` request this backend has seen, in order — the
    /// keystrokes a program "typed", stored in a list.
    sendkeys_requests: RefCell<Vec<SendKeysRecord>>,
    /// Source of synthetic task IDs once the scripted list runs dry;
    /// starts at 1.0 because 0 reads as failure in VB6.
    next_task_id: Cell<f64>,
}

impl MemoryBackend {
    /// Create a new empty in-memory backend.
    pub fn new() -> Self {
        Self {
            command_args: Vec::new(),
            break_requested: Cell::new(false),
            msgbox_responses: RefCell::new(VecDeque::new()),
            msgbox_requests: RefCell::new(Vec::new()),
            input_responses: RefCell::new(VecDeque::new()),
            input_requests: RefCell::new(Vec::new()),
            activate_responses: RefCell::new(VecDeque::new()),
            activate_requests: RefCell::new(Vec::new()),
            shell_responses: RefCell::new(VecDeque::new()),
            shell_requests: RefCell::new(Vec::new()),
            sendkeys_responses: RefCell::new(VecDeque::new()),
            sendkeys_requests: RefCell::new(Vec::new()),
            next_task_id: Cell::new(1.0),
        }
    }

    /// Create a backend with pre-set command-line arguments.
    pub fn with_args(args: Vec<String>) -> Self {
        let mut backend = Self::new();
        backend.command_args = args;
        backend
    }

    /// Create a backend whose `MsgBox` dialogs are answered from `responses`.
    pub fn with_msgbox_responses(responses: impl IntoIterator<Item = MsgBoxButton>) -> Self {
        let mut backend = Self::new();
        backend.msgbox_responses = RefCell::new(responses.into_iter().collect());
        backend
    }

    /// Create a backend whose `InputBox` dialogs are answered from `responses`.
    pub fn with_input_responses(responses: impl IntoIterator<Item = impl Into<String>>) -> Self {
        let mut backend = Self::new();
        backend.input_responses = RefCell::new(
            responses
                .into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .into(),
        );
        backend
    }

    /// Create a backend whose `AppActivate` calls succeed or fail per
    /// `responses` (`true` = activated, `false` = no matching window).
    pub fn with_activate_responses(responses: impl IntoIterator<Item = bool>) -> Self {
        let mut backend = Self::new();
        backend.activate_responses = RefCell::new(responses.into_iter().collect());
        backend
    }

    /// Create a backend whose `Shell` calls return task IDs from `responses`.
    pub fn with_shell_responses(responses: impl IntoIterator<Item = f64>) -> Self {
        let mut backend = Self::new();
        backend.shell_responses = RefCell::new(responses.into_iter().collect());
        backend
    }

    /// Create a backend whose `SendKeys` calls succeed or fail per
    /// `responses` (`true` = delivered, `false` = error 5).
    pub fn with_sendkeys_responses(responses: impl IntoIterator<Item = bool>) -> Self {
        let mut backend = Self::new();
        backend.sendkeys_responses = RefCell::new(responses.into_iter().collect());
        backend
    }

    /// Replace the command-line arguments.
    pub fn set_command_args(&mut self, args: Vec<String>) {
        self.command_args = args;
    }

    /// Whether a `Stop` statement has requested a break.
    pub fn break_requested(&self) -> bool {
        self.break_requested.get()
    }

    /// Queue a scripted answer for the next `MsgBox` call.
    ///
    /// The button must be one the dialog actually offers; otherwise the
    /// `MsgBox` call fails with error 5 describing the mismatch. Queue as
    /// many responses as the program will show dialogs.
    pub fn push_msgbox_response(&self, button: MsgBoxButton) {
        self.msgbox_responses.borrow_mut().push_back(button);
    }

    /// Queue several scripted answers at once.
    pub fn extend_msgbox_responses(&self, responses: impl IntoIterator<Item = MsgBoxButton>) {
        self.msgbox_responses.borrow_mut().extend(responses);
    }

    /// Drop all queued (not yet consumed) `MsgBox` responses.
    pub fn clear_msgbox_responses(&self) {
        self.msgbox_responses.borrow_mut().clear();
    }

    /// How many scripted answers are still queued.
    pub fn pending_msgbox_responses(&self) -> usize {
        self.msgbox_responses.borrow().len()
    }

    /// Snapshot of every `MsgBox` request made so far, oldest first.
    pub fn msgbox_requests(&self) -> Vec<MsgBoxRecord> {
        self.msgbox_requests.borrow().clone()
    }

    /// Take the recorded `MsgBox` requests, leaving the log empty.
    pub fn take_msgbox_requests(&self) -> Vec<MsgBoxRecord> {
        std::mem::take(&mut *self.msgbox_requests.borrow_mut())
    }

    /// Queue a scripted answer for the next `InputBox` call.
    ///
    /// The string is returned verbatim, as if typed into the box; queue `""`
    /// to simulate Cancel. Queue as many responses as the program will show
    /// dialogs — once the list runs dry the dialog's default text is
    /// returned instead.
    pub fn push_input_response(&self, response: impl Into<String>) {
        self.input_responses.borrow_mut().push_back(response.into());
    }

    /// Queue several scripted answers at once.
    pub fn extend_input_responses(&self, responses: impl IntoIterator<Item = impl Into<String>>) {
        self.input_responses
            .borrow_mut()
            .extend(responses.into_iter().map(Into::into));
    }

    /// Drop all queued (not yet consumed) `InputBox` responses.
    pub fn clear_input_responses(&self) {
        self.input_responses.borrow_mut().clear();
    }

    /// How many scripted answers are still queued.
    pub fn pending_input_responses(&self) -> usize {
        self.input_responses.borrow().len()
    }

    /// Snapshot of every `InputBox` request made so far, oldest first.
    pub fn inputbox_requests(&self) -> Vec<InputBoxRecord> {
        self.input_requests.borrow().clone()
    }

    /// Take the recorded `InputBox` requests, leaving the log empty.
    pub fn take_inputbox_requests(&self) -> Vec<InputBoxRecord> {
        std::mem::take(&mut *self.input_requests.borrow_mut())
    }

    /// Queue the outcome of the next `AppActivate` call.
    ///
    /// `true` scripts a successful activation; `false` makes the statement
    /// fail with VB6 error 5 ("Invalid procedure call or argument"), as it
    /// does when no window matches. Queue as many outcomes as the program
    /// will call `AppActivate` — once the list runs dry every activation
    /// succeeds instead.
    pub fn push_activate_response(&self, activated: bool) {
        self.activate_responses.borrow_mut().push_back(activated);
    }

    /// Queue several scripted outcomes at once.
    pub fn extend_activate_responses(&self, responses: impl IntoIterator<Item = bool>) {
        self.activate_responses.borrow_mut().extend(responses);
    }

    /// Drop all queued (not yet consumed) `AppActivate` outcomes.
    pub fn clear_activate_responses(&self) {
        self.activate_responses.borrow_mut().clear();
    }

    /// How many scripted outcomes are still queued.
    pub fn pending_activate_responses(&self) -> usize {
        self.activate_responses.borrow().len()
    }

    /// Snapshot of every `AppActivate` request made so far, oldest first.
    pub fn appactivate_requests(&self) -> Vec<AppActivateRecord> {
        self.activate_requests.borrow().clone()
    }

    /// Take the recorded `AppActivate` requests, leaving the log empty.
    pub fn take_appactivate_requests(&self) -> Vec<AppActivateRecord> {
        std::mem::take(&mut *self.activate_requests.borrow_mut())
    }

    // ---- Shell scripting ----

    /// Queue the task ID the next `Shell` call returns.
    ///
    /// Values are returned verbatim, as if the program had really started;
    /// queue as many as the program will shell out — once the list runs dry
    /// synthetic IDs (1.0, 2.0, ...) are handed out instead so programs keep
    /// running.
    pub fn push_shell_response(&self, task_id: f64) {
        self.shell_responses.borrow_mut().push_back(task_id);
    }

    /// Queue several scripted task IDs at once.
    pub fn extend_shell_responses(&self, responses: impl IntoIterator<Item = f64>) {
        self.shell_responses.borrow_mut().extend(responses);
    }

    /// Drop all queued (not yet consumed) `Shell` responses.
    pub fn clear_shell_responses(&self) {
        self.shell_responses.borrow_mut().clear();
    }

    /// How many scripted task IDs are still queued.
    pub fn pending_shell_responses(&self) -> usize {
        self.shell_responses.borrow().len()
    }

    /// Snapshot of every `Shell` request made so far, oldest first.
    pub fn shell_requests(&self) -> Vec<ShellRecord> {
        self.shell_requests.borrow().clone()
    }

    /// Take the recorded `Shell` requests, leaving the log empty.
    pub fn take_shell_requests(&self) -> Vec<ShellRecord> {
        std::mem::take(&mut *self.shell_requests.borrow_mut())
    }

    // ---- SendKeys scripting ----

    /// Queue the outcome of the next `SendKeys` call.
    ///
    /// `true` scripts a successful delivery; `false` makes the statement
    /// fail with VB6 error 5 ("Invalid procedure call or argument"), as it
    /// does when the keys cannot reach the active window. Queue as many
    /// outcomes as the program will call `SendKeys` — once the list runs
    /// dry every delivery succeeds instead.
    pub fn push_sendkeys_response(&self, delivered: bool) {
        self.sendkeys_responses.borrow_mut().push_back(delivered);
    }

    /// Queue several scripted outcomes at once.
    pub fn extend_sendkeys_responses(&self, responses: impl IntoIterator<Item = bool>) {
        self.sendkeys_responses.borrow_mut().extend(responses);
    }

    /// Drop all queued (not yet consumed) `SendKeys` outcomes.
    pub fn clear_sendkeys_responses(&self) {
        self.sendkeys_responses.borrow_mut().clear();
    }

    /// How many scripted outcomes are still queued.
    pub fn pending_sendkeys_responses(&self) -> usize {
        self.sendkeys_responses.borrow().len()
    }

    /// Snapshot of every `SendKeys` request made so far, oldest first —
    /// the keystrokes the program has "typed", in order.
    pub fn sendkeys_requests(&self) -> Vec<SendKeysRecord> {
        self.sendkeys_requests.borrow().clone()
    }

    /// Take the recorded `SendKeys` requests, leaving the list empty.
    pub fn take_sendkeys_requests(&self) -> Vec<SendKeysRecord> {
        std::mem::take(&mut *self.sendkeys_requests.borrow_mut())
    }
}

impl Default for MemoryBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl InteractionBackend for MemoryBackend {
    fn command_args(&self) -> Vec<String> {
        self.command_args.clone()
    }

    fn do_events(&self) -> i16 {
        0
    }

    fn beep(&self) {
        // No-op in the memory backend.
    }

    fn stop(&self) {
        // Record the request so hosts can observe it. Interior mutability
        // keeps the trait's `&self` signature intact.
        self.break_requested.set(true);
    }

    fn msg_box(&self, request: &MsgBoxRequest) -> VBResult<MsgBoxButton> {
        // Record what was shown before answering, so even a failing call
        // leaves a trace of the offending request.
        self.msgbox_requests
            .borrow_mut()
            .push(MsgBoxRecord::of(request));

        match self.msgbox_responses.borrow_mut().pop_front() {
            // Nothing scripted: auto-answer with the default button so
            // non-interactive runs (WASM playground, batch tests) proceed.
            None => Ok(request.default_button_value()),
            Some(response) if request.offered_buttons().contains(&response) => Ok(response),
            Some(response) => Err(VBError::with_description(
                err_number::INVALID_PROCEDURE_CALL,
                format!(
                    "MsgBox response mismatch: dialog offers {} but the queued response \
                     is {}",
                    offered_list(request.offered_buttons()),
                    response.name(),
                ),
            )),
        }
    }

    fn input_box(&self, request: &InputBoxRequest) -> VBResult<String> {
        // Record what was shown before answering, so hosts can inspect the
        // request even after the program moved on.
        self.input_requests
            .borrow_mut()
            .push(InputBoxRecord::of(request));

        Ok(self
            .input_responses
            .borrow_mut()
            .pop_front()
            .unwrap_or_else(|| request.default_response.clone()))
    }

    fn app_activate(&self, request: &AppActivateRequest) -> VBResult<()> {
        // Record what was requested before answering, so even a failing call
        // leaves a trace of the offending request.
        self.activate_requests
            .borrow_mut()
            .push(AppActivateRecord::of(request));

        match self.activate_responses.borrow_mut().pop_front() {
            // Nothing scripted: report success so non-interactive runs
            // (WASM playground, batch tests) proceed.
            None | Some(true) => Ok(()),
            Some(false) => Err(VBError::with_description(
                err_number::INVALID_PROCEDURE_CALL,
                format!(
                    "Invalid procedure call or argument: AppActivate found no window \
                     titled \"{}\"",
                    request.title,
                ),
            )),
        }
    }

    fn shell(&self, request: &ShellRequest) -> VBResult<f64> {
        // Record what was requested before answering, so hosts can inspect
        // the command line and window style even after the program moved on.
        self.shell_requests
            .borrow_mut()
            .push(ShellRecord::of(request));

        Ok(self
            .shell_responses
            .borrow_mut()
            .pop_front()
            .unwrap_or_else(|| {
                // Nothing scripted: hand out a synthetic task ID so
                // non-interactive runs (WASM playground, batch tests) proceed.
                let task_id = self.next_task_id.get();
                self.next_task_id.set(task_id + 1.0);
                task_id
            }))
    }

    fn send_keys(&self, request: &SendKeysRequest) -> VBResult<()> {
        // Record what was "typed" before answering, so even a failing call
        // leaves a trace of the offending request.
        self.sendkeys_requests
            .borrow_mut()
            .push(SendKeysRecord::of(request));

        match self.sendkeys_responses.borrow_mut().pop_front() {
            // Nothing scripted: report success so non-interactive runs
            // (WASM playground, batch tests) proceed.
            None | Some(true) => Ok(()),
            Some(false) => Err(VBError::with_description(
                err_number::INVALID_PROCEDURE_CALL,
                format!(
                    "Invalid procedure call or argument: SendKeys could not deliver \"{}\"",
                    request.keys,
                ),
            )),
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Format offered buttons for error messages: `"Yes|No"` style.
fn offered_list(buttons: &[MsgBoxButton]) -> String {
    buttons
        .iter()
        .map(|b| b.name())
        .collect::<Vec<_>>()
        .join("|")
}

#[cfg(test)]
mod tests {
    use super::super::shell::WindowStyle;
    use super::*;

    #[test]
    fn empty_by_default() {
        let backend = MemoryBackend::new();
        assert!(backend.command_args().is_empty());
    }

    #[test]
    fn with_args_returns_injected() {
        let backend = MemoryBackend::with_args(vec!["--debug".into(), "file.txt".into()]);
        assert_eq!(backend.command_args(), vec!["--debug", "file.txt"]);
    }

    #[test]
    fn set_command_args_replaces() {
        let mut backend = MemoryBackend::new();
        backend.set_command_args(vec!["/server:localhost".into()]);
        assert_eq!(backend.command_args(), vec!["/server:localhost"]);
    }

    #[test]
    fn do_events_returns_zero() {
        let backend = MemoryBackend::new();
        assert_eq!(backend.do_events(), 0i16);
    }

    #[test]
    fn stop_sets_break_requested() {
        let backend = MemoryBackend::new();
        assert!(!backend.break_requested());
        backend.stop();
        assert!(backend.break_requested());
    }

    // ---- MsgBox scripting ----

    #[test]
    fn empty_queue_answers_default_button() {
        let backend = MemoryBackend::new();
        let request = MsgBoxRequest::parse("save?", 4 + 32 + 256).unwrap(); // YesNo+Question+Default2
        let answer = backend.msg_box(&request).unwrap();
        assert_eq!(answer, MsgBoxButton::No);
    }

    #[test]
    fn queued_response_is_returned_in_order() {
        let backend = MemoryBackend::with_msgbox_responses([MsgBoxButton::Yes, MsgBoxButton::No]);
        let request = MsgBoxRequest::parse("again?", 4).unwrap();
        assert_eq!(backend.msg_box(&request).unwrap(), MsgBoxButton::Yes);
        assert_eq!(backend.msg_box(&request).unwrap(), MsgBoxButton::No);
        // Queue exhausted: back to default answers.
        assert_eq!(backend.msg_box(&request).unwrap(), MsgBoxButton::Yes);
    }

    #[test]
    fn incompatible_response_is_reported_not_coerced() {
        let backend = MemoryBackend::with_msgbox_responses([MsgBoxButton::Cancel]);
        // Dialog offers Yes/No only; Cancel is not among them.
        let request = MsgBoxRequest::parse("overwrite?", 4).unwrap();
        let err = backend.msg_box(&request).unwrap_err();
        assert_eq!(err.number, err_number::INVALID_PROCEDURE_CALL);
        assert!(err.description.contains("Yes|No"), "{}", err.description);
        assert!(err.description.contains("Cancel"), "{}", err.description);
    }

    #[test]
    fn matching_response_for_okcancel_works() {
        let backend = MemoryBackend::with_msgbox_responses([MsgBoxButton::Cancel]);
        let request = MsgBoxRequest::parse("continue?", 1).unwrap();
        assert_eq!(backend.msg_box(&request).unwrap(), MsgBoxButton::Cancel);
    }

    #[test]
    fn requests_are_recorded_even_when_rejected() {
        let backend = MemoryBackend::with_msgbox_responses([MsgBoxButton::Ignore]);
        let request = MsgBoxRequest::parse("retry?", 5)
            .unwrap()
            .with_title(Some("Ops".into()));
        let _ = backend.msg_box(&request);

        let requests = backend.take_msgbox_requests();
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].prompt, "retry?");
        assert_eq!(requests[0].title.as_deref(), Some("Ops"));
        assert_eq!(
            requests[0].offered_buttons,
            vec![MsgBoxButton::Retry, MsgBoxButton::Cancel]
        );
        assert!(backend.msgbox_requests().is_empty());
    }

    #[test]
    fn response_queue_helpers_round_trip() {
        let backend = MemoryBackend::new();
        backend.push_msgbox_response(MsgBoxButton::Abort);
        backend.extend_msgbox_responses([MsgBoxButton::Retry]);
        assert_eq!(backend.pending_msgbox_responses(), 2);
        backend.clear_msgbox_responses();
        assert_eq!(backend.pending_msgbox_responses(), 0);
    }

    // ---- InputBox scripting ----

    #[test]
    fn empty_input_queue_answers_default_response() {
        let backend = MemoryBackend::new();
        let request = InputBoxRequest::new("name?").with_default("Arthur");
        let answer = backend.input_box(&request).unwrap();
        assert_eq!(answer, "Arthur");
    }

    #[test]
    fn empty_input_queue_without_default_returns_empty_string() {
        let backend = MemoryBackend::new();
        let answer = backend.input_box(&InputBoxRequest::new("name?")).unwrap();
        assert_eq!(answer, "");
    }

    #[test]
    fn queued_input_responses_are_returned_in_order() {
        let backend = MemoryBackend::with_input_responses(["first", "second", ""]);
        let request = InputBoxRequest::new("value?").with_default("default");
        assert_eq!(backend.input_box(&request).unwrap(), "first");
        assert_eq!(backend.input_box(&request).unwrap(), "second");
        assert_eq!(backend.input_box(&request).unwrap(), "");
        // Queue exhausted: back to default answers.
        assert_eq!(backend.input_box(&request).unwrap(), "default");
    }

    #[test]
    fn input_requests_are_recorded() {
        let backend = MemoryBackend::new();
        backend
            .input_box(
                &InputBoxRequest::new("port?")
                    .with_title(Some("Config".into()))
                    .with_default("8080")
                    .with_position(100, 200),
            )
            .unwrap();

        let requests = backend.take_inputbox_requests();
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].prompt, "port?");
        assert_eq!(requests[0].title.as_deref(), Some("Config"));
        assert_eq!(requests[0].default_response, "8080");
        assert_eq!(requests[0].xpos, Some(100));
        assert_eq!(requests[0].ypos, Some(200));
        assert!(backend.inputbox_requests().is_empty());
    }

    #[test]
    fn input_response_queue_helpers_round_trip() {
        let backend = MemoryBackend::new();
        backend.push_input_response("a");
        backend.extend_input_responses(["b", "c"]);
        assert_eq!(backend.pending_input_responses(), 3);
        backend.clear_input_responses();
        assert_eq!(backend.pending_input_responses(), 0);
    }

    // ---- AppActivate scripting ----

    #[test]
    fn empty_activate_queue_succeeds() {
        let backend = MemoryBackend::new();
        assert!(backend
            .app_activate(&AppActivateRequest::new("Calculator"))
            .is_ok());
    }

    #[test]
    fn queued_activate_outcomes_are_returned_in_order() {
        let backend = MemoryBackend::with_activate_responses([true, false, true]);
        let request = AppActivateRequest::new("Notepad");
        assert!(backend.app_activate(&request).is_ok());
        assert!(backend.app_activate(&request).is_err());
        assert!(backend.app_activate(&request).is_ok());
        // Queue exhausted: back to success.
        assert!(backend.app_activate(&request).is_ok());
    }

    #[test]
    fn failed_activation_is_error_5_describing_the_title() {
        let backend = MemoryBackend::with_activate_responses([false]);
        let request = AppActivateRequest::new("Missing Window").with_wait(true);
        let err = backend.app_activate(&request).unwrap_err();
        assert_eq!(err.number, err_number::INVALID_PROCEDURE_CALL);
        assert!(
            err.description.contains("Missing Window"),
            "{}",
            err.description
        );
    }

    #[test]
    fn activate_requests_are_recorded_even_when_rejected() {
        let backend = MemoryBackend::with_activate_responses([false]);
        let _ = backend.app_activate(&AppActivateRequest::new("Calc"));

        let requests = backend.take_appactivate_requests();
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].title, "Calc");
        assert!(!requests[0].wait);
        assert!(backend.appactivate_requests().is_empty());
    }

    #[test]
    fn activate_response_queue_helpers_round_trip() {
        let backend = MemoryBackend::new();
        backend.push_activate_response(true);
        backend.extend_activate_responses([false, true]);
        assert_eq!(backend.pending_activate_responses(), 3);
        backend.clear_activate_responses();
        assert_eq!(backend.pending_activate_responses(), 0);
    }

    // ---- Shell scripting ----

    #[test]
    fn empty_shell_queue_hands_out_synthetic_task_ids() {
        let backend = MemoryBackend::new();
        let request = ShellRequest::new("notepad.exe");
        assert_eq!(backend.shell(&request).unwrap(), 1.0);
        assert_eq!(backend.shell(&request).unwrap(), 2.0);
        // IDs never repeat and never read as failure (0).
        assert!(backend.shell(&request).unwrap() > 0.0);
    }

    #[test]
    fn queued_shell_responses_are_returned_in_order() {
        let backend = MemoryBackend::with_shell_responses([4242.0, 7.0]);
        let request = ShellRequest::new("calc.exe");
        assert_eq!(backend.shell(&request).unwrap(), 4242.0);
        assert_eq!(backend.shell(&request).unwrap(), 7.0);
        // List exhausted: back to synthetic IDs.
        assert_eq!(backend.shell(&request).unwrap(), 1.0);
    }

    #[test]
    fn shell_requests_are_recorded() {
        let backend = MemoryBackend::with_shell_responses([99.0]);
        backend
            .shell(
                &ShellRequest::new(r#""C:\Program Files\App.exe" /flag"#)
                    .with_window_style(WindowStyle::MaximizedFocus),
            )
            .unwrap();

        let requests = backend.take_shell_requests();
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].pathname, r#""C:\Program Files\App.exe" /flag"#);
        assert_eq!(requests[0].window_style, WindowStyle::MaximizedFocus);
        assert!(backend.shell_requests().is_empty());
    }

    #[test]
    fn shell_requests_are_recorded_even_with_an_empty_list() {
        let backend = MemoryBackend::new();
        let _ = backend.shell(&ShellRequest::new("backup.bat"));
        let requests = backend.shell_requests();
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].pathname, "backup.bat");
        assert_eq!(requests[0].window_style, WindowStyle::MinimizedFocus);
    }

    #[test]
    fn shell_response_queue_helpers_round_trip() {
        let backend = MemoryBackend::new();
        backend.push_shell_response(10.0);
        backend.extend_shell_responses([20.0, 30.0]);
        assert_eq!(backend.pending_shell_responses(), 3);
        backend.clear_shell_responses();
        assert_eq!(backend.pending_shell_responses(), 0);
    }

    // ---- SendKeys scripting ----

    #[test]
    fn empty_sendkeys_queue_succeeds_without_touching_the_os() {
        let backend = MemoryBackend::new();
        assert!(backend
            .send_keys(&SendKeysRequest::parse("Hello{ENTER}", false).unwrap())
            .is_ok());
    }

    #[test]
    fn sent_keys_are_stored_in_a_list_and_returned_as_requested() {
        let backend = MemoryBackend::new();
        backend
            .send_keys(&SendKeysRequest::parse("User{TAB}", true).unwrap())
            .unwrap();
        backend
            .send_keys(&SendKeysRequest::parse("{ENTER}", false).unwrap())
            .unwrap();

        let requests = backend.take_sendkeys_requests();
        assert_eq!(requests.len(), 2);
        assert_eq!(requests[0].keys, "User{TAB}");
        assert!(requests[0].wait);
        assert_eq!(requests[1].keys, "{ENTER}");
        assert!(!requests[1].wait);
        // Taken: the list is empty afterwards.
        assert!(backend.sendkeys_requests().is_empty());
    }

    #[test]
    fn queued_sendkeys_outcomes_are_returned_in_order() {
        let backend = MemoryBackend::with_sendkeys_responses([true, false, true]);
        let request = SendKeysRequest::parse("^c", false).unwrap();
        assert!(backend.send_keys(&request).is_ok());
        assert!(backend.send_keys(&request).is_err());
        assert!(backend.send_keys(&request).is_ok());
        // Queue exhausted: back to success.
        assert!(backend.send_keys(&request).is_ok());
    }

    #[test]
    fn failed_delivery_is_error_5_describing_the_keys() {
        let backend = MemoryBackend::with_sendkeys_responses([false]);
        let request = SendKeysRequest::parse("%{F4}", true).unwrap();
        let err = backend.send_keys(&request).unwrap_err();
        assert_eq!(err.number, err_number::INVALID_PROCEDURE_CALL);
        assert!(err.description.contains("%{F4}"), "{}", err.description);
    }

    #[test]
    fn sendkeys_requests_are_recorded_even_when_rejected() {
        let backend = MemoryBackend::with_sendkeys_responses([false]);
        let _ = backend.send_keys(&SendKeysRequest::parse("hi", false).unwrap());

        let requests = backend.take_sendkeys_requests();
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].keys, "hi");
        assert!(!requests[0].wait);
    }

    #[test]
    fn sendkeys_response_queue_helpers_round_trip() {
        let backend = MemoryBackend::new();
        backend.push_sendkeys_response(true);
        backend.extend_sendkeys_responses([false, true]);
        assert_eq!(backend.pending_sendkeys_responses(), 3);
        backend.clear_sendkeys_responses();
        assert_eq!(backend.pending_sendkeys_responses(), 0);
    }
}
