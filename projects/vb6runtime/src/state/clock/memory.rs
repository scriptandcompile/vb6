//! In-memory system clock backend — used where there is no host clock to
//! write to (e.g. wasm). The clock is anchored to a timestamp at
//! construction (or the last `set` call) and advances live in real time
//! from that anchor, so callers see a ticking clock without needing a
//! background timer; `set` re-anchors without ever writing (or being
//! echoed back to) the real system clock.

use super::backend::{ClockBackend, SystemClockError};
use jiff::Timestamp;

/// Advances live in real time from an anchor point; never writes the host
/// clock.
#[derive(Debug, Clone, Copy)]
pub struct MemoryBackend {
    /// The clock's value at `anchored_at`.
    anchor: Timestamp,
    /// The real time `anchor` was recorded at, so [`now`](Self::now) can
    /// keep advancing from it.
    anchored_at: Timestamp,
}

impl MemoryBackend {
    /// Create a new memory clock backend seeded with `start`.
    pub fn new(start: Timestamp) -> Self {
        Self {
            anchor: start,
            anchored_at: Timestamp::now(),
        }
    }
}

impl ClockBackend for MemoryBackend {
    fn now(&self) -> Timestamp {
        self.anchor + (Timestamp::now() - self.anchored_at)
    }

    fn set(&mut self, ts: Timestamp) -> Result<(), SystemClockError> {
        self.anchor = ts;
        self.anchored_at = Timestamp::now();
        Ok(())
    }
}
