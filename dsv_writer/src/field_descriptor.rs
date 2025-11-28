use crate::field_description::Description;

pub struct FieldDescriptor<'a, T>(Box<dyn Fn(&T) -> Description<'_> + 'a>);

impl<'a, T, F: Fn(&T) -> Description<'_> + 'a> From<F> for FieldDescriptor<'a, T> {
	fn from(f: F) -> Self {
		todo!()
	}
}

impl<'a, T> FieldDescriptor<'a, T> {
	pub fn describe<'b>(&self, t: &'b T) -> Description<'b> {
		todo!()
	}
}
