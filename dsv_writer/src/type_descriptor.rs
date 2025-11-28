use crate::field_description::Description;
use crate::field_descriptor::Descriptor as FieldDescriptor;

pub struct Descriptor<'a, T>(Vec<FieldDescriptor<'a, T>>);

impl<'a, T> Default for Descriptor<'a, T> {
	fn default() -> Self {
		todo!()
	}
}

impl<'a, T> Descriptor<'a, T> {
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
