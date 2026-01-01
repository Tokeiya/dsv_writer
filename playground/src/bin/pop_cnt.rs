use std::arch::x86_64::{_popcnt32, _popcnt64};

fn main() {
	for i in 0..128 {
		println!("{i}:{}", unsafe { _popcnt32(i) })
	}
}
