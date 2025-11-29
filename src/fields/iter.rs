use crate::*;

pub struct Slice<M>(M);

impl<T, M> Serialize<Slice<M>> for [T]
where
    T: Serialize<M>,
{
    fn serialize(&self, writer: &mut Writer) {
        let mut inner_writer = ArrayWriter {
            len: 0,
            inner: Writer::new(self.len()),
        };

        for field in self {
            inner_writer.insert(field);
        }

        writer.insert(IntEncoder::new(inner_writer.len as u128).chain(inner_writer.inner.finish()));
    }
}

impl<'a, T> Deserialize<'a> for Vec<T>
where
    T: Deserialize<'a>,
{
    fn collect(reader: &mut Reader<'a>) -> Result<Self>
    where
        Self: Sized,
    {
        let len = reader.dump::<usize>()?;
        let mut result = Vec::with_capacity(len);
        for _ in 0..len {
            result.push(reader.dump::<T>()?);
        }
        Ok(result)
    }
}

pub struct ArrayWriter {
    len: usize,
    inner: Writer,
}
impl ArrayWriter {
    pub fn insert<M, T: Serialize<M>>(&mut self, val: &T) {
        self.len += 1;
        val.serialize(&mut self.inner);
    }
}
