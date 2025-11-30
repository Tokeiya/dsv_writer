use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
	#[error("Invalid number of elements expected {0} but got {1}")]
	InvalidNumberOfElements(usize, usize),
	#[error(transparent)]
	IOError(#[from] std::io::Error),
}
