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
//! The demoscene didn't add more transistors. It removed more assumptions.
//! Staunen doesn't add more FLOPS. It removes the assumption that
//! thinking requires floating point.

pub mod xor;
pub mod popcount;
pub mod majority;
pub mod factorize;
pub mod seal;
pub mod threshold;
