mod position_word;

use core::fmt;

use position_word::PositionWord;

use crate::uci_error::UciError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position<'line> {
    fen: Option<&'line str>,
    moves: &'line str,
}

impl<'line> Position<'line> {
    #[must_use]
    pub const fn new(fen: Option<&'line str>, moves: &'line str) -> Position<'line> {
        Position { fen, moves }
    }

    #[must_use]
    pub const fn fen(&self) -> Option<&'line str> {
        self.fen
    }

    pub fn moves(&self) -> impl Iterator<Item = &'line str> {
        self.moves.split_whitespace()
    }
}

impl<'line> TryFrom<&'line str> for Position<'line> {
    type Error = UciError;

    fn try_from(rest: &'line str) -> Result<Position<'line>, UciError> {
        let (setup, moves) = rest
            .split_once(<&str>::from(PositionWord::Moves))
            .unwrap_or((rest, ""));
        let setup = setup.trim();
        let (head, fen) = setup.split_once(char::is_whitespace).unwrap_or((setup, ""));
        let fen = match head.parse::<PositionWord>() {
            Ok(PositionWord::StartPos) => None,
            Ok(PositionWord::Fen) => Some(fen.trim()),
            _ => return Err(UciError::UnknownPosition),
        };
        Ok(Position {
            fen,
            moves: moves.trim(),
        })
    }
}

impl fmt::Display for Position<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.fen {
            Some(fen) => write!(formatter, "{} {fen}", PositionWord::Fen)?,
            None => write!(formatter, "{}", PositionWord::StartPos)?,
        }
        if self.moves.is_empty() {
            Ok(())
        } else {
            write!(formatter, " {} {}", PositionWord::Moves, self.moves)
        }
    }
}
