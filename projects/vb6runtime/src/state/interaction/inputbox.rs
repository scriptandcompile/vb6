//! Input-box request model shared by every interaction backend.
//!
//! Unlike [`MsgBoxRequest`](super::msgbox::MsgBoxRequest), `InputBox` takes
//! no flag soup — its arguments are plain values — so this model is mostly a
//! carrier: it collects what the dialog should show and lets backends ignore
//! what their platform cannot honor (`helpfile`/`context` never reach a
//! backend at all; `xpos`/`ypos` are honored only where dialogs can be
//! positioned).

/// A fully decoded `InputBox` invocation.
///
/// Backends receive this instead of raw arguments; implementations render
/// the prompt with a single-line text entry seeded from
/// [`default_response`](Self::default_response) and report either the
/// entered text or `""` for Cancel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputBoxRequest {
    /// The message text displayed above the text box.
    pub prompt: String,
    /// Title-bar text; `None` means "use the application name".
    pub title: Option<String>,
    /// Text pre-filled in the box, returned when the user accepts without
    /// typing (also the memory backend's answer once its scripted list runs
    /// dry).
    pub default_response: String,
    /// Horizontal distance from the left screen edge, in twips. Backends
    /// whose dialogs cannot be positioned ignore this.
    pub xpos: Option<i32>,
    /// Vertical distance from the top screen edge, in twips. Backends
    /// whose dialogs cannot be positioned ignore this.
    pub ypos: Option<i32>,
}

impl InputBoxRequest {
    /// Create a request showing `prompt` with no title, default, or position.
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            title: None,
            default_response: String::new(),
            xpos: None,
            ypos: None,
        }
    }

    /// Set the title-bar text (`None` restores the application-name default).
    pub fn with_title(mut self, title: Option<String>) -> Self {
        self.title = title;
        self
    }

    /// Set the pre-filled response text.
    pub fn with_default(mut self, default_response: impl Into<String>) -> Self {
        self.default_response = default_response.into();
        self
    }

    /// Set the dialog position in twips from the screen's top-left corner.
    pub fn with_position(mut self, xpos: i32, ypos: i32) -> Self {
        self.xpos = Some(xpos);
        self.ypos = Some(ypos);
        self
    }
}

/// One recorded `InputBox` invocation — what the dialog showed, kept so
/// hosts and tests can assert on the requests a program made.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputBoxRecord {
    /// The message text shown.
    pub prompt: String,
    /// The title-bar text shown (`None` = application name).
    pub title: Option<String>,
    /// The pre-filled text shown in the box.
    pub default_response: String,
    /// Requested horizontal position in twips, if any.
    pub xpos: Option<i32>,
    /// Requested vertical position in twips, if any.
    pub ypos: Option<i32>,
}

impl InputBoxRecord {
    /// Capture the display-relevant parts of `request`.
    pub fn of(request: &InputBoxRequest) -> Self {
        Self {
            prompt: request.prompt.clone(),
            title: request.title.clone(),
            default_response: request.default_response.clone(),
            xpos: request.xpos,
            ypos: request.ypos,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_starts_minimal() {
        let request = InputBoxRequest::new("name?");
        assert_eq!(request.prompt, "name?");
        assert_eq!(request.title, None);
        assert_eq!(request.default_response, "");
        assert_eq!(request.xpos, None);
        assert_eq!(request.ypos, None);
    }

    #[test]
    fn builders_chain() {
        let request = InputBoxRequest::new("age?")
            .with_title(Some("Age".into()))
            .with_default("18")
            .with_position(1000, 2000);
        assert_eq!(request.title.as_deref(), Some("Age"));
        assert_eq!(request.default_response, "18");
        assert_eq!(request.xpos, Some(1000));
        assert_eq!(request.ypos, Some(2000));
    }

    #[test]
    fn record_captures_display_relevant_parts() {
        let request = InputBoxRequest::new("Proceed?")
            .with_title(Some("App".into()))
            .with_default("yes")
            .with_position(-5, 10);
        let record = InputBoxRecord::of(&request);
        assert_eq!(record.prompt, "Proceed?");
        assert_eq!(record.title.as_deref(), Some("App"));
        assert_eq!(record.default_response, "yes");
        assert_eq!(record.xpos, Some(-5));
        assert_eq!(record.ypos, Some(10));
    }
}
