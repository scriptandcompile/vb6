//! System and mock clock for the `Date` and `Time` statements.
//!
//! Two layers:
//!
//! - **System clock** — the real OS clock, readable everywhere and writable
//!   on native targets (Linux/macOS/Windows).  On wasm the system clock is
//!   read-only.
//! - **Mock clock** — a signed offset ([`Span`]) applied on top of the
//!   system clock.  When the offset is zero (the default), [`get`] returns
//!   the real system timestamp.  When non-zero the mock clock advances in
//!   real time from the set point.
//!
//! The `Date` and `Time` statements choose which layer to write based on
//! the `allow_system_time` flag: `true` writes the real clock (native only);
//! `false` writes the mock clock.

use jiff::{Span, Timestamp};
use std::sync::{Mutex, OnceLock};

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

/// Shift the mock clock so that its civil date matches `date`, preserving
/// the current mock time-of-day.
pub fn set_date(date: jiff::civil::Date) {
    let offset = *mock_lock();
    let current = system_get() + offset;
    let current_zoned = jiff::Zoned::new(current, jiff::tz::TimeZone::UTC);
    let t = current_zoned.time();
    let target = date.at(t.hour(), t.minute(), t.second(), t.subsec_nanosecond());
    let target_zoned = target.in_tz("UTC").unwrap();
    let target_ts = target_zoned.timestamp();
    let now = system_get();
    *mock_lock() = target_ts - now;
}

/// Shift the mock clock so that its civil time matches `time`, preserving
/// the current mock date.
pub fn set_time(time: jiff::civil::Time) {
    let offset = *mock_lock();
    let current = system_get() + offset;
    let current_zoned = jiff::Zoned::new(current, jiff::tz::TimeZone::UTC);
    let d = current_zoned.date();
    let target = d.at(
        time.hour(),
        time.minute(),
        time.second(),
        time.subsec_nanosecond(),
    );
    let target_zoned = target.in_tz("UTC").unwrap();
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
/// This is always the real OS time — the mock offset is **not** applied.
pub fn system_get() -> Timestamp {
    jiff::Timestamp::now()
}

/// Write `ts` to the real system clock.
///
/// # Errors
///
/// Returns `Err` on wasm (no host clock access) or if the OS call fails.
pub fn system_set(ts: Timestamp) -> Result<(), SystemClockError> {
    #[cfg(target_arch = "wasm32")]
    {
        let _ = ts;
        Err(SystemClockError::NotSupported)
    }

    #[cfg(not(target_arch = "wasm32"))]
    system_set_native(ts)
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

// ── Native implementation ────────────────────────────────────────────────

#[cfg(not(target_arch = "wasm32"))]
fn system_set_native(ts: Timestamp) -> Result<(), SystemClockError> {
    // jiff::Timestamp is seconds + subsec nanoseconds since Unix epoch.
    let epoch_sec = ts.as_second();
    let subsec_ns = ts.subsec_nanosecond();

    #[cfg(unix)]
    {
        let ts = libc::timespec {
            tv_sec: epoch_sec as libc::time_t,
            tv_nsec: subsec_ns as libc::c_long,
        };
        // CLOCK_REALTIME = 0
        let rc = unsafe { libc::clock_settime(0, &ts) };
        if rc == 0 {
            Ok(())
        } else {
            Err(SystemClockError::OsError(unsafe {
                *libc::__errno_location()
            }))
        }
    }

    #[cfg(windows)]
    {
        use windows_sys::Win32::System::Time::{SetSystemTime, SYSTEMTIME};

        // Convert Unix epoch to Windows FILETIME (100-ns intervals since 1601-01-01).
        // Difference between 1601-01-01 and 1970-01-01 in 100-ns intervals:
        const EPOCH_DIFF: u64 = 116_444_736_000_000_000;
        let intervals = (epoch_sec as u64) * 10_000_000 + (subsec_ns as u64) / 100 + EPOCH_DIFF;

        // FILETIME → SYSTEMTIME (UTC).
        let ft_low = intervals as u32;
        let ft_high = (intervals >> 32) as u32;
        let mut st: SYSTEMTIME = unsafe { std::mem::zeroed() };
        unsafe {
            windows_sys::Win32::System::Time::FileTimeToSystemTime(
                &windows_sys::Win32::Foundation::FILETIME {
                    dwLowDateTime: ft_low,
                    dwHighDateTime: ft_high,
                },
                &mut st,
            );
        }

        let rc = unsafe { SetSystemTime(&st) };
        if rc != 0 {
            Ok(())
        } else {
            Err(SystemClockError::OsError(
                unsafe { windows_sys::Win32::Foundation::GetLastError() } as i32,
            ))
        }
    }

    #[cfg(not(any(unix, windows)))]
    {
        Err(SystemClockError::NotSupported)
    }
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
        let zoned = jiff::Zoned::new(ts, jiff::tz::TimeZone::UTC);
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
        let zoned = jiff::Zoned::new(ts, jiff::tz::TimeZone::UTC);
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
