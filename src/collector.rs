use std::borrow::Cow;

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

enum FieldValue<'a> {
    String(Cow<'a, str>),
    Bool(bool),
    Number(i128),
    ArrayStart,
    ArrayEnd,
}
