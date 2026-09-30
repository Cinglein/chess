use core::fmt;

use board::LongAlgebraic;
use itertools::{Either, Itertools};

use crate::uci_error::UciError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Moves<'source> {
    Played(&'source [LongAlgebraic]),
    Written(&'source str),
}

impl Moves<'_> {
    pub fn iter(&self) -> impl Iterator<Item = Result<LongAlgebraic, UciError>> + '_ {
        match self {
            Moves::Played(moves) => Either::Left(moves.iter().copied().map(Ok)),
            Moves::Written(text) => Either::Right(text.split_whitespace().map(Self::parsed)),
        }
    }

    fn parsed(word: &str) -> Result<LongAlgebraic, UciError> {
        word.parse().map_err(|_| UciError::UnknownMove)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            Moves::Played(moves) => moves.is_empty(),
            Moves::Written(text) => text.trim().is_empty(),
        }
    }
}

impl fmt::Display for Moves<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Moves::Played(moves) => write!(formatter, "{}", moves.iter().format(" ")),
            Moves::Written(text) => formatter.write_str(text.trim()),
        }
    }
}
