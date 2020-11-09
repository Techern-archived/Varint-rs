//! An implementation of Google Protobuf's Variable-Length Integers

#![cfg_attr(feature = "nightly", feature(test))]

extern crate bit_utils;

mod zigzag;

pub use zigzag::ZigZag;

mod rawio;

pub use rawio::VarintRead as VarintRead;
pub use rawio::VarintWrite as VarintWrite;

#[cfg(feature = "io-operations")]
extern crate io_operations;

#[cfg(feature = "io-operations")]
mod iooperations;

#[cfg(feature = "io-operations")]
pub use iooperations::VarintReader as VarintReader;
#[cfg(feature = "io-operations")]
pub use iooperations::VarintWriter as VarintWriter;

/// The maximum number of bytes used by a 32-bit Varint
pub const VARINT_32_MAX_BYTES: usize = 5;

/// The maximum number of bytes used by a 64-bit Varint
pub const VARINT_64_MAX_BYTES: usize = 10;
