use std::arch::x86_64::{
	__m128i, _mm_add_epi8, _mm_and_si128, _mm_cvtsi128_si64, _mm_extract_epi64, _mm_sad_epu8,
	_mm_set1_epi8, _mm_setr_epi8, _mm_setzero_si128, _mm_shuffle_epi8, _mm_srli_epi16,
};

fn popcnt_m128i_simd(v: __m128i) -> u32 {
	let lut = unsafe { _mm_setr_epi8(0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4) };

	//Create the nibble mask.
	let low_mask = unsafe { _mm_set1_epi8(0x0f) };

	//extract the low nibble.
	let lo = unsafe { _mm_and_si128(v, low_mask) };

	//extract the high nibble.
	let hi = unsafe { _mm_and_si128(_mm_srli_epi16(v, 4), low_mask) };

	//count the low nibble with LUT
	let cnt_lo = unsafe { _mm_shuffle_epi8(lut, lo) };

	//count the high nibble with LUT
	let cnt_hi = unsafe { _mm_shuffle_epi8(lut, hi) };

	//add the low and high nibble counts
	let sum = unsafe { _mm_add_epi8(cnt_lo, cnt_hi) };

	// horizontal sum of bytes to get the total popcount
	let sum16 = unsafe { _mm_sad_epu8(sum, _mm_setzero_si128()) };
	(unsafe { _mm_cvtsi128_si64(sum16) + _mm_extract_epi64(sum16, 1) }) as u32
}

fn main() {
	let target = unsafe { _mm_setr_epi8(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15) };
	println!("{:?}", popcnt_m128i_simd(target));
}
