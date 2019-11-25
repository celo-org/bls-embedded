//! # `bls12_377`
//!
//! This crate provides an implementation of the BLS12-377 pairing-friendly elliptic
//! curve construction.
//!
//! * **This implementation has not been reviewed or audited. Use at your own risk.**
//! * This implementation targets Rust `1.36` or later.
//! * This implementation does not require the Rust standard library.
//! * All operations are constant time unless explicitly noted.

#![no_std]
#![deny(missing_debug_implementations)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::many_single_char_names)]
// This lint is described at
// https://rust-lang.github.io/rust-clippy/master/index.html#suspicious_arithmetic_impl
// In our library, some of the arithmetic involving extension fields will necessarily
// involve various binary operators, and so this lint is triggered unnecessarily.
#![allow(clippy::suspicious_arithmetic_impl)]

#[cfg(test)]
#[macro_use]
extern crate std;

#[macro_use]
pub mod util;

mod scalar;

pub use scalar::Scalar;

#[cfg(feature = "groups")]
pub mod fp;
#[cfg(feature = "groups")]
mod fp2;
#[cfg(feature = "groups")]
mod g1;
#[cfg(feature = "groups")]
mod g2;

#[cfg(feature = "groups")]
pub use g1::{G1Affine, G1Projective};
#[cfg(feature = "groups")]
pub use g2::{G2Affine, G2Projective};

// TODO: This should be upstreamed to subtle.
// See https://github.com/dalek-cryptography/subtle/pull/48
trait CtOptionExt<T> {
    /// Calls f() and either returns self if it contains a value,
    /// or returns the output of f() otherwise.
    fn or_else<F: FnOnce() -> subtle::CtOption<T>>(self, f: F) -> subtle::CtOption<T>;
}

impl<T: subtle::ConditionallySelectable> CtOptionExt<T> for subtle::CtOption<T> {
    fn or_else<F: FnOnce() -> subtle::CtOption<T>>(self, f: F) -> subtle::CtOption<T> {
        let is_none = self.is_none();
        let f = f();

        subtle::ConditionallySelectable::conditional_select(&self, &f, is_none)
    }
}
