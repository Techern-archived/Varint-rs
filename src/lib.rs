//! An implementation of Google Protobuf's Variable-Length Integers

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
    pub data: Vec<u8>

}

impl Varint {

    /// Gets the number of bytes currently contained by this Varint
    pub fn number_of_bytes(&self) -> usize {
        self.data.len()
    }

}

#[cfg(test)]
mod test {

    use super::*;
    
    #[test]
    fn test_new_varint_has_no_bytes() {
        
        let abc: Varint = Varint { data: Vec::new() };
        
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