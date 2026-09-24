mod fen_error;

pub use fen_error::FenError;

use core::fmt::Display;
use core::str::FromStr;

pub trait Fen: Display + FromStr<Err = FenError> {}
