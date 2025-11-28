use crate::field_description::Description;

pub struct Descriptor<'a, T>(Box<dyn Fn(&T) -> Description<'_> + 'a>);

impl<'a, T, F: Fn(&T) -> Description<'_> + 'a> From<F> for Descriptor<'a, T> {
	fn from(f: F) -> Self {
		todo!()
	}
}

impl<'a, T> Descriptor<'a, T> {
	pub fn describe<'b>(&self, t: &'b T) -> Description<'b> {
		todo!()
	}
}
