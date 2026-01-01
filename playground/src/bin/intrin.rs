// use playground::scalar::scalar;
// use playground::shared::check_delimiter;
// use playground::should_quote_datum::{ShouldQuoteDatum, ShouldQuoteResult};
// use std::arch::x86_64::{
// 	__m128i, _mm_cmpeq_epi8, _mm_loadu_si128, _mm_or_si128, _mm_popcnt_epi8, _mm_set1_epi8,
// 	_mm_storeu_si128, _mm_test_all_zeros,
// };
//
// use playground::quote_masks::QuoteMasks;
//
// #[inline(always)]
// pub fn is_contain(target: __m128i, mask: __m128i) -> bool {
// 	let dq = unsafe { _mm_cmpeq_epi8(target, mask) };
// 	(unsafe { _mm_test_all_zeros(dq, dq) } == 0)
// }
//
// #[inline(always)]
// pub fn raw_copy(scr: __m128i, target: &mut Vec<u8>) {
// 	if target.capacity() - target.len() < 16 {
// 		target.reserve(1024);
// 	}
//
// 	let ptr = target.as_mut_ptr();
//
// 	unsafe {
// 		let dst = ptr.add(target.len());
// 		_mm_storeu_si128(dst as *mut __m128i, scr);
// 		target.set_len(target.len() + 16);
// 	}
// }
//
// #[inline(always)]
// pub fn escape_and_add(scr: __m128i, dq: __m128i, target: &mut Vec<u8>) {
// 	let a = _mm_popcnt_epi8(scr);
// }
//
// #[inline(always)]
// pub fn should_quote_impl(target: __m128i, quote_masks: QuoteMasks) -> ShouldQuoteResult {
// 	if is_contain(target, quote_masks.dq) {
// 		return Ok(ShouldQuoteDatum::new(true, true));
// 	}
//
// 	let delim_result = unsafe { _mm_cmpeq_epi8(target, quote_masks.delim) };
// 	let new_line_result = unsafe { _mm_cmpeq_epi8(target, quote_masks.nl) };
// 	let carriage_return_result = unsafe { _mm_cmpeq_epi8(target, quote_masks.cr) };
//
// 	let aggregation = unsafe {
// 		_mm_or_si128(
// 			delim_result,
// 			_mm_or_si128(new_line_result, carriage_return_result),
// 		)
// 	};
// 	if unsafe { _mm_test_all_zeros(aggregation, aggregation) == 0 } {
// 		Ok(ShouldQuoteDatum::new(true, false))
// 	} else {
// 		Ok(ShouldQuoteDatum::new(false, false))
// 	}
// }
//
// /// # Safety
// #[target_feature(enable = "sse4.1")]
// pub fn should_quote(target: &str, delimiter: char) -> ShouldQuoteResult {
// 	check_delimiter(delimiter)?;
//
// 	let mut cnt = target.len();
// 	let mut cursor = target.as_ptr();
//
// 	let quote_masks = QuoteMasks::new(delimiter);
//
// 	while cnt >= 16 {
// 		let chunk = unsafe { _mm_loadu_si128(cursor as *const __m128i) };
//
// 		let result = should_quote_impl(chunk, quote_masks)?;
//
// 		if result.should_quote() || result.double_quote() {
// 			return Ok(result);
// 		}
//
// 		cursor = unsafe { cursor.add(16) };
// 		cnt -= 16;
// 	}
// 	let cursor = unsafe { std::slice::from_raw_parts(cursor, cnt) };
// 	Ok(scalar(cursor, delimiter as u8))
// }
//
// fn main() {}
//
// #[cfg(test)]
// mod test {
// 	use super::*;
//
// 	#[test]
// 	fn should_quote_test() {
// 		unsafe {
// 			let result = playground::cmp_mask::should_quote("test", '\n');
// 			assert!(result.is_err());
//
// 			let result = playground::cmp_mask::should_quote("test", '"');
// 			assert!(result.is_err());
//
// 			let result = playground::cmp_mask::should_quote("test", '\r');
// 			assert!(result.is_err());
//
// 			let result = playground::cmp_mask::should_quote("test", '\t').unwrap();
// 			assert!(!result.should_quote());
// 			assert!(!result.double_quote());
//
// 			let result = playground::cmp_mask::should_quote("test\"test", '\t').unwrap();
// 			assert!(result.should_quote());
// 			assert!(result.double_quote());
//
// 			let result = playground::cmp_mask::should_quote("test\ttest", '\t').unwrap();
// 			assert!(result.should_quote());
// 			assert!(!result.double_quote());
//
// 			let result = playground::cmp_mask::should_quote("test\rtest", '\t').unwrap();
// 			assert!(result.should_quote());
// 			assert!(!result.double_quote());
//
// 			let result = playground::cmp_mask::should_quote("test\ntest", '\t').unwrap();
// 			assert!(result.should_quote());
// 			assert!(!result.double_quote());
//
// 			let result = playground::cmp_mask::should_quote("test\ntest\ntest", '\t').unwrap();
// 			assert!(result.should_quote());
//
// 			let result = playground::cmp_mask::should_quote(
// 				"それほどジェスチャー必須でない場面でもサービスするDEEP DIVE理事🦀",
// 				'\t',
// 			)
// 			.unwrap();
// 			assert!(!result.should_quote());
// 			assert!(!result.double_quote());
//
// 			let result = playground::cmp_mask::should_quote(
// 				"それほどジェスチャー必須でない場面でも\"サービスするDEEP DIVE理事🦀",
// 				'\t',
// 			)
// 			.unwrap();
// 			assert!(result.should_quote());
// 			assert!(result.double_quote());
//
// 			let result = playground::cmp_mask::should_quote(
// 				"それほどジェスチャー必須でない場面でも\tサービスするDEEP DIVE理事🦀",
// 				'\t',
// 			)
// 			.unwrap();
// 			assert!(result.should_quote());
// 			assert!(!result.double_quote());
// 		}
// 	}
// }
fn main() {}
