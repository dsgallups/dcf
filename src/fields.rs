use crate::*;

pub trait Fields<'a, M = ()> {
    fn dump(&self, collector: &mut Collector<'a>);

    fn collect(&self, dumper: &mut Dumper<'a>) -> Result<Self>
    where
        Self: Sized,
    {
        todo!()
    }
    //todo: would probably want to pop off one by one using a byte slice
    //fn from_field_iter(iter: FieldIter<'a>) -> Self;
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
    fn collect(&self, dumper: &mut Dumper<'a>) -> Result<Self>
    where
        Self: Sized,
    {
        let byte = dumper.dump::<Self>()?;
        todo!()
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
