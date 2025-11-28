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
    pub fn dump_bool(&mut self) -> Result<bool> {
        let value = self.inner[self.cursor];

        self.cursor += 1;
        Ok(value != 0)
    }
    pub fn dump_str(&mut self) -> Result<String> {
        todo!()
    }

    pub fn dump<T>(&mut self) -> Result<T> {
        todo!()
    }
}
