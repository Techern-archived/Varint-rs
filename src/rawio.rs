//! Integration with the IoOperations library

use bit_utils::BitInformation;

use std::io::{ Error, Read, Write, ErrorKind };

/// Extends the I/O Read trait to provide functions to read (currently only 32-bit) Variable-Length Integers
pub trait VarintRead : Read {

    /// Reads a signed 32-bit Varint from this VarintRead
    fn read_signed_varint_32(&mut self) -> Result<i32, Error> {

        use zigzag::ZigZag;

        match self.read_unsigned_varint_32() {
            Ok(value) => {
                return Ok(value.zigzag());
            }

            Err(error) => {
                return Err(error);
            }
        }
    }

    /// Reads an unsigned 32-bit Varint from this VarintReader
    fn read_unsigned_varint_32(&mut self) -> Result<u32, Error> {

        // The number of bits to shift by. <<0, <<7, <<14, etc
        let mut shift_amount: u32 = 0;

        // The decoded value
        let mut decoded_value: u32 = 0;

        let mut raw_buffer = vec![0u8; 1];

        let mut next_byte: u8 = 0;

        loop {

            match self.read(&mut raw_buffer) {

                Ok(count) => {
                    if count == 1 {
                        next_byte = raw_buffer[0];
                    } else {
                        return Err(Error::new(ErrorKind::Other, "Could not read one byte (end of stream?)"));
                    }
                },
                Err(error) => {
                    return Err(error);
                }

            }

            decoded_value |= ((next_byte & 0b01111111) as u32) << shift_amount;
            // See if we're supposed to keep reading
            if next_byte.has_most_signifigant_bit() {
                shift_amount += 7;
            } else {
                return Ok(decoded_value);
            }

        }

    }

}

/// Extends the I/O Write trait to provide functions for writing (currently only 32-bit) variable-length integers
pub trait VarintWrite : Write {

    /// Writes a signed varint 32 to this VarintWrite
    fn write_signed_varint_32(&mut self, value: i32) -> Result<(), Error> {

        use zigzag::ZigZag;

        self.write_unsigned_varint_32(value.zigzag())
    }

    /// Writes an unsigned 32-bit Varint to this VarintWrite
    fn write_unsigned_varint_32(&mut self, value: u32) -> Result<(), Error> {

        let mut _value: u32 = value;

        if value == 0 {
            let raw_buffer = vec![0];

            // Reassign to a buffer of raw u8s
            let raw_buffer: &[u8] = &raw_buffer[..];

            return self.write_all(raw_buffer);
        } else {

            while _value >= 0b10000000 {

                let next_byte: u8 = ((_value & 0b01111111) as u8) | 0b10000000;

                _value = _value >> 7;

                let raw_buffer = vec![next_byte];

                // Reassign to a buffer of raw u8s
                let raw_buffer: &[u8] = &raw_buffer[..];

                let temp = self.write_all(raw_buffer);

                if temp.is_err() {
                    return temp;
                }

            }

            let raw_buffer = vec![(_value & 0b01111111) as u8];

            // Reassign to a buffer of raw u8s
            let raw_buffer: &[u8] = &raw_buffer[..];

            return self.write_all(raw_buffer);

        }

    }
}

impl VarintRead for ::std::io::Cursor<Vec<u8>> { }
impl VarintRead for ::std::net::TcpStream { }
impl VarintWrite for ::std::io::Cursor<Vec<u8>> { }
impl VarintWrite for ::std::net::TcpStream { }


#[cfg(test)]
mod test {

    use super::{ VarintRead, VarintWrite };

    use std::io::Cursor;

    #[test]
    fn test_read_write_unsigned_varint_32() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.write_unsigned_varint_32(15).is_ok());
        assert!(vector.write_unsigned_varint_32(0).is_ok());
        assert!(vector.write_unsigned_varint_32(2111111111).is_ok());
        assert!(vector.write_unsigned_varint_32(3463465).is_ok());

        vector.set_position(0);

        assert_eq!(15, vector.read_unsigned_varint_32().unwrap());
        assert_eq!(0, vector.read_unsigned_varint_32().unwrap());
        assert_eq!(2111111111, vector.read_unsigned_varint_32().unwrap());
        assert_eq!(3463465, vector.read_unsigned_varint_32().unwrap());


    }

    #[test]
    fn test_read_write_signed_varint_32() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        assert!(vector.write_signed_varint_32(-4).is_ok());
        assert!(vector.write_signed_varint_32(0).is_ok());
        assert!(vector.write_signed_varint_32(2111111111).is_ok());
        assert!(vector.write_signed_varint_32(-2111111111).is_ok());
        assert!(vector.write_signed_varint_32(-3463465).is_ok());

        vector.set_position(0);

        assert_eq!(-4, vector.read_signed_varint_32().unwrap());
        assert_eq!(-0, vector.read_signed_varint_32().unwrap());
        assert_eq!(2111111111, vector.read_signed_varint_32().unwrap());
        assert_eq!(-2111111111, vector.read_signed_varint_32().unwrap());
        assert_eq!(-3463465, vector.read_signed_varint_32().unwrap());


    }

    #[test]
    fn test_read_write_lots_of_unsigned_varint_32() {

        let mut vector = Cursor::new(vec![0u8; 0]);

        let mut index = 0;

        while index < 123456 {
            index += 1;

            assert!(vector.write_unsigned_varint_32(index).is_ok());
        }

        vector.set_position(0);
        index = 0;

        while index < 123456 {
            index += 1;

            assert_eq!(index, vector.read_unsigned_varint_32().unwrap());
        }

    }

}
