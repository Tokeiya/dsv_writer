use crate::primitive_writer::QuoteMode;

#[derive(Debug)]
pub struct Description {
	value: String,
	quoted_mode: QuoteMode,
}

impl Description {
	pub fn from_str(value: &str, quoted_mode: QuoteMode) -> Self {
		Self {
			value: value.to_string(),
			quoted_mode,
		}
	}

	pub fn from_string(value: String, quoted_mode: QuoteMode) -> Self {
		Self { value, quoted_mode }
	}

	pub fn from_value<T: ToString>(value: &T, quote_mode: QuoteMode) -> Self {
		Self::from_string(value.to_string(), quote_mode)
	}

	pub fn quoted_mode(&self) -> &QuoteMode {
		&self.quoted_mode
	}

	pub fn value(&self) -> &str {
		&self.value
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_description() {
		let d = Description::from_str("hello", QuoteMode::Quoted);
		assert_eq!(d.value(), "hello");
		assert!(matches!(d.quoted_mode(), QuoteMode::Quoted));
	}

	#[test]
	fn test_from_value() {
		let d = Description::from_value(&123, QuoteMode::Quoted);
		assert_eq!(d.value(), "123");
		assert!(matches!(d.quoted_mode(), QuoteMode::Quoted));
	}

	#[test]
	fn test_from_string() {
		let d = Description::from_value(&"hello".to_string(), QuoteMode::AutoDetect);
		assert_eq!(d.value(), "hello");
		assert!(matches!(d.quoted_mode(), QuoteMode::AutoDetect));
	}

	#[test]
	fn test_from_str() {
		let d = Description::from_str("hello", QuoteMode::AutoDetect);
		assert_eq!(d.value(), "hello");
		assert!(matches!(d.quoted_mode(), QuoteMode::AutoDetect));
	}
}
