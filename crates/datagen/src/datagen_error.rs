use std::io;
use std::num::TryFromIntError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatagenError {
    #[error("bullet found an occupied square without a piece")]
    Piece,
    #[error("a score does not fit a bullet record: {0}")]
    Score(#[from] TryFromIntError),
    #[error("could not write the examples: {0}")]
    Write(#[from] io::Error),
}
