#![doc = r#"
Assumptions:
- I can use proc macros
"#]

mod fields;
pub use fields::*;

mod collector;
pub use collector::*;

pub fn serialize<'a, S: Fields<'a>>(value: &S) -> Vec<u8> {
    let mut collector = Collector::default();
    value.dump(&mut collector);
    collector.finish()
}
