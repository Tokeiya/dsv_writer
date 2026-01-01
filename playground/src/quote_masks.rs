#![cfg(target_arch = "x86_64")]

use std::arch::x86_64::{__m128i, _mm_set1_epi8};

#[derive(Debug, Clone, Copy)]
pub struct QuoteMasks {
	pub dq: __m128i,
	pub delim: __m128i,
	pub nl: __m128i,
	pub cr: __m128i,
}

impl QuoteMasks {
	#[inline(always)]
	pub fn new(delimiter: char) -> Self {
		Self {
			dq: unsafe { _mm_set1_epi8(b'"' as i8) },
			delim: unsafe { _mm_set1_epi8(delimiter as i8) },
			nl: unsafe { _mm_set1_epi8(b'\n' as i8) },
			cr: unsafe { _mm_set1_epi8(b'\r' as i8) },
		}
	}
}
