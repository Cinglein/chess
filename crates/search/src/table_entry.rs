use board::{ChessMove, Zobrist};
use eval::Score;

use crate::bound_kind::BoundKind;
use crate::conclusion::Conclusion;
use crate::depth::Depth;
use crate::root_distance::RootDistance;
use crate::stored_score::StoredScore;
use crate::window::Window;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TableEntry {
    hash: Zobrist,
    depth: Depth,
    best_move: Option<ChessMove>,
    score: StoredScore,
    kind: BoundKind,
}

impl TableEntry {
    pub const EMPTY: TableEntry = TableEntry {
        hash: Zobrist::EMPTY,
        depth: Depth::ZERO,
        best_move: None,
        score: StoredScore::from_root_relative(Score::DRAW),
        kind: BoundKind::Exact,
    };

    pub(crate) fn remember(
        hash: Zobrist,
        depth: Depth,
        distance: RootDistance,
        conclusion: Conclusion,
    ) -> TableEntry {
        TableEntry {
            hash,
            depth,
            best_move: conclusion.best_move(),
            score: distance.store(conclusion.score()),
            kind: conclusion.kind(),
        }
    }

    pub(crate) const fn hash(&self) -> Zobrist {
        self.hash
    }

    pub(crate) const fn depth(&self) -> Depth {
        self.depth
    }

    pub(crate) const fn best_move(&self) -> Option<ChessMove> {
        self.best_move
    }

    pub(crate) fn settles(
        &self,
        depth: Depth,
        distance: RootDistance,
        window: Window,
    ) -> Option<Score> {
        if self.depth < depth {
            return None;
        }
        let score = distance.recall(self.score);
        match self.kind {
            BoundKind::Exact => Some(score),
            BoundKind::AtMost if window.lower().admits_no_more_than(score) => Some(score),
            BoundKind::AtLeast if window.upper().excludes(score) => Some(score),
            BoundKind::AtMost | BoundKind::AtLeast => None,
        }
    }
}
