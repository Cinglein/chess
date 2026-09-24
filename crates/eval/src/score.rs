use core::iter::Sum;
use core::ops::{Add, Neg};

use derive_more::{Add, Display, Sub};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Add, Sub, Display)]
pub struct Score(i32);

impl Score {
    pub const DRAW: Score = Score(0);
    pub const INFINITY: Score = Score(32_000);
    const MATE: Score = Score(31_000);
    const LONGEST_MATE: Score = Score(30_000);

    #[must_use]
    pub const fn new(centipawns: i32) -> Score {
        Score(centipawns)
    }

    #[must_use]
    pub const fn centipawns(self) -> i32 {
        self.0
    }

    #[must_use]
    pub const fn negated(self) -> Score {
        Score(-self.0)
    }

    #[must_use]
    pub fn mate_in(plies: u8) -> Score {
        Self::MATE - Score(i32::from(plies))
    }

    #[must_use]
    pub fn mated_in(plies: u8) -> Score {
        -Self::mate_in(plies)
    }

    #[must_use]
    pub fn stored_from_root_distance(self, ply: u8) -> Score {
        match self.mate_direction() {
            MateDirection::Winning => self + Score(i32::from(ply)),
            MateDirection::Losing => self - Score(i32::from(ply)),
            MateDirection::None => self,
        }
    }

    #[must_use]
    pub fn seen_from_root_distance(self, ply: u8) -> Score {
        match self.mate_direction() {
            MateDirection::Winning => self - Score(i32::from(ply)),
            MateDirection::Losing => self + Score(i32::from(ply)),
            MateDirection::None => self,
        }
    }

    fn mate_direction(self) -> MateDirection {
        if self >= Self::LONGEST_MATE {
            MateDirection::Winning
        } else if self <= -Self::LONGEST_MATE {
            MateDirection::Losing
        } else {
            MateDirection::None
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MateDirection {
    Winning,
    Losing,
    None,
}

impl Neg for Score {
    type Output = Score;

    fn neg(self) -> Score {
        self.negated()
    }
}

impl Sum for Score {
    fn sum<I: Iterator<Item = Score>>(scores: I) -> Score {
        scores.fold(Score::DRAW, Add::add)
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::Score;

    const PLIES: u8 = 3;

    #[test]
    fn faster_mates_score_higher_and_being_mated_is_the_negation() {
        let mate = Score::mate_in(PLIES);
        assert!(Score::INFINITY > mate && mate > Score::mate_in(PLIES + 1));
        assert!(Score::mate_in(PLIES + 1) > Score::new(i32::from(u8::MAX)));
        assert_eq!(Score::mated_in(PLIES), -mate);
    }

    #[test]
    fn storing_and_recalling_a_score_at_the_same_ply_is_the_identity() {
        proptest!(|(centipawns in -31_000_i32..=31_000, ply: u8)| {
            let score = Score::new(centipawns);
            let recalled = score.stored_from_root_distance(ply).seen_from_root_distance(ply);
            prop_assert_eq!(recalled, score);
        });
    }
}
