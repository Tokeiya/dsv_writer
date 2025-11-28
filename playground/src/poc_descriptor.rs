use crate::field_datum::FieldDatum;
use dsv_writer::primitive_writer::QuoteMode;
use std::borrow::Cow;
use std::marker::PhantomData;

pub trait TypeDescriptor<T, const N: usize> {
	fn field_size() -> usize {
		N
	}
	fn describe<'a>(&self, value: &'a T) -> [FieldDatum<'a>; N];

	fn mode(&self) -> QuoteMode {
		QuoteMode::AutoDetect
	}
}

pub struct ToStringDescriptor<T>(PhantomData<T>);

impl<T: ToString> ToStringDescriptor<T> {
	pub fn new() -> Self {
		Self(PhantomData)
	}
}

impl<T: ToString> TypeDescriptor<T, 1> for ToStringDescriptor<T> {
	fn describe<'a>(&self, value: &'a T) -> [FieldDatum<'a>; 1] {
		[FieldDatum::new(
			Cow::Owned(value.to_string()),
			<ToStringDescriptor<T> as TypeDescriptor<T, 1>>::mode(self),
		)]
	}
}
