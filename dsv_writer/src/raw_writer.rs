use super::raw_encoder_error::Result as EncoderResult;
use crate::quote_mode::QuoteMode;
use crate::raw_encoder::Encoder;
use common_errors::invalid_argument::{Error as ArgumentError, Information};
use std::borrow::Cow;
use std::collections::HashSet;
use std::io::Write;

type ArgumentResult<T> = Result<T, ArgumentError>;

#[derive(Debug)]
pub struct RawWriter<W> {
	writer: W,
	cnt: usize,
	buffer: String,
	delimiter: char,
	escape_set: HashSet<char>,
}

impl<W: Write> RawWriter<W> {
	pub fn try_new(writer: W, delimiter: char) -> ArgumentResult<Self> {
		if delimiter == '"' {
			Err(ArgumentError::InvalidArgument(Information::new_both(
				"delimiter".to_string(),
				"\" is not allowed as delimiter".to_string(),
			)))
		} else {
			Ok(RawWriter {
				writer,
				cnt: 0,
				buffer: String::new(),
				delimiter,
				escape_set: HashSet::from(['"', '\n', '\r', delimiter]),
			})
		}
	}
}

impl<W: Write> Encoder for RawWriter<W> {
	fn should_quoting(&self, value: &str) -> bool {
		value.chars().any(|c| self.escape_set.contains(&c))
	}

	fn write_str_field(&mut self, value: &str, quote_mode: QuoteMode) -> EncoderResult<usize> {
		let tmp: Cow<str> = if quote_mode == QuoteMode::Quoted || self.should_quoting(value) {
			self.add_quote(value.into()).into()
		} else {
			value.into()
		};

		self.buffer.push_str(&tmp);
		self.buffer.push(self.delimiter);
		self.cnt += 1;

		Ok(self.cnt)
	}

	fn end_of_record(&mut self, should_flush: bool) -> EncoderResult<usize> {
		if self.cnt > 0 {
			self.buffer.pop();
		}

		self.buffer.push_str("\r\n");
		self.writer.write_all(self.buffer.as_bytes())?;
		let c = self.cnt;
		self.cnt = 0;

		if should_flush {
			self.writer.flush()?;
		}

		Ok(c)
	}

	fn cnt(&self) -> usize {
		self.cnt
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use common_errors::invalid_argument::Error as ArgumentError;
	use mockall::{mock, predicate, Predicate};
	use predicates_core::reflection::PredicateReflection;
	use std::fmt::{Display, Formatter};
	use std::io::Write;
	
	#[test]
	fn playground() {
		let mut vec=Vec::new();
		
		let mut writer=RawWriter::try_new(&mut vec, ',').unwrap();
		writer.write_str_field("hello", QuoteMode::AutoDetect).unwrap();
		writer.write_str_field("world", QuoteMode::AutoDetect).unwrap();
		writer.end_of_record(true).unwrap();
		
		
		dbg!(&vec);
		let str=String::from_utf8(vec).unwrap();
		dbg!(&str);
	}
	
	mock! {
		pub Writer{}
		impl Write for Writer{
			fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>;
			fn flush(&mut self) -> std::io::Result<()>;
		}
	}

	pub struct EqStrPredicate<'a>(&'a str);

	impl<'a> EqStrPredicate<'a> {
		pub fn new(value: &'a str) -> Self {
			EqStrPredicate(value)
		}
	}

