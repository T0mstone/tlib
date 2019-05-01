//! A collection of some useful tools (functions, types, traits, macros).
//!
//! **Crate Features:**
//!
//! - `cfg-if` (enabled by default): enables the [`cfg-if`](https://docs.rs/cfg-if/0.1.7/cfg_if/) crate and re-exports its `cfg_if` macro (which somehow appears as if it were declared in this crate)
//! - `either` (enabled by default): enables the [`either`](https://docs.rs/either/1.5.2/either/) crate and re-exports it and its `Either` type (which somehow appears as if it were declared in this crate)
//! - `either_use_std`: enables the `use_std` feature of the `either` crate
//! - `no_std`: enables no_std compatibility, removing the `vec_zip` module and the `hashmap` macro

#![cfg_attr(feature = "no_std", no_std)]

#[cfg(all(feature = "no_std", feature = "either_use_std"))]
compile_error!("The features \"no_std\" and \"either_use_std\" are mutually exclusive");

mod macros;
mod singleton;

/// Bézier interpolation functions (for more info, [click here](https://en.wikipedia.org/wiki/B%C3%A9zier_curve))
pub mod interpolation;
/// Some usefule Functions for converting between tuples of `Vec`s and `Vec`s of tuples
#[cfg(not(feature = "no_std"))]
pub mod vec_zip;

#[cfg(feature = "cfg-if")]
pub use cfg_if::cfg_if;
#[cfg(feature = "either")]
pub use either::{self, Either};
pub use singleton::Singleton;
