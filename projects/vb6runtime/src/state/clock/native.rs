//! Native system clock backend — reads/writes the real OS clock.

use super::backend::{ClockBackend, SystemClockError};
use jiff::Timestamp;

/// Reads and writes the real OS clock.
#[derive(Debug, Default)]
pub struct NativeBackend;

impl NativeBackend {
    /// Create a new native clock backend.
    pub fn new() -> Self {
        Self
    }
}

impl ClockBackend for NativeBackend {
    fn now(&self) -> Timestamp {
        jiff::Timestamp::now()
    }

    fn set(&mut self, ts: Timestamp) -> Result<(), SystemClockError> {
        system_set_native(ts)
    }
}

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

#[cfg(target_arch = "wasm32")]
fn system_set_native(_ts: Timestamp) -> Result<(), SystemClockError> {
    Err(SystemClockError::NotSupported)
}
