use dcf::*;

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
    assert_eq!(w.current_byte, 0b1011_0100);

    w.insert_bitlen(0b1011);

    assert_eq!(w.stack[0], 0b1011_0110);
    println!("current byte: {:08b}", w.current_byte);
    assert_eq!(w.current_byte, 0b1100_0000);

    let bytes = w.finish();
    let first = bytes[0];
    let second = bytes[1];
    //10110110 11000000
    println!("Success encoding!\n{first:08b} {second:08b}\n\n");

    let mut reader = TinyReader::new(&bytes);

    assert_eq!(1, reader.pull_byte(1).unwrap());
    assert_eq!(0, reader.pull_byte(1).unwrap());
    assert_eq!(1, reader.pull_byte(1).unwrap());
    assert_eq!(2, reader.pull_byte(2).unwrap());
    assert_eq!(1, reader.pull_byte(1).unwrap());
    assert_eq!(0b1011, reader.pull_byte(4).unwrap());
}
