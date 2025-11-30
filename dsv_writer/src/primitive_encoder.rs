use super::error::Result;
use crate::quote_mode::QuoteMode;
use std::borrow::Cow;

pub type StrCow<'a> = Cow<'a, str>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EscapeOutcome {
	NotEscaped,
	DuplicatedQuote,
}

pub trait Encoder {
	fn classify_char(&self, value: char) -> EscapeOutcome;
	fn should_quoting(&self, value: &str) -> bool;
	fn write_str_field(&mut self, value: &str, quote_mode: QuoteMode) -> Result<usize>;
	fn write_string_field(&mut self, value: String, quote_mode: QuoteMode) -> Result<usize> {
		self.write_str_field(&value, quote_mode)
	}
	fn write_value_field<T: ToString>(
		&mut self,
		value: &T,
		quote_mode: QuoteMode,
	) -> Result<usize> {
		self.write_str_field(value.to_string().as_str(), quote_mode)
	}
	fn end_of_record(&mut self, should_flush: bool) -> Result<usize>;
	fn add_quote(&self, value: Cow<'_, str>) -> String {
		let mut buff = String::new();

		buff.push('"');
		buff.push_str(&value.replace('"', r#""""#));
		buff.push('"');
		buff
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

	pub struct Writer {
		pub buff: Vec<Vec<String>>,
	}

	impl Default for Writer {
		fn default() -> Self {
			Self { buff: vec![vec![]] }
		}
	}

	impl Encoder for Writer {
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
		let mut writer = Writer::default();
		let cnt = writer
			.write_string_field("test".to_string(), QuoteMode::AutoDetect)
			.unwrap();
		assert_eq!(cnt, 1);
		assert_eq!(writer.buff.last().unwrap()[0], "test");

		let cnt = writer
			.write_string_field("test,test".to_string(), QuoteMode::AutoDetect)
			.unwrap();
		assert_eq!(cnt, 2);
		assert_eq!(writer.buff.last().unwrap()[1], r#""test,test""#);

		let cnt = writer
			.write_string_field("hoge".to_string(), QuoteMode::Quoted)
			.unwrap();
		assert_eq!(cnt, 3);
		assert_eq!(writer.buff.last().unwrap()[2], r#""hoge""#);
	}

	#[test]
	fn write_value_field_test() {
		let mut writer = Writer::default();
		let cnt = writer
			.write_value_field(&100, QuoteMode::AutoDetect)
			.unwrap();
		assert_eq!(cnt, 1);
		assert_eq!(writer.buff.last().unwrap()[0], "100");

		let cnt = writer.write_value_field(&42, QuoteMode::Quoted).unwrap();
		assert_eq!(cnt, 2);
		assert_eq!(writer.buff.last().unwrap()[1], r#""42""#);
	}
	#[test]
	fn add_quote_test() {
		let writer = Writer::default();
		let quoted = writer.add_quote("test".into());
		assert_eq!(quoted, r#""test""#);

		let quoted = writer.add_quote(r#""te,st""#.into());
		assert_eq!(quoted, r####""""te,st""""####);

		let quoted = writer.add_quote(r#"te"st"#.into());
		assert_eq!(quoted, r#""te""st""#);
	}
}
