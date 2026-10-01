use core::ops::ControlFlow;

use board::ChessMove;
use eval::Score;

use super::super::table::{BoundKind, Conclusion};
use super::Window;
use super::bound::Bound;
use super::lower::Lower;
use super::upper::Upper;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Bounds {
    original_lower: Bound<Lower>,
    lower: Bound<Lower>,
    upper: Bound<Upper>,
    best: Score,
    best_move: Option<ChessMove>,
}

impl Bounds {
    pub(crate) fn new(window: Window, floor: Score) -> Bounds {
        Bounds {
            original_lower: window.lower(),
            lower: window.lower().raised(floor),
            upper: window.upper(),
            best: floor,
            best_move: None,
        }
    }

    pub(crate) fn child_window(self) -> Window {
        Window::new(-self.upper, -self.lower)
    }

    pub(crate) fn conclude(self) -> Conclusion {
        let kind = if self.upper.excludes(self.best) {
            BoundKind::AtLeast
        } else if self.original_lower.admits_no_more_than(self.best) {
            BoundKind::AtMost
        } else {
            BoundKind::Exact
        };
        Conclusion::new(self.best_move, self.best, kind)
    }

    pub(crate) fn admit(self, chess_move: ChessMove, score: Score) -> ControlFlow<Bounds, Bounds> {
        let bounds = Bounds {
            lower: self.lower.raised(score),
            best: self.best.max(score),
            best_move: if score > self.best {
                Some(chess_move)
            } else {
                self.best_move
            },
            ..self
        };
        if self.upper.excludes(score) {
            ControlFlow::Break(bounds)
        } else {
            ControlFlow::Continue(bounds)
        }
    }
}

#[cfg(test)]
mod tests {
    use core::ops::ControlFlow;

    use board::Board;
    use eval::Score;

    use super::{Bound, BoundKind, Bounds, Window};

    const ALPHA: Score = Score::new(-10);
    const BETA: Score = Score::new(10);
    const WORSE: Score = Score::new(-5);
    const HOPELESS: Score = Score::new(-20);
    const WINDOW: Window = Window::new(Bound::new(ALPHA), Bound::new(BETA));
    const FLOOR: Score = Score::INFINITY.negated();
    const ADMITTED: [(&[Score], Option<usize>, BoundKind, bool); 4] = [
        (
            &[Score::DRAW, WORSE, Score::DRAW],
            Some(0),
            BoundKind::Exact,
            false,
        ),
        (
            &[Score::DRAW, BETA, HOPELESS],
            Some(1),
            BoundKind::AtLeast,
            true,
        ),
        (&[HOPELESS], Some(0), BoundKind::AtMost, false),
        (&[], None, BoundKind::AtMost, false),
    ];

    #[test]
    fn admitted_scores_keep_the_first_best_move_classify_the_bound_and_cut_at_the_upper_edge() {
        let moves = Board::START.legal_moves();
        for (scores, best, kind, cut) in ADMITTED {
            let searched = scores
                .iter()
                .enumerate()
                .try_fold(Bounds::new(WINDOW, FLOOR), |bounds, (index, score)| {
                    bounds.admit(moves[index], *score)
                });
            let (bounds, is_cut) = match searched {
                ControlFlow::Break(bounds) => (bounds, true),
                ControlFlow::Continue(bounds) => (bounds, false),
            };
            let concluded = bounds.conclude();
            assert_eq!(
                (concluded.best_move(), concluded.kind(), is_cut),
                (best.map(|index| moves[index]), kind, cut),
                "{scores:?}"
            );
        }
    }
}
