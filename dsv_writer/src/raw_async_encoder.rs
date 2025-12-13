use crate::quote_mode::QuoteMode;
use futures::io::Result as IoResult;
use std::borrow::Cow;

pub type StrCow<'a> = Cow<'a, str>;

pub trait RawAsyncEncode {
	fn write_str_field(
		&mut self,
		value: &str,
		quote_mode: QuoteMode,
	) -> impl Future<Output = IoResult<usize>> + Send;
	fn end_of_record(&mut self, should_flush: bool)
	-> impl Future<Output = IoResult<usize>> + Send;

	fn write_string_field(
		&mut self,
		value: String,
		quote_mode: QuoteMode,
	) -> impl Future<Output = IoResult<usize>> + Send
	where
		Self: Send,
	{
		async move { self.write_str_field(value.as_str(), quote_mode).await }
	}

	fn write_value_field<T: ToString + ?Sized>(
		&mut self,
		value: &T,
		quote_mode: QuoteMode,
	) -> impl Future<Output = IoResult<usize>> + Send
	where
		Self: Send,
	{
		let bind = value.to_string();
		async move { self.write_str_field(bind.as_str(), quote_mode).await }
	}
	fn add_quote(value: &str) -> String {
		let mut buf = String::new();
		buf.push('"');

		for c in value.chars() {
			if c == '"' {
				buf.push('"');
			}
			buf.push(c);
		}

		buf.push('"');

		buf
	}
	fn cnt(&self) -> usize;
}

#[cfg(test)]
mod tests {
	use super::*;
	use futures::io::AsyncWriteExt;
	use futures::AsyncWrite;
	use std::collections::HashSet;
	
	type MockType = MockWriter<Vec<u8>>;

	struct MockWriter<T> {
		buff: T,
		set: HashSet<char>,
		cnt: usize,
	}

	impl Default for MockWriter<Vec<u8>> {
		fn default() -> Self {
			let mut set = HashSet::new();
			set.insert('"');
			set.insert('\n');
			set.insert('\r');
			set.insert('\t');

			Self {
				buff: Vec::new(),
				set,
				cnt: 0,
			}
		}
	}

	impl MockWriter<Vec<u8>> {
		pub fn raw(&self) -> &[u8] {
			&self.buff
		}

		pub async fn mock_end_of_record(&mut self) -> String {
			self.end_of_record(true).await.unwrap();
			let s: String = String::from_utf8_lossy(&self.buff).into();
			self.buff.clear();
			s
		}
	}

	impl<T> MockWriter<T> {
		pub fn should_quote(&self, value: &str) -> bool {
			value.chars().any(|c| self.set.contains(&c))
		}
	}

	impl<T: AsyncWrite + Unpin + std::marker::Send> RawAsyncEncode for MockWriter<T> {
		async fn write_str_field(&mut self, value: &str, quote_mode: QuoteMode) -> IoResult<usize> {
			if self.cnt != 0 {
				self.buff.write_all(b"\t").await?;
			}

			let scr = match quote_mode {
				QuoteMode::Quoted => StrCow::from(Self::add_quote(value)),
				QuoteMode::AutoDetect => {
					if self.should_quote(value) {
						StrCow::from(Self::add_quote(value))
					} else {
						StrCow::from(value)
					}
				}
			};

			self.buff.write_all(scr.as_bytes()).await?;
			self.cnt += 1;
			Ok(scr.len())
		}

		async fn end_of_record(&mut self, should_flush: bool) -> IoResult<usize> {
			self.buff.write_all(b"\r\n").await?;
			let c = self.cnt;

			if should_flush {
				self.buff.flush().await?;
			}

			self.cnt = 0;

			Ok(c)
		}

		fn cnt(&self) -> usize {
			self.cnt
		}
	}

	#[tokio::test]
	async fn write_str_field_test() {
		let mut mock = MockWriter::default();
		mock.write_str_field("test", QuoteMode::AutoDetect)
			.await
			.unwrap();

		assert_eq!(mock.mock_end_of_record().await, "test\r\n");

		mock.write_str_field("test\ttest", QuoteMode::AutoDetect)
			.await
			.unwrap();

		assert_eq!(mock.mock_end_of_record().await, "\"test\ttest\"\r\n");
	}

	#[tokio::test]
	async fn write_string_field_test() {
		let mut mock = MockWriter::default();
		mock.write_string_field("test".into(), QuoteMode::AutoDetect)
			.await
			.unwrap();

		assert_eq!(mock.mock_end_of_record().await, "test\r\n");

		mock.write_string_field("test\ttest".into(), QuoteMode::AutoDetect)
			.await
			.unwrap();

		assert_eq!(mock.mock_end_of_record().await, "\"test\ttest\"\r\n");
	}

	#[tokio::test]
	async fn write_value_field_test() {
		let mut mock = MockWriter::default();

		mock.write_value_field(&123, QuoteMode::AutoDetect)
			.await
			.unwrap();
		assert_eq!(mock.mock_end_of_record().await, "123\r\n");

		mock.write_value_field("test", QuoteMode::Quoted)
			.await
			.unwrap();
		assert_eq!(mock.mock_end_of_record().await, "\"test\"\r\n");

		mock.write_value_field("test\ttest", QuoteMode::AutoDetect)
			.await
			.unwrap();

		assert_eq!(mock.mock_end_of_record().await, "\"test\ttest\"\r\n");
	}

	#[test]
	fn add_quote_test() {
		let actual = MockType::add_quote("test");
		assert_eq!(MockType::add_quote("test"), "\"test\"");
		assert_eq!(&MockType::add_quote("test\ttest"), "\"test\ttest\"");
		assert_eq!(&MockType::add_quote("test\"test"), "\"test\"\"test\"");
	}
}
