use dsv_writer::field_description::Description;

pub struct Selector<'a, T> {
	proc: Box<dyn Fn(&T) -> Description<'_> + 'a>,
}

impl<'a, T> Selector<'a, T> {
	pub fn new<F: Fn(&T) -> Description<'_> + 'a>(proc: F) -> Self {
		Self {
			proc: Box::new(proc),
		}
	}

	pub fn select<'b>(&self, t: &'b T) -> Description<'b> {
		(self.proc)(t)
	}
}
