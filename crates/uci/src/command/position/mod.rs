mod moves;
mod position_word;

use moves::Moves;

use core::fmt;

use board::{Board, LongAlgebraic};
use position_word::PositionWord;

use crate::uci_error::UciError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position<'moves> {
    start: Board,
    moves: Moves<'moves>,
}

impl<'moves> Position<'moves> {
    #[must_use]
    pub const fn played(start: Board, moves: &'moves [LongAlgebraic]) -> Position<'moves> {
        Position {
            start,
            moves: Moves::Played(moves),
        }
    }

    #[must_use]
    pub const fn start(&self) -> Board {
        self.start
    }

    #[must_use]
    pub const fn moves(&self) -> Moves<'moves> {
        self.moves
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
        let start = match head.parse::<PositionWord>() {
            Ok(PositionWord::StartPos) => Board::START,
            Ok(PositionWord::Fen) => fen.trim().parse().map_err(|_| UciError::UnknownPosition)?,
            _ => return Err(UciError::UnknownPosition),
        };
        Ok(Position {
            start,
            moves: Moves::Written(moves),
        })
    }
}

impl fmt::Display for Position<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.start == Board::START {
            write!(formatter, "{}", PositionWord::StartPos)?;
        } else {
            write!(formatter, "{} {}", PositionWord::Fen, self.start)?;
        }
        if self.moves.is_empty() {
            Ok(())
        } else {
            write!(formatter, " {} {}", PositionWord::Moves, self.moves)
        }
    }
}

#[cfg(test)]
mod tests {
    use board::{Board, LongAlgebraic};
    use proptest::prelude::*;

    use super::Position;

    #[test]
    fn a_played_position_prints_as_a_line_that_parses_to_the_same_start_and_moves() {
        proptest!(|(board: Board, played in proptest::collection::vec(any::<LongAlgebraic>(), 0..8))| {
            let line = Position::played(board, &played).to_string();
            let parsed = Position::try_from(line.as_str()).unwrap();
            prop_assert_eq!(parsed.start(), board);
            prop_assert_eq!(parsed.moves().iter().collect::<Result<Vec<_>, _>>().unwrap(), played);
        });
    }
}
