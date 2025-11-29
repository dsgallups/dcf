#[derive(Default)]
pub struct TinyWriter {
    stack: Vec<u8>,
    current_bit_cursor: u8,
    current_bit: u8,
}

impl TinyWriter {
    pub fn insert_bitlen(&mut self, bit: u8) {
        let mut leading_zeros = bit.leading_zeros() as u8;
        if bit == 0 {
            leading_zeros = 7;
        }
        /*
         000000
         push 10
         100000
         push 0
         100000
         push 1
         100100
         push 1
         100110
        */

        println!(
            "leading_zeroes: {leading_zeros}, cursor: {}",
            self.current_bit_cursor
        );
        let shift = leading_zeros as i8 - self.current_bit_cursor as i8;
        println!("shift: {shift}",);

        self.current_bit |= bit << shift;

        self.current_bit_cursor += 8 - leading_zeros;
        //let cursor = self.current_bit_cursor;

        //self.current_bit_cursor = cursor & (bit << 7);
        //self.current_bit = ();

        println!(
            "Bit: {bit:08b}\nRes: {:08b}\nCur: {}",
            self.current_bit, self.current_bit_cursor
        );
        //panic!("leading zeros: {leading_zeros}");
    }
}

fn main() {
    let mut w = TinyWriter::default();
    w.insert_bitlen(1);
    w.insert_bitlen(0);
    w.insert_bitlen(1);
    w.insert_bitlen(2);
    w.insert_bitlen(1);
    assert_eq!(w.current_bit, 0b1011_0100);
}
