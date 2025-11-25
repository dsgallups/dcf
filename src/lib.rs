use std::marker::PhantomData;

pub trait IntoFieldIter<'a> {
    fn field_iter(&'a self) -> FieldIter<'a>;
}

#[derive(Default)]
pub struct FieldIter<'a> {
    fields: Vec<FieldValue<'a>>,
}
impl<'a> FieldIter<'a> {
    pub fn new(cap: usize) -> Self {
        Self {
            fields: Vec::with_capacity(cap),
        }
    }
    pub fn push(&mut self, value: impl Into<FieldValue<'a>>) -> &mut Self {
        todo!()
    }
}

pub enum FieldValue<'a> {
    String(&'a str),
    Bool(bool),
    Number(i128),
    Array(Vec<FieldValue<'a>>),
}

impl<'a> From<&'a u32> for FieldValue<'a> {
    fn from(value: &'a u32) -> Self {
        Self::Number(*value as i128)
    }
}

impl<'a> From<&'a bool> for FieldValue<'a> {
    fn from(value: &'a bool) -> Self {
        Self::Bool(*value)
    }
}
impl<'a> From<&'a str> for FieldValue<'a> {
    fn from(value: &'a str) -> Self {
        Self::String(value)
    }
}

impl<'a> From<&'a String> for FieldValue<'a> {
    fn from(value: &'a String) -> Self {
        Self::String(value.as_str())
    }
}
