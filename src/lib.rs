//! An implementation of Google Protobuf's Variable-Length Integers

//extern crate bit_utils;

//use bit_utils::BitInformation;

mod zigzag;

pub use zigzag::{ zigzag_unsigned_int, zigzag_unsigned_long, zigzag_signed_int, zigzag_signed_long };

/// The maximum number of bytes used by a 32-bit Varint
pub const VARINT_32_MAX_BYTES: usize = 5;

/// The maximum number of bytes used by a 32-bit Varint
pub const VARINT_64_MAX_BYTES: usize = 10;
