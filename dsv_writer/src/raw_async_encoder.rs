use crate::quote_mode::QuoteMode;
use futures::io::{AsyncWrite, Error as IoError, Result as IoResult};
use std::borrow::Cow;

pub type StrCow<'a> = Cow<'a, str>;

pub trait RawAsyncEncode {
	async fn write_str_field(&mut self, value: &str, quote_mode: QuoteMode) -> IoResult<usize>;
	async fn end_of_record(&mut self, should_flush: bool) -> IoResult<usize>;

	async fn write_string_field(
		&mut self,
		value: String,
		quote_mode: QuoteMode,
	) -> IoResult<usize> {
		todo!()
	}
	async fn write_value_field<T: ToString>(
		&mut self,
		value: &T,
		quote_mode: QuoteMode,
	) -> IoResult<usize> {
		todo!()
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
	use futures::io::{AsyncWriteExt, IoSlice};
	use std::borrow::Cow;
	use std::collections::HashSet;

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
		pub fn clear(&mut self) {
			self.buff.clear();
		}

		pub fn raw(&self) -> &[u8] {
			&self.buff
		}

		pub fn to_str(&self) -> Cow<'_, str> {
			String::from_utf8_lossy(&self.buff)
		}
	}

	impl<T> MockWriter<T> {
		pub fn should_quote(&self, value: &str) -> bool {
			value.chars().any(|c| self.set.contains(&c))
		}
	}

	impl<T: AsyncWrite + Unpin> RawAsyncEncode for MockWriter<T> {
		async fn write_str_field(&mut self, value: &str, quote_mode: QuoteMode) -> IoResult<usize> {
			if self.cnt != 0 {
				self.buff.write_all(&[b'\t']).await?;
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
			Ok(scr.as_bytes().len())
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

	#[test]
	fn add_quote_test() {
		type MockType = MockWriter<Vec<u8>>;

		dbg!(MockType::add_quote("te\"st"));

		let actual = MockType::add_quote("test");
		dbg!(&actual);
		assert_eq!(MockType::add_quote("test"), "\"test\"");
		assert_eq!(&MockType::add_quote("test\ttest"), "\"test\ttest\"");
		assert_eq!(&MockType::add_quote("test\"test"), "\"test\"\"test\"");
	}
}
