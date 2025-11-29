use crate::field_description::Description;
use crate::field_descriptor::Descriptor as FieldDescriptor;

pub struct Descriptor<'a, T>(Vec<FieldDescriptor<'a, T>>);

impl<'a, T> Default for Descriptor<'a, T> {
	fn default() -> Self {
		Self(Vec::new())
	}
}

impl<'a, T> Descriptor<'a, T> {
	pub fn push<F: Fn(&T) -> Description + 'a>(&mut self, f: F) -> usize {
		self.0.push(FieldDescriptor::from(f));
		self.0.len()
	}

	pub fn describe(&self, value: &T) -> Vec<Description> {
		self.0.iter().map(|f| f.describe(value)).collect()
	}

	pub fn fill(&self, value: &T, buff: &mut Vec<Description>) -> usize {
		buff.clear();
		self.append(value, buff)
	}

	pub fn append(&self, value: &T, buff: &mut Vec<Description>) -> usize {
		for d in self.0.iter() {
			buff.push(d.describe(value));
		}

		self.0.len()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::primitive_writer::QuoteMode;
	use crate::primitive_writer::QuoteMode::{AutoDetect, Quoted};
	
	struct Sample {
		pub int: i32,
		pub string: String,
		pub str: &'static str,
	}

	impl Sample {
		fn gen_sample() -> Sample {
			Sample {
				int: 42,
				string: String::from("hello"),
				str: "ref str",
			}
		}
	}

	#[test]
	fn default_test() {
		let sample = Descriptor::<Sample>::default();
		assert_eq!(sample.0.len(), 0);
	}

	#[test]
	fn push_test() {
		let mut fixture = Descriptor::<Sample>::default();
		fixture
			.push(|x| Description::new_str(x.str, crate::primitive_writer::QuoteMode::AutoDetect));

		let sample = Sample::gen_sample();

		let actual = fixture.describe(&sample);
		assert_eq!(actual.len(), 1);
		assert_eq!(actual[0].value(), "ref str");
		assert!(matches!(
			actual[0].quoted_mode(),
			crate::primitive_writer::QuoteMode::AutoDetect
		));

		fixture.push(|x| Description::new_string(x.int.to_string(), QuoteMode::AutoDetect));
		let actual = fixture.describe(&sample);
		assert_eq!(actual.len(), 2);
		assert_eq!(actual[1].value(), "42");
		assert!(matches!(actual[1].quoted_mode(), QuoteMode::AutoDetect));
	}

	#[test]
	fn describe_test() {
		let mut fixture = Descriptor::<Sample>::default();

		let sample = Sample::gen_sample();
		let actual = fixture.describe(&sample);
		assert_eq!(actual.len(), 0);

		fixture.push(|x| Description::new_str(x.str, AutoDetect));
		let actual = fixture.describe(&sample);
		assert_eq!(actual.len(), 1);
		assert_eq!(actual[0].value(), "ref str");

		fixture.push(|x| Description::new_string(x.int.to_string(), AutoDetect));
		let actual = fixture.describe(&sample);
		assert_eq!(actual.len(), 2);
		assert_eq!(actual[1].value(), "42");

		fixture.push(|x| Description::new_string(x.string.clone(), AutoDetect));
		let actual = fixture.describe(&sample);
		assert_eq!(actual.len(), 3);
		assert_eq!(actual[2].value(), "hello");
	}

	#[test]
	fn fill_test() {
		let mut fixture = Descriptor::<Sample>::default();
		let sample = Sample::gen_sample();
		let mut buff: Vec<Description> = Vec::new();

		let n = fixture.fill(&sample, &mut buff);
		assert_eq!(n, 0);
		assert_eq!(buff.len(), 0);

		fixture.push(|x| Description::new_str(x.str, AutoDetect));
		let n = fixture.fill(&sample, &mut buff);
		assert_eq!(n, 1);
		assert_eq!(buff.len(), 1);
		assert_eq!(buff[0].value(), "ref str");
		assert!(matches!(buff[0].quoted_mode(), QuoteMode::AutoDetect));

		fixture.push(|x| Description::new_string(x.int.to_string(), AutoDetect));
		let n = fixture.fill(&sample, &mut buff);
		assert_eq!(n, 2);
		assert_eq!(buff.len(), 2);

		assert_eq!(buff[0].value(), "ref str");
		assert!(matches!(buff[0].quoted_mode(), QuoteMode::AutoDetect));

		assert_eq!(buff[1].value(), "42");
		assert!(matches!(buff[1].quoted_mode(), QuoteMode::AutoDetect));

		fixture.push(|x| Description::new_string(x.string.clone(), Quoted));
		let n = fixture.fill(&sample, &mut buff);
		assert_eq!(n, 3);
		assert_eq!(buff.len(), 3);

		assert_eq!(buff[0].value(), "ref str");
		assert!(matches!(buff[0].quoted_mode(), QuoteMode::AutoDetect));

		assert_eq!(buff[1].value(), "42");
		assert!(matches!(buff[1].quoted_mode(), QuoteMode::AutoDetect));

		assert_eq!(buff[2].value(), "hello");
		assert!(matches!(buff[2].quoted_mode(), QuoteMode::Quoted));
	}

	#[test]
	fn append_test() {
		let sample = Sample::gen_sample();
		let mut fixture_int = Descriptor::<Sample>::default();
		fixture_int.push(|x| Description::new_string(x.int.to_string(), AutoDetect));

		let mut fixture_string = Descriptor::<Sample>::default();
		fixture_string.push(|x| Description::new_str(x.string.as_str(), Quoted));

		let mut fixture_str = Descriptor::<Sample>::default();
		fixture_str.push(|x| Description::new_str(x.str, AutoDetect));

		let mut act = Vec::default();

		assert_eq!(fixture_int.append(&sample, &mut act), 1);
		assert_eq!(act.len(), 1);
		assert_eq!(act[0].value(), "42");
		assert!(matches!(act[0].quoted_mode(), QuoteMode::AutoDetect));

		assert_eq!(fixture_string.append(&sample, &mut act), 1);
		assert_eq!(act.len(), 2);
		assert_eq!(act[1].value(), "hello");
		assert!(matches!(act[1].quoted_mode(), QuoteMode::Quoted));

		assert_eq!(fixture_str.append(&sample, &mut act), 1);
		assert_eq!(act.len(), 3);
		assert_eq!(act[2].value(), "ref str");
		assert!(matches!(act[2].quoted_mode(), QuoteMode::AutoDetect));
	}
}
