use super::error::{Error, Result};

pub enum QuoteMode {
	Quoted,
	AutoDetect,
}

pub trait Writer {
	fn easy_write_str(&mut self, value: &str) -> Result<usize> {
		todo!()
	}
	fn easy_write<T: ToString>(&mut self, value: &T) -> Result<usize> {
		todo!()
	}

	fn write<T: ToString>(&mut self, value: &T, quote: QuoteMode) -> Result<usize> {
		todo!()
	}
	fn write_str(&mut self, value: &str, quote: QuoteMode) -> Result<usize>;
	fn quote_required(&self, value: &str) -> bool;
	fn delimiter(&self) -> char;
}
