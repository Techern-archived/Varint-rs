use bit_utils::BitInformation;

use io_operations::reader::Reader;
use io_operations::writer::Writer;

use std::io::Error;

pub trait VarintReader : Reader {

    /// Reads a signed 32-bit Varint from this VarintReader
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

        loop {

            match self.read_unsigned_byte() {
                Err(error) => {
                    return Err(error);
                }

                Ok(byte_value) => {
                    decoded_value |= ((byte_value & 0b01111111) as u32) << shift_amount;

                    // See if we're supposed to keep reading
                    if byte_value.has_most_signifigant_bit() {
                        shift_amount += 7;
                    } else {
                        return Ok(decoded_value);
                    }
                }
            }

        }

    }

}

pub trait VarintWriter : Writer {

    /// Writes a signed varint 32 to this VarintWriter
    fn write_signed_varint_32(&mut self, value: i32) -> Result<(), Error> {

        use zigzag::ZigZag;

        self.write_unsigned_varint_32(value.zigzag())
    }

    /// Writes an unsigned 32-bit Varint to this VarintWriter
    ///
    /// TODO: Currently, this returns the Result of the last byte written. We need to roll our own
    /// eventually
    fn write_unsigned_varint_32(&mut self, value: u32) -> Result<(), Error> {

        let mut _value: u32 = value;

        if value == 0 {
            return self.write_unsigned_byte(0)
        } else {

            while _value >= 0b10000000 {

                let next_byte: u8 = ((_value & 0b01111111) as u8) | 0b10000000;

                _value = _value >> 7;

                let temp = self.write_unsigned_byte(next_byte);

                if temp.is_err() {
                    return temp;
                }

            }

            return self.write_unsigned_byte((_value & 0b01111111) as u8);

        }

    }
}

impl VarintReader for ::std::io::Cursor<Vec<u8>> { }
impl VarintReader for ::std::net::TcpStream { }
impl VarintWriter for ::std::io::Cursor<Vec<u8>> { }
impl VarintWriter for ::std::net::TcpStream { }


#[cfg(test)]
mod test {

    use super::{ VarintReader, VarintWriter };

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
