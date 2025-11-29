use crate::*;

pub struct StringType;

impl<S> Serialize<StringType> for S
where
    S: AsRef<str>,
{
    fn dump(&self, collector: &mut Writer) {
        todo!()
        // let val: &str = self.as_ref();
        // collector.string(val);
    }
}
