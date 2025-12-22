#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

use crate::scalar::scalar;
use crate::shared::check_delimiter;
use crate::should_quote_datum::{ShouldQuoteDatum, ShouldQuoteResult};
use anyhow::{Result as AnyResult, anyhow};

pub fn should_quoted(target: &str, delimiter: char) -> ShouldQuoteResult {
	check_delimiter(delimiter)?;

	let esc: i32 = (delimiter as i32) | ((b'\r' as i32) << 8) | ((b'\n' as i32) << 16);

	let escape_vec: __m128i = unsafe { _mm_cvtsi32_si128(esc) };

	let double_quote_vec: __m128i = unsafe { _mm_set1_epi8(b'"' as i8) };

	let mut cursor = target.as_bytes();

	let mut result = ShouldQuoteDatum::new(false, false);
	while cursor.len() >= 16 {
		let chunk = unsafe { _mm_loadu_si128(cursor.as_ptr() as *const __m128i) };
		cursor = &cursor[16..];
		result |= simd(chunk, double_quote_vec, escape_vec);

		if result.double_quote() {
			return Ok(result);
		}
	}

	result |= scalar(cursor, delimiter as u8);

	Ok(result)
}

fn simd(target: __m128i, double_quote: __m128i, escape_chars: __m128i) -> ShouldQuoteDatum {
	const CONTROL: i32 = 0b00_00_00_00;

	let result = unsafe { _mm_cmpestri(target, 16, double_quote, 1, CONTROL) };
	if result != 16 {
		return ShouldQuoteDatum::new(true, true);
	}

	let result = unsafe { _mm_cmpestri(target, 16, escape_chars, 3, CONTROL) };
	if result != 16 {
		return ShouldQuoteDatum::new(true, false);
	}

	ShouldQuoteDatum::new(false, false)
}

#[cfg(test)]
mod tests {
	use super::*;

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

	#[test]
	fn simd_test() {
		let escape: [u8; 3] = [b'\r', b'\n', b'\t'];
		let dq: [u8; 1] = [b'"'];
		let mut input = b"0123456789ABCDEF";
		let vec_input = unsafe { _mm_loadu_si128(input.as_ptr() as *const __m128i) };
		let vec_escape = unsafe { _mm_loadu_si128(escape.as_ptr() as *const __m128i) };
		let vec_dq = unsafe { _mm_loadu_si128(dq.as_ptr() as *const __m128i) };

		let result = simd(vec_input, vec_dq, vec_escape);
		assert!(!result.should_quote());
		assert!(!result.double_quote());

		input = b"01234\r6789ABCDEF";
		let vec_input = unsafe { _mm_loadu_si128(input.as_ptr() as *const __m128i) };
		let result = simd(vec_input, vec_dq, vec_escape);
		assert!(result.should_quote());
		assert!(!result.double_quote());

		input = b"01234\t6789ABCDEF";
		let vec_input = unsafe { _mm_loadu_si128(input.as_ptr() as *const __m128i) };
		let result = simd(vec_input, vec_dq, vec_escape);
		assert!(result.should_quote());
		assert!(!result.double_quote());

		input = b"01234\n6789ABCDEF";
		let vec_input = unsafe { _mm_loadu_si128(input.as_ptr() as *const __m128i) };
		let result = simd(vec_input, vec_dq, vec_escape);
		assert!(result.should_quote());
		assert!(!result.double_quote());

		input = b"01234\"6789ABCDEF";
		let vec_input = unsafe { _mm_loadu_si128(input.as_ptr() as *const __m128i) };
		let result = simd(vec_input, vec_dq, vec_escape);
		assert!(result.should_quote());
		assert!(result.double_quote());
	}

	#[test]
	fn test_scalar() {
		let result = scalar(b"test", b't');
		assert!(result.should_quote());
		assert!(!result.double_quote());

		let result = scalar(b"test\"test", b't');
		assert!(result.should_quote());
		assert!(result.double_quote());
	}
}
