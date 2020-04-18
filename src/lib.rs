//! A collection of some useful tools (functions, types, traits, macros).
//!
//! **Crate Features:**
//!
//! - `use_std` disables no_std compatibility, adding the `vec_zip` module and the `hashmap` and `hashset` macros

#![cfg_attr(not(feature = "use_std"), no_std)]
#![deny(missing_docs)]

pub use self::linear::{lerp, MapRange};
pub use self::singleton::Singleton;

mod linear;
mod macros;
mod singleton;

/// Info about your cargo package
pub mod cargo_tools;

/// Fully generic exponentiation functions
#[cfg(feature = "pow")]
pub mod pow;

/// Some useful Functions for converting between tuples of `Vec`s and `Vec`s of tuples
#[cfg(feature = "use_std")]
pub mod vec_zip;
