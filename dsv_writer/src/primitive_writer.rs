use super::raw_encoder_error::Result;
use crate::quote_mode::QuoteMode;
use std::borrow::Cow;
use std::collections::HashSet;

type StrCow<'a> = Cow<'a, str>;

pub struct PrimitiveWriter<W, const D: char> {
	writer: W,
	esc_set: HashSet<char>,
	buffer: String,
	cnt: usize,
}

impl<W, const D: char> PrimitiveWriter<W, D>
where
	W: std::io::Write,
{
	pub fn new(writer: W) -> Self {
		let mut esc_set = HashSet::new();
		esc_set.insert('"');
		esc_set.insert('\n');
		esc_set.insert('\r');
		esc_set.insert(D);
		Self {
			writer,
			esc_set,
			buffer: String::new(),
			cnt: 0,
		}
	}

	pub fn write_str_field(&mut self, field: StrCow<'_>, quote_mode: QuoteMode) -> usize {
		let cow = match quote_mode {
			QuoteMode::Quoted => StrCow::Owned(self.add_quote(field)),
			QuoteMode::AutoDetect => self.escape(field).0,
		};

		self.buffer.push_str(&cow);
		self.buffer.push(D);
		self.cnt += 1;
		self.cnt
	}

	pub fn write_value_field<T: ToString>(&mut self, field: &T, quote_mode: QuoteMode) -> usize {
		let s = field.to_string();
		self.write_str_field(StrCow::Owned(s), quote_mode)
	}
	pub fn end_of_record(&mut self, should_flush: bool) -> Result<usize> {
		if !self.buffer.is_empty() {
			self.writer
				.write_all(&self.buffer.as_bytes()[..self.buffer.len() - 1])?;
		}

		self.writer.write_all(b"\r\n")?;

		if should_flush {
			self.writer.flush()?;
		}

		let n = self.cnt;
		self.cnt = 0;
		self.buffer.clear();
		Ok(n)
	}
	pub fn escape<'a>(&self, value: StrCow<'a>) -> (StrCow<'a>, bool) {
		let mut should_quoted = false;
		let mut has_quote = false;

		//Precheck
		for c in value.chars() {
			if self.esc_set.contains(&c) {
				should_quoted = true;
			}

			if c == '"' {
				has_quote = true;
			}

			if should_quoted && has_quote {
				break;
			}
		}

		if has_quote {
			let mut buff = String::new();

			buff.push('"');
			for c in value.chars() {
				if c == '"' {
					buff.push_str("\"\"");
				} else {
					buff.push(c);
				}
			}
			buff.push('"');

			(StrCow::Owned(buff), true)
		} else if should_quoted {
			let mut buff = String::new();
			buff.push('"');
			buff.push_str(&value);
			buff.push('"');

			let cow = StrCow::Owned(buff);
			(cow, true)
		} else {
			(value, false)
		}
	}
	pub fn add_quote(&self, value: StrCow<'_>) -> String {
		let (cow, flg) = self.escape(value);

		if flg {
			cow.into_owned()
		} else {
			let mut buff = String::new();
			buff.push('"');
			buff.push_str(&cow);
			buff.push('"');
			buff
		}
	}

	pub fn count(&self) -> usize {
		self.cnt
	}
}

#[cfg(test)]
mod tests {
	use super::{PrimitiveWriter, QuoteMode};
	use mockall::{mock, predicate};
	use std::borrow::Cow;
	use std::io::Write;
	
	mock! {
		pub Writer{}
		impl std::io::Write for Writer{
			fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>;
			fn flush(&mut self) -> std::io::Result<()>;
		}
	}

	pub fn with_dbg(actual: &[u8], expected: &str) -> bool {
		let a = String::from_utf8_lossy(actual);
		println!("actual:{} expected:{}", &a, expected);
		println!("esc_actual:{:?} esc_expected:{:?}", &a, expected);
		a == expected
	}

	pub const CSV_ESCAPE_CHARS: &[char] = &['"', '\n', '\r', ','];
	pub const TSV_ESCAPE_CHARS: &[char] = &['"', '\n', '\r', '\t'];
	#[test]
	fn new_test() {
		let mut mock = MockWriter::new();
		mock.expect_write()
			.with(predicate::function(|x: &[u8]| x.eq("hello".as_bytes())))
			.returning(|x| Ok(x.len()))
			.times(1);

		let mut fixture = PrimitiveWriter::<MockWriter, ','>::new(mock);
		assert_eq!(fixture.buffer, "");
		assert_eq!(fixture.esc_set.len(), CSV_ESCAPE_CHARS.len());
		for expected in CSV_ESCAPE_CHARS.iter() {
			assert!(fixture.esc_set.contains(expected));
		}

		assert!(!fixture.esc_set.contains(&'a'));
		let _ = fixture.writer.write("hello".as_bytes());
	}

