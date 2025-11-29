use crate::*;

fn encode_int(mut num: u128, bytes: &mut Vec<u8>) {
    while num >= 0x80 {
        bytes.push((num as u8) | 0x8);
        num >>= 7;
    }
    bytes.push(num as u8);
}

impl<'a> Fields<'a> for i128 {
    fn dump(&self, collector: &mut Collector) {
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

macro_rules! prim_field {
    ($prim:ty) => {
        impl<'a> Fields<'a> for $prim {
            fn dump(&self, collector: &mut Collector) {
                (*self as i128).dump(collector);
            }
            fn collect(dumper: &mut Dumper<'a>) -> Result<Self>
            where
                Self: Sized,
            {
                Ok(i128::collect(dumper)? as $prim)
            }
        }
    };
}
prim_field!(u8);
prim_field!(u16);
prim_field!(u32);
prim_field!(u64);
prim_field!(u128);
prim_field!(i8);
prim_field!(i16);
prim_field!(i32);
prim_field!(i64);
