use core::fmt::Display;
use core::str::FromStr;

use crate::fen_error::FenError;

pub trait Fen: Display + FromStr<Err = FenError> {
    fn from_fen(text: &str) -> Result<Self, FenError> {
        text.parse()
    }
}
