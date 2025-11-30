use crate::quote_mode::QuoteMode;
use crate::raw_encoder::Encoder;
use std::collections::HashSet;
use std::io::Write;

pub struct RawWriter<W> {
	writer: W,
	cnt: usize,
	buffer: String,
	delimiter: char,
	escape_set: HashSet<char>,
}

impl<W: Write> RawWriter<W> {
	pub fn new(writer: W, delimiter: char) -> Self {
		todo!()
	}
}

impl<W: Write> Encoder for RawWriter<W> {
	fn should_quoting(&self, value: &str) -> bool {
		todo!()
	}

	fn write_str_field(
		&mut self,
		value: &str,
		quote_mode: QuoteMode,
	) -> crate::error::Result<usize> {
		todo!()
	}

	fn end_of_record(&mut self, should_flush: bool) -> crate::error::Result<usize> {
		todo!()
	}

	fn cnt(&self) -> usize {
		todo!()
	}
}
