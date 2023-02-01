extern crate core;

mod ir;
mod iterator;
mod tables;
mod tests;
mod matcher;
mod serde;

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