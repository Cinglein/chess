use std::fmt;

use board::{Board, LongAlgebraic, Zobrist};
use uci::{Command, Position};

use super::repetition_count::RepetitionCount;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Record {
    positions: Vec<Board>,
    moves: Vec<LongAlgebraic>,
}

impl Record {
    #[must_use]
    pub fn new(start: Board) -> Record {
        Record {
            positions: vec![start],
            moves: Vec::new(),
        }
    }

    #[must_use]
    pub fn start(&self) -> &Board {
        self.positions.first().unwrap_or(&Board::START)
    }

    #[must_use]
    pub fn moves(&self) -> &[LongAlgebraic] {
        &self.moves
    }

    #[must_use]
    pub fn extended(self, notation: LongAlgebraic, reached: Board) -> Record {
        let mut positions = self.positions;
        positions.push(reached);
        let mut moves = self.moves;
        moves.push(notation);
        Record { positions, moves }
    }

    #[must_use]
    pub fn repetitions(&self, hash: Zobrist) -> RepetitionCount {
        RepetitionCount::new(
            self.positions
                .iter()
                .filter(|position| position.hash() == hash)
                .count(),
        )
    }
}

impl fmt::Display for Record {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let position = Position::played(*self.start(), &self.moves);
        write!(formatter, "{}", Command::Position(position))
    }
}
