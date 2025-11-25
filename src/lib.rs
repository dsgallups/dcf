#![doc = r#"
Assumptions:
- I can use proc macros
"#]

use std::borrow::Cow;

pub trait Fields<'a, M = ()> {
    fn dump(&self, collector: &mut Collector<'a>);
    //todo: would probably want to pop off one by one using a byte slice
    //fn from_field_iter(iter: FieldIter<'a>) -> Self;
}

#[derive(Default)]
pub struct Collector<'a> {
    fields: Vec<FieldValue<'a>>,
}
impl<'a> Collector<'a> {
    pub fn new(cap: usize) -> Self {
        Self {
            fields: Vec::with_capacity(cap),
        }
    }

    pub fn bool(&mut self, value: bool) {
        self.fields.push(FieldValue::Bool(value));
    }
    pub fn number(&mut self, number: i128) {
        self.fields.push(FieldValue::Number(number))
    }
    pub fn string(&mut self, val: &str) {
        //ill deal with you later
        let val = val.to_string();
        self.fields.push(FieldValue::String(val.into()))
    }
    pub fn arr_start(&mut self) {
        self.fields.push(FieldValue::ArrayStart);
    }
    pub fn arr_end(&mut self) {
        self.fields.push(FieldValue::ArrayEnd);
    }
}

pub enum FieldValue<'a> {
    String(Cow<'a, str>),
    Bool(bool),
    Number(i128),
    ArrayStart,
    ArrayEnd,
}

impl<'a> Fields<'a> for bool {
    fn dump(&self, collector: &mut Collector<'a>) {
        collector.bool(*self)
    }
}

impl<'a> Fields<'a> for u8 {
    fn dump(&self, collector: &mut Collector<'a>) {
        collector.number(*self as i128);
    }
}
impl<'a> Fields<'a> for u32 {
    fn dump(&self, collector: &mut Collector<'a>) {
        collector.number(*self as i128);
    }
}

pub struct StringType;

impl<'a, S> Fields<'a, StringType> for S
where
    S: AsRef<str>,
{
    fn dump(&self, collector: &mut Collector<'a>) {
        let val: &str = self.as_ref();
        collector.string(val);
    }
}

pub struct IterField;

impl<'a, I, T> Fields<'a, IterField> for I
where
    I: IntoIterator<Item = T>,
    T: Fields<'a>,
{
    fn dump(&self, collector: &mut Collector<'a>) {
        collector.arr_start();

        collector.arr_end();
    }
}
