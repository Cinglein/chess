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

    use board::{Board, ChessMove};
    use eval::Score;

    use super::{Bound, BoundKind, Bounds, Window};

    const ALPHA: Score = Score::new(-10);
    const BETA: Score = Score::new(10);
    const WORSE: Score = Score::new(-5);
    const HOPELESS: Score = Score::new(-20);
    const WINDOW: Window = Window::new(Bound::new(ALPHA), Bound::new(BETA));
    const FLOOR: Score = Score::INFINITY.negated();

    impl Bounds {
        fn admitting(self, chess_move: ChessMove, score: Score) -> Bounds {
            match self.admit(chess_move, score) {
                ControlFlow::Break(bounds) | ControlFlow::Continue(bounds) => bounds,
            }
        }
    }

    #[test]
    fn the_first_of_equal_best_moves_stays_and_a_score_inside_the_window_concludes_exactly() {
        let [first, second, third, ..] = Board::START.legal_moves()[..] else {
            panic!()
        };
        let settled = Bounds::new(WINDOW, FLOOR)
            .admitting(first, Score::DRAW)
            .admitting(second, WORSE)
            .admitting(third, Score::DRAW)
            .conclude();
        assert_eq!(
            (settled.best_move(), settled.kind(), settled.score()),
            (Some(first), BoundKind::Exact, Score::DRAW)
        );
    }

    #[test]
    fn a_score_at_the_upper_bound_cuts_the_search_and_one_below_the_lower_concludes_at_most() {
        let [first, second, ..] = Board::START.legal_moves()[..] else {
            panic!()
        };
        let open = Bounds::new(WINDOW, FLOOR).admitting(first, Score::DRAW);
        let cut = open.admitting(second, BETA).conclude();
        assert_eq!(
            (
                open.admit(second, BETA).is_break(),
                cut.best_move(),
                cut.kind()
            ),
            (true, Some(second), BoundKind::AtLeast)
        );
        let hopeless = Bounds::new(WINDOW, FLOOR).admitting(first, HOPELESS);
        assert_eq!(hopeless.conclude().kind(), BoundKind::AtMost);
    }
}
