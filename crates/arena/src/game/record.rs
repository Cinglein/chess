use std::fmt;

use board::{Board, LongAlgebraic, Zobrist};
use eval::Score;
use uci::{Command, Position};

use super::label::Label;
use super::outcome::Verdict;
use super::repetition_count::RepetitionCount;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Record {
    positions: Vec<Board>,
    moves: Vec<LongAlgebraic>,
    scores: Vec<Option<Score>>,
}

impl Record {
    #[must_use]
    pub fn new(start: Board) -> Record {
        Record {
            positions: vec![start],
            moves: Vec::new(),
            scores: Vec::new(),
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

    pub fn labelled(&self, verdict: Verdict) -> impl Iterator<Item = Label> + '_ {
        self.positions
            .iter()
            .zip(&self.scores)
            .filter_map(move |(board, score)| score.map(|score| Label::new(*board, score, verdict)))
    }

    #[must_use]
    pub fn extended(self, notation: LongAlgebraic, reached: Board, score: Option<Score>) -> Record {
        let mut positions = self.positions;
        positions.push(reached);
        let mut moves = self.moves;
        moves.push(notation);
        let mut scores = self.scores;
        scores.push(score);
        Record {
            positions,
            moves,
            scores,
        }
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
