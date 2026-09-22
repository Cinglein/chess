use eval::Score;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Window {
    alpha: Score,
    beta: Score,
}

impl Window {
    pub(crate) const FULL: Window = Window {
        alpha: Score::INFINITY.negated(),
        beta: Score::INFINITY,
    };

    pub(crate) const fn new(alpha: Score, beta: Score) -> Window {
        Window { alpha, beta }
    }

    pub(crate) const fn alpha(self) -> Score {
        self.alpha
    }

    pub(crate) const fn beta(self) -> Score {
        self.beta
    }

    pub(crate) const fn below(self, beta: Score) -> Window {
        Window { beta, ..self }
    }

    pub(crate) fn cuts(self, score: Score) -> bool {
        score >= self.beta
    }
}
