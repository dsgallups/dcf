use dcf::*;

pub struct SimpleStruct {
    val_one: bool,
    val_two: u32,
    val_three: String,
}

impl<'a> IntoFieldIter<'a> for SimpleStruct {
    fn field_iter(self) -> FieldIter<'a> {
        let mut field_iter = FieldIter::new(3);

        field_iter
            .push(self.val_one)
            .push(self.val_two)
            .push(self.val_three);

        field_iter
    }
}

fn main() {}
