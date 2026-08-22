//! `AppActivate` request model shared by every interaction backend.
//!
//! Like [`InputBoxRequest`](super::inputbox::InputBoxRequest), `AppActivate`
//! takes no flag soup — its arguments are a title and an optional wait flag —
//! so this model is mostly a carrier. The `title` arrives already converted
//! to its string form; a numeric title (the task ID returned by `Shell`)
//! reaches backends as the decimal digits of that number, which platform
//! implementations may interpret as a process/window id.

/// A fully decoded `AppActivate` invocation.
///
/// Backends receive this instead of raw arguments; implementations bring
/// the window whose caption matches [`title`](Self::title) to the foreground,
/// honoring VB6 matching rules (prefix match first, suffix match second).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppActivateRequest {
    /// Window caption to activate. A string activates the first window
    /// whose title starts with it (falling back to one that ends with it);
    /// a numeric string is also tried as a Shell task ID / process id.
    pub title: String,
    /// Whether to defer activation until the calling application itself has
    /// focus (`True`), instead of activating immediately (`False`, VB6's
    /// default). Platforms without a window manager treat this as a hint.
    pub wait: bool,
}

impl AppActivateRequest {
    /// Create a request activating `title` immediately (VB6's default).
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            wait: false,
        }
    }

    /// Set the deferred-activation flag (`wait` argument).
    pub fn with_wait(mut self, wait: bool) -> Self {
        self.wait = wait;
        self
    }

    /// The title parsed as a numeric Shell task ID, if it is one.
    ///
    /// VB6 allows `AppActivate taskId` where `taskId` came from `Shell`;
    /// such values arrive here as their decimal string form.
    pub fn as_task_id(&self) -> Option<i64> {
        self.title.trim().parse::<i64>().ok()
    }
}

/// One recorded `AppActivate` invocation — what was requested, kept so hosts
/// and tests can assert on the activations a program attempted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppActivateRecord {
    /// The requested window title.
    pub title: String,
    /// The requested `wait` flag.
    pub wait: bool,
}

impl AppActivateRecord {
    /// Capture the relevant parts of `request`.
    pub fn of(request: &AppActivateRequest) -> Self {
        Self {
            title: request.title.clone(),
            wait: request.wait,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_starts_minimal() {
        let request = AppActivateRequest::new("Calculator");
        assert_eq!(request.title, "Calculator");
        assert!(!request.wait);
    }

    #[test]
    fn builders_chain() {
        let request = AppActivateRequest::new("Notepad").with_wait(true);
        assert_eq!(request.title, "Notepad");
        assert!(request.wait);
    }

    #[test]
    fn numeric_titles_parse_as_task_ids() {
        assert_eq!(AppActivateRequest::new("1234").as_task_id(), Some(1234));
        assert_eq!(AppActivateRequest::new(" 42 ").as_task_id(), Some(42));
        assert_eq!(AppActivateRequest::new("Notepad").as_task_id(), None);
    }

    #[test]
    fn record_captures_relevant_parts() {
        let request = AppActivateRequest::new("My App").with_wait(true);
        let record = AppActivateRecord::of(&request);
        assert_eq!(record.title, "My App");
        assert!(record.wait);
    }
}
