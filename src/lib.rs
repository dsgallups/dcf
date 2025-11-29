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

pub fn serialize<S: Serialize<M>, M>(value: S) -> Vec<u8> {
    let mut collector = Writer::default();
    value.dump(&mut collector);
    collector.finish()
}

pub fn deserialize<'a, S: Deserialize<'a, M>, M>(bytes: &'a [u8]) -> Result<S> {
    let mut dumper = Reader::new(bytes);
    S::collect(&mut dumper)
}
