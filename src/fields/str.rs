use crate::*;

pub struct StringType;

impl<'a, S> Fields<'a, StringType> for S
where
    S: AsRef<str>,
{
    fn dump(&self, collector: &mut Writer) {
        let val: &str = self.as_ref();
        collector.string(val);
    }
}
