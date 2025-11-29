mod bool;
mod iter;
mod num;
mod str;
pub(crate) mod utils;

use crate::*;

pub trait Serialize<M = ()> {
    fn dump(self, writer: &mut Writer);
}

pub trait Deserialize<'a, M = ()> {
    fn collect(dumper: &mut Reader<'a>) -> Result<Self>
    where
        Self: Sized;
}
