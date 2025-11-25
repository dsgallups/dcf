#![doc = r#"
Assumptions:
- I can use proc macros
"#]

use std::{borrow::Cow, marker::PhantomData};

pub trait IntoFieldIter<'a> {
    fn field_iter(self) -> FieldIter<'a>;
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
    String(Cow<'a, str>),
    Bool(bool),
    Number(i128),
    Array(Vec<FieldValue<'a>>),
}

impl<'a> From<u32> for FieldValue<'a> {
    fn from(value: u32) -> Self {
        Self::Number(value as i128)
    }
}

impl<'a> From<bool> for FieldValue<'a> {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}
impl<'a> From<&'a str> for FieldValue<'a> {
    fn from(value: &'a str) -> Self {
        Self::String(Cow::Borrowed(value))
    }
}

impl<'a> From<&'a String> for FieldValue<'a> {
    fn from(value: &'a String) -> Self {
        Self::String(Cow::Borrowed(value.as_str()))
    }
}

impl<'a> From<String> for FieldValue<'a> {
    fn from(value: String) -> Self {
        Self::String(Cow::Owned(value))
    }
}
