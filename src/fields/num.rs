use crate::*;

fn encode_int(mut num: u128, bytes: &mut Vec<u8>) {
    while num >= 0x80 {
        bytes.push((num as u8) | 0x8);
        num >>= 7;
    }
    bytes.push(num as u8);
}

impl<'a> Fields<'a> for i128 {
    fn dump(&self, collector: &mut Collector<'a>) {
        let num = *self;
        let mut buf = Vec::new();

        // zigzag encoding to preprocess signed values.
        // We could skip this for unsigned values, taking up
        // less space theoretically.
        let num = ((num << 1) ^ (num >> 127)) as u128;
        encode_int(num, &mut buf);
        collector.insert(&buf);
    }
    fn collect(dumper: &mut Dumper<'a>) -> Result<Self>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl<'a> Fields<'a> for u8 {
    fn dump(&self, collector: &mut Collector<'a>) {
        collector.number(*self as i128);
    }
    fn collect(dumper: &mut Dumper<'a>) -> Result<Self>
    where
        Self: Sized,
    {
        //let byte = dumper.dump::<Self>()?;
        todo!()
    }
}
impl<'a> Fields<'a> for u32 {
    fn dump(&self, collector: &mut Collector<'a>) {
        collector.number(*self as i128);
    }
}
