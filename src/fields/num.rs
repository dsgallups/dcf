use crate::{utils::IntEncoder, *};

impl Serialize for i128 {
    fn dump(self, writer: &mut Writer) {
        // zigzag encoding to preprocess signed values.
        // We could skip this for unsigned values, taking up
        // less space theoretically.
        let num = ((self << 1) ^ (self >> 127)) as u128;
        writer.insert(IntEncoder::new(num));
    }
}
impl<'a> Deserialize<'a> for i128 {
    fn collect(reader: &mut Reader<'a>) -> Result<Self>
    where
        Self: Sized,
    {
        let mut zigzag = 0;
        let mut shift = 0;

        loop {
            let byte = reader.dump_byte()?;

            zigzag |= ((byte & 0x7F) as u128) << shift;
            if byte & 0x80 == 0 {
                break;
            }
            shift += 7;
            if shift >= 128 {
                bail!("Varint too large");
            }
        }

        let result = ((zigzag >> 1) as i128) ^ -((zigzag & 1) as i128);
        Ok(result)
    }
}

#[cfg(test)]
fn test_i128(num: i128) {
    let varint = serialize(num);
    let decode: i128 = deserialize(&varint).unwrap();

    assert_eq!(num, decode);
}
#[test]
fn serde_i128() {
    test_i128(i128::MAX);
    test_i128(i128::MIN);
    test_i128(3819012083);
    test_i128(1239812301847803570212);
    test_i128(5);
}

macro_rules! prim_field {
    ($prim:ty) => {
        impl Serialize for $prim {
            fn dump(self, writer: &mut Writer) {
                (self as i128).dump(writer);
            }
        }
        impl<'a> Deserialize<'a> for $prim {
            fn collect(reader: &mut Reader<'a>) -> Result<Self>
            where
                Self: Sized,
            {
                Ok(i128::collect(reader)? as $prim)
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
