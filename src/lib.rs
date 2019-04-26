//! A collection of some useful tools (functions, types, traits, macros).
//!
//! This crate is very lightweight and has no dependencies or cargo features

mod either;
mod macros;
mod singleton;

/// Some usefule Functions for converting between tuples of `Vec`s and `Vec`s of tuples
pub mod vec_zip;

pub use either::Either;
pub use singleton::Singleton;
