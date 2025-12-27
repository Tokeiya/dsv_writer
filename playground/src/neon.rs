use crate::scalar::scalar;
use crate::shared::check_delimiter;
use crate::should_quote_datum::{ShouldQuoteDatum, ShouldQuoteResult};
#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::*;

#[cfg(target_arch = "aarch64")]
#[inline(always)]
fn any_match(mask: uint8x16_t) -> bool {
	// vceqq_u8 の結果は一致なら 0xFF、不一致なら 0x00
	// どれか1バイトでも 0xFF があれば vmaxvq_u8 は 0xFF になる
	unsafe { vmaxvq_u8(mask) != 0 }
}

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
pub fn should_quote(target: &str, delimiter: char) -> ShouldQuoteResult {
	check_delimiter(delimiter)?;

	let bytes = target.as_bytes();
	let mut cnt = bytes.len();
	let mut cursor = bytes.as_ptr();

	let dq_128: uint8x16_t = unsafe { vdupq_n_u8(b'"') };
	let delimiter_128: uint8x16_t = unsafe { vdupq_n_u8(delimiter as u8) };
	let new_line_128: uint8x16_t = unsafe { vdupq_n_u8(b'\n') };
	let carriage_return_128: uint8x16_t = unsafe { vdupq_n_u8(b'\r') };

	while cnt >= 16 {
		// AArch64 の vld1q_u8 は未整列ロードOK
		let chunk: uint8x16_t = unsafe { vld1q_u8(cursor) };

		// '"' があれば should_quote=true, should_escape=true
		let dq = vceqq_u8(chunk, dq_128);
		if any_match(dq) {
			return Ok(ShouldQuoteDatum::new(true, true));
		}

		// delimiter or '\n' or '\r' があれば should_quote=true, should_escape=false
		let delim = vceqq_u8(chunk, delimiter_128);
		let nl = vceqq_u8(chunk, new_line_128);
		let cr = vceqq_u8(chunk, carriage_return_128);

		let aggregation = vorrq_u8(delim, vorrq_u8(nl, cr));
		if any_match(aggregation) {
			return Ok(ShouldQuoteDatum::new(true, false));
		}

		cursor = unsafe { cursor.add(16) };
		cnt -= 16;
	}

	let tail = unsafe { core::slice::from_raw_parts(cursor, cnt) };
	Ok(scalar(tail, delimiter as u8))
}

mod tests {
	use super::*;

	#[test]
	fn should_quote_test() {
		// let result = should_quote("test", '\n');
		// assert!(result.is_err());
		//
		// let result = should_quote("test", '"');
		// assert!(result.is_err());
		//
		// let result = should_quote("test", '\r');
		// assert!(result.is_err());
		//
		// let result = should_quote("test", '\t').unwrap();
		// assert!(!result.should_quote());
		// assert!(!result.double_quote());
		//
		// let result = should_quote("test\"test", '\t').unwrap();
		// assert!(result.should_quote());
		// assert!(result.double_quote());
		//
		// let result = should_quote("test\ttest", '\t').unwrap();
		// assert!(result.should_quote());
		// assert!(!result.double_quote());
		//
		// let result = should_quote("test\rtest", '\t').unwrap();
		// assert!(result.should_quote());
		// assert!(!result.double_quote());
		//
		// let result = should_quote("test\ntest", '\t').unwrap();
		// assert!(result.should_quote());
		// assert!(!result.double_quote());
		//
		// let result = should_quote("test\ntest\ntest", '\t').unwrap();
		// assert!(result.should_quote());
		//
		// let result = should_quote(
		// 	"それほどジェスチャー必須でない場面でもサービスするDEEP DIVE理事🦀",
		// 	'\t',
		// )
		// .unwrap();
		// assert!(!result.should_quote());
		// assert!(!result.double_quote());
		//
		// let result = should_quote(
		// 	"それほどジェスチャー必須でない場面でも\"サービスするDEEP DIVE理事🦀",
		// 	'\t',
		// )
		// .unwrap();
		// assert!(result.should_quote());
		// assert!(result.double_quote());

		let result = unsafe {
			should_quote(
				"それほどジェスチャー必須でない場面でも\tサービスするDEEP DIVE理事🦀",
				'\t',
			)
			.unwrap()
		};
		assert!(result.should_quote());
		assert!(!result.double_quote());
	}
}
