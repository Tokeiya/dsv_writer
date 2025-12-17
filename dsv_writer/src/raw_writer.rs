use super::raw_encoder_error::Result as EncoderResult;
use crate::quote_mode::QuoteMode;
use crate::raw_encoder::Encoder;
use common_errors::invalid_argument::{Error as ArgumentError, Information};
use std::collections::HashSet;
use std::io::Write;

type ArgumentResult<T> = Result<T, ArgumentError>;

#[derive(Debug)]
pub struct RawWriter<W> {
	writer: W,
	cnt: usize,
	delimiter: u8,
}

impl<W: Write> RawWriter<W> {
	pub fn try_new(writer: W, delimiter: char) -> ArgumentResult<Self> {
		if !delimiter.is_ascii() {
			return Err(ArgumentError::InvalidArgument(Information::new_both(
				"delimiter".to_string(),
				format!("delimiter must be ascii, but got {}", delimiter),
			)));
		}
		if delimiter == '"' {
			Err(ArgumentError::InvalidArgument(Information::new_both(
				"delimiter".to_string(),
				"\" is not allowed as delimiter".to_string(),
			)))
		} else {
			let d = delimiter as u8;

			Ok(RawWriter {
				writer,
				cnt: 0,
				delimiter: delimiter as u8,
			})
		}
	}

	fn should_quote(&self, value: &str) -> (bool, bool) {
		let mut should_quote = false;
		let mut should_escape = false;

		for elem in value.bytes() {
			todo!()
		}
	}

	fn add_quote(&mut self, value: &str, should_quote: Option<bool>) -> EncoderResult<()> {
		todo!()
	}
}

impl<W: Write> Encoder for RawWriter<W> {
	fn write_str_field(&mut self, value: &str, quote_mode: QuoteMode) -> EncoderResult<usize> {
		todo!()
	}

	fn end_of_record(&mut self, should_flush: bool) -> EncoderResult<usize> {
		todo!()
	}

	fn cnt(&self) -> usize {
		self.cnt
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use mockall::mock;
	use mockall::{Predicate, Sequence};
	use predicates::reflection::PredicateReflection;
	use predicates_core::reflection::Case;
	use std::fmt::{Display, Formatter};

	struct StrPredicate(String);

	impl From<&str> for StrPredicate {
		fn from(value: &str) -> Self {
			StrPredicate(value.to_string())
		}
	}

	impl PredicateReflection for StrPredicate {}

	impl Display for StrPredicate {
		fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
			write!(f, "StrPredicate({})", self.0)
		}
	}

	impl Predicate<[u8]> for StrPredicate {
		fn eval(&self, variable: &[u8]) -> bool {
			let actual = String::from_utf8(variable.to_vec()).unwrap();

			if actual != self.0 {
				println!("actual:{} expected:{}", &actual, self.0)
			}

			actual == self.0
		}
	}

	mock! {
		Write{}

		impl Write for Write {
			fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>;
			fn flush(&mut self) -> std::io::Result<()>;
		}
	}

	#[test]
	fn try_new_tet() {
		let mut mock = MockWrite::new();
		mock.expect_write().once();
		let mut fixture = RawWriter::try_new(mock, ',').unwrap();

		assert_eq!(fixture.delimiter, ',');
		assert_eq!(fixture.escape_set.len(), 4);
		assert_eq!(fixture.escape_set, HashSet::from([',', '\n', '\r', '"']));

		fixture.write_str_field("test", QuoteMode::Quoted).unwrap();

		let mut mock = MockWrite::new();
		let err = RawWriter::try_new(mock, '"').err().unwrap();
		assert!(matches!(err, ArgumentError::InvalidArgument(_)))
	}

	#[test]
	fn hoge() {
		let mut seq = Sequence::new();
		let mut mock = MockWrite::new();

		mock.expect_write()
			.with(StrPredicate::from("test"))
			.returning(|x| Ok(x.len()));
		mock.write(b"test").unwrap();
	}

	#[test]
	fn write_str_field_test() {
		let mut seq = Sequence::new();
		let mut mock = MockWrite::new();

		mock.expect_write()
			.with(StrPredicate::from("test"))
			.returning(|x| Ok(x.len()))
			.in_sequence(&mut seq);

		mock.expect_write()
			.with(StrPredicate::from("\"te,st\""))
			.returning(|x| Ok(x.len()))
			.in_sequence(&mut seq);

		mock.expect_write()
			.with(StrPredicate::from("\"test\""))
			.returning(|x| Ok(x.len()))
			.in_sequence(&mut seq);

		let mut fixture = RawWriter::try_new(mock, ',').unwrap();
		fixture
			.write_str_field("test", QuoteMode::AutoDetect)
			.unwrap();

		fixture
			.write_str_field("te,st", QuoteMode::AutoDetect)
			.unwrap();

		fixture.write_str_field("test", QuoteMode::Quoted).unwrap();
	}

	#[test]
	fn end_of_record_test() {
		let mut seq = Sequence::new();
		let mut mock = MockWrite::new();

		mock.expect_write()
			.with(StrPredicate::from("hello"))
			.returning(|x| Ok(x.len()))
			.in_sequence(&mut seq);

		mock.expect_write()
			.with(StrPredicate::from("\r\n"))
			.returning(|x| Ok(x.len()))
			.in_sequence(&mut seq);

		mock.expect_write()
			.with(StrPredicate::from("world"))
			.returning(|x| Ok(x.len()))
			.in_sequence(&mut seq);

		mock.expect_write()
			.with(StrPredicate::from("\r\n"))
			.returning(|x| Ok(x.len()))
			.in_sequence(&mut seq);
		mock.expect_flush().in_sequence(&mut seq);

		let mut fixture = RawWriter::try_new(mock, ',').unwrap();
		fixture
			.write_str_field("hello", QuoteMode::AutoDetect)
			.unwrap();
		fixture.end_of_record(true).unwrap();
		fixture
			.write_str_field("world", QuoteMode::AutoDetect)
			.unwrap();
		fixture.end_of_record(false).unwrap();
	}
}
