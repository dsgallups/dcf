use dcf::{Serialize, TinyWriter, Writer};

fn main() {
    let values = [128usize, 3, 352703, 1820234];
    // let values: [usize; _] = [0b1000_0000];
    for value in values {
        println!("Serializing {value}");
        let mut writer = Writer::new(3);

        value.serialize(&mut writer);
        let values = writer.finish();

        let mut tiny = TinyWriter::default();

        println!("Writer Values:");
        print!("[");
        for value in &values {
            print!("{value:08b} ");
        }
        println!("]\n======");
        tiny.insert_bitlen(0);
        for value in values {
            tiny.insert_shifted_bit(value, 8);
        }

        println!("Tiny Writer Values:");
        print!("[");
        for value in tiny.finish() {
            print!("{value:08b} ");
        }
        println!("]");
        println!("===================\n");
    }
}
