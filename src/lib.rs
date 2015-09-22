//! An implementation of Google Protobuf's Variable-Length Integers

#![cfg_attr(feature = "nightly", feature(test))]

//extern crate bit_utils;

//use bit_utils::BitInformation;

mod zigzag;

pub use zigzag::ZigZag;

/// The maximum number of bytes used by a 32-bit Varint
pub const VARINT_32_MAX_BYTES: usize = 5;

/// The maximum number of bytes used by a 32-bit Varint
pub const VARINT_64_MAX_BYTES: usize = 10;
