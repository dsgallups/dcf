use crate::*;

pub struct Dumper<'a> {
    cursor: usize,
    inner: &'a [u8],
}
impl<'a> Dumper<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            cursor: 0,
            inner: bytes,
        }
    }
    pub fn dump<T>(&mut self) -> Result<T> {
        todo!()
    }
}
