mod bool;
mod iter;
mod num;
mod str;

use crate::*;

pub trait Serialize<M = ()> {
    fn serialize(&self, writer: &mut Writer);
}

pub trait Deserialize<'a, M = ()> {
    fn collect(reader: &mut Reader<'a>) -> Result<Self>
    where
        Self: Sized;
}
