#[derive(Default)]
pub struct TinyWriter {
    stack: Vec<u8>,
    current_bit_cursor: u8,
    current_bit: u8,
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
        self.current_bit |= shifted >> self.current_bit_cursor;

        self.current_bit_cursor += bits_occupied;

        if self.current_bit_cursor < 8 {
            return;
        }

        let remainder_bits_occupied = self.current_bit_cursor - 8;

        self.stack.push(self.current_bit);
        self.current_bit_cursor = 0;
        self.current_bit = 0;
        if remainder_bits_occupied == 0 {
            return;
        }

        let remainder_bits = shifted << remainder_bits_occupied;
        self.insert_shifted_bit(remainder_bits, 8 - remainder_bits_occupied);
    }
    pub fn insert_bitlen(&mut self, bit: u8) {
        let (shifted, bits_occupied) = shift(bit);
        self.insert_shifted_bit(shifted, bits_occupied);
        println!("leading zeros: {}", bit.leading_zeros());

        println!(
            "Bit: {bit:08b}\nRes: {:08b}\nCur: {}\n==========",
            self.current_bit, self.current_bit_cursor
        );
    }

    pub fn finish(mut self) -> Vec<u8> {
        self.stack.push(self.current_bit);
        self.stack
    }
}

fn main() {
    let bit: u8 = 0b00101;
    let bit_after = bit << bit.leading_zeros() as u8;

    println!("bit: {:08b}, aft: {:08b}", bit, bit_after);

    let mut w = TinyWriter::default();
    w.insert_bitlen(1);
    w.insert_bitlen(0);
    w.insert_bitlen(1);
    w.insert_bitlen(2);
    w.insert_bitlen(1);
    assert_eq!(w.current_bit, 0b1011_0100);

    w.insert_bitlen(0b1011);

    assert_eq!(w.stack[0], 0b1011_0110);
    assert_eq!(w.current_bit, 0b1100_0000);

    let bytes = w.finish();

    println!("Success encoding! {bytes:?}");
}
