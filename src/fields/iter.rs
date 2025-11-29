use crate::*;

pub struct IterField;

impl<I, T> Serialize<IterField> for I
where
    I: IntoIterator<Item = T>,
    T: Serialize,
{
    fn dump(&self, writer: &mut impl Writer) {
        //let mut

        //let mut arr_stack = Vec::new();

        todo!()
        // collector.arr_start();

        // collector.arr_end();
    }
}
