use core::ops::ControlFlow;

use eval::Score;

use crate::bound::Bound;
use crate::lower::Lower;
use crate::upper::Upper;
use crate::window::Window;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Bounds {
    lower: Bound<Lower>,
    upper: Bound<Upper>,
    best: Score,
}

impl Bounds {
    pub(crate) fn new(window: Window, floor: Score) -> Bounds {
        Bounds {
            lower: window.lower().raised(floor),
            upper: window.upper(),
            best: floor,
        }
    }

    pub(crate) const fn best(self) -> Score {
        self.best
    }

    pub(crate) fn child_window(self) -> Window {
        Window::new(-self.upper, -self.lower)
    }

    pub(crate) fn admit(self, score: Score) -> ControlFlow<Score, Bounds> {
        let best = self.best.max(score);
        if self.upper.excludes(score) {
            ControlFlow::Break(best)
        } else {
            ControlFlow::Continue(Bounds {
                lower: self.lower.raised(score),
                best,
                ..self
            })
        }
    }
}
