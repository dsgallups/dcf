fn main() {
    // Simulate inserting byte 0b10000001 at bit position 1
    let shifted = 0b10000001u8;
    let bit_cursor = 1u8;
    let bits_occupied = 8u8;
    
    println!("Original shifted: {:08b}", shifted);
    println!("Bit cursor: {}", bit_cursor);
    
    // What gets put in current byte
    let into_current = shifted >> bit_cursor;
    println!("Into current byte: {:08b} (shifted right by {})", into_current, bit_cursor);
    
    // Calculate remainder
    let new_cursor = bit_cursor + bits_occupied;
    let remainder_bits_occupied = new_cursor - 8;
    println!("Remainder bits occupied: {}", remainder_bits_occupied);
    
    // Current (wrong) calculation
    let wrong_remainder = shifted << remainder_bits_occupied;
    println!("Wrong remainder (shifted << {}): {:08b}", remainder_bits_occupied, wrong_remainder);
    
    // What we actually need: the bits that were shifted out
    // These are the lower `bit_cursor` bits of the original shifted value
    let correct_remainder = shifted << (8 - remainder_bits_occupied);
    println!("Attempt 1 (shifted << {}): {:08b}", 8 - remainder_bits_occupied, correct_remainder);
    
    // Actually, we need the bits that didn't fit
    // When we shift right by bit_cursor, we lose the lower bit_cursor bits
    // We need to preserve those and align them correctly
    let mask = (1u8 << remainder_bits_occupied) - 1;
    let preserved_bits = shifted & mask;
    let correct_remainder2 = preserved_bits << (8 - remainder_bits_occupied);
    println!("Attempt 2 (mask {:08b}, preserved {:08b}, result): {:08b}", 
             mask, preserved_bits, correct_remainder2);
}
