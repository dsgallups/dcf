use crate::*;

#[derive(Default)]
pub struct TinyWriter {
    pub stack: Vec<u8>,
    pub bit_cursor: u8,
    pub current_byte: u8,
}

/// Returns a bit with information shifted to the left
///
/// (byte, num_valuable_bits)
fn shift(bit: u8) -> (u8, u8) {
    if bit == 0 {
        (0, 1)
    } else {
        let leading_zeroes = bit.leading_zeros() as u8;
        (bit << bit.leading_zeros(), 8 - leading_zeroes)
    }
}

impl TinyWriter {
    pub fn insert_shifted_bit(&mut self, shifted: u8, bits_occupied: u8) {
        // println!(
        //     "inpt: {:08b}, len = {}, cursor: {}",
        //     shifted, bits_occupied, self.bit_cursor
        // );
        self.current_byte |= shifted >> self.bit_cursor;
        //println!("curb: {:08b}", self.current_byte);

        self.bit_cursor += bits_occupied;
        //println!("cursor after insert: {}", self.bit_cursor);

        if self.bit_cursor < 8 {
            return;
        }

        let remainder_bits_occupied = self.bit_cursor - 8;

        // println!("remainder_bits_occupied: {}", remainder_bits_occupied);

        self.stack.push(self.current_byte);
        self.bit_cursor = 0;
        self.current_byte = 0;
        if remainder_bits_occupied == 0 {
            return;
        }

        let remainder_bits = shifted << (bits_occupied - remainder_bits_occupied);
        // println!(
        //     "remainder bits: {:08b}, remainder_bits_occupied: {}",
        //     remainder_bits, remainder_bits_occupied
        // );
        self.insert_shifted_bit(remainder_bits, remainder_bits_occupied);
    }

    pub fn insert_bytes(&mut self, bytes: impl IntoIterator<Item = u8>) {
        for byte in bytes {
            self.insert_shifted_bit(byte, 8);
        }
    }

    pub fn insert_bitlen(&mut self, bit: u8) {
        let (shifted, bits_occupied) = shift(bit);
        self.insert_shifted_bit(shifted, bits_occupied);
        // println!("leading zeros: {}", bit.leading_zeros());

        // println!(
        //     "Bit: {bit:08b}\nRes: {:08b}\nCur: {}\n==========",
        //     self.current_byte, self.bit_cursor
        // );
    }

    pub fn finish(mut self) -> Vec<u8> {
        self.stack.push(self.current_byte);
        self.stack
    }
}

pub struct TinyReader<'a> {
    pub bytes: &'a [u8],
    pub cursor: usize,
    pub bit_cursor: u8,
}
impl<'a> TinyReader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            cursor: 0,
            bit_cursor: 0,
        }
    }
    pub fn pull_byte(&mut self, width: u8) -> Result<u8> {
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
        println!("bit_cursor: {}, width: {}", self.bit_cursor, width);
        println!("re0: {:08b}. Shifting left {from}", result);
        result <<= from;

        println!("re1: {:08b}. Shifting right {}", result, 8 - width);
        result >>= 8 - width;
        println!("re2: {:08b}", result);
        self.bit_cursor += width;
        if self.bit_cursor < 8 {
            println!("===================");
            return Ok(result);
        }

        let remainder_bits_needed = self.bit_cursor - 8;
        println!("{} bits remain\n>>", remainder_bits_needed);
        self.bit_cursor = 0;
        self.cursor += 1;
        let other_bits = self.pull_byte(remainder_bits_needed)?;
        println!(">>");
        let ord = result | other_bits;
        println!(
            "re2: {:08b}\notb: {:08b} =\nord: {:08b}",
            result, other_bits, ord
        );

        //println!("byt: {:08b}\nres: {:08b}", self.current_byte, result);

        println!("===================");
        Ok(ord)
    }
}
