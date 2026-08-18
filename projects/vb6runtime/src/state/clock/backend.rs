//! Trait abstracting over different system clock backends.
//!
//! The mock clock in [`super`] always reads/writes the *system* clock through
//! a pluggable backend, mirroring [`crate::state::file`]'s [`FileBackend`]
//! and [`crate::state::settings`]'s `SettingsBackend`:
//!
//! - **Native** ([`NativeBackend`](super::native::NativeBackend)): the real OS clock, readable
//!   everywhere and writable on native targets (Linux/macOS/Windows).
//! - **Memory** ([`MemoryBackend`](super::memory::MemoryBackend)): anchored to a starting value at
//!   construction and advances live in real time from there; used on
//!   targets with no host clock access (e.g. wasm). Writes just re-anchor
//!   it, never touching the real system clock.
//!
//! [`FileBackend`]: crate::state::file::FileBackend

use jiff::Timestamp;

/// Abstraction over reading and writing the system clock.
///
/// Implementations must be `Send` so the backend can be shared across
/// threads via a `Mutex<Box<dyn ClockBackend>>`.
pub trait ClockBackend: Send {
    /// Read the current system clock.
    fn now(&self) -> Timestamp;

    /// Write `ts` to the system clock.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the backend cannot write the system clock (e.g. no
    /// host clock access) or if the underlying OS call fails.
    fn set(&mut self, ts: Timestamp) -> Result<(), SystemClockError>;
}

/// Errors from system clock operations.
#[derive(Debug, Clone)]
pub enum SystemClockError {
    /// The platform does not support setting the system clock (e.g. wasm).
    NotSupported,
    /// The OS rejected the clock change (insufficient privileges, etc.).
    OsError(i32),
}

impl std::fmt::Display for SystemClockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotSupported => write!(
                f,
                "setting the system clock is not supported on this platform"
            ),
            Self::OsError(code) => write!(f, "system clock set failed with OS error {code}"),
        }
    }
}

impl std::error::Error for SystemClockError {}
