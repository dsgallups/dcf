use dcf::*;

pub struct SimpleStruct {
    val_one: bool,
    val_two: u32,
    val_three: String,
    val_four: Vec<u8>,
}

impl<'a> Fields<'a> for SimpleStruct {
    fn dump(&self, collector: &mut Collector<'a>) {
        self.val_one.dump(collector);
        self.val_two.dump(collector);
        self.val_three.dump(collector);
        self.val_four.dump(collector);
    }
}

fn main() {
    let simple_struct = SimpleStruct {
        val_one: false,
        val_two: 182,
        val_three: "yoyo".to_string(),
        val_four: vec![2, 2, 1, 3],
    };
    let bytes = dcf::serialize(&simple_struct);

    let struct_copy: SimpleStruct = dcf::deserialize(&bytes).unwrap();
}
