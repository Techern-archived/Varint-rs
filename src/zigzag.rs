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

#[cfg(test)]
mod tests {

    use super::ZigZag;

    #[test]
    fn test_u8_i8_zigzag() {
        let mut unsigned: u8 = 0u8;

        assert_eq!(0i8, unsigned.zigzag());

        unsigned = 1;

        assert_eq!(-1i8, unsigned.zigzag());

        unsigned = 2;

        assert_eq!(1i8, unsigned.zigzag());

        let mut signed: i8 = 0i8;

        assert_eq!(0u8, signed.zigzag());

        signed = -1i8;

        assert_eq!(1u8, signed.zigzag());

        signed = 1i8;

        assert_eq!(2u8, signed.zigzag());

    }

    #[test]
    fn test_u16_i16_zigzag() {
        let mut unsigned: u16 = 0u16;

        assert_eq!(0i16, unsigned.zigzag());

        unsigned = 1;

        assert_eq!(-1i16, unsigned.zigzag());

        unsigned = 2;

        assert_eq!(1i16, unsigned.zigzag());

        let mut signed: i16 = 0i16;

        assert_eq!(0u16, signed.zigzag());

        signed = -1i16;

        assert_eq!(1u16, signed.zigzag());

        signed = 1i16;

        assert_eq!(2u16, signed.zigzag());

    }

    #[test]
    fn test_u32_i32_zigzag() {
        let mut unsigned: u32 = 0u32;

        assert_eq!(0i32, unsigned.zigzag());

        unsigned = 1;

        assert_eq!(-1i32, unsigned.zigzag());

        unsigned = 2;

        assert_eq!(1i32, unsigned.zigzag());

        let mut signed: i32 = 0i32;

        assert_eq!(0u32, signed.zigzag());

        signed = -1i32;

        assert_eq!(1u32, signed.zigzag());

        signed = 1i32;

        assert_eq!(2u32, signed.zigzag());

    }

    #[test]
    fn test_u64_i64_zigzag() {
        let mut unsigned: u64 = 0u64;

        assert_eq!(0i64, unsigned.zigzag());

        unsigned = 1;

        assert_eq!(-1i64, unsigned.zigzag());

        unsigned = 2;

        assert_eq!(1i64, unsigned.zigzag());

        let mut signed: i64 = 0i64;

        assert_eq!(0u64, signed.zigzag());

        signed = -1i64;

        assert_eq!(1u64, signed.zigzag());

        signed = 1i64;

        assert_eq!(2u64, signed.zigzag());

    }

}

#[cfg(feature = "nightly")]
#[cfg(test)]
mod benchmarks {

    use super::ZigZag;

    extern crate test;
    use self::test::Bencher;

    #[bench]
    fn bench_i8_zigzag(b: &mut Bencher) {
        b.iter(|| (-15i8).zigzag())
    }

    #[bench]
    fn bench_u8_zigzag(b: &mut Bencher) {
        b.iter(|| 224u8.zigzag())
    }

    #[bench]
    fn bench_i16_zigzag(b: &mut Bencher) {
        b.iter(|| 27424i16.zigzag())
    }

    #[bench]
    fn bench_u16_zigzag(b: &mut Bencher) {
        b.iter(|| 22447u16.zigzag())
    }

    #[bench]
    fn bench_i32_zigzag(b: &mut Bencher) {
        b.iter(|| 22472745i32.zigzag())
    }

    #[bench]
    fn bench_u32_zigzag(b: &mut Bencher) {
        b.iter(|| 22376257u32.zigzag())
    }

    #[bench]
    fn bench_i64_zigzag(b: &mut Bencher) {
        b.iter(|| 234678386538365i64.zigzag())
    }

    #[bench]
    fn bench_u64_zigzag(b: &mut Bencher) {
        b.iter(|| 254724572473468u64.zigzag())
    }

}
