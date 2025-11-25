use dcf::*;

pub struct SimpleStruct {
    val_one: bool,
    val_two: u32,
    val_three: String,
    val_four: Vec<u8>,
}

impl<'a> Fields<'a> for SimpleStruct {
    fn dump(self, collector: &mut Collector<'a>) {
        self.val_one.dump(collector);
        self.val_two.dump(collector);
        self.val_three.dump(collector);
        self.val_four.dump(collector);
    }
}

fn main() {}
