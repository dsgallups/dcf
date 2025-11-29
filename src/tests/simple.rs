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
    fn collect(reader: &mut Reader<'a>) -> Result<Self>
    where
        Self: Sized,
    {
        let val_one = reader.dump::<bool>()?;
        let val_two = reader.dump::<u32>()?;
        let val_three = reader.dump::<String>()?;
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
fn test_simple_structs() {
    for val in [
        SimpleStruct::new(false, 1823, "oiwejoi"),
        SimpleStruct::new(true, 0, "0182"),
    ] {
        test_serde(val);
    }
}
