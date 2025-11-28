mod poc_selector;

use dsv_writer::field_description::Description;
use dsv_writer::primitive_writer::QuoteMode;
use poc_selector::*;

pub struct Point {
	x: i32,
	y: i32,
}

fn main() {
	let mut vec: Vec<Selector<Point>> = Vec::new();

	let x =
		Selector::new(|p: &Point| Description::new_string(p.x.to_string(), QuoteMode::AutoDetect));

	let y =
		Selector::new(|p: &Point| Description::new_string(p.y.to_string(), QuoteMode::AutoDetect));

	vec.push(x);
	vec.push(y);

	let point = Point { x: 1, y: 2 };

	for s in vec.iter() {
		println!("{:?}", s.select(&point));
	}

	println!("Hello, world!");
}
