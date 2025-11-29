use super::error::Result;
use crate::quote_mode::QuoteMode;
use std::borrow::Cow;

pub type StrCow<'a> = Cow<'a, str>;

pub enum EscapeOutcome {
	NotEscaped(char),
	DuplicatedQuote,
}

pub trait Encoder<const D: char, const N: usize> {
	fn classify_char(&self, value: char) -> EscapeOutcome;
	fn should_quoting(&self, value: StrCow) -> bool;
	fn write_str_field(&mut self, value: &str, quote_mode: QuoteMode) -> Result<usize>;
	fn write_string_field(&mut self, value: String, quote_mode: QuoteMode) -> Result<usize> {
		todo!()
	}
	fn write_value_field<T: ToString>(
		&mut self,
		value: &T,
		quote_mode: QuoteMode,
	) -> Result<usize> {
		todo!()
	}
	fn end_of_record(&mut self, should_flush: bool) -> Result<usize>;
	fn escape<'a>(&self, value: Cow<'a, str>) -> (StrCow<'a>, bool) {
		todo!()
	}
	fn add_quote(&self, value: Cow<'_, str>) -> String {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use crate::primitive_encoder::{Encoder, EscapeOutcome, StrCow};
	use crate::quote_mode::QuoteMode;
	
	pub struct TestEncoder {
		pub buff: String,
	}

	impl Encoder<',', 3> for TestEncoder {
		fn classify_char(&self, value: char) -> EscapeOutcome {
			todo!()
		}

		fn should_quoting(&self, value: StrCow) -> bool {
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
	}
}
