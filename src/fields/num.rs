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

#[test]
fn serde_i128() {
    utils::test_serde(i128::MAX);
    utils::test_serde(i128::MIN);
    utils::test_serde(3819012083i128);
    utils::test_serde(1239812301847803570212i128);
    utils::test_serde(5i128);
}

macro_rules! prim_field {
    ($prim:ident) => {
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

        pastey::paste! {
            #[test]
            fn  [<serde_ $prim>]() {
                utils::test_serde($prim::MAX);
                utils::test_serde($prim::MIN);
                utils::test_serde(42);
                utils::test_serde(255);
            }
        }
    };
}
prim_field!(u8);
prim_field!(u16);
prim_field!(u32);
prim_field!(u64);
prim_field!(u128);
prim_field!(usize);
prim_field!(i8);
prim_field!(i16);
prim_field!(i32);
prim_field!(i64);
prim_field!(isize);
