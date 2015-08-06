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

/// Gets the number of bytes required to store an unsigned value in a Varint
pub fn unsigned_varint_bytes_required(input: usize) -> u8 {

    let mut value: u64 = input as u64;
    
    let mut returnable: u8 = 0;
    
    if value == 0 {
        return 1;
    }
    
    while value >= 1 {
        
        value >>= 7;
                
        returnable += 1;
    }
    
    returnable

}
    
///Encodes an unsigned value as a Varint.
pub fn encode_unsigned_varint(input: usize) -> Varint {

    let mut returnable: Varint = Varint { data: Vec::<u8>::new() };
    
    let mut value: u64 = input as u64;
    
    if value == 0 {
        returnable.data.push(0);
        return returnable;
    } else {
        
        while value >= 1 {
            let mut next_byte: u8 = (value & 0b01111111) as u8;
            
            value >>= 7;
            
            if value >= 1 {
                next_byte |= 0b10000000;
            }
        
            returnable.data.push(next_byte);
        }
        
        return returnable;
    }

}

#[cfg(test)]
mod test {

    use super::*;
    
    #[test]
    fn test_varint_bytes_required() {
        let mut abc: usize = 0;
        
        assert_eq!(1, unsigned_varint_bytes_required(abc));
        
        abc = 120;
        
        assert_eq!(1, unsigned_varint_bytes_required(abc));
        
        abc = 128;
        
        assert_eq!(2, unsigned_varint_bytes_required(abc));
        
        abc = 300;
        
        assert_eq!(2, unsigned_varint_bytes_required(abc));
        
        abc = 0b000111111101010110101100;
        
        assert_eq!(3, unsigned_varint_bytes_required(abc));
        
        abc = 0b011111111101010110101100;
        
        assert_eq!(4, unsigned_varint_bytes_required(abc));
        
        abc = 4147110142;
        
        assert_eq!(5, unsigned_varint_bytes_required(abc));
    }
    
    #[test]
    fn test_new_varint_has_no_bytes() {
        
        let abc: Varint = Varint { data: Vec::<u8>::new() };
        
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