	impl<'a> Display for EqStrPredicate<'a> {
		fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
			write!(f, "EqStrPredicate(Disp:{} Dbg:{:?})", self.0, self.0)
		}
	}

	impl<'a> PredicateReflection for EqStrPredicate<'a> {}

	impl<'a> Predicate<[u8]> for EqStrPredicate<'a> {
		fn eval(&self, variable: &[u8]) -> bool {
			let exp = self.0.as_bytes();

			let result =
				variable.len() == exp.len() && variable.iter().zip(exp).all(|(a, b)| a == b);

			if result {
				result
			} else {
				let act = String::from_utf8_lossy(variable);
				let exp = self.0;
				println!("actual:{act} expected:{exp}");
				println!("dbg_actual:{:?} dbg_expected:{:?}", act, exp);
				false
			}
		}
	}

	fn str_eq(value: &str) -> EqStrPredicate<'_> {
		EqStrPredicate::new(value)
	}

	#[test]
	fn end_of_record_test() {
		let mut mock = MockWriter::new();
		let mut seq = mockall::Sequence::new();

		mock.expect_write()
			.once()
			.in_sequence(&mut seq)
			.returning(|x| Ok(x.len()))
			.with(str_eq("hello,world\r\n"))
			.returning(|x| Ok(x.len()));

		mock.expect_flush()
			.once()
			.returning(|| Ok(()))
			.in_sequence(&mut seq);

		let mut fixture = RawWriter::<MockWriter>::try_new(mock, ',').unwrap();
		fixture
			.write_str_field("hello", QuoteMode::AutoDetect)
			.unwrap();
		fixture
			.write_str_field("world", QuoteMode::AutoDetect)
			.unwrap();

		let act = fixture.end_of_record(true).unwrap();
		assert_eq!(act, 2);
	}

	#[test]
	fn new_test() {
		let mut mock = MockWriter::new();
		mock.expect_write()
			.with(predicate::always())
			.returning(|x| Ok(x.len()));

		let fixture = RawWriter::<MockWriter>::try_new(mock, ',').unwrap();
		assert_eq!(fixture.cnt, 0);
		assert_eq!(fixture.buffer, "");
		assert_eq!(fixture.delimiter, ',');

		assert_eq!(fixture.escape_set.len(), 4);
		assert!(fixture.escape_set.contains(&'"'));
		assert!(fixture.escape_set.contains(&'\n'));
		assert!(fixture.escape_set.contains(&'\r'));
		assert!(fixture.escape_set.contains(&','));

		let mock = MockWriter::new();
		let fixture = RawWriter::<MockWriter>::try_new(mock, '\"');
		
		assert!(matches!(fixture, Err(ArgumentError::InvalidArgument(_))));
	}

	#[test]
	fn should_quoting_test() {
		let mock = MockWriter::new();
		let fixture = RawWriter::<MockWriter>::try_new(mock, ',').unwrap();
		assert!(!fixture.should_quoting("hello"));
		assert!(fixture.should_quoting("\"hello\""));
		assert!(fixture.should_quoting("hello,world"));
		assert!(fixture.should_quoting("\r"));
		assert!(fixture.should_quoting("\n"));
		assert!(!fixture.should_quoting("\t"));
	}

	#[test]
	fn write_str_field_test() {
		let mock = MockWriter::new();

		let mut fixture = RawWriter::<MockWriter>::try_new(mock, ',').unwrap();
		let cnt = fixture
			.write_str_field("hel,lo", QuoteMode::AutoDetect)
			.unwrap();

		assert_eq!(cnt, 1);

		let cnt = fixture
			.write_str_field("world", QuoteMode::AutoDetect)
			.unwrap();

		assert_eq!(cnt, 2);
		assert_eq!(fixture.buffer, r##""hel,lo",world,"##);
	}

	#[test]
	fn cnt_test() {
		let mut mock = MockWriter::new();
		mock.expect_write()
			.with(predicate::always())
			.returning(|x| Ok(x.len()));

		let mut fixture = RawWriter::<MockWriter>::try_new(mock, ',').unwrap();
		assert_eq!(fixture.cnt(), 0);

		let cnt = fixture
			.write_str_field("hello", QuoteMode::AutoDetect)
			.unwrap();
		assert_eq!(fixture.cnt(), cnt);
		assert_eq!(fixture.cnt(), 1);

		let cnt = fixture
			.write_value_field(&42, QuoteMode::AutoDetect)
			.unwrap();
		assert_eq!(fixture.cnt(), cnt);
		assert_eq!(fixture.cnt(), 2);

		_ = fixture.end_of_record(false).unwrap();
		assert_eq!(fixture.cnt(), 0);
	}
}
