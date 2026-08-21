//! In-memory interaction backend for WASM and tests.
//!
//! Stores injectable command-line arguments, records every `MsgBox` and
//! `InputBox` request a program makes, and answers those requests from
//! scripted response lists — no OS side effects anywhere.

use std::cell::{Cell, RefCell};
use std::collections::VecDeque;

use crate::error::{err_number, VBError, VBResult};

use super::backend::InteractionBackend;
use super::inputbox::{InputBoxRecord, InputBoxRequest};
use super::msgbox::{MsgBoxButton, MsgBoxRecord, MsgBoxRequest};

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
}
