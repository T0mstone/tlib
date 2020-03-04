//! A collection of some useful tools (functions, types, traits, macros).
//!
//! **Crate Features:**
//!
//! - `interpolation` enables the `interpolation` module and adds a dependency on [`num_traits`](https://docs.rs/num-traits/0.2.11/num_traits/)
//! - `use_std` disables no_std compatibility, adding the `vec_zip` module, the `hashmap` macro and the `interpolation::bez` function
//! - `tuple_tools` enables the `tuple_tools` module with tuples of length 0 to 25 supported by default
//! - `tuple_tools_50` enables support of tuples up to length 50
//! - `tuple_tools_100` enables support of tuples up to length 100
//! - `tuple_tools_150` enables support of tuples up to length 150
//! - `tuple_tools_200` enables support of tuples up to length 200
//! - `tuple_tools_all` enables support of tuples up to the highest supported length (at the moment that is 200)

#![cfg_attr(not(feature = "use_std"), no_std)]
#![deny(missing_docs)]

mod macros;
mod singleton;

/// Bézier interpolation functions (for more info, see [the Wikipedia article](https://en.wikipedia.org/wiki/B%C3%A9zier_curve) on Bézier curves)
#[cfg(feature = "interpolation")]
pub mod interpolation;

/// Info about your cargo package
pub mod cargo_tools;

/// Some useful Functions for converting between tuples of `Vec`s and `Vec`s of tuples
#[cfg(feature = "use_std")]
pub mod vec_zip;

/// Tools for working with tuples in generic code (At the moment, this can't do much)
#[cfg(feature = "tuple_tools")]
pub mod tuple_tools;

pub use self::singleton::Singleton;
