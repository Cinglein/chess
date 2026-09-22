use core::ops::ControlFlow;

use eval::Score;

use crate::window::Window;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Bounds {
    alpha: Score,
    beta: Score,
    best: Score,
}

impl Bounds {
    pub(crate) fn new(window: Window, floor: Score) -> Bounds {
        Bounds {
            alpha: window.alpha().max(floor),
            beta: window.beta(),
            best: floor,
        }
    }

    pub(crate) const fn best(self) -> Score {
        self.best
    }

    pub(crate) const fn child_window(self) -> Window {
        Window::new(self.beta.negated(), self.alpha.negated())
    }

    pub(crate) fn admit(self, score: Score) -> ControlFlow<Score, Bounds> {
        let best = self.best.max(score);
        if score >= self.beta {
            ControlFlow::Break(best)
        } else {
            ControlFlow::Continue(Bounds {
                alpha: self.alpha.max(score),
                best,
                ..self
            })
        }
    }
}
