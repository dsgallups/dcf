use crate::{utils::IntEncoder, *};

pub struct StringType;

impl<S> Serialize<StringType> for S
where
    S: AsRef<str>,
{
    fn serialize(&self, writer: &mut Writer) {
        let val: &str = self.as_ref();
        writer.insert(IntEncoder::new(val.len() as u128).chain(val.as_bytes().iter().copied()));
    }
}

impl<'a> Deserialize<'a> for String {
    fn collect(reader: &mut Reader<'a>) -> Result<Self>
    where
        Self: Sized,
    {
        let len = reader.dump::<usize>()?;

        let result = reader.read_exact(len)?;
        let result = String::from_utf8(result.to_vec())?;
        Ok(result)
    }
}

#[test]
fn test_strings() {
    for value in ["", "one", "two", "iowe and a space", "\0 lol"] {
        utils::test_serde(value.to_string());
    }
}
