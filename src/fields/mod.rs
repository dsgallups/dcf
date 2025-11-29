mod bool;
mod iter;
mod num;
mod str;

use crate::*;

pub trait Fields<'a, M = ()> {
    fn dump(&self, collector: &mut Writer);

    fn collect(dumper: &mut Dumper<'a>) -> Result<Self>
    where
        Self: Sized,
    {
        todo!()
    }
}
