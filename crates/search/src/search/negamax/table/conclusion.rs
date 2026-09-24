use board::ChessMove;
use eval::Score;

use super::bound_kind::BoundKind;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Conclusion {
    best_move: Option<ChessMove>,
    score: Score,
    kind: BoundKind,
}

impl Conclusion {
    pub(crate) const fn new(
        best_move: Option<ChessMove>,
        score: Score,
        kind: BoundKind,
    ) -> Conclusion {
        Conclusion {
            best_move,
            score,
            kind,
        }
    }

    pub(crate) const fn best_move(&self) -> Option<ChessMove> {
        self.best_move
    }

    pub(crate) const fn score(&self) -> Score {
        self.score
    }

    pub(crate) const fn kind(&self) -> BoundKind {
        self.kind
    }
}
