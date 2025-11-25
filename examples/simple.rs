use dcf::*;

pub struct SimpleStruct {
    value: bool,
}

impl<'a> Dcf<'a> for SimpleStruct {
    fn serialize(&self) -> Vec<u8> {
        todo!()
    }
    fn deserialize(bytes: &'a [u8]) -> Self
    where
        Self: 'a,
    {
        todo!()
    }
}

fn main() {}
