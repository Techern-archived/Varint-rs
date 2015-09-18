//! An implementation of Google Protobuf's Variable-Length Integers

//extern crate bit_utils;

//use bit_utils::BitInformation;

/// The maximum number of bytes used by a 32-bit Varint
pub const VARINT_32_MAX_BYTES: usize = 5;

/// The maximum number of bytes used by a 32-bit Varint
pub const VARINT_64_MAX_BYTES: usize = 10;

/// Transforms a signed int to an unsigned int via zig-zag transformation
pub fn zigzag_signed_int(input: i32) -> u32 {
    ((input << 1) ^ (input >> 31)) as u32
}

/// Transforms a signed long to an unsigned long via zig-zag transformation
pub fn zigzag_signed_long(input: i64) -> u64 {
    ((input << 1) ^ (input >> 63)) as u64
}

/// Transforms an unsigned int to a signed int via zig-zag transformation
pub fn zigzag_unsigned_int(input: u32) -> i32 {
    ((input >> 1) as i32) ^ (-((input & 1) as i32))
}

/// Transforms an unsignigned long to a signed long via zig-zag transformation
pub fn zigzag_unsigned_long(input: u64) -> i64 {
    ((input >> 1) as i64) ^ (-((input & 1) as i64))
}
