#[cfg(test)]
use std::fmt::Debug;

#[cfg(test)]
use crate::{Deserialize, Serialize};

pub(crate) struct IntEncoder {
    num: Option<u128>,
}
impl IntEncoder {
    pub fn new(num: u128) -> Self {
        Self { num: Some(num) }
    }
}

impl Iterator for IntEncoder {
    type Item = u8;
    fn size_hint(&self) -> (usize, Option<usize>) {
        //todo
        (0, None)
    }
    fn next(&mut self) -> Option<Self::Item> {
        let Some(num) = &mut self.num else {
            return None;
        };
        if *num >= 0x80 {
            let res = *num as u8 | 0x80;
            *num >>= 7;
            Some(res)
        } else {
            let res = *num as u8;
            self.num = None;
            Some(res)
        }
    }
}

#[test]
fn encode_num() {
    let mut writer = crate::Writer::new(1);
    183_u8.serialize(&mut writer);
    let result = writer.finish();
    panic!("result: {result:?}");
}

#[cfg(test)]
pub fn test_serde<T, M1, M2>(val: T)
where
    T: Serialize<M1> + for<'a> Deserialize<'a, M2> + Debug + PartialEq,
{
    let varint = crate::to_dcf(&val);
    let decode: T = crate::from_dcf(&varint).unwrap();

    assert_eq!(val, decode);
}
