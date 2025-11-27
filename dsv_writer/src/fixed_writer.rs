use super::error::Result;
use super::field_descriptor::Descriptor as FieldDescriptor;
use super::primitive_writer::QuoteMode;
use super::primitive_writer::Writer as PrimitiveWriter;
pub trait Writer {
	type W: PrimitiveWriter;
	const N: usize;

	fn writer(&mut self) -> &mut Self::W;

	fn write(&mut self, value: &[FieldDescriptor; Self::N]) -> Result<usize>;
}
