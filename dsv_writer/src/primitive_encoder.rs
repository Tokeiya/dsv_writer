use super::error::Result;
use crate::quote_mode::QuoteMode;
use std::borrow::Cow;

pub type StrCow<'a> = Cow<'a, str>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EscapeOutcome {
	NotEscaped,
	DuplicatedQuote,
}

pub trait Encoder<const D: char, const N: usize> {
	fn classify_char(&self, value: char) -> EscapeOutcome;
	fn should_quoting(&self, value: &str) -> bool;
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
	use crate::primitive_encoder::{Encoder, EscapeOutcome};
	use crate::quote_mode::QuoteMode;
	use std::collections::HashSet;
	use std::sync::LazyLock;

	static DICT: LazyLock<HashSet<char>> = LazyLock::new(|| {
		let mut set = HashSet::new();
		set.insert('"');
		set.insert('\n');
		set.insert('\r');
		set.insert('\t');
		set.insert(',');
		set
	});

	pub struct TestEncoder {
		pub buff: Vec<String>,
		pub cnt: usize,
	}

	impl Encoder<',', 3> for TestEncoder {
		fn classify_char(&self, value: char) -> EscapeOutcome {
			if value == '"' {
				EscapeOutcome::DuplicatedQuote
			} else {
				EscapeOutcome::NotEscaped
			}
		}

		fn should_quoting(&self, value: &str) -> bool {
			value.chars().any(|c| DICT.contains(&c))
		}

		fn write_str_field(
			&mut self,
			value: &str,
			quote_mode: QuoteMode,
		) -> crate::error::Result<usize> {
			todo!()
		}

		fn end_of_record(&mut self, _: bool) -> crate::error::Result<usize> {
			let mut size = 0usize;

			for idx in self.cnt..self.buff.len() {
				size += self.buff[idx].len();
			}

			self.cnt = self.buff.len() - 1;
			Ok(size)
		}
	}

	#[test]
	fn write_string_field_test() {
		todo!();
	}

	#[test]
	fn write_value_field_test() {
		todo!();
	}

	#[test]
	fn escape_test() {
		todo!();
	}

	#[test]
	fn add_quote_test() {
		todo!();
	}
}
