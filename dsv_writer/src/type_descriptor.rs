use crate::field_description::Description;
use crate::field_descriptor::FieldDescriptor;

pub struct TypeDescriptor<'a, T>(Vec<FieldDescriptor<'a, T>>);

impl<'a, T> Default for TypeDescriptor<'a, T> {
	fn default() -> Self {
		todo!()
	}
}

impl<'a, T> TypeDescriptor<'a, T> {
	pub fn push<F: Fn(&T) -> Description<'_> + 'a>(&mut self, f: F) -> usize {
		todo!()
	}

	pub fn delete(idx: usize) -> Description<'a> {
		todo!()
	}

	pub fn describe(&self, value: &T) -> Vec<Description> {
		todo!()
	}

	pub fn fill(&self, value: &T, buff: &mut Vec<Description>) -> usize {
		todo!()
	}

	pub fn append(&self, value: &T, buff: &mut Vec<Description>) -> usize {
		todo!()
	}
}
