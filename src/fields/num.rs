use crate::{utils::IntEncoder, *};

#[test]
fn serde_i128_others() {
    utils::test_serde(i128::MAX);
    utils::test_serde(i128::MIN);
    utils::test_serde(3819012083i128);
    utils::test_serde(1239812301847803570212i128);
    utils::test_serde(5i128);
}

macro_rules! prim_field {
    ($prim:ident, $width:literal) => {
        impl Serialize for $prim {
            fn dump(self, writer: &mut Writer) {
                writer.insert(IntEncoder::new(self as u128));
            }
        }
        impl<'a> Deserialize<'a> for $prim {
            fn collect(reader: &mut Reader<'a>) -> Result<Self>
            where
                Self: Sized,
            {
                let mut result = 0;
                let mut shift = 0;

                loop {
                    let byte = match reader.dump_byte() {
                        Ok(byte) => byte,
                        Err(e) => {
                            if result == 0 {
                                return Ok(0);
                            }
                            return Err(e);
                        }
                    };

                    result |= ((byte & 0x7F) as $prim) << shift;
                    if byte & 0x80 == 0 {
                        break;
                    }
                    shift += 7;
                    if shift >= $width {
                        bail!("Varint too large");
                    }
                }

                Ok(result)
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
macro_rules! prim_field_signed {
    ($prim:ident, $width:literal) => {
        impl Serialize for $prim {
            fn dump(self, writer: &mut Writer) {
                let num = self as i128;
                // zigzag encoding to preprocess signed values.
                // We could skip this for unsigned values, taking up
                // less space theoretically.
                let num = ((num << 1) ^ (num >> 127)) as u128;
                writer.insert(IntEncoder::new(num));
            }
        }
        impl<'a> Deserialize<'a> for $prim {
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

                let result = ((zigzag >> 1) as $prim) ^ -((zigzag & 1) as $prim);
                Ok(result)
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

prim_field!(u8, 8);
prim_field!(u16, 16);
prim_field!(u32, 32);
prim_field!(u64, 64);
prim_field!(u128, 128);
prim_field_signed!(i8, 8);
prim_field_signed!(i16, 16);
prim_field_signed!(i32, 32);
prim_field_signed!(i64, 64);
prim_field_signed!(i128, 128);

// these would be conditionally compiled
prim_field!(usize, 64);
prim_field_signed!(isize, 64);
