use board::{Board, Color};
use eval::Score;

use super::outcome::Verdict;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Label {
    board: Board,
    score: Score,
    verdict: Verdict,
}

impl Label {
    #[must_use]
    pub const fn new(board: Board, score: Score, verdict: Verdict) -> Label {
        Label {
            board,
            score,
            verdict,
        }
    }

    #[must_use]
    pub const fn board(&self) -> &Board {
        &self.board
    }

    #[must_use]
    pub fn score_for(&self, side: Color) -> Score {
        if side == self.board.side_to_move() {
            self.score
        } else {
            -self.score
        }
    }

    #[must_use]
    pub const fn verdict(&self) -> Verdict {
        self.verdict
    }
}
