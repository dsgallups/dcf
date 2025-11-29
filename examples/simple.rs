use dcf::*;

#[derive(Debug)]
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
        let val_one = reader.visit::<bool>()?;
        let val_two = reader.visit::<u32>()?;
        let val_three = reader.visit::<String>()?;
        let val_four = reader.visit::<Vec<u8>>()?;
        Ok(Self {
            val_one,
            val_two,
            val_three,
            val_four,
        })
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
    println!("bytes:\n{bytes:?}");

    let struct_copy: SimpleStruct = dcf::deserialize(&bytes).unwrap();

    println!("struct copy:\n{:#?}", struct_copy);
}
