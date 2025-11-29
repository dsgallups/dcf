use crate::*;

impl Serialize for bool {
    fn dump(&self, collector: &mut Writer) {
        collector.insert(&[*self as u8]);
    }
}

impl<'a> Deserialize<'a> for bool {
    fn collect(dumper: &mut Reader<'a>) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(dumper.read_exact(1)?[0] != 0)
    }
}
