use dsv_writer::primitive_writer::QuoteMode;
use std::borrow::Cow;
pub struct FieldDatum<'a> {
	value: Cow<'a, str>,
	quote_mode: QuoteMode,
}

impl<'a> FieldDatum<'a> {
	pub fn new(value: Cow<'a, str>, quote_mode: QuoteMode) -> Self {
		Self { value, quote_mode }
	}

	pub fn value(&self) -> &str {
		&self.value
	}

	pub fn quote_mode(&self) -> QuoteMode {
		self.quote_mode
	}
}
