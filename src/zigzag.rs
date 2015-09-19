//! Zig-zag encoding of integral values

/// A trait for enabling zig-zag encoding of various values
pub trait ZigZag<T> {

    /// Encodes this ZigZag-enabled type into the type specified by implementation
    fn zigzag(&self) -> T;

}

impl ZigZag<u8> for i8 {

    /// Encodes this i8 as a zigzagged u8
    fn zigzag(&self) -> u8 {
        ((self << 1) ^ (self >> 7)) as u8
    }

}

impl ZigZag<i8> for u8 {

    /// Encodes this u8 as a zigzagged i8
    fn zigzag(&self) -> i8 {
        ((self >> 1) as i8) ^ (-((self & 1) as i8))
    }

}

impl ZigZag<u16> for i16 {

    /// Encodes this i16 as a zigzagged u16
    fn zigzag(&self) -> u16 {
        ((self << 1) ^ (self >> 15)) as u16
    }

}

impl ZigZag<i16> for u16 {

    /// Encodes this u16 as a zigzagged i16
    fn zigzag(&self) -> i16 {
        ((self >> 1) as i16) ^ (-((self & 1) as i16))
    }

}

impl ZigZag<u32> for i32 {

    /// Encodes this i32 as a zigzagged u32
    fn zigzag(&self) -> u32 {
        ((self << 1) ^ (self >> 31)) as u32
    }

}

impl ZigZag<i32> for u32 {

    /// Encodes this u32 as a zigzagged i32
    fn zigzag(&self) -> i32 {
        ((self >> 1) as i32) ^ (-((self & 1) as i32))
    }

}

impl ZigZag<u64> for i64 {

    /// Encodes this i64 as a zigzagged u64
    fn zigzag(&self) -> u64 {
        ((self << 1) ^ (self >> 63)) as u64
    }

}

impl ZigZag<i64> for u64 {

    /// Encodes this u64 as a zigzagged i64
    fn zigzag(&self) -> i64 {
        ((self >> 1) as i64) ^ (-((self & 1) as i64))
    }

}
