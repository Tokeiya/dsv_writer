// use crate::field_description::Description;
// use crate::field_descriptor::Descriptor as FieldDescriptor;
//
// pub struct Descriptor<T>(Vec<FieldDescriptor<T>>);
//
// impl<T> Default for Descriptor<T> {
// 	fn default() -> Self {
// 		Self(Vec::new())
// 	}
// }
//
// impl<T> Descriptor<T> {
// 	pub fn push<F: Fn(&T) -> Description<'_>>(&mut self, f: F) -> usize {
// 		self.0.push(FieldDescriptor::from(f));
// 		self.0.len()
// 	}
//
// 	pub fn delete(&mut self, idx: usize) {
// 		todo!()
// 	}
//
// 	pub fn describe(&self, value: &T) -> Vec<Description> {
// 		todo!()
// 	}
//
// 	pub fn fill(&self, value: &T, buff: &mut Vec<Description>) -> usize {
// 		todo!()
// 	}
//
// 	pub fn append(&self, value: &T, buff: &mut Vec<Description>) -> usize {
// 		todo!()
// 	}
// }
