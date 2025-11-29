use crate::primitive_writer::QuoteMode;

#[derive(Debug)]
pub struct Description {
	value: String,
	quoted_mode: QuoteMode,
}

impl Description {
	pub fn new_str(value: &str, quoted_mode: QuoteMode) -> Self {
		Self {
			value: value.to_string(),
			quoted_mode,
		}
	}

	pub fn new_string(value: String, quoted_mode: QuoteMode) -> Self {
		Self { value, quoted_mode }
	}

	pub fn quoted_mode(&self) -> &QuoteMode {
		&self.quoted_mode
	}

	pub fn value(&self) -> &str {
		&self.value
	}
}
