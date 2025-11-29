mod bool;
mod iter;
mod num;
mod str;

use crate::*;

pub trait Fields<'a, M = ()> {
    fn dump(&self, collector: &mut Collector);

    fn collect(dumper: &mut Dumper<'a>) -> Result<Self>
    where
        Self: Sized,
    {
        todo!()
    }
    //todo: would probably want to pop off one by one using a byte slice
    //fn from_field_iter(iter: FieldIter<'a>) -> Self;
}

// pub trait Test {
//     fn todo<W: Write>(&self, buf: &mut W);
// }
// impl Test for bool {
//     fn todo<W: Write>(&self, buf: &mut W) {
//         buf
//         //buf.w
//     }
// }
