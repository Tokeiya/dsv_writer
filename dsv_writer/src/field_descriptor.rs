use crate::field_description::Description;

// Use higher-ranked trait bound instead of a struct-level lifetime parameter.
pub struct Descriptor<T>(Box<dyn for<'a> Fn(&'a T) -> Description<'a>>);

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
		fn describe_sample(x: &Sample) -> Description<'_> {
			Description::new_str(x.0, QuoteMode::AutoDetect)
		}

		// 関数ポインタとして渡す
		let desc = Descriptor::from(describe_sample);

		{
			let sample = Sample("test");
			let described = desc.describe(&sample);
			assert_eq!(described.value(), r##""test""##);
		}
	}
}
// ... existing code ...
