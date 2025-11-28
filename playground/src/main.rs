mod field_datum;
mod poc_descriptor;
mod poc_selector;

fn main() {
	let a = "hello".as_bytes();
	let b = "hello".as_bytes();

	let ant = a.eq(b);
}
