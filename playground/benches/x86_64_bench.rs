use criterion::{Criterion, criterion_group, criterion_main};
use playground::{cmp_mask, cmpestri, scalar};
use std::hint::black_box;

const N: usize = 10_000;

fn scalar(sample: &[String]) {
	let mut cnt = 0usize;
	for elem in sample {
		if scalar::should_quote(elem, '\t').unwrap().should_quote() {
			cnt += 1;
		}
	}

	black_box(cnt);
}

#[target_feature(enable = "sse4.1")]
fn cmp_mask(sample: &[String]) {
	let mut cnt = 0usize;
	for elem in sample {
		if cmp_mask::should_quote(elem, '\t').unwrap().should_quote() {
			cnt += 1;
		}
	}

	black_box(cnt);
}
#[target_feature(enable = "sse4.1")]
fn cmp_string(sample: &[String]) {
	let mut cnt = 0usize;
	for elem in sample {
		if cmpestri::should_quoted(elem, '\t').unwrap().should_quote() {
			cnt += 1;
		}
	}

	black_box(cnt);
}

pub fn benchmark(c: &mut Criterion) {
	let sample = playground::sample_generator::gen_sample(42, 1_000_0000, 20.0, 3.0, 0.1, 0.1);
	c.bench_function("scalar", |b| b.iter(|| scalar(sample.as_slice())));
	c.bench_function("cmp_mask", |b| {
		b.iter(|| unsafe { cmp_mask(sample.as_slice()) })
	});
	c.bench_function("cmp_string", |b| {
		b.iter(|| unsafe { cmp_string(&sample.as_slice()) })
	});
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
