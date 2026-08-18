//! The default, VB6-compatible random backend.
//!
//! Reproduces VB6's own 24-bit linear congruential generator (LCG) bit for
//! bit, including the seed-splicing behavior of the `Randomize` statement.
//!
//! The stored seed is normally kept in `[0, MODULUS)` by `Rnd`, but VB6's
//! `Randomize` splices a value into the middle of the seed and can leave the
//! top byte set, so the full 32-bit value is preserved here; `Rnd` keeps the
//! sequence 24-bit by masking when it advances. VB6 keeps the sequence's low
//! byte in the low 8 bits of the stored seed so that consecutive `Randomize`
//! calls preserve part of the previous seed, which is why re-randomizing with
//! the same value does not repeat a sequence.

use std::any::Any;
use std::sync::atomic::{AtomicU32, Ordering};

use crate::value::VBVariant;

use super::backend::RandomBackend;

/// LCG multiplier `a` = 16,598,013.
///
/// VB6 stores this as the 32-bit constant `0x43FD43FD`; only its low 24 bits
/// (`0xFD43FD` = 16,598,013) affect the result because the generator keeps the
/// seed modulo 2^24.
const MULTIPLIER: u32 = 0x43FD_43FD;

/// LCG increment `c` = 12,820,163 (`0xC39EC3`).
const INCREMENT: u32 = 0x00C3_9EC3;

/// LCG modulus `m` = 2^24 = 16,777,216. The generator has full period `m`.
pub const MODULUS: u32 = 1 << 24;

/// Bit mask keeping a seed in `[0, MODULUS)`.
const SEED_MASK: u32 = MODULUS - 1;

/// The initial seed used by the VB6 runtime before any `Randomize` statement.
pub const DEFAULT_SEED: u32 = 327_680;

/// Advance the LCG one step: `seed = (seed * a + c) mod 2^24`.
///
/// The 32-bit multiplier is applied with wrapping arithmetic; the low 24 bits
/// of the product match `(seed * 16,598,013) mod 2^24`, exactly as the VB6
/// runtime computes it.
fn next_seed(seed: u32) -> u32 {
    seed.wrapping_mul(MULTIPLIER).wrapping_add(INCREMENT) & SEED_MASK
}

/// Derive the 24-bit seed from a negative `Single` argument.
///
/// VB6 takes the IEEE-754 bit pattern of the `Single`, adds its top byte to
/// itself, and masks the result to 24 bits. The same negative argument always
/// produces the same seed, which is what makes `Rnd(negative)` repeatable.
fn seed_from_negative(value: f32) -> u32 {
    let bits = value.to_bits() as u64;
    ((bits + (bits >> 24)) & SEED_MASK as u64) as u32
}

/// The normalized `Single` value `seed / 2^24`, the raw form of an `Rnd` result.
fn normalize(seed: u32) -> VBVariant {
    VBVariant::from_single(seed as f32 / MODULUS as f32)
}

/// Splice a value into the middle 16 bits of the current seed.
///
/// Matches VB6's `rtRandomize`/`rtRandomizeValue`: the low and high 16-bit
/// words of `bits` are XORed together, shifted up 8 bits, and ORed into
/// `current` while preserving its top and bottom bytes. `bits >> 16` is the
/// C-style arithmetic shift, so a negative value's sign extension survives the
/// XOR and can set the seed's top byte.
fn splice(bits: u32, current: u32) -> u32 {
    let l = bits as i32;
    let mixed = ((l & 0xFFFF) ^ (l >> 16)) as u32;
    (current & 0xFF00_00FF) | (mixed << 8)
}

/// The default random backend, reproducing VB6's own LCG.
pub struct ClassicBackend {
    seed: AtomicU32,
}

impl ClassicBackend {
    /// Create a backend starting from VB6's default seed.
    pub fn new() -> Self {
        Self::with_seed(DEFAULT_SEED)
    }

    /// Create a backend starting from a specific raw seed.
    pub fn with_seed(seed: u32) -> Self {
        Self {
            seed: AtomicU32::new(seed),
        }
    }

    /// The current raw seed.
    pub fn seed(&self) -> u32 {
        self.seed.load(Ordering::Relaxed)
    }

    /// Replace the raw seed.
    pub fn set_seed(&self, value: u32) {
        self.seed.store(value, Ordering::Relaxed);
    }
}

impl Default for ClassicBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl RandomBackend for ClassicBackend {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn next(&mut self) -> VBVariant {
        let next = next_seed(self.seed());
        self.set_seed(next);
        normalize(next)
    }

    fn current(&self) -> VBVariant {
        normalize(self.seed())
    }

    fn seed_from_rnd_argument(&mut self, value: f32) -> VBVariant {
        let seeded = seed_from_negative(value);
        let next = next_seed(seeded);
        self.set_seed(next);
        normalize(next)
    }

    fn randomize(&mut self, bits: u32) {
        let current = self.seed();
        self.set_seed(splice(bits, current));
    }
}
