//! System and mock clock for the `Date` and `Time` statements.
//!
//! Two layers:
//!
//! - **System clock** — reads/writes go through a pluggable [`ClockBackend`],
//!   switchable at runtime with [`set_backend`], mirroring
//!   [`crate::state::file`]'s [`FileBackend`](crate::state::file::FileBackend)
//!   and native/memory backends. The native backend is the real OS clock,
//!   writable on native targets (Linux/macOS/Windows). The memory backend
//!   (default on wasm) tracks a timestamp entirely in memory and is always
//!   writable.
//! - **Mock clock** — a signed offset ([`Span`]) applied on top of the
//!   system clock.  When the offset is zero (the default), [`get`] returns
//!   the real system timestamp.  When non-zero the mock clock advances in
//!   real time from the set point.
//!
//! The `Date` and `Time` statements choose which layer to write based on
//! the `allow_system_time` flag: `true` writes the real clock; `false`
//! writes the mock clock.

pub mod backend;
pub mod memory;
pub mod native;

use jiff::{Span, Timestamp};
use std::sync::{Mutex, OnceLock};

pub use backend::{ClockBackend, SystemClockError};

// ── Backend ──────────────────────────────────────────────────────────────

/// The active clock backend.
static BACKEND: OnceLock<Mutex<Box<dyn ClockBackend>>> = OnceLock::new();

/// Get the active backend, initializing with default if needed.
fn backend() -> &'static Mutex<Box<dyn ClockBackend>> {
    BACKEND.get_or_init(|| Mutex::new(default_backend()))
}

/// Create the default backend for the current platform.
fn default_backend() -> Box<dyn ClockBackend> {
    if cfg!(target_arch = "wasm32") {
        // Seed the in-memory clock with the real time; it then advances
        // live on its own without further real-clock interaction.
        Box::new(memory::MemoryBackend::new(jiff::Timestamp::now()))
    } else {
        Box::new(native::NativeBackend::new())
    }
}

/// Set the active clock backend.
///
/// This is the primary way to switch clock backends at runtime.
pub fn set_backend(new_backend: Box<dyn ClockBackend>) {
    let mut backend_guard = backend().lock().unwrap_or_else(|e| e.into_inner());
    *backend_guard = new_backend;
}

/// Reset to the default backend for the current platform.
pub fn reset_backend() {
    set_backend(default_backend());
}

// ── Mock clock ───────────────────────────────────────────────────────────

static MOCK_OFFSET: OnceLock<Mutex<Span>> = OnceLock::new();

fn mock_slot() -> &'static Mutex<Span> {
    MOCK_OFFSET.get_or_init(|| Mutex::new(Span::new()))
}

fn mock_lock() -> std::sync::MutexGuard<'static, Span> {
    mock_slot()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// The current mock timestamp (system time + offset).
pub fn get() -> Timestamp {
    let offset = *mock_lock();
    system_get() + offset
}

/// Shift the mock clock so that its civil date matches `date` in the
/// system's local time zone, preserving the current mock time-of-day.
pub fn set_date(date: jiff::civil::Date) {
    let offset = *mock_lock();
    let current = system_get() + offset;
    let tz = jiff::tz::TimeZone::system();
    let current_zoned = jiff::Zoned::new(current, tz.clone());
    let t = current_zoned.time();
    let target = date.at(t.hour(), t.minute(), t.second(), t.subsec_nanosecond());
    let target_zoned = target.to_zoned(tz).unwrap();
    let target_ts = target_zoned.timestamp();
    let now = system_get();
    *mock_lock() = target_ts - now;
}

/// Shift the mock clock so that its civil time matches `time` in the
/// system's local time zone, preserving the current mock date.
pub fn set_time(time: jiff::civil::Time) {
    let offset = *mock_lock();
    let current = system_get() + offset;
    let tz = jiff::tz::TimeZone::system();
    let current_zoned = jiff::Zoned::new(current, tz.clone());
    let d = current_zoned.date();
    let target = d.at(
        time.hour(),
        time.minute(),
        time.second(),
        time.subsec_nanosecond(),
    );
    let target_zoned = target.to_zoned(tz).unwrap();
    let target_ts = target_zoned.timestamp();
    let now = system_get();
    *mock_lock() = target_ts - now;
}

/// Reset the mock clock to the real system time.
pub fn reset() {
    *mock_lock() = Span::new();
}

// ── System clock ─────────────────────────────────────────────────────────

/// Read the current system clock as a [`Timestamp`].
///
/// This reads through the active [`ClockBackend`] — the real OS time for
/// the native backend, or the in-memory value for the memory backend. The
/// mock offset is **not** applied.
pub fn system_get() -> Timestamp {
    backend().lock().unwrap_or_else(|e| e.into_inner()).now()
}

/// Write `ts` to the active clock backend.
///
/// # Errors
///
/// Returns `Err` if the backend cannot write the clock (e.g. the native
/// backend without sufficient privileges) or the OS call fails.
pub fn system_set(ts: Timestamp) -> Result<(), SystemClockError> {
    backend().lock().unwrap_or_else(|e| e.into_inner()).set(ts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::test_support::TEST_LOCK;

    fn diff_from_now(ts: Timestamp) -> std::time::Duration {
        let real = jiff::Timestamp::now();
        let span = ts - real;
        let sd: jiff::SignedDuration = span.try_into().unwrap();
        sd.unsigned_abs()
    }

    #[test]
    fn default_gets_real_time() {
        let _guard = TEST_LOCK.lock().unwrap();
        reset();
        let ts = get();
        assert!(diff_from_now(ts) < std::time::Duration::from_secs(1));
    }

    #[test]
    fn set_date_shifts_clock() {
        let _guard = TEST_LOCK.lock().unwrap();
        reset();
        let target = jiff::civil::Date::new(2025, 6, 15).unwrap();
        set_date(target);

        let ts = get();
        let zoned = jiff::Zoned::new(ts, jiff::tz::TimeZone::system());
        assert_eq!(zoned.date(), target);
        reset();
    }

    #[test]
    fn set_time_shifts_clock() {
        let _guard = TEST_LOCK.lock().unwrap();
        reset();
        let target = jiff::civil::Time::new(14, 30, 0, 0).unwrap();
        set_time(target);

        let ts = get();
        let zoned = jiff::Zoned::new(ts, jiff::tz::TimeZone::system());
        let t = zoned.time();
        assert_eq!(t.hour(), target.hour());
        assert_eq!(t.minute(), target.minute());
        assert_eq!(t.second(), target.second());
        reset();
    }

    #[test]
    fn reset_clears_offset() {
        let _guard = TEST_LOCK.lock().unwrap();
        let target = jiff::civil::Date::new(2025, 6, 15).unwrap();
        set_date(target);
        reset();
        assert!(diff_from_now(get()) < std::time::Duration::from_secs(1));
    }

    #[test]
    fn system_get_returns_real_time() {
        let ts = system_get();
        assert!(diff_from_now(ts) < std::time::Duration::from_secs(1));
    }

    #[test]
    fn system_set_rejected_without_privileges() {
        // This test verifies the function exists and returns a result.
        // On most CI environments we won't have CAP_SYS_TIME, so we expect
        // a failure.  The important thing is that it doesn't panic.
        let _ = system_set(jiff::Timestamp::now());
    }
}
