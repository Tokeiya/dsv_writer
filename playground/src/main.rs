mod cmp_mask;
mod cmpestri;
mod scalar;
mod shared;
pub mod should_quote_datum;

use std::pin::Pin;

pub trait AsyncFoo {
	fn foo(&self) -> impl Future<Output = i32> + Send;
}
pub trait DynAsyncFoo {
	fn foo(&self) -> Pin<Box<dyn Future<Output = i32> + Send + '_>>;
}

impl<T: AsyncFoo + Send> DynAsyncFoo for T {
	fn foo(&self) -> Pin<Box<dyn Future<Output = i32> + Send + '_>> {
		Box::pin(self.foo())
	}
}
fn main() {}
