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
	fn add_quote(&self, value: Cow<'_, str>) -> String {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use crate::primitive_encoder::{Encoder, EscapeOutcome, StrCow};
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
		pub buff: Vec<Vec<String>>,
	}

	impl Default for TestEncoder {
		fn default() -> Self {
			Self { buff: vec![vec![]] }
		}
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
			let tmp: StrCow = if quote_mode == QuoteMode::Quoted || self.should_quoting(value) {
				self.add_quote(value.into()).into()
			} else {
				value.into()
			};

			self.buff.last_mut().unwrap().push(tmp.into());
			Ok(self.buff.last().unwrap().len())
		}

		fn end_of_record(&mut self, _: bool) -> crate::error::Result<usize> {
			self.buff.push(vec![]);
			Ok(self.buff.len())
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
