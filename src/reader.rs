use std::io::Cursor;

use crate::*;

pub struct Reader<'a> {
    cursor: usize,
    // this could be a `Read`
    inner: &'a [u8],
}
impl<'a> Reader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            cursor: 0,
            inner: bytes,
        }
    }
    pub fn read_exact(&mut self, len: usize) -> Result<&[u8]> {
        let slice = &self.inner[self.cursor..self.cursor + len];
        self.cursor += len;
        Ok(slice)
    }
    pub fn dump_bool(&mut self) -> Result<bool> {
        let value = self.inner[self.cursor];

        self.cursor += 1;
        Ok(value != 0)
    }
    pub fn dump_str(&mut self) -> Result<String> {
        todo!()
    }

    /// this method just inverts the caller of
    /// [`Fields::collect`].
    ///
    /// Useful for reading the proc-macro expansion code.
    pub fn dump<T: Deserialize<'a>>(&mut self) -> Result<T> {
        T::collect(self)
    }
}
