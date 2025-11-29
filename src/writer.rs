#[derive(Default)]
pub struct Writer {
    stack: Vec<u8>,
    bit_cursor: u8,
    current_byte: u8,
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

impl Writer {
    pub fn new(cap: usize) -> Self {
        Self {
            stack: Vec::with_capacity(cap),
            bit_cursor: 0,
            current_byte: 0,
        }
    }

    pub fn insert(&mut self, iter: impl IntoIterator<Item = u8>) {
        self.stack.extend(iter);
    }

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

    pub fn insert_packed_byte(&mut self, bit: u8) {
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
