use core::iter::Sum;
use core::ops::Add;

use derive_more::{Add, Display, Neg, Sub};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Add, Sub, Neg, Display,
)]
pub struct Score(i32);

impl Score {
    pub const DRAW: Score = Score(0);

    #[must_use]
    pub const fn new(centipawns: i32) -> Score {
        Score(centipawns)
    }

    #[must_use]
    pub const fn centipawns(self) -> i32 {
        self.0
    }
}

impl Sum for Score {
    fn sum<I: Iterator<Item = Score>>(scores: I) -> Score {
        scores.fold(Score::DRAW, Add::add)
    }
}
