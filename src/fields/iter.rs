use crate::*;

pub struct IterField<T>(T);

impl<I, T> Serialize<IterField<T>> for I
where
    I: IntoIterator<Item = T>,
    T: Serialize,
{
    fn dump(self, writer: &mut impl DcfWriter) {
        // let slice: &[T] = self.as_ref();

        // let mut varint = Vec::new();
        // utils::encode_int(slice.len() as u128, &mut varint);

        // //let mut iter = self.into_iter();
        // //let (lb, up) = self.size_hint();
        // let mut inner = IterCollector {
        //     inner: Vec::with_capacity(slice.len() + 1),
        // };

        // for field in slice {
        //     field.dump(&mut inner);
        //     //field.
        // }

        //let mut

        //let mut arr_stack = Vec::new();

        todo!()
        // collector.arr_start();

        // collector.arr_end();
    }
}

struct IterCollector {
    inner: Vec<u8>,
}
impl DcfWriter for IterCollector {
    fn insert(&mut self, values: &[u8]) {
        self.inner.extend(values);
    }
}
