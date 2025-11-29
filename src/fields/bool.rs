use crate::*;

impl<'a> Fields<'a> for bool {
    fn dump(&self, collector: &mut Collector) {
        collector.insert(&[*self as u8]);
    }
    fn collect(dumper: &mut Dumper<'a>) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(dumper.read_exact(1)?[0] != 0)
    }
}
