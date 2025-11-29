use crate::field_description::Description;

// Use higher-ranked trait bound instead of a struct-level lifetime parameter.
pub struct Descriptor<T>(Box<dyn for<'a> Fn(&'a T) -> Description<'a> + 'static>);

impl<T, F> From<F> for Descriptor<T>
where
	// The closure must work for any lifetime 'a of &T, returning Description<'a>
	F: for<'a> Fn(&'a T) -> Description<'a> + 'static,
{
	fn from(f: F) -> Self {
		Descriptor(Box::new(f))
	}
}

impl<T> Descriptor<T> {
	pub fn with_hr_closure<F>(f: F) -> F
	where
		F: for<'a> Fn(&'a T) -> Description<'a>,
	{
		f
	}

	// describe is now generic over the borrow lifetime instead of tying it to the struct
	pub fn describe<'a>(&self, t: &'a T) -> Description<'a> {
		(self.0)(t)
	}
}

// ... existing code ...
#[cfg(test)]
mod test {
	use super::*;
	use crate::primitive_writer::QuoteMode;

	pub struct Sample(pub &'static str);

	#[test]
	fn test_descriptor() {
		// 解決策: クロージャではなく、明示的なシグネチャを持つ関数を定義する
		// これにより、HRTB（for<'a>）の条件を厳密に満たすことを保証します。

		// 関数ポインタとして渡す
		let desc = Descriptor::from(Descriptor::with_hr_closure(|x: &Sample| {
			Description::new_str(x.0, QuoteMode::Quoted)
		}));

		{
			let sample = Sample("test");
			let described = desc.describe(&sample);
			assert_eq!(described.value(), r##""test""##);
		}
	}
}
// ... existing code ...
