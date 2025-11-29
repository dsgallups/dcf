use crate::*;

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

        let inner_writer = Writer::new(cap);

        todo!()
        //inner_writer.inner.push(0);
        // let mut len = 0;
        // for field in iter {
        //     field.dump(&mut iter_collector);
        //     len += 1;
        // }
        //iter_collector.inner[0] = len;

        //writer.insert(iter_collector.inner);
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
