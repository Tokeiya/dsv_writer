use std::error::Error as StdError;
use crate::raw_encoder::Encoder;
use super::raw_writer::RawWriter;
use thiserror::Error as ThisError;
use std::result::Result as StdResult;

#[cfg(test)]
use mockall::automock;

#[derive(Debug,ThisError)]
pub enum Error<E:StdError>{
	#[error("raw writer error:{0}")]
	RawWriter(#[from]RawWriter<E>),
	#[error("invalid argument:{0}")]
	ArgumentError(#[from]common_errors::invalid_argument::Error),
}

pub type Result<T,E> = StdResult<T,Error<E>>;


pub trait ToOptionedDsv<E:StdError>{
	type Opt;
	fn to_dsv<T: Encoder>(&self, option: &Self::Opt, writer: &mut T) ->Result<(),E>;
}

pub trait ToDsv<E:StdError>{
	fn to_dsv<T: Encoder>(&self, writer: &mut T) ->Result<(),E>;
}

impl <E,O,X:ToOptionedDsv<E,Opt=O>>  ToDsv<E> for X
where
	E:StdError,
	O:Default
{
	fn to_dsv<T: Encoder>(&self, writer: &mut T) -> Result<(),E> {
		self.to_dsv(&O::default(),writer)
	}
}


#[cfg(test)]
mod test{
	use mockall::predicate::{eq,always};
	use std::error::Error as StdError;
	use std::fmt::{Debug, Display, Formatter};
	use crate::quote_mode::QuoteMode;
	use super::*;
	#[test]
	fn playground(){
		let i:u16=u16::default();
		assert_eq!(i,0);
	}
	
	pub struct DummyErr;
	
	impl Debug for DummyErr {
		fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
			todo!()
		}
	}
	
	impl Display for DummyErr {
		fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
			todo!()
		}
	}
	
	struct DummyTarget;
	
	impl ToOptionedDsv<DummyErr> for DummyTarget{
		type Opt = u16;
		
		fn to_dsv<T: Encoder>(&self, option: &Self::Opt, writer: &mut T) -> Result<(), DummyErr> {
			assert_eq!(*option,0);
			Ok(())
		}
	}
	
	struct DummyEnc;
	
	impl Encoder for DummyEnc{
		fn should_quoting(&self, value: &str) -> bool {
			todo!()
		}
		
		fn write_str_field(&mut self, value: &str, quote_mode: QuoteMode) -> crate::raw_encoder_error::Result<usize> {
			todo!()
		}
		
		fn end_of_record(&mut self, should_flush: bool) -> crate::raw_encoder_error::Result<usize> {
			todo!()
		}
		
		fn cnt(&self) -> usize {
			todo!()
		}
	}
	
	
	
	impl StdError for DummyErr{}
	
	
	#[test]
	fn to_dsv_test() {
		let target = DummyTarget;
		let mut enc = DummyEnc;
		ToDsv::<DummyErr>::to_dsv(&target,&mut enc).unwrap();
	}
}