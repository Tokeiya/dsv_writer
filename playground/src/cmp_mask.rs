use crate::scalar::scalar;
use crate::shared::check_delimiter;
use crate::should_quote_datum::{ShouldQuoteDatum, ShouldQuoteResult};
use std::arch::x86_64::{
	__m128i, _mm_cmpeq_epi8, _mm_loadu_si128, _mm_or_si128,
	_mm_set1_epi8, _mm_test_all_zeros,
};

#[target_feature(enable = "sse4.1")]
pub fn should_quote(target: &str, delimiter: char) -> ShouldQuoteResult {
	check_delimiter(delimiter)?;

	let mut cnt = target.len();
	let mut cursor = target.as_ptr();

	let dq_128 = _mm_set1_epi8(b'"' as i8);
	let delimiter_128 = _mm_set1_epi8(delimiter as i8);
	let new_line_128 = _mm_set1_epi8(b'\n' as i8);
	let carriage_return_128 = _mm_set1_epi8(b'\r' as i8);

	while cnt >= 16 {
		let chunk = unsafe{_mm_loadu_si128(cursor as *const __m128i)};

		let dq = _mm_cmpeq_epi8(chunk, dq_128);
		if _mm_test_all_zeros(dq, dq) == 0 {
			return Ok(ShouldQuoteDatum::new(true, true));
		}

		let delim_result = _mm_cmpeq_epi8(chunk, delimiter_128);
		let new_line_result = _mm_cmpeq_epi8(chunk, new_line_128);
		let carriage_return_result = _mm_cmpeq_epi8(chunk, carriage_return_128);

		let aggregation = _mm_or_si128(
			delim_result,
			_mm_or_si128(new_line_result, carriage_return_result),
		);

		if _mm_test_all_zeros(aggregation, aggregation) == 0 {
			return Ok(ShouldQuoteDatum::new(true, false));
		}

		cursor = unsafe{cursor.add(16)};
		cnt -= 16;
	}
	let cursor =unsafe{ std::slice::from_raw_parts(cursor, cnt)};
	Ok(scalar(cursor, delimiter as u8))
}

#[cfg(test)]
mod test {
	use super::*;
	
	const TXT: &str = "0123456789abcdef";

	#[test]
	fn should_quote_test() {
		unsafe {
			let result = should_quote("test", '\n');
			assert!(result.is_err());

			let result = should_quote("test", '"');
			assert!(result.is_err());

			let result = should_quote("test", '\r');
			assert!(result.is_err());

			let result = should_quote("test", '\t').unwrap();
			assert!(!result.should_quote());
			assert!(!result.double_quote());

			let result = should_quote("test\"test", '\t').unwrap();
			assert!(result.should_quote());
			assert!(result.double_quote());

			let result = should_quote("test\ttest", '\t').unwrap();
			assert!(result.should_quote());
			assert!(!result.double_quote());

			let result = should_quote("test\rtest", '\t').unwrap();
			assert!(result.should_quote());
			assert!(!result.double_quote());

			let result = should_quote("test\ntest", '\t').unwrap();
			assert!(result.should_quote());
			assert!(!result.double_quote());

			let result = should_quote("test\ntest\ntest", '\t').unwrap();
			assert!(result.should_quote());

			let result = should_quote(
				"それほどジェスチャー必須でない場面でもサービスするDEEP DIVE理事🦀",
				'\t',
			)
			.unwrap();
			assert!(!result.should_quote());
			assert!(!result.double_quote());

			let result = should_quote(
				"それほどジェスチャー必須でない場面でも\"サービスするDEEP DIVE理事🦀",
				'\t',
			)
			.unwrap();
			assert!(result.should_quote());
			assert!(result.double_quote());

			let result = should_quote(
				"それほどジェスチャー必須でない場面でも\tサービスするDEEP DIVE理事🦀",
				'\t',
			)
			.unwrap();
			assert!(result.should_quote());
			assert!(!result.double_quote());
		}
	}
}
