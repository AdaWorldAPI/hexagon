//! staunen-core: The 6 RISC instruction kernel.
//!
//! Every cognitive operation in the staunen architecture compiles to
//! sequences of these six instructions. Nothing else is needed.
//! Nothing else is permitted.
//!
//! ```text
//! XOR         bind / unbind           1 cycle (VPXORD)
//! POPCOUNT    distance / similarity   1 cycle (VPOPCNTDQ)
//! MAJORITY    bundle / superpose      O(n) saturating add
//! AND/NOT     2^3 factorization       1 cycle (VPANDD/VPANDND)
//! BLAKE3      seal / verify           ~10 cycles
//! THRESHOLD   σ-band gating           1 cycle (compare)
//! ```
//!
//! Two things ride on the six without being a seventh instruction
//! (`ALPHA_SPEC.md`, `ENCODING_SPEC.md`): the **alpha modifier** — every
//! instruction has a `.α` form that carries a second vector saying which bits
//! are defined — and **ENCODE**, the ingestion path (text → BLAKE3 → i8
//! accumulator → `(data, alpha)`), which is how data enters the substrate,
//! never part of the compute loop.
//!
//! # The carrier
//!
//! One vector is 16 384 bits = 256 × `u64` = 2 048 bytes: [`Vector`], which
//! is `ndarray::simd::Fingerprint<256>` — the stack's canonical binary
//! fingerprint, not a local copy of it. Bit `i` lives at bit `i % 64` of word
//! `i / 64` (little-endian within the word). That order is normative for every
//! module here and for every scalar reference in their tests.
//!
//! # The SIMD rule
//!
//! This crate names exactly one SIMD surface: `ndarray::simd::*`, the
//! polyfill facade. Backend choice (AVX-512 / AVX2 / NEON / scalar) is
//! ndarray's and compile-time; nothing here selects, names, or branches on a
//! backend, and nothing here contains an intrinsic. Where the facade lacks a
//! primitive, the gap is a shopping-list item for ndarray — never a local SIMD
//! path. Domain types that the facade does not re-export yet (`Plane`,
//! `MerkleRoot`, `SigmaGate`, `CrossPlaneVote`) are reached through
//! `ndarray::hpc::*`, which is a type surface, not a SIMD backend.
//!
//! The demoscene didn't add more transistors. It removed more assumptions.
//! Staunen doesn't add more FLOPS. It removes the assumption that
//! thinking requires floating point.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod alpha;
pub mod encode;
pub mod factorize;
pub mod majority;
pub mod popcount;
pub mod seal;
pub mod threshold;
pub mod xor;

pub use ndarray::simd::Fingerprint;

/// Words per vector. 256 × 64 = 16 384 bits.
pub const WORDS: usize = 256;
/// Bits per vector.
pub const BITS: usize = WORDS * 64;
/// Bytes per vector.
pub const BYTES: usize = WORDS * 8;

/// The 16 384-bit carrier every instruction operates on.
///
/// Data and alpha vectors share this one type; an alpha vector is a
/// [`Vector`] read as "bit set = this position is defined".
pub type Vector = Fingerprint<WORDS>;