	#[test]
	fn csv_escape_test() {
		let mock = MockWriter::new();

		let fixture = PrimitiveWriter::<MockWriter, ','>::new(mock);
		let (c, flg) = fixture.escape("hello".into());
		assert!(!flg);
		assert_eq!(c, "hello");

		let (c, flg) = fixture.escape("hello,world".into());
		assert!(flg);
		assert_eq!(c, r#""hello,world""#);
	}

	#[test]
	fn tsv_escape_test() {
		let mock = MockWriter::new();

		let fixture = PrimitiveWriter::<MockWriter, '\t'>::new(mock);
		let input = Cow::Borrowed("hello");
		let (c, flg) = fixture.escape(input);
		assert!(!flg);
		assert_eq!(c, "hello");

		let input = Cow::Borrowed("hello\tworld");
		let (c, flg) = fixture.escape(input);
		assert!(flg);
		assert_eq!(c, r#""hello	world""#);
	}

	#[test]
	fn add_quote_test() {
		let mock = MockWriter::new();

		let fixture = PrimitiveWriter::<MockWriter, ','>::new(mock);
		let input = Cow::Borrowed("hello");

		let c = fixture.add_quote(input);
		assert_eq!(&c, r#""hello""#);
	}

	#[test]
	fn count_test() {
		let mut mock = MockWriter::new();
		mock.expect_write()
			.with(predicate::always())
			.returning(|x| Ok(x.len()));

		mock.write_all(b"hello").unwrap();

		let mut fixture = PrimitiveWriter::<MockWriter, ','>::new(mock);
		assert_eq!(fixture.count(), 0);

		let cnt = fixture.write_str_field("hello".into(), QuoteMode::Quoted);

		assert_eq!(fixture.count(), 1);
		assert_eq!(fixture.count(), cnt);

		fixture.end_of_record(false).unwrap();
		assert_eq!(fixture.count(), 0);
	}

	#[test]
	fn write_str_field_test() {
		let mut mock = MockWriter::new();
		let mut seq = mockall::Sequence::new();

		mock.expect_write()
			.once()
			.withf(|x| with_dbg(x, "hello,world,\"quoted\",\"\"\"\""))
			.returning(|x| Ok(x.len()))
			.in_sequence(&mut seq);

		mock.expect_write()
			.once()
			.withf(|x| with_dbg(x, "\r\n"))
			.in_sequence(&mut seq)
			.returning(|x| Ok(x.len()));

		mock.expect_flush().once().returning(|| Ok(()));

		let mut fixture = PrimitiveWriter::<MockWriter, ','>::new(mock);
		let a = fixture.write_str_field(Cow::Borrowed("hello"), QuoteMode::AutoDetect);
		assert_eq!(a, 1);

		let a = fixture.write_str_field(Cow::Borrowed("world"), QuoteMode::AutoDetect);
		assert_eq!(a, 2);

		let a = fixture.write_str_field(Cow::Borrowed("quoted"), QuoteMode::Quoted);
		assert_eq!(a, 3);

		let a = fixture.write_str_field(Cow::Borrowed("\""), QuoteMode::Quoted);
		assert_eq!(a, 4);

		let n = fixture.end_of_record(true).unwrap();
		assert_eq!(n, 4);
	}

	#[test]
	fn write_value_field_test() {
		let mut mock = MockWriter::new();
		let mut seq = mockall::Sequence::new();

		mock.expect_write()
			.withf(|x| with_dbg(x, "123,45.67,\"true\",\"\"\"text\"\"\""))
			.returning(|x| Ok(x.len()))
			.once()
			.in_sequence(&mut seq);

		mock.expect_write()
			.withf(|x| with_dbg(x, "\r\n"))
			.returning(|x| Ok(x.len()))
			.once()
			.in_sequence(&mut seq);

		mock.expect_flush()
			.returning(|| Ok(()))
			.in_sequence(&mut seq);

		let mut fixture = PrimitiveWriter::<MockWriter, ','>::new(mock);
		let a = fixture.write_value_field(&123, QuoteMode::AutoDetect);
		assert_eq!(a, 1);

		let a = fixture.write_value_field(&45.67, QuoteMode::AutoDetect);
		assert_eq!(a, 2);

		let a = fixture.write_value_field(&true, QuoteMode::Quoted);
		assert_eq!(a, 3);

		let a = fixture.write_value_field(&"\"text\"", QuoteMode::Quoted);
		assert_eq!(a, 4);

		let n = fixture.end_of_record(true).unwrap();
		assert_eq!(n, 4);
	}

	#[test]
	fn end_of_record_test() {
		let mut mock = MockWriter::new();
		let mut seq = mockall::Sequence::new();

		mock.expect_write()
			.withf(|x| with_dbg(x, "\r\n"))
			.once()
			.returning(|x| Ok(x.len()))
			.in_sequence(&mut seq);

		mock.expect_flush()
			.returning(|| Ok(()))
			.in_sequence(&mut seq);

		let mut fixture = PrimitiveWriter::<MockWriter, ','>::new(mock);
		let n = fixture.end_of_record(true).unwrap();
		assert_eq!(n, 0);

		let mut mock = MockWriter::new();
		let mut seq = mockall::Sequence::new();

		mock.expect_write()
			.withf(|x| with_dbg(x, "\r\n"))
			.once()
			.returning(|x| Ok(x.len()))
			.in_sequence(&mut seq);

		let mut fixture = PrimitiveWriter::<MockWriter, ','>::new(mock);
		let n = fixture.end_of_record(false).unwrap();
		assert_eq!(n, 0);
	}
}
