use crate::*;

impl Serialize for i128 {
    fn dump(&self, collector: &mut Writer) {
        let num = *self;
        let mut buf = Vec::new();

        // zigzag encoding to preprocess signed values.
        // We could skip this for unsigned values, taking up
        // less space theoretically.
        let num = ((num << 1) ^ (num >> 127)) as u128;
        utils::encode_int(num, &mut buf);
        collector.insert(&buf);
    }
}
impl<'a> Deserialize<'a> for i128 {
    fn collect(dumper: &mut Reader<'a>) -> Result<Self>
    where
        Self: Sized,
    {
        todo!()
    }
}

macro_rules! prim_field {
    ($prim:ty) => {
        impl Serialize for $prim {
            fn dump(&self, collector: &mut Writer) {
                (*self as i128).dump(collector);
            }
        }
        impl<'a> Deserialize<'a> for $prim {
            fn collect(dumper: &mut Reader<'a>) -> Result<Self>
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
