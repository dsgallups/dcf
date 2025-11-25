pub trait Dcf<'a> {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(bytes: &'a [u8]) -> Self
    where
        Self: 'a;
}
