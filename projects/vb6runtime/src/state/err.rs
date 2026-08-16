//! Current run-time error state shared by the `Error`/`Error$` functions.
//!
//! VB6's `Error` function without an argument returns the message for the most
//! recent run-time error (the current value of `Err.Number`). The runtime
//! tracks that number here; it starts at 0 (no error), which is what makes the
//! omitted-argument form return a zero-length string until an error occurs.

use std::sync::atomic::{AtomicI32, Ordering};

/// The current `Err.Number`; 0 means no error is current.
static CURRENT_NUMBER: AtomicI32 = AtomicI32::new(0);

/// The number of the most recent run-time error (0 when none).
pub fn current_number() -> i32 {
    CURRENT_NUMBER.load(Ordering::Relaxed)
}

/// Record a run-time error by number, as `Err.Raise` does.
pub fn set_number(number: i32) {
    CURRENT_NUMBER.store(number, Ordering::Relaxed);
}

/// Clear the current error, as `Err.Clear` does.
pub fn clear() {
    CURRENT_NUMBER.store(0, Ordering::Relaxed);
}
