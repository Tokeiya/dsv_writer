use crate::raw_encoder::Encoder;
use crate::raw_encoder_error::Error as RawEncoderError;
use std::error::Error as StdError;
use std::result::Result as StdResult;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error<E: StdError + 'static> {
	#[error(transparent)]
	RawEnc(#[from] RawEncoderError),
	#[error(transparent)]
	Backend(E),
}

pub type Result<T, E> = StdResult<T, Error<E>>;

pub trait ToOptionedDsv<E: StdError> {
	type Opt;
	fn to_opt_dsv<T: Encoder>(&self, option: &Self::Opt, writer: &mut T) -> Result<(), E>;
}

pub trait ToDsv<E: StdError> {
	fn to_dsv<T: Encoder>(&self, writer: &mut T) -> Result<(), E>;
}

impl<E, O, X: ToOptionedDsv<E, Opt = O>> ToDsv<E> for X
where
	E: StdError,
	O: Default,
{
	fn to_dsv<T: Encoder>(&self, writer: &mut T) -> Result<(), E> {
		self.to_opt_dsv(&O::default(), writer)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::quote_mode::QuoteMode;
	use std::error::Error as StdError;
	use std::fmt::{Debug, Display, Formatter};
	#[test]
	fn playground() {
		let i: u16 = u16::default();
		assert_eq!(i, 0);
	}

	pub struct DummyErr;

	impl Debug for DummyErr {
		fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
			unreachable!()
		}
	}

	impl Display for DummyErr {
		fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
			unreachable!()
		}
	}

	struct DummyTarget;

	impl ToOptionedDsv<DummyErr> for DummyTarget {
		type Opt = u16;

		fn to_opt_dsv<T: Encoder>(
			&self,
			option: &Self::Opt,
			_writer: &mut T,
		) -> Result<(), DummyErr> {
			assert_eq!(*option, 0);
			Ok(())
		}
	}

	struct DummyEnc;

	impl Encoder for DummyEnc {
		fn write_str_field(
			&mut self,
			_value: &str,
			_quote_mode: QuoteMode,
		) -> crate::raw_encoder_error::Result<usize> {
			unreachable!()
		}

		fn end_of_record(
			&mut self,
			_should_flush: bool,
		) -> crate::raw_encoder_error::Result<usize> {
			unreachable!()
		}

		fn cnt(&self) -> usize {
			unreachable!()
		}
	}

	impl StdError for DummyErr {}

	#[test]
	fn to_dsv_test() {
		let target = DummyTarget;
		let mut enc = DummyEnc;
		ToDsv::<DummyErr>::to_dsv(&target, &mut enc).unwrap();
	}
}
