//! A collection of some useful tools (functions, types, traits, macros).
//!
//! This crate is very lightweight and has no dependencies or cargo features

mod macros;
mod singleton;
mod union;

pub mod vec;

pub use singleton::Singleton;
pub use union::Union;
