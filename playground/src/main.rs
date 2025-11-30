enum Foo {
	Hoge,
	Piyo,
}

fn main() {
	let s = r##""""##.to_string();
	println!("{}", &s);

	println!("{}", s.as_str().replace("\"", r#""""#))
}

fn f(a: &Foo) {
	if let Foo::Hoge = a {}
}
