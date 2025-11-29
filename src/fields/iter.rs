use crate::*;

pub struct IterField;

impl<I, T> Serialize<IterField> for I
where
    I: IntoIterator<Item = T>,
    T: Serialize,
{
    fn dump(&self, collector: &mut Writer) {
        collector.arr_start();

        collector.arr_end();
    }
}
