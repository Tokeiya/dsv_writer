use anyhow::{Result as AnyResult, anyhow};
use std::ops::BitOrAssign;

#[derive(Debug)]
pub struct ShouldQuoteDatum {
	should_quote: bool,
	double_quote: bool,
}

impl ShouldQuoteDatum {
	pub fn new(should_quote: bool, double_quote: bool) -> Self {
		Self {
			should_quote,
			double_quote,
		}
	}

	pub fn should_quote(&self) -> bool {
		self.should_quote
	}

	pub fn double_quote(&self) -> bool {
		self.double_quote
	}
}

impl BitOrAssign<ShouldQuoteDatum> for ShouldQuoteDatum {
	fn bitor_assign(&mut self, rhs: ShouldQuoteDatum) {
		self.should_quote |= rhs.should_quote;
		self.double_quote |= rhs.double_quote;
	}
}

pub type ShouldQuoteResult = AnyResult<ShouldQuoteDatum>;
