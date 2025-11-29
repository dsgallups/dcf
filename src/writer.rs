#[derive(Default)]
pub struct Writer {
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

impl Writer {
    pub fn new(cap: usize) -> Self {
        Self {
            stack: Vec::with_capacity(cap),
            bit_cursor: 0,
            current_byte: 0,
        }
    }

    pub fn insert_shifted_bit(&mut self, shifted: u8, bits_occupied: u8) {
        self.current_byte |= shifted >> self.bit_cursor;

        self.bit_cursor += bits_occupied;

        if self.bit_cursor < 8 {
            return;
        }

        let remainder_bits_occupied = self.bit_cursor - 8;

        self.stack.push(self.current_byte);
        self.bit_cursor = 0;
        self.current_byte = 0;
        if remainder_bits_occupied == 0 {
            return;
        }

        let remainder_bits = shifted << (bits_occupied - remainder_bits_occupied);
        self.insert_shifted_bit(remainder_bits, remainder_bits_occupied);
    }

    pub fn insert(&mut self, bytes: impl IntoIterator<Item = u8>) {
        for byte in bytes {
            self.insert_shifted_bit(byte, 8);
        }
    }

    pub fn insert_packed_byte(&mut self, bit: u8) {
        let (shifted, bits_occupied) = shift(bit);
        self.insert_shifted_bit(shifted, bits_occupied);
    }

    pub fn finish(mut self) -> Vec<u8> {
        if self.bit_cursor > 0 {
            self.stack.push(self.current_byte);
        }
        self.stack
    }
}
