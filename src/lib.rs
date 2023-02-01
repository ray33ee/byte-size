mod ir;
mod iterator;
mod tables;
mod tests;
mod serde;

///Contains the `Builder` struct used to construct and run the compressor/decompressor
pub mod builder;

///Contains all possible error types raised by the decompressor
pub mod error;

///Convenience function to compress using default options
pub use crate::builder::compress;

///Convenience function to compress using default options
pub use crate::builder::decompress;

pub use crate::error::Result;