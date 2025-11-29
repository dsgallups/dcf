use crate::{utils::IntEncoder, *};

pub struct StringType;

impl<S> Serialize<StringType> for S
where
    S: AsRef<str>,
{
    fn dump(self, writer: &mut Writer) {
        let val: &str = self.as_ref();
        writer.insert(IntEncoder::new(val.len() as u128).chain(val.as_bytes().iter().copied()));
    }
}
