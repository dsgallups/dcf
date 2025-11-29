use std::iter;

use crate::*;

impl Serialize for bool {
    fn dump(self, writer: &mut Writer) {
        writer.insert(iter::once(self as u8));
    }
}

impl<'a> Deserialize<'a> for bool {
    fn collect(reader: &mut Reader<'a>) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(reader.read_exact(1)?[0] != 0)
    }
}
