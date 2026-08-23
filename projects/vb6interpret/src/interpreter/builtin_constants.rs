//! Registration of the built-in VB6 constants as global variables.
//!
//! These constants (`vbCrLf`, `vbTab`, the `MsgBox` styles, ...) are
//! available in every VB6 program without declaration. They are registered
//! as case-insensitive globals at the start of every run so that source code
//! referencing them resolves correctly.

use vb6runtime::VBVariant;

use crate::interpreter::Interpreter;

impl Interpreter {
    /// Register the built-in VB6 data constants as globals.
    ///
    /// These constants (`vbCrLf`, `vbTab`, etc.) are available in every VB6
    /// program without declaration.  They are registered as case-insensitive
    /// globals so that VB6 source code referencing them resolves correctly.
    pub(crate) fn register_builtin_constants(&mut self) {
        use vb6runtime::library::constants::*;
        self.globals.declare("vbcr", VBVariant::from_string(VB_CR));
        self.globals
            .declare("vbcrlf", VBVariant::from_string(VB_CRLF));
        self.globals.declare("vblf", VBVariant::from_string(VB_LF));
        self.globals
            .declare("vbnewline", VBVariant::from_string(VB_NEW_LINE));
        self.globals
            .declare("vbnullchar", VBVariant::from_string(VB_NULL_CHAR));
        self.globals
            .declare("vbnullstring", VBVariant::from_string(VB_NULL_STRING));
        self.globals
            .declare("vbtab", VBVariant::from_string(VB_TAB));
        self.globals
            .declare("vbback", VBVariant::from_string(VB_BACK));
        self.globals
            .declare("vbformfeed", VBVariant::from_string(VB_FORM_FEED));
        self.globals
            .declare("vbverticaltab", VBVariant::from_string(VB_VERTICAL_TAB));
        self.globals
            .declare("vbunicode", VBVariant::from_long(VB_UNICODE));
        self.globals
            .declare("vbfromunicode", VBVariant::from_long(VB_FROM_UNICODE));

        // Message-box style constants (`VbMsgBoxStyle`).
        self.globals
            .declare("vbokonly", VBVariant::from_long(VB_OK_ONLY));
        self.globals
            .declare("vbokcancel", VBVariant::from_long(VB_OK_CANCEL));
        self.globals.declare(
            "vbabortretryignore",
            VBVariant::from_long(VB_ABORT_RETRY_IGNORE),
        );
        self.globals
            .declare("vbyesnocancel", VBVariant::from_long(VB_YES_NO_CANCEL));
        self.globals
            .declare("vbyesno", VBVariant::from_long(VB_YES_NO));
        self.globals
            .declare("vbretrycancel", VBVariant::from_long(VB_RETRY_CANCEL));
        self.globals
            .declare("vbcritical", VBVariant::from_long(VB_CRITICAL));
        self.globals
            .declare("vbquestion", VBVariant::from_long(VB_QUESTION));
        self.globals
            .declare("vbexclamation", VBVariant::from_long(VB_EXCLAMATION));
        self.globals
            .declare("vbinformation", VBVariant::from_long(VB_INFORMATION));
        self.globals.declare(
            "vbdefaultbutton1",
            VBVariant::from_long(VB_DEFAULT_BUTTON_1),
        );
        self.globals.declare(
            "vbdefaultbutton2",
            VBVariant::from_long(VB_DEFAULT_BUTTON_2),
        );
        self.globals.declare(
            "vbdefaultbutton3",
            VBVariant::from_long(VB_DEFAULT_BUTTON_3),
        );
        self.globals.declare(
            "vbdefaultbutton4",
            VBVariant::from_long(VB_DEFAULT_BUTTON_4),
        );
        self.globals.declare(
            "vbapplicationmodal",
            VBVariant::from_long(VB_APPLICATION_MODAL),
        );
        self.globals
            .declare("vbsystemmodal", VBVariant::from_long(VB_SYSTEM_MODAL));
        self.globals.declare(
            "vbmsgboxhelpbutton",
            VBVariant::from_long(VB_MSG_BOX_HELP_BUTTON),
        );
        self.globals.declare(
            "vbmsgboxsetforeground",
            VBVariant::from_long(VB_MSG_BOX_SET_FOREGROUND),
        );
        self.globals
            .declare("vbmsgboxright", VBVariant::from_long(VB_MSG_BOX_RIGHT));
        self.globals.declare(
            "vbmsgboxrtlreading",
            VBVariant::from_long(VB_MSG_BOX_RTL_READING),
        );

        // Message-box result constants (`VbMsgBoxResult`).
        self.globals.declare("vbok", VBVariant::from_long(VB_OK));
        self.globals
            .declare("vbcancel", VBVariant::from_long(VB_CANCEL));
        self.globals
            .declare("vbabort", VBVariant::from_long(VB_ABORT));
        self.globals
            .declare("vbretry", VBVariant::from_long(VB_RETRY));
        self.globals
            .declare("vbignore", VBVariant::from_long(VB_IGNORE));
        self.globals.declare("vbyes", VBVariant::from_long(VB_YES));
        self.globals.declare("vbno", VBVariant::from_long(VB_NO));

        // Shell window-style constants (`VbAppWinStyle`).
        self.globals
            .declare("vbhide", VBVariant::from_long(VB_HIDE));
        self.globals
            .declare("vbnormalfocus", VBVariant::from_long(VB_NORMAL_FOCUS));
        self.globals
            .declare("vbminimizedfocus", VBVariant::from_long(VB_MINIMIZED_FOCUS));
        self.globals
            .declare("vbmaximizedfocus", VBVariant::from_long(VB_MAXIMIZED_FOCUS));
        self.globals
            .declare("vbnormalnofocus", VBVariant::from_long(VB_NORMAL_NO_FOCUS));
        self.globals.declare(
            "vbminimizednofocus",
            VBVariant::from_long(VB_MINIMIZED_NO_FOCUS),
        );
    }
}
