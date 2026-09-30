use board::LongAlgebraic;
use eval::Score;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChosenMove {
    notation: LongAlgebraic,
    score: Option<Score>,
}

impl ChosenMove {
    #[must_use]
    pub const fn new(notation: LongAlgebraic, score: Option<Score>) -> ChosenMove {
        ChosenMove { notation, score }
    }

    #[must_use]
    pub fn from_best_move(
        best_move: Option<LongAlgebraic>,
        score: Option<Score>,
    ) -> Option<ChosenMove> {
        best_move.map(|notation| ChosenMove::new(notation, score))
    }

    #[must_use]
    pub const fn notation(&self) -> LongAlgebraic {
        self.notation
    }

    #[must_use]
    pub const fn score(&self) -> Option<Score> {
        self.score
    }
}
