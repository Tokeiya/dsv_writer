use super::raw_encoder_error::Result;
use crate::quote_mode::QuoteMode;
use std::borrow::Cow;

pub type StrCow<'a> = Cow<'a, str>;

pub trait Encoder {
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

	fn cnt(&self) -> usize;
}

#[cfg(test)]
mod tests {
	use crate::quote_mode::QuoteMode;
	use crate::raw_encoder::Encoder;
	use mockall::mock;
	use mockall::predicate;
	
	mock! {
		Writer{}

		impl Encoder for Writer{
			fn write_str_field(&mut self, value: &str, quote_mode: QuoteMode) -> crate::raw_encoder_error::Result<usize>;
			fn end_of_record(&mut self, should_flush: bool) -> crate::raw_encoder_error::Result<usize>;
			fn cnt(&self) -> usize ;
		}

	}

	#[test]
	fn write_value_field_test() {
		let mut mock = MockWriter::new();
		mock.expect_write_str_field()
			.with(predicate::eq("42"), predicate::eq(QuoteMode::Quoted))
			.returning(|_, _| Ok(42));

		mock.write_value_field(&42, QuoteMode::Quoted).unwrap();
	}

	#[test]
	fn write_string_test() {
		let mut mock = MockWriter::new();
		mock.expect_write_str_field()
			.with(predicate::eq("hello"), predicate::eq(QuoteMode::AutoDetect))
			.returning(|_, _| Ok(42));

		mock.write_string_field("hello".to_string(), QuoteMode::AutoDetect)
			.unwrap();
	}
}
