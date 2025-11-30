use thiserror::Error;

pub type Result<T> = std::result::Result<T, ArgumentError>;
#[derive(Error, Debug)]
pub enum ArgumentError {
	#[error("Invalid argument: {0}")]
	ArgumentError(String),
	#[error("Argument out of range: {0}")]
	ArgumentOutOfRange(String),
}
