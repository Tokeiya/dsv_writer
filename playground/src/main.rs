#[cfg(target_arch = "x86_64")]
pub mod cmp_mask;
#[cfg(target_arch = "x86_64")]
pub mod cmpestri;

pub mod sample_generator;
pub mod scalar;
pub mod shared;
pub mod should_quote_datum;

#[cfg(target_arch = "aarch64")]
pub mod neon;

const TXT: &str = "Have a nice day!";
fn main() {
	let vec = sample_generator::gen_sample(114514, 2_000_000, 30.0, 5.0, 0.1, 0.1);

	for elem in vec.iter().take(10) {
		println!("-------------------------");
		println!("{}", elem);
		println!("-------------------------");
	}

	println!("{}", vec.iter().map(|s| s.len()).sum::<usize>())
}
