use crate::*;

#[derive(PartialEq, Clone, Debug)]
pub struct SimpleStruct {
    val_one: bool,
    val_two: u32,
    val_three: String,
}

impl Serialize for SimpleStruct {
    fn serialize(&self, writer: &mut Writer) {
        self.val_one.serialize(writer);
        self.val_two.serialize(writer);
        self.val_three.serialize(writer);
    }
}

impl<'a> Deserialize<'a> for SimpleStruct {
    fn deserialize(reader: &mut Reader<'a>) -> Result<Self>
    where
        Self: Sized,
    {
        let val_one = reader.visit::<bool>()?;
        let val_two = reader.visit::<u32>()?;
        let val_three = reader.visit::<String>()?;
        Ok(Self {
            val_one,
            val_two,
            val_three,
        })
    }
}

impl SimpleStruct {
    pub fn new(one: bool, two: u32, three: impl Into<String>) -> Self {
        Self {
            val_one: one,
            val_two: two,
            val_three: three.into(),
        }
    }
}
#[test]
fn test_simple_struct() {
    let val = SimpleStruct::new(false, 1823, "oiwejoi");

    let varint = crate::to_dcf(&val);
    println!("Writer Values:");
    print!("[");
    for value in &varint {
        print!("{value:08b} ");
    }
    let decode: SimpleStruct = crate::from_dcf(&varint).unwrap();

    assert_eq!(val, decode);
}

#[test]
fn test_simple_structs() {
    for val in [
        SimpleStruct::new(false, 1823, "oiwejoi"),
        SimpleStruct::new(true, 0, "0182"),
    ] {
        test_serde(val);
    }
}
