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
    pub fn insert(&mut self, values: &[u8]) {
        todo!()
    }

    // pub fn bool(&mut self, value: bool) {
    //     self.fields.push(FieldValue::Bool(value));
    // }
    // pub fn number(&mut self, number: i128) {
    //     self.fields.push(FieldValue::Number(number))
    // }
    // pub fn string(&mut self, val: &str) {
    //     //ill deal with you later
    //     let val = val.to_string();
    //     self.fields.push(FieldValue::String(val.into()))
    // }
    // pub fn arr_start(&mut self) {
    //     self.fields.push(FieldValue::ArrayStart);
    // }
    // pub fn arr_end(&mut self) {
    //     self.fields.push(FieldValue::ArrayEnd);
    // }
    pub fn finish(self) -> Vec<u8> {
        todo!()
        //let mut result = Vec::new();

        // for field in self.fields {
        //     field.into_bytes(&mut result);
        // }

        // result
    }
}

enum FieldValue<'a> {
    String(Cow<'a, str>),
    Bool(bool),
    Number(i128),
    ArrayStart,
    ArrayEnd,
}

impl<'a> FieldValue<'a> {
    fn into_bytes(self, bytes: &mut Vec<u8>) {
        match self {
            FieldValue::Bool(value) => {
                bytes.push(value as u8);
            }
            FieldValue::Number(num) => {
                // zigzag encoding to preprocess signed values.
                // We could skip this for unsigned values, taking up
                // less space theoretically.
                let num = ((num << 1) ^ (num >> 127)) as u128;
                encode_int(num, bytes);
            }
            FieldValue::String(val) => {
                encode_int(val.len() as u128, bytes);
                bytes.extend(val.as_bytes());
            }
            FieldValue::ArrayStart => {
                bytes.push(b'[');
            }
            FieldValue::ArrayEnd => {
                // we will perform escaping in a bit
                bytes.push(b']');
            }
        }
    }
}
