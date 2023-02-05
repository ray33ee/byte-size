
//! Byte Size is a library that can compress short strings more efficiently than smaz, at the expense of extra runtime cost.
//!
//! See [readme](https://github.com/ray33ee/byte-size/blob/main/readme.md) for more information

mod ir;
mod iterator;
mod tables;
mod tests;
mod matcher;
mod serde;
mod map;

///Contains the `Builder` struct used to construct `Engine`s
pub mod builder;

///Contains the `Engine` struct used to compress and decompress
pub mod engine;

///Contains all possible error types raised by the decompressor
pub mod error;

///Convenience function to compress using default options
pub use crate::engine::compress;

///Convenience function to compress using default options
pub use crate::engine::decompress;

pub use crate::error::Result;