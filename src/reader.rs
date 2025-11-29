use std::borrow::Cow;

use anyhow::Error;

use crate::*;

pub struct Reader<'a> {
    cursor: usize,
    bit_cursor: u8,
    // this could be a `Read`
    bytes: &'a [u8],
}
impl<'a> Reader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            cursor: 0,
            bytes,
            bit_cursor: 0,
        }
    }
    pub fn read_exact(&mut self, len: usize) -> Result<Cow<'_, [u8]>> {
        if self.bit_cursor == 0 {
            let slice = &self.bytes[self.cursor..self.cursor + len];
            self.cursor += len;
            Ok(Cow::Borrowed(slice))
        } else {
            let result = (0..len)
                .map(|_| self.dump_byte())
                .collect::<Result<Vec<_>, Error>>()?;

            Ok(Cow::Owned(result))
        }
    }
    /// Dumps the next byte, assuming that it is not packed.
    pub fn dump_byte(&mut self) -> Result<u8> {
        self.dump_packed_bits(8)
        // this should work, but there's a bug, so I'm going to call dump packed bits with a full width.
        // if self.bit_cursor == 0 {
        //     if self.cursor >= self.bytes.len() {
        //         bail!("Unexpected end of buffer");
        //     }
        //     let value = self.bytes[self.cursor];

        //     self.cursor += 1;
        //     Ok(value)
        // } else {
        // }
    }

    /// this method just inverts the caller of
    /// [`Fields::collect`].
    ///
    /// Useful for reading the proc-macro expansion code.
    pub fn visit<T: Deserialize<'a>>(&mut self) -> Result<T> {
        T::deserialize(self)
    }

    pub fn dump_packed_bits(&mut self, width: u8) -> Result<u8> {
        if self.bytes.is_empty() || self.cursor == self.bytes.len() {
            bail!("Unexpectend end of buffer");
        }
        let mut result = self.bytes[self.cursor];

        let from = self.bit_cursor;
        //let to = self.bit_cursor + width;
        /*
        cursor at 3, width is 4
        byte
        10110101
        shift left 3
        10101000
        shift right 4
        00001010

        cursor is now at 7
        */
        // println!("bit_cursor: {}, width: {}", self.bit_cursor, width);
        // println!("re0: {:08b}. Shifting left {from}", result);
        result <<= from;

        // println!("re1: {:08b}. Shifting right {}", result, 8 - width);
        result >>= 8 - width;
        // println!("re2: {:08b}", result);
        self.bit_cursor += width;
        if self.bit_cursor < 8 {
            // println!("===================");
            return Ok(result);
        }

        let remainder_bits_needed = self.bit_cursor - 8;
        // println!("{} bits remain\n>>", remainder_bits_needed);
        self.bit_cursor = 0;
        self.cursor += 1;
        let other_bits = self.dump_packed_bits(remainder_bits_needed)?;
        // println!(">>");
        let ord = result | other_bits;
        // println!(
        //     "re2: {:08b}\notb: {:08b} =\nord: {:08b}",
        //     result, other_bits, ord
        // );

        //println!("byt: {:08b}\nres: {:08b}", self.current_byte, result);

        // println!("===================");
        Ok(ord)
    }
}
