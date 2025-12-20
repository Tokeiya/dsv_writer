use crate::shared::check_delimiter;
use crate::should_quote_datum::{ShouldQuoteDatum, ShouldQuoteResult};

pub fn should_quote(target: &str, delimiter: char) -> ShouldQuoteResult {
	check_delimiter(delimiter)?;

	Ok(scalar(target.as_bytes(), delimiter as u8))
}

pub fn scalar(target: &[u8], delimiter: u8) -> ShouldQuoteDatum {
	let mut should_quote = false;
	let mut contain_dq = false;

	for elem in target {
		should_quote |= *elem == b'\r' || *elem == b'\n' || *elem == delimiter;

		if *elem == b'"' {
			contain_dq = true;
			should_quote = true;
			break;
		}
	}

	ShouldQuoteDatum::new(should_quote, contain_dq)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::cmpestri::should_quoted;

	#[test]
	fn should_quote_test() {
		let result = should_quoted("test", '\n');
		assert!(result.is_err());

		let result = should_quoted("test", '"');
		assert!(result.is_err());

		let result = should_quoted("test", '\r');
		assert!(result.is_err());

		let result = should_quoted("test", '\t').unwrap();
		assert!(!result.should_quote());
		assert!(!result.double_quote());

		let result = should_quoted("test\"test", '\t').unwrap();
		assert!(result.should_quote());
		assert!(result.double_quote());

		let result = should_quoted("test\ttest", '\t').unwrap();
		assert!(result.should_quote());
		assert!(!result.double_quote());

		let result = should_quoted("test\rtest", '\t').unwrap();
		assert!(result.should_quote());
		assert!(!result.double_quote());

		let result = should_quoted("test\ntest", '\t').unwrap();
		assert!(result.should_quote());
		assert!(!result.double_quote());

		let result = should_quoted("test\ntest\ntest", '\t').unwrap();
		assert!(result.should_quote());

		let result = should_quoted(
			"それほどジェスチャー必須でない場面でもサービスするDEEP DIVE理事🦀",
			'\t',
		)
		.unwrap();
		assert!(!result.should_quote());
		assert!(!result.double_quote());

		let result = should_quoted(
			"それほどジェスチャー必須でない場面でも\"サービスするDEEP DIVE理事🦀",
			'\t',
		)
		.unwrap();
		assert!(result.should_quote());
		assert!(result.double_quote());

		let result = should_quoted(
			"それほどジェスチャー必須でない場面でも\tサービスするDEEP DIVE理事🦀",
			'\t',
		)
		.unwrap();
		assert!(result.should_quote());
		assert!(!result.double_quote());
	}
}
