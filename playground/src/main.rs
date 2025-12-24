use playground::sample_generator;

use std::arch::x86_64::{
	__m128i, _mm_cmpeq_epi8, _mm_loadu_epi8, _mm_loadu_si128, _mm_movemask_epi8, _mm_set_epi8,
	_mm_set1_epi8,
};
use std::collections::HashSet;
use std::slice;

const TXT: &str = "Have a nice day!";
fn main() {
	let vec = sample_generator::gen_sample(114514, 2_000_000, 30.0, 5.0, 0.1, 0.1);

	for elem in vec.iter() {
		println!("-------------------------");
		println!("{}", elem);
		println!("-------------------------");
	}

	println!("{}", vec.iter().map(|s| s.len()).sum::<usize>())
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
