use super::raw_async_encoder::RawAsyncEncode;
use crate::quote_mode::QuoteMode;
use common_errors::invalid_argument::{Error as InvalidArgumentError, Information};
use futures::io::Result as IoResult;
use futures::{AsyncWrite, AsyncWriteExt};
use std::borrow::Cow;
use std::collections::HashSet;
type ArgumentResult<T> = Result<T, common_errors::invalid_argument::Error>;

pub struct RawAsyncWriter<W> {
	writer: W,
	set: HashSet<char>,
	cnt: usize,
	delimiter: Vec<u8>,
}

impl<W> RawAsyncWriter<W> {
	pub fn try_new(writer: W, delimiter: char) -> ArgumentResult<Self> {
		if delimiter == '"' {
			let info = Information::new_both(
				"delimiter".to_string(),
				"double quote cannot be used as delimiter".to_string(),
			);
			return Err(InvalidArgumentError::InvalidArgument(info));
		}

		Ok(Self {
			writer,
			set: [delimiter, '\n', '\r', '"'].iter().copied().collect(),
			cnt: 0,
			delimiter: delimiter.to_string().into(),
		})
	}

	fn should_quoting(&self, value: &str) -> bool {
		value.chars().any(|c| self.set.contains(&c))
	}
}

impl<W: AsyncWrite + Unpin + Send> RawAsyncEncode for RawAsyncWriter<W> {
	async fn write_str_field(&mut self, value: &str, quote_mode: QuoteMode) -> IoResult<usize> {
		let tmp: Cow<str> = if quote_mode == QuoteMode::Quoted || self.should_quoting(value) {
			Self::add_quote(value).into()
		} else {
			value.into()
		};

		if self.cnt > 0 {
			self.writer.write_all(&self.delimiter).await?;
		}

		self.writer.write_all(tmp.as_bytes()).await?;
		self.cnt += 1;

		Ok(self.cnt)
	}

	async fn end_of_record(&mut self, should_flush: bool) -> IoResult<usize> {
		self.writer.write_all(b"\r\n").await?;

		if should_flush {
			self.writer.flush().await?;
		}
		let c = self.cnt;
		self.cnt = 0;
		Ok(c)
	}

	fn cnt(&self) -> usize {
		self.cnt
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::quote_mode::QuoteMode::AutoDetect;
	use futures::AsyncWriteExt;
	
	#[tokio::test]
	async fn playground() {
		async fn foo<W: AsyncWrite + std::marker::Unpin>(writer: &mut W) {
			writer.write_all(b"hello").await.unwrap();
		}

		let mut vec = Vec::new();
		foo(&mut vec).await;
		dbg!(String::from_utf8(vec).unwrap());
	}

	#[tokio::test]
	async fn try_new_test() {
		let mut vec = Vec::<u8>::new();
		let fixture = RawAsyncWriter::try_new(&mut vec, ',').unwrap();
		assert_eq!(fixture.cnt, 0);
		assert_eq!(fixture.set.len(), 4);

		assert!(fixture.set.contains(&'"'));
		assert!(fixture.set.contains(&'\n'));
		assert!(fixture.set.contains(&'\r'));
		assert!(fixture.set.contains(&','));

		let fixture = RawAsyncWriter::try_new(&mut vec, '\"').err().unwrap();
		assert!(matches!(
			fixture,
			common_errors::invalid_argument::Error::InvalidArgument(_)
		));
	}

	#[test]
	fn should_quoting_test() {
		let mut binding = Vec::<u8>::new();
		let fixture = RawAsyncWriter::try_new(&mut binding, ',').unwrap();
		assert!(!fixture.should_quoting("hello"));
		assert!(fixture.should_quoting("hel\"lo"));
		assert!(fixture.should_quoting("hello,world"));
		assert!(fixture.should_quoting("\r"));
		assert!(fixture.should_quoting("\n"));
		assert!(!fixture.should_quoting("\t"));
	}

	#[tokio::test]
	async fn write_str_field_test() {
		async fn f(quote_mode: QuoteMode, input: &str, expected: &str) {
			let mut binding = Vec::<u8>::new();
			let mut fixture = RawAsyncWriter::try_new(&mut binding, ',').unwrap();

			fixture.write_str_field(input, quote_mode).await.unwrap();

			println!(
				"act:{} exp:{} result:{}",
				String::from_utf8_lossy(&binding),
				expected,
				String::from_utf8_lossy(&binding) == expected
			);

			assert_eq!(binding, expected.as_bytes());
		}

		async fn g(quote_mode: QuoteMode, input: &[&str], expected: &str) {
			let mut binding = Vec::<u8>::new();
			let mut fixture = RawAsyncWriter::try_new(&mut binding, ',').unwrap();

			for s in input {
				fixture.write_str_field(s, quote_mode).await.unwrap();
			}

			std::println!(
				"g act:{} exp:{} result:{}",
				String::from_utf8_lossy(&binding),
				expected,
				String::from_utf8_lossy(&binding) == expected
			);

			assert_eq!(binding, expected.as_bytes());
		}

		f(QuoteMode::Quoted, "hello", r#""hello""#).await;
		f(QuoteMode::AutoDetect, "hello", "hello").await;

		f(QuoteMode::AutoDetect, "hello,world", r#""hello,world""#).await;
		f(QuoteMode::AutoDetect, "hello\rworld", "\"hello\rworld\"").await;
		f(QuoteMode::AutoDetect, "hello\nworld", "\"hello\nworld\"").await;
		f(
			QuoteMode::AutoDetect,
			"hello\"world",
			r###""hello""world""###,
		)
		.await;

		g(QuoteMode::AutoDetect, &["hello", "world"], "hello,world").await;
		g(QuoteMode::AutoDetect, &["hello", ""], "hello,").await;
		g(
			QuoteMode::AutoDetect,
			&["hello", "wo,rld"],
			"hello,\"wo,rld\"",
		)
		.await;
	}

	#[tokio::test]
	async fn end_of_record_test() {
		async fn f(input: &[(&str, QuoteMode)], expected: &str) {
			let mut binding = Vec::<u8>::new();
			let mut fixture = RawAsyncWriter::try_new(&mut binding, ',').unwrap();

			for (s, q) in input {
				fixture.write_str_field(s, *q).await.unwrap();
			}

			let act = fixture.end_of_record(true).await.unwrap();
			assert_eq!(binding, expected.as_bytes());
			assert_eq!(act, input.len());
		}

		f(
			&[("hello", QuoteMode::Quoted), ("world", AutoDetect)],
			"\"hello\",world\r\n",
		)
		.await;
	}

	#[tokio::test]
	async fn cnt_test() {
		let mut buffer = Vec::<u8>::new();
		let mut fixture = RawAsyncWriter::try_new(&mut buffer, ',').unwrap();
		assert_eq!(fixture.cnt(), 0);
		fixture
			.write_str_field("hello", QuoteMode::Quoted)
			.await
			.unwrap();
		assert_eq!(fixture.cnt(), 1);

		fixture.end_of_record(true).await.unwrap();
		assert_eq!(fixture.cnt(), 0);

		fixture
			.write_str_field("hello", QuoteMode::Quoted)
			.await
			.unwrap();
		assert_eq!(fixture.cnt(), 1);
	}
}
