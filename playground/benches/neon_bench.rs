use criterion::{criterion_group, criterion_main};
#[cfg(target_arch = "aarch64")]
pub mod neon {
	use criterion::{criterion_group, criterion_main};
	use playground::{neon, scalar};
	use std::hint::black_box;

	fn scalar(sample: &[String]) {
		let mut cnt = 0usize;
		for elem in sample {
			if scalar::should_quote(elem, '\t').unwrap().should_quote() {
				cnt += 1;
			}
		}

		black_box(cnt);
	}

	fn neon(sample: &[String]) {
		unsafe {
			let mut cnt = 0usize;
			for elem in sample {
				if neon::should_quote(elem, '\t').unwrap().should_quote() {
					cnt += 1;
				}
			}

			black_box(cnt);
		}
	}

	pub fn benchmark(c: &mut criterion::Criterion) {
		let sample = playground::sample_generator::gen_sample(42, 1_000_0000, 20.0, 3.0, 0.1, 0.1);
		c.bench_function("scalar", |b| b.iter(|| scalar(sample.as_slice())));
		c.bench_function("neon", |b| b.iter(|| neon(sample.as_slice())));
	}
}

#[cfg(target_arch = "aarch64")]
criterion_group!(benches, neon::benchmark);

#[cfg(target_arch = "aarch64")]
criterion_main!(benches);

#[cfg(target_arch = "x86_64")]
fn main() {
	println!("This is x86_64 bench dummy");
}
