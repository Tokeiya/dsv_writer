use super::type_descriptor::Descriptor as TypeDescriptor;

pub struct Writer<'a, W, T> {
	writer: W,
	type_descriptor: TypeDescriptor<'a, T>,
}

impl<'a, W, T> Writer<'a, W, T>
where
	W: std::io::Write,
{
	pub fn new(writer: W, type_descriptor: TypeDescriptor<'a, T>) -> Self {
		todo!()
	}
}
