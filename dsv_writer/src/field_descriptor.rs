use crate::field_description::Description;

// Use higher-ranked trait bound instead of a struct-level lifetime parameter.
pub struct Descriptor<T>(Box<dyn Fn(&T) -> Description + 'static>);

impl<T, F> From<F> for Descriptor<T>
where
	// The closure must work for any lifetime 'a of &T, returning Description<'a>
	F: Fn(&T) -> Description + 'static,
{
	fn from(f: F) -> Self {
		Descriptor(Box::new(f))
	}
}

impl<T> Descriptor<T> {
	pub fn describe(&self, t: &T) -> Description {
		(self.0)(t)
	}
}

// ... existing code ...
#[cfg(test)]
mod test {
	use super::*;
	
	pub struct Sample(pub &'static str);

	#[test]
	fn test_descriptor() {
		todo!()
	}
}
// ... existing code ...
