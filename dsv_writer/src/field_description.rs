use crate::primitive_writer::QuoteMode;
use std::borrow::Cow;

#[derive(Debug)]
pub struct Description<'a> {
	value: Cow<'a, str>,
	quoted_mode: QuoteMode,
}

impl<'a> Description<'a> {
	pub fn new(value: Cow<'a, str>, quoted_mode: QuoteMode) -> Self {
		Self { value, quoted_mode }
	}

	pub fn new_str(value: &'a str, quoted_mode: QuoteMode) -> Self {
		Self::new(Cow::Borrowed(value), quoted_mode)
	}

	pub fn new_string(value: String, quoted_mode: QuoteMode) -> Self {
		Self::new(Cow::Owned(value), quoted_mode)
	}

	pub fn quoted_mode(&self) -> &QuoteMode {
		&self.quoted_mode
	}

	pub fn value(&self) -> &str {
		&self.value
	}
}
