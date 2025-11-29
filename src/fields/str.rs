use crate::*;

pub struct StringType;

impl<S> Serialize<StringType> for S
where
    S: AsRef<str>,
{
    fn dump(self, writer: &mut impl DcfWriter) {
        let val: &str = self.as_ref();
        let mut varint = Vec::new();
        utils::encode_int(val.len() as u128, &mut varint);

        varint.extend(val.as_bytes());

        writer.insert(&varint);
    }
}
