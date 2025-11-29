pub fn encode_int(mut num: u128, bytes: &mut Vec<u8>) {
    while num >= 0x80 {
        bytes.push((num as u8) | 0x8);
        num >>= 7;
    }
    bytes.push(num as u8);
}
// pub fn encode_signed_int(num: i128, bytes: &mut Vec<u8>) {
//     let num = ((num << 1) ^ (num >> 127)) as u128;
//     encode_int(num, bytes);
// }
