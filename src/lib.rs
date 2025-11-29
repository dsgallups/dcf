#![doc = r#"
Assumptions:
- I can use proc macro crates (syn, proc_macro2)
- I can use `anyhow` for results


A tradeoff design choice:

"#]

mod dumper;
pub use dumper::*;

mod fields;
pub use fields::*;

mod writer;
pub use writer::*;

pub use anyhow::Result;

pub fn serialize<'a, S: Fields<'a>>(value: &S) -> Vec<u8> {
    let mut collector = Writer::default();
    value.dump(&mut collector);
    collector.finish()
}

pub fn deserialize<'a, S: Fields<'a>>(bytes: &'a [u8]) -> Result<S> {
    let mut dumper = Dumper::new(bytes);
    S::collect(&mut dumper)
}
