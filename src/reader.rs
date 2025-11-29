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

    pub fn dump_byte(&mut self) -> Result<u8> {
        // this branch shortcircuits a lot of logic
        if self.bit_cursor == 0 {
            if self.cursor >= self.bytes.len() {
                bail!("Unexpected end of buffer");
            }
            let value = self.bytes[self.cursor];

            self.cursor += 1;
            Ok(value)
        } else {
            self.dump_packed_bits(8)
        }
    }

    /// this method just inverts the caller of
    /// [`Fields::collect`].
    ///
    /// Useful for reading the proc-macro expansion code.
    pub fn visit<T: Deserialize<'a>>(&mut self) -> Result<T> {
        T::deserialize(self)
    }

    pub fn dump_packed_bits(&mut self, width: u8) -> Result<u8> {
        if width == 0 {
            return Ok(0);
        }
        if self.bytes.is_empty() || self.cursor == self.bytes.len() {
            bail!("Unexpectend end of buffer");
        }
        let mut result = self.bytes[self.cursor];

        let from = self.bit_cursor;
        result <<= from;
        result >>= 8 - width;
        self.bit_cursor += width;
        if self.bit_cursor < 8 {
            return Ok(result);
        }

        let remainder_bits_needed = self.bit_cursor - 8;
        self.bit_cursor = 0;
        self.cursor += 1;
        if remainder_bits_needed == 0 {
            return Ok(result);
        }
        let other_bits = self.dump_packed_bits(remainder_bits_needed)?;
        let ord = result | other_bits;

        Ok(ord)
    }
}
