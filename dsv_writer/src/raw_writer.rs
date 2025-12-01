use super::argument_error::Result as ArgumentResult;
use super::raw_encoder_error::Result as EncoderResult;
use crate::quote_mode::QuoteMode;
use crate::raw_encoder::Encoder;
use std::collections::HashSet;
use std::io::Write;
pub struct RawWriter<W> {
	writer: W,
	cnt: usize,
	buffer: String,
	delimiter: char,
	escape_set: HashSet<char>,
}

impl<W: Write> RawWriter<W> {
	pub fn try_new(writer: W, delimiter: char) -> ArgumentResult<Self> {
		todo!()
	}
}

impl<W: Write> Encoder for RawWriter<W> {
	fn should_quoting(&self, value: &str) -> bool {
		todo!()
	}

	fn write_str_field(&mut self, value: &str, quote_mode: QuoteMode) -> EncoderResult<usize> {
		todo!()
	}

	fn end_of_record(&mut self, should_flush: bool) -> EncoderResult<usize> {
		todo!()
	}

	fn cnt(&self) -> usize {
		todo!()
	}
}

#[cfg(test)]
mod test {
	use std::fmt::{Display, Formatter};
	use super::*;
	use crate::argument_error::ArgumentError;
	use mockall::{mock, predicate, Predicate};
	use std::io::Write;
	use predicates_core::reflection::PredicateReflection;
	
	mock! {
		pub Writer{}
		impl Write for Writer{
			fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>;
			fn flush(&mut self) -> std::io::Result<()>;
		}
	}
	
	pub struct VerboseEqPredicate([u8]);
	
	pub struct EqStrPredicate<'a>(&'a str);
	
	impl<'a> EqStrPredicate<'a> {
		pub fn new(value: &'a str) -> Self {
			EqStrPredicate(value)
		}
	}
	
	impl<'a> PredicateReflection for EqStrPredicate<'a> {}
	
	impl<'a> Display for EqStrPredicate<'a> {
		fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
			todo!()
		}
	}
	
	impl<'a> Predicate<[u8]> for EqStrPredicate<'a>{
		fn eval(&self, variable: &[u8]) -> bool {
			todo!()
		}
	}
	
	fn srt_eq(value: &str) -> EqStrPredicate<'_> {
		EqStrPredicate::new(value)
	}
	
	#[test]
	fn end_of_record_test() {
		let mut mock = MockWriter::new();
		let mut seq = mockall::Sequence::new();
		
		let eq=predicate::eq(b"hello,world\r\n".as_slice());
		eq.eval(b"hello,world\r".as_slice());
		println!("{:?}", eq);
		
		let eq=srt_eq("\r\n");
		mock.expect_write().once()
			.with(eq);
		
		
		mock.write(b"\r\n".as_slice()).unwrap();
		
		
		
		
		//todo!()
	}
	
	
	
	
	#[test]
	fn new_test() {
		let mut mock = MockWriter::new();
		mock.expect_write()
			.with(predicate::always())
			.returning(|x| Ok(x.len()))
			.once();

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

		assert!(matches!(fixture, Err(ArgumentError::ArgumentError(_))));
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
		let mut mock = MockWriter::new();
		let mut seq = mockall::Sequence::new();

		mock.expect_write()
			.once()
			.in_sequence(&mut seq)
			.returning(|x| Ok(x.len()))
			.with(predicate::eq(b"hello,world\r\n".as_slice()));
		
	}


	#[test]
	fn cnt_test() {
		todo!()
	}
}
