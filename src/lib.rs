//! A collection of some (more or less) useful tools (functions, types, traits, macros).
//!
//! **Crate Features:**
//!
//! - `use_std` disables no_std compatibility, adding the `vec_zip` module and the `hashmap` and `hashset` macros
//! - `pow` enables the `pow` module

#![cfg_attr(not(feature = "use_std"), no_std)]
#![warn(missing_docs)]

pub use self::linear::{lerp, MapRange};
pub use self::singleton::Singleton;

mod auto_escape;
#[cfg(feature = "use_std")]
mod iter_split;
mod linear;
mod macros;
mod singleton;

mod into_iter_seal {
    pub trait IntoIterSeal {}

    impl<I: IntoIterator> IntoIterSeal for I {}
}

/// Info about your cargo package
pub mod cargo_tools;

/// Fully generic exponentiation functions
#[cfg(feature = "pow")]
pub mod pow;

/// Useful Functions for converting between tuples of `Vec`s and `Vec`s of tuples
#[cfg(feature = "use_std")]
pub mod vec_zip;

/// Some tools for working with iterators
///
/// Some of these are quite specific but very useful in their respective use cases
pub mod iter_tools {
    pub use crate::auto_escape::*;
    #[cfg(feature = "use_std")]
    pub use crate::iter_split::*;
}
