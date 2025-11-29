pub fn encode_int(mut num: u128, bytes: &mut Vec<u8>) {
    while num >= 0x80 {
        bytes.push((num as u8) | 0x8);
        num >>= 7;
    }
    bytes.push(num as u8);
}
