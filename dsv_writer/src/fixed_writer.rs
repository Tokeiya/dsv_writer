use super::primitive_writer::QuoteMode;
use super::primitive_writer::Writer as PrimitiveWriter;

pub trait Writer {
	type W: PrimitiveWriter;
	const N: usize;
}
