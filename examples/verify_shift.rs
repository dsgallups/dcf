use dcf::Writer;

fn main() {
    println!("Verifying that bits are shifted right by 1 after inserting a 0 bit\n");

    // Test with simple byte patterns
    let test_bytes = vec![0b10000000u8, 0b11111111u8, 0b10101010u8, 0b11001100u8];

    for &byte in &test_bytes {
        println!("Testing with byte: {:08b}", byte);

        let mut tiny = Writer::default();

        // Insert a 0 bit (takes 1 bit)
        tiny.insert_packed_byte(0);

        // Insert the test byte
        tiny.insert_shifted_bit(byte, 8);

        let result = tiny.finish();

        // Expected: the byte should be shifted right by 1
        // The first byte should have 7 bits from our test byte
        // The second byte should have the last bit in the MSB position
        let expected_first = byte >> 1;
        let expected_second = (byte & 1) << 7;

        println!("  Input:    {:08b}", byte);
        println!("  Output:   {:08b} {:08b}", result[0], result[1]);
        println!("  Expected: {:08b} {:08b}", expected_first, expected_second);

        if result[0] == expected_first && result[1] == expected_second {
            println!("PASS");
        } else {
            println!("FAIL");
        }
        println!();
    }

    // Test with multiple bytes
    println!("Testing with multiple bytes: [0xFF, 0xAA]");
    let mut tiny = Writer::default();

    tiny.insert_packed_byte(0);

    tiny.insert(vec![0xFF, 0xAA]);

    let result = tiny.finish();

    println!("  Input:    11111111 10101010");
    print!("  Output:   ");
    for byte in &result {
        print!("{:08b} ", byte);
    }
    println!();

    // Expected: all bits shifted right by 1
    // 0 | 1111111 | 1101010 | 10
    // = 01111111 11010101 00000000 (last byte only has 1 bit)
    println!("  Expected: 01111111 11010101 00000000");

    if result[0] == 0b01111111 && result[1] == 0b11010101 && result[2] == 0b00000000 {
        println!("PASS");
    } else {
        println!("FAIL");
    }
}
