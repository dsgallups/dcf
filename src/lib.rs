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

mod utils;
#[allow(unused_imports)]
pub use utils::*;

#[cfg(test)]
mod tests;

pub use anyhow::{Result, bail};

pub fn to_dcf<S: Serialize<M>, M>(value: &S) -> Vec<u8> {
    let mut collector = Writer::default();
    value.serialize(&mut collector);
    collector.finish()
}

pub fn from_dcf<'a, S: Deserialize<'a, M>, M>(bytes: &'a [u8]) -> Result<S> {
    let mut dumper = Reader::new(bytes);
    S::deserialize(&mut dumper)
}
