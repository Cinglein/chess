use std::fmt;

use board::{Board, LongAlgebraic, Zobrist};
use uci::Position;

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
    pub fn moves_text(&self) -> String {
        self.moves
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(" ")
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
    pub fn repetitions(&self, hash: Zobrist) -> usize {
        self.positions
            .iter()
            .filter(|position| position.hash() == hash)
            .count()
    }
}

impl fmt::Display for Record {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let start = self.start().to_string();
        let moves = self.moves_text();
        write!(
            formatter,
            "position {}",
            Position::new(Some(&start), &moves)
        )
    }
}
