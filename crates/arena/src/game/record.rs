use std::fmt;

use board::{Board, LongAlgebraic, Zobrist};
use uci::Position;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Record {
    start_fen: String,
    moves_text: String,
    hashes: Vec<Zobrist>,
}

impl Record {
    #[must_use]
    pub fn new(start: &Board) -> Record {
        Record {
            start_fen: start.to_string(),
            moves_text: String::new(),
            hashes: vec![start.hash()],
        }
    }

    #[must_use]
    pub fn position(&self) -> Position<'_> {
        Position::new(Some(&self.start_fen), &self.moves_text)
    }

    #[must_use]
    pub fn extended(self, notation: LongAlgebraic, reached: Zobrist) -> Record {
        let moves_text = if self.moves_text.is_empty() {
            notation.to_string()
        } else {
            format!("{} {notation}", self.moves_text)
        };
        let mut hashes = self.hashes;
        hashes.push(reached);
        Record {
            moves_text,
            hashes,
            ..self
        }
    }

    #[must_use]
    pub fn repetitions(&self, hash: Zobrist) -> usize {
        self.hashes.iter().filter(|seen| **seen == hash).count()
    }
}

impl fmt::Display for Record {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "position {}", self.position())
    }
}
