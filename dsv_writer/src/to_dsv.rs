use std::error::Error as StdError;
use crate::raw_encoder::Encoder;
use super::raw_writer::RawWriter;
use thiserror::Error as ThisError;

#[derive(Debug,ThisError)]
pub enum Error<E:StdError>{
	#[error("raw writer error:{0}")]
	RawWriter(#[from]RawWriter<E>),
	#[error("invalid argument:{0}")]
	ArgumentError(#[from]common_errors::invalid_argument::Error),
}

pub trait ToOptionedDsv<E:StdError>{
	fn to_dsv<T: Encoder, U>(&self, option: &U, writer: &mut T) ->Error<E>;
}

pub trait ToDsv<E:StdError>{
	fn to_dsv<T: Encoder>(&self, writer: &mut T) ->Error<E>;
}