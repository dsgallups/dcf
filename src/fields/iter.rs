use crate::*;

pub struct IterField;

impl<'a, I, T> Fields<'a, IterField> for I
where
    I: IntoIterator<Item = T>,
    T: Fields<'a>,
{
    fn dump(&self, collector: &mut Collector) {
        collector.arr_start();

        collector.arr_end();
    }
}
