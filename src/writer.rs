#[derive(Default)]
pub struct Writer {
    stack: Vec<u8>,
}

impl Writer {
    pub fn new(cap: usize) -> Self {
        Self {
            stack: Vec::with_capacity(cap),
        }
    }

    pub fn insert(&mut self, iter: impl IntoIterator<Item = u8>) {
        self.stack.extend(iter);
    }

    pub fn finish(self) -> Vec<u8> {
        self.stack
    }
}
