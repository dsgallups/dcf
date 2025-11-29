use dcf::TinyWriter;

fn main() {
    println!("Testing TinyWriter bit shifting behavior\n");

    // Test 1: Insert 1 bit (0), then a byte
    println!("=== Test 1: Insert 0 bit, then byte 0b10000001 ===");
    let mut tiny = TinyWriter::default();

    // Insert a 0 bit (takes 1 bit of space)
    tiny.insert_bitlen(0);
    println!("After inserting 0 bit:");
    println!("  current_byte: {:08b}", tiny.current_byte);
    println!("  bit_cursor: {}", tiny.bit_cursor);

    // Now insert byte 0b10000001
    let test_byte = 0b10000001u8;
    println!("\nInserting byte: {:08b}", test_byte);

    // Manually trace what should happen:
    println!("\nExpected behavior:");
    println!("  - We're at bit position 1");
    println!(
        "  - Shifting {:08b} right by 1 gives: {:08b}",
        test_byte,
        test_byte >> 1
    );
    println!(
        "  - This goes into current byte: {:08b} | {:08b} = {:08b}",
        tiny.current_byte,
        test_byte >> 1,
        tiny.current_byte | (test_byte >> 1)
    );
    println!("  - We have 1 + 8 = 9 bits total, so 1 bit overflows");
    println!(
        "  - The overflow bit is the rightmost bit of the original byte: {}",
        test_byte & 1
    );
    println!(
        "  - This bit should be placed at the leftmost position of next byte: {:08b}",
        (test_byte & 1) << 7
    );

    tiny.insert_shifted_bit(test_byte, 8);

    let result = tiny.finish();
    println!("\nActual result:");
    print!("  [");
    for byte in &result {
        print!("{:08b} ", byte);
    }
    println!("]");

    println!("\nExpected: [01000000 10000000]");

    // Test 2: More complex example
    println!("\n=== Test 2: Insert 0 bit, then bytes [128, 1] (varint for 128) ===");
    let mut tiny = TinyWriter::default();

    tiny.insert_bitlen(0);
    tiny.insert_bytes(vec![0b10000000, 0b00000001]);

    let result = tiny.finish();
    println!("Result:");
    print!("  [");
    for byte in &result {
        print!("{:08b} ", byte);
    }
    println!("]");

    // The expected result should be all bits shifted right by 1
    // Original: 10000000 00000001
    // Shifted:  01000000 00000000 10000000 (last byte only has 1 bit used)
    println!("Expected: [01000000 00000000 10000000]");

    // Test 3: Insert 3 bits, then a byte
    println!("\n=== Test 3: Insert value 5 (takes 3 bits: 101), then byte 0xFF ===");
    let mut tiny = TinyWriter::default();

    // Value 5 = 0b101, which takes 3 bits
    tiny.insert_bitlen(5); // This will insert 101 at the leftmost position
    println!("After inserting 5 (0b101):");
    println!("  current_byte: {:08b}", tiny.current_byte);
    println!("  bit_cursor: {}", tiny.bit_cursor);

    tiny.insert_shifted_bit(0xFF, 8);
    let result = tiny.finish();

    println!("\nResult:");
    print!("  [");
    for byte in &result {
        print!("{:08b} ", byte);
    }
    println!("]");

    // Expected: 101 followed by 11111111
    // In bits: 10111111 11100000
    println!("Expected: [10111111 11100000]");
}
