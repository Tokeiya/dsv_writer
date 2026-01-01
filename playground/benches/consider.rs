use criterion::{criterion_group, criterion_main};
use playground::quote_masks::QuoteMasks;
use playground::scalar::scalar;
use playground::shared::check_delimiter;
use playground::should_quote_datum::{ShouldQuoteDatum, ShouldQuoteResult};
use std::arch::x86_64::{
	__m128i, _mm_cmpeq_epi8, _mm_loadu_si128, _mm_or_si128, _mm_set1_epi8, _mm_test_all_zeros,
};
use std::hint::black_box;

#[inline(always)]
pub fn is_contain(target: __m128i, mask: __m128i) -> bool {
	let dq = unsafe { _mm_cmpeq_epi8(target, mask) };
	(unsafe { _mm_test_all_zeros(dq, dq) } == 0)
}

#[inline(always)]
pub fn should_quote_impl_a(target: __m128i, quote_masks: QuoteMasks) -> ShouldQuoteResult {
	if is_contain(target, quote_masks.dq) {
		return Ok(ShouldQuoteDatum::new(true, true));
	}

	let delim_result = unsafe { _mm_cmpeq_epi8(target, quote_masks.delim) };
	let new_line_result = unsafe { _mm_cmpeq_epi8(target, quote_masks.nl) };
	let carriage_return_result = unsafe { _mm_cmpeq_epi8(target, quote_masks.cr) };

	let aggregation = unsafe {
		_mm_or_si128(
			delim_result,
			_mm_or_si128(new_line_result, carriage_return_result),
		)
	};
	if unsafe { _mm_test_all_zeros(aggregation, aggregation) == 0 } {
		Ok(ShouldQuoteDatum::new(true, false))
	} else {
		Ok(ShouldQuoteDatum::new(false, false))
	}
}

#[inline(always)]
pub fn should_quote_impl_b(target: __m128i, quote_masks: QuoteMasks) -> ShouldQuoteResult {
	if is_contain(target, quote_masks.dq) {
		return Ok(ShouldQuoteDatum::new(true, true));
	}

	let delim_result = is_contain(target, quote_masks.delim);
	let new_line_result = is_contain(target, quote_masks.nl);
	let carriage_return_result = is_contain(target, quote_masks.cr);

	let aggregation = delim_result || new_line_result || carriage_return_result;

	if aggregation {
		Ok(ShouldQuoteDatum::new(true, false))
	} else {
		Ok(ShouldQuoteDatum::new(false, false))
	}
}

/// # Safety
#[target_feature(enable = "sse4.1")]
pub fn should_quote_a(target: &str, delimiter: char) -> ShouldQuoteResult {
	check_delimiter(delimiter)?;

	let mut cnt = target.len();
	let mut cursor = target.as_ptr();

	let quote_masks = QuoteMasks::new(delimiter);

	while cnt >= 16 {
		let chunk = unsafe { _mm_loadu_si128(cursor as *const __m128i) };

		let result = should_quote_impl_a(chunk, quote_masks)?;

		if result.should_quote() || result.double_quote() {
			return Ok(result);
		}

		cursor = unsafe { cursor.add(16) };
		cnt -= 16;
	}
	let cursor = unsafe { std::slice::from_raw_parts(cursor, cnt) };
	Ok(scalar(cursor, delimiter as u8))
}

/// # Safety
#[target_feature(enable = "sse4.1")]
pub fn should_quote_b(target: &str, delimiter: char) -> ShouldQuoteResult {
	check_delimiter(delimiter)?;

	let mut cnt = target.len();
	let mut cursor = target.as_ptr();

	let quote_masks = QuoteMasks::new(delimiter);

	while cnt >= 16 {
		let chunk = unsafe { _mm_loadu_si128(cursor as *const __m128i) };

		let result = should_quote_impl_b(chunk, quote_masks)?;

		if result.should_quote() || result.double_quote() {
			return Ok(result);
		}

		cursor = unsafe { cursor.add(16) };
		cnt -= 16;
	}
	let cursor = unsafe { std::slice::from_raw_parts(cursor, cnt) };
	Ok(scalar(cursor, delimiter as u8))
}

pub fn f_a(sample: &[String]) {
	let mut cnt = 0usize;
	for elem in sample {
		if unsafe { should_quote_a(elem, '\t') }
			.unwrap()
			.should_quote()
		{
			cnt += 1;
		}
	}
	black_box(cnt);
}

pub fn f_b(sample: &[String]) {
	let mut cnt = 0usize;
	for elem in sample {
		if unsafe { should_quote_b(elem, '\t') }
			.unwrap()
			.should_quote()
		{
			cnt += 1;
		}
	}
	black_box(cnt);
}

pub fn benchmark(c: &mut criterion::Criterion) {
	let sample = playground::sample_generator::gen_sample(114514, 1_000_000, 20.0, 3.0, 0.1, 0.1);

	c.bench_function("should_quote_a", |b| {
		b.iter(|| {
			f_a(&sample);
		});
	});

	c.bench_function("should_quote_b", |b| {
		b.iter(|| {
			f_b(&sample);
		});
	});
}

criterion_group!(consider, benchmark);
criterion_main!(consider);
