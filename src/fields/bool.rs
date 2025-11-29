use std::iter;

use crate::*;

impl Serialize for bool {
    fn serialize(&self, writer: &mut Writer) {
        writer.insert_packed_byte(*self as u8);
    }
}

impl<'a> Deserialize<'a> for bool {
    fn deserialize(reader: &mut Reader<'a>) -> Result<Self>
    where
        Self: Sized,
    {
        let result = reader.dump_packed_byte(1)?;
        Ok(result != 0)
    }
}
