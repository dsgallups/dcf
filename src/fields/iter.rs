use crate::{utils::IntEncoder, *};

pub struct IterField<T>(T);

impl<I, T> Serialize<IterField<T>> for I
where
    I: IntoIterator<Item = T>,
    T: Serialize,
{
    fn dump(self, writer: &mut Writer) {
        // let slice: &[T] = self.as_ref();

        // let mut varint = Vec::new();
        // utils::encode_int(slice.len() as u128, &mut varint);

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

// struct IterCollector {
//     inner: Vec<u8>,
// }
// impl DcfWriter for IterCollector {
//     fn insert(&mut self, values: impl IntoIterator<Item = u8>) {
//         self.inner.extend(values);
//     }
// }
