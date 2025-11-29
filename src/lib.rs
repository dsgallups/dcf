#![doc = r#"
Assumptions:
- I can use proc macro crates (syn, proc_macro2)
- I can use `anyhow` for results


A tradeoff design choice:

"#]

mod reader;
pub use reader::*;

mod fields;
pub use fields::*;

mod writer;
pub use writer::*;

pub(crate) use fields::utils;

pub use anyhow::Result;

pub fn serialize<S: Serialize>(value: &S) -> Vec<u8> {
    let mut collector = Writer::default();
    value.dump(&mut collector);
    collector.finish()
}

pub fn deserialize<'a, S: Deserialize<'a>>(bytes: &'a [u8]) -> Result<S> {
    let mut dumper = Reader::new(bytes);
    S::collect(&mut dumper)
}
