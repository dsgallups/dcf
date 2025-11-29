use crate::{utils::IntEncoder, *};

pub struct IterField<T>(T);

impl<I, T> Serialize<IterField<T>> for I
where
    I: IntoIterator<Item = T>,
    T: Serialize,
{
    fn dump(self, writer: &mut Writer) {
        let iter = self.into_iter();
        let (lb, up) = iter.size_hint();
        let cap = up.unwrap_or(lb) + 1;

        let mut inner_writer = ArrayWriter {
            len: 0,
            inner: Writer::new(cap),
        };

        for field in iter {
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
    pub fn insert<M, T: Serialize<M>>(&mut self, val: T) {
        self.len += 1;
        val.dump(&mut self.inner);
    }
}
