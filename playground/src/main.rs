enum Foo {
	Hoge,
	Piyo,
}

fn main() {}

fn f(a: &Foo) {
	if let Foo::Hoge = a {}
}
