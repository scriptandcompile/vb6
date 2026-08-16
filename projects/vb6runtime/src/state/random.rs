//! Random-number generator state shared by the `Rnd` function and the
//! `Randomize` statement.
//!
//! Both operate on one process-wide seed. The stored seed is normally kept in
//! `[0, MODULUS)` by `Rnd`, but VB6's `Randomize` splices a value into the
//! middle of the seed and can leave the top byte set, so the full 32-bit value
//! is preserved here. `Rnd` keeps the sequence 24-bit by masking when it
//! advances. VB6 keeps the sequence's low byte in the low 8 bits of the stored
//! seed so that consecutive `Randomize` calls preserve part of the previous
//! seed, which is why re-randomizing with the same value does not repeat a
//! sequence.

use std::sync::atomic::{AtomicU32, Ordering};

use crate::value::VBVariant;

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

/// Process-wide RNG seed, shared by `Rnd` and the `Randomize` statement.
static SEED: AtomicU32 = AtomicU32::new(DEFAULT_SEED);

/// The current RNG seed.
pub fn seed() -> u32 {
    SEED.load(Ordering::Relaxed)
}

/// Replace the RNG seed.
///
/// `Randomize` uses this to reseed the generator. The full value is stored;
/// `Rnd` masks the seed to 24 bits when it advances, and returns the raw seed
/// (divided by `MODULUS`) for `Rnd(0)`.
pub fn set_seed(value: u32) {
    SEED.store(value, Ordering::Relaxed);
}

/// Advance the LCG one step: `seed = (seed * a + c) mod 2^24`.
///
/// The 32-bit multiplier is applied with wrapping arithmetic; the low 24 bits
/// of the product match `(seed * 16,598,013) mod 2^24`, exactly as the VB6
/// runtime computes it.
pub(crate) fn next_seed(seed: u32) -> u32 {
    seed.wrapping_mul(MULTIPLIER).wrapping_add(INCREMENT) & SEED_MASK
}

/// Derive the 24-bit seed from a negative `Single` argument.
///
/// VB6 takes the IEEE-754 bit pattern of the `Single`, adds its top byte to
/// itself, and masks the result to 24 bits. The same negative argument always
/// produces the same seed, which is what makes `Rnd(negative)` repeatable.
pub(crate) fn seed_from_negative(value: f32) -> u32 {
    let bits = value.to_bits() as u64;
    ((bits + (bits >> 24)) & SEED_MASK as u64) as u32
}

/// The normalized `Single` value `seed / 2^24`, the raw form of an `Rnd` result.
pub(crate) fn normalize(seed: u32) -> VBVariant {
    VBVariant::from_single(seed as f32 / MODULUS as f32)
}
