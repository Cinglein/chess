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
