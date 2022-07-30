#![allow(clippy::std_instead_of_core)]

use core::result::Result as StdResult;
use std::io;

use thiserror::Error;

/// An [`Error`](core::error::Error) for the crate.
#[derive(Debug, Error)]
pub enum Error
{
	#[allow(missing_docs)]
	#[error(transparent)]
	Decimal(#[from] rust_decimal::Error),

	#[allow(missing_docs)]
	#[error("There was an error decoding {0}: {1}")]
	Decode(String, String),

	#[allow(missing_docs)]
	#[error(transparent)]
	Io(#[from] io::Error),

	#[allow(missing_docs)]
	#[error(transparent)]
	Reqwest(#[from] reqwest::Error),

	#[allow(missing_docs)]
	#[error("The {0} currency is not recognized by CLInvoice. Please see https://github.com/Iron-E/clinvoice/wiki/Usage for a list of supported currencies")]
	UnsupportedCurrency(String),

	#[allow(missing_docs)]
	#[error(transparent)]
	Zip(#[from] zip::result::ZipError),
}

/// A [`Result`](StdResult) for the crate.
pub type Result<T> = StdResult<T, Error>;
