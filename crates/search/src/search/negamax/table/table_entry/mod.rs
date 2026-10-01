mod stored_score;

use board::{ChessMove, Zobrist};
use eval::Score;

use super::super::super::depth::Depth;
use super::super::window::Window;
use super::bound_kind::BoundKind;
use super::conclusion::Conclusion;
use super::root_distance::RootDistance;
use stored_score::StoredScore;

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
        score: StoredScore::DRAW,
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
            score: StoredScore::new(conclusion.score(), distance),
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
        let score = self.score.seen_from(distance);
        if self.depth < depth {
            None
        } else {
            match self.kind {
                BoundKind::Exact => Some(score),
                BoundKind::AtMost if window.lower().admits_no_more_than(score) => Some(score),
                BoundKind::AtLeast if window.upper().excludes(score) => Some(score),
                BoundKind::AtMost | BoundKind::AtLeast => None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use board::Board;
    use eval::Score;

    use super::super::super::window::Bound;
    use super::{BoundKind, Conclusion, Depth, RootDistance, TableEntry, Window, Zobrist};

    const DEEP: Depth = Depth::new(2);
    const ALPHA: Score = Score::new(-10);
    const BETA: Score = Score::new(10);
    const SETTLED: [(BoundKind, Score, Option<Score>); 5] = [
        (BoundKind::Exact, Score::DRAW, Some(Score::DRAW)),
        (BoundKind::AtMost, ALPHA, Some(ALPHA)),
        (BoundKind::AtMost, Score::DRAW, None),
        (BoundKind::AtLeast, BETA, Some(BETA)),
        (BoundKind::AtLeast, Score::DRAW, None),
    ];

    impl TableEntry {
        fn concluding(kind: BoundKind, score: Score) -> TableEntry {
            let conclusion = Conclusion::new(None, score, kind);
            TableEntry::remember(Zobrist::EMPTY, DEEP, RootDistance::ROOT, conclusion)
        }
    }

    #[test]
    fn an_entry_settles_a_node_only_when_deep_enough_and_its_bound_falls_outside_the_window() {
        let window = Window::new(Bound::new(ALPHA), Bound::new(BETA));
        for (kind, score, settled) in SETTLED {
            let entry = TableEntry::concluding(kind, score);
            assert_eq!(
                entry.settles(DEEP, RootDistance::ROOT, window),
                settled,
                "{kind:?} {score}"
            );
            assert_eq!(
                entry.settles(DEEP.incremented(), RootDistance::ROOT, window),
                None
            );
        }
        let chess_move = Board::START.legal_moves()[0];
        let conclusion = Conclusion::new(Some(chess_move), Score::DRAW, BoundKind::Exact);
        let remembered = TableEntry::remember(Zobrist::EMPTY, DEEP, RootDistance::ROOT, conclusion);
        assert_eq!(remembered.best_move(), Some(chess_move));
    }
}
