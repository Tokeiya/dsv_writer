use crate::raw_encoder::Encoder;
use crate::raw_encoder_error::Error as RawEncoderError;
use std::error::Error as StdError;
use std::result::Result as StdResult;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum ToDsvError<E: StdError + 'static> {
	#[error(transparent)]
	RawEnc(#[from] RawEncoderError),
	#[error(transparent)]
	Backend(E),
}

pub type ToDsvResult<T, E> = StdResult<T, ToDsvError<E>>;

pub trait ToOptionedDsv {
	type Opt;
	type Error: StdError;
	fn to_opt_dsv<T: Encoder>(
		&self,
		option: &Self::Opt,
		writer: &mut T,
	) -> ToDsvResult<(), Self::Error>;
}

pub trait ToDsv {
	type Error: StdError;
	fn to_dsv<T: Encoder>(&self, writer: &mut T) -> ToDsvResult<(), Self::Error>;
}

impl<X> ToDsv for X
where
	X: ToOptionedDsv,
	X::Opt: Default,
	X::Error: StdError + 'static,
{
	type Error = X::Error;

	fn to_dsv<T: Encoder>(&self, writer: &mut T) -> ToDsvResult<(), Self::Error> {
		let option = X::Opt::default();
		self.to_opt_dsv(&option, writer)
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

	impl ToOptionedDsv for DummyTarget {
		type Opt = u16;
		type Error = DummyErr;

		fn to_opt_dsv<T: Encoder>(
			&self,
			option: &Self::Opt,
			_writer: &mut T,
		) -> ToDsvResult<(), DummyErr> {
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

		fn end_of_record(&mut self, _: bool) -> crate::raw_encoder_error::Result<usize> {
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
		ToDsv::to_dsv(&target, &mut enc).unwrap();
	}
}
