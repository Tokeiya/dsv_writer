use std::arch::x86_64::{
	__m128i, _mm_cmpeq_epi8, _mm_loadu_epi8, _mm_movemask_epi8, _mm_set1_epi8,
	_mm_set_epi8,
};
use std::slice;

mod cmp_mask;
mod cmpestri;
mod scalar;
mod shared;
pub mod should_quote_datum;
mod sample_gen;

const TXT: &str = "Have a nice day!";
fn main() {
	fn f(value: u16) -> String {
		let mut result = String::new();
		for i in 0..16 {
			result.push(if value & (1 << i) != 0 { '1' } else { '0' });

			if i % 4 == 3 {
				result.push('_');
			}
		}
		result
	}

	let cursor = TXT.as_bytes();

	let chunk = unsafe { _mm_loadu_epi8(cursor.as_ptr() as *const i8) };
	let mask = unsafe { _mm_set1_epi8(b'a' as i8) };

	let result = unsafe { _mm_cmpeq_epi8(chunk, mask) };
	let mask_result = unsafe { _mm_movemask_epi8(result) } as u16;
	println!("{}:{}", f(mask_result), mask_result);
}

fn tmp() {
	let vec = unsafe {
		_mm_set_epi8(
			0,
			1,
			2,
			4,
			8,
			16,
			32,
			64,
			128u8 as i8,
			3,
			7,
			15,
			31,
			63,
			127,
			255u8 as i8,
		)
	};

	let mask = unsafe { _mm_set1_epi8(0x00) };

	let result = unsafe { _mm_cmpeq_epi8(vec, mask) };

	let mask_result = unsafe { _mm_movemask_epi8(result) } as u16;

	let r_s: &[u8] = unsafe { slice::from_raw_parts(&result as *const __m128i as *const u8, 16) };
	let v_s: &[u8] = unsafe { slice::from_raw_parts(&vec as *const __m128i as *const u8, 16) };

	for idx in 0..16 {
		println!("{}:{}", v_s[15 - idx], r_s[15 - idx]);
	}

	println!("{:b}", mask_result);
}
