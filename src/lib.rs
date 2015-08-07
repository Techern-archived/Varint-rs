//! An implementation of Google Protobuf's Variable-Length Integers

use std::collections::VecDeque;

/// The maximum number of bytes used by a 32-bit Varint
pub const VARINT_32_MAX_BYTES: u8 = 5;

/// The maximum number of bytes used by a 32-bit Varint
pub const VARINT_64_MAX_BYTES: u8 = 10;

/// Checks to see if the most signifigant bit exists in the specified byte
pub fn most_signifigant_bit_exists(input: u8) -> bool {
    input & 0b10000000 != 0
}

/// A struct defining a variable-length integer
#[derive(Clone, Debug)]
pub struct Varint {

    /// The internal data representation
    pub data: VecDeque<u8>

}

impl Varint {

    /// Gets the number of bytes currently contained by this Varint
    pub fn number_of_bytes(&self) -> usize {
        self.data.len()
    }

}

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

/// Encodes a signed i32 as a Varint
pub fn encode_signed_varint32(input: i32) -> Varint {
    encode_unsigned_varint32(zigzag_signed_int(input))
}

/// Encodes a signed i64 as a Varint
pub fn encode_signed_varint64(input: i64) -> Varint {
    encode_unsigned_varint64(zigzag_signed_long(input))
}
    
/// Encodes an unsigned u32 as a Varint.
pub fn encode_unsigned_varint32(input: u32) -> Varint {

    let mut returnable: Varint = Varint { data: VecDeque::<u8>::new() };
    
    let mut value: u32 = input;
    
    if value == 0 {
        returnable.data.push_back(0);
        return returnable;
    } else {
        
        while value >= 1 {
            let mut next_byte: u8 = (value & 0b01111111) as u8;
            
            value >>= 7;
            
            if value >= 1 {
                next_byte |= 0b10000000;
            }
        
            returnable.data.push_back(next_byte);
        }
        
        return returnable;
    }

}
    
/// Encodes an unsigned u64 as a Varint.
pub fn encode_unsigned_varint64(input: u64) -> Varint {

    let mut returnable: Varint = Varint { data: VecDeque::<u8>::new() };
    
    let mut value: u64 = input;
    
    if value == 0 {
        returnable.data.push_back(0);
        return returnable;
    } else {
        
        while value >= 1 {
            let mut next_byte: u8 = (value & 0b01111111) as u8;
            
            value >>= 7;
            
            if value >= 1 {
                next_byte |= 0b10000000;
            }
        
            returnable.data.push_back(next_byte);
        }
        
        return returnable;
    }

}

#[cfg(test)]
mod test {

    use super::*;
    
    use std::collections::VecDeque;
    
    #[test]
    fn test_zigzag_signed_value() {
        let mut signed: i32 = 0;
        
        assert_eq!(signed, zigzag_signed_int(signed) as i32);
        
        signed = -1;
        
        assert_eq!(1, zigzag_signed_int(signed));
        
        signed = 1;
        
        assert_eq!(2, zigzag_signed_int(signed));
        
        signed = -2;
        
        assert_eq!(3, zigzag_signed_int(signed));
        
        let mut signed: i64 = 9223372036854775806;
        
        assert_eq!(18446744073709551612, zigzag_signed_long(signed));
        
        signed = -9223372036854775808;
        
        assert_eq!(18446744073709551615, zigzag_signed_long(signed));
    }
    
    #[test]
    fn test_new_varint_has_no_bytes() {
        
        let abc: Varint = Varint { data: VecDeque::<u8>::new() };
        
        assert_eq!(0, abc.number_of_bytes());
        
    }
    
    #[test]
    fn test_most_signifigant_bit() {
        let mut value: u8 = 1;
        
        assert!(most_signifigant_bit_exists(value) == false);
        
        value = 120;
        
        assert!(most_signifigant_bit_exists(value) == false);
        
        value = 128;
        
        assert!(most_signifigant_bit_exists(value) == true);
        
        value = 129;
        
        assert!(most_signifigant_bit_exists(value) == true);
    }
    
}