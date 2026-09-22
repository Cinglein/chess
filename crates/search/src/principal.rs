use board::ChessMove;
use eval::Score;

use crate::window::Window;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Principal {
    chess_move: Option<ChessMove>,
    score: Score,
}

impl Principal {
    pub(crate) const NONE: Principal = Principal {
        chess_move: None,
        score: Score::INFINITY.negated(),
    };

    pub(crate) const fn chess_move(self) -> Option<ChessMove> {
        self.chess_move
    }

    pub(crate) const fn score(self) -> Score {
        self.score
    }

    pub(crate) const fn window(self) -> Window {
        Window::FULL.below(self.score.negated())
    }

    pub(crate) fn improved(self, chess_move: ChessMove, score: Score) -> Principal {
        if score > self.score {
            Principal {
                chess_move: Some(chess_move),
                score,
            }
        } else {
            self
        }
    }
}
