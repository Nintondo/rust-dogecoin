// Rust Dogecoin Library - Written by the rust-bitcoin developers.
// SPDX-License-Identifier: CC0-1.0

//! Cryptography
//!
//! Cryptography related functionality: keys and signatures.
//!

pub mod ecdsa;
pub mod key;
pub mod sighash;
// Contents re-exported in `nintondo_dogecoin::taproot`.
pub(crate) mod taproot;
