use crate::field_description::Description;

// Use higher-ranked trait bound instead of a struct-level lifetime parameter.
pub struct Descriptor<'a, T>(Box<dyn Fn(&T) -> Description + 'a>);

impl<'a, T, F> From<F> for Descriptor<'a, T>
where
	// The closure must work for any lifetime 'a of &T, returning Description<'a>
	F: Fn(&T) -> Description + 'a,
{
	fn from(f: F) -> Self {
		Descriptor(Box::new(f))
	}
}

impl<'a, T> Descriptor<'a, T> {
	pub fn describe(&self, t: &T) -> Description {
		self.0(t)
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
		let d = Descriptor::from(|s: &Sample| Description::new_str(s.0, QuoteMode::Quoted));
		let actual = d.describe(&Sample("hello"));

		assert_eq!(actual.value(), "hello");
		assert!(matches!(actual.quoted_mode(), QuoteMode::Quoted));

		let d = Descriptor::from(|s: &Sample| {
			Description::new_string(s.0.to_string(), QuoteMode::AutoDetect)
		});

		let actual = d.describe(&Sample("hello"));
		assert_eq!(actual.value(), "hello");
		assert!(matches!(actual.quoted_mode(), QuoteMode::AutoDetect));
	}
}
// ... existing code ...
