use dcf::*;

pub struct SimpleStruct {
    val_one: bool,
    val_two: u32,
    val_three: String,
    val_four: Vec<u8>,
}

impl Serialize for SimpleStruct {
    fn serialize(&self, writer: &mut Writer) {
        self.val_one.serialize(writer);
        self.val_two.serialize(writer);
        self.val_three.serialize(writer);
        self.val_four.serialize(writer);
    }
}

impl<'a> Deserialize<'a> for SimpleStruct {
    fn collect(reader: &mut Reader<'a>) -> Result<Self>
    where
        Self: Sized,
    {
        let val_one = reader.dump::<bool>()?;
        let val_two = reader.dump::<u32>()?;
        let val_three = reader.dump::<String>()?;
        let val_four = reader.dump::<Vec<u8>>()?;
        //let val_four = reader.dump
        todo!()
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
