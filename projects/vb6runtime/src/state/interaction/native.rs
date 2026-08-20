//! Native implementation of user interaction operations.
//!
//! Uses real process environment and OS primitives. Suitable for
//! Windows, Linux, and macOS.

use super::backend::InteractionBackend;

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
}
