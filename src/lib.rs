//! A collection of some useful tools (functions, types, traits, macros).
//!
//! **Crate Features:**
//!
//! - `cfg-if` (enabled by default) enables the [`cfg-if`](https://docs.rs/cfg-if/0.1.7/cfg_if/) crate and re-exports its `cfg_if` macro (which may appear as if it were declared in this crate)
//! - `either` (enabled by default) enables the [`either`](https://docs.rs/either/1.5.2/either/) crate and re-exports it and its `Either` type (which may appear as if it were declared in this crate)
//! - `either_use_std` enables the `use_std` feature of the `either` crate (and enables the `either` crate if that isn't already enabled)
//! - `use_std` disables no_std compatibility, adding the `vec_zip` module, the `hashmap` macro and the `interpolation::bez` function
//! - `tuple_tools` enables the `tuple_tools` module with tuples of length 0 to 25 supported by default
//! - `tuple_tools_50` enables support of tuples up to length 50
//! - `tuple_tools_100` enables support of tuples up to length 100
//! - `tuple_tools_150` enables support of tuples up to length 150
//! - `tuple_tools_200` enables support of tuples up to length 200
//! - `tuple_tools_all` enables support of tuples up to the highest supported length (at the moment that is 200)

#![cfg_attr(not(feature = "use_std"), no_std)]

mod macros;
mod singleton;

/// Bézier interpolation functions (for more info, [click here](https://en.wikipedia.org/wiki/B%C3%A9zier_curve))
pub mod interpolation;
/// Some usefule Functions for converting between tuples of `Vec`s and `Vec`s of tuples
#[cfg(feature = "use_std")]
pub mod vec_zip;

/// Tools for working with tuples in generic code (At the moment, this can't do much)
#[cfg(feature = "tuple_tools")]
pub mod tuple_tools;

pub use self::singleton::Singleton;
#[cfg(feature = "cfg-if")]
pub use cfg_if::cfg_if;
#[cfg(feature = "either")]
pub use either::{self, Either};
