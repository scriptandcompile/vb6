//! # `Beep` Statement
//!
//! Emits a standard system beep sound.
//!
//! ## Syntax
//!
//! ```vb
//! Beep
//! ```
//!
//! ## Platform Behavior
//!
//! - **Windows/Linux/macOS**: Writes the terminal bell character (`\x07`) to stderr.
//! - **WASM**: No-op (no audio output).
//!
//! [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/beep-statement)

use crate::state;

/// Implement VB6's `Beep` statement.
///
/// Plays the system beep sound. On native platforms this writes the
/// terminal bell character to stderr; on WASM it is a silent no-op.
pub fn beep() {
    state::interaction::beep();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::test_support::lock_test;

    #[test]
    fn beep_does_not_panic() {
        let _guard = lock_test();
        beep();
    }
}
