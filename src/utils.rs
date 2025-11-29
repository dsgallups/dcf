pub(crate) struct IntEncoder {
    num: u128,
}
impl IntEncoder {
    pub fn new(num: u128) -> Self {
        Self { num }
    }
}

impl Iterator for IntEncoder {
    type Item = u8;
    fn size_hint(&self) -> (usize, Option<usize>) {
        //todo
        (0, None)
    }
    fn next(&mut self) -> Option<Self::Item> {
        if self.num == 0 {
            return None;
        }
        if self.num >= 0x80 {
            let res = self.num as u8 | 0x80;
            self.num >>= 7;
            Some(res)
        } else {
            let res = self.num as u8;
            self.num = 0;
            Some(res)
        }
    }
}
