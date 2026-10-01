use core::iter::Sum;
use core::ops::{Add, Neg};

use derive_more::{Add, Display, FromStr, Sub};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Add, Sub, Display, FromStr,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
pub struct Score(i32);

impl Score {
    pub const DRAW: Score = Score(0);
    pub const INFINITY: Score = Score(32_000);
    const MATE: Score = Score(31_000);
    pub const LONGEST_MATE: Score = Score(30_000);

    #[must_use]
    pub const fn new(centipawns: i32) -> Score {
        Score(centipawns)
    }

    #[must_use]
    pub const fn centipawns(self) -> i32 {
        self.0
    }

    #[must_use]
    pub const fn table<const N: usize>(centipawns: [i32; N]) -> [Score; N] {
        let mut scores = [Score::DRAW; N];
        let mut remaining: &[i32] = &centipawns;
        let mut written = 0;
        while let [head, rest @ ..] = remaining {
            scores[written] = Score(*head);
            written += 1;
            remaining = rest;
        }
        scores
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
    pub fn mating_in_moves(moves: i32) -> Score {
        let plies = |count: i32| u8::try_from(count).unwrap_or(u8::MAX);
        if moves > 0 {
            Self::mate_in(plies(2 * moves - 1))
        } else {
            Self::mated_in(plies(-2 * moves))
        }
    }

    #[must_use]
    pub fn mate_in_moves(self) -> Option<i32> {
        match self.mate_direction() {
            MateDirection::Winning => Some(((Self::MATE - self).0 + 1) / 2),
            MateDirection::Losing => Some(-(((Self::MATE + self).0 + 1) / 2)),
            MateDirection::None => None,
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

#[cfg(any(test, feature = "proptest"))]
impl proptest::arbitrary::Arbitrary for Score {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Score>;

    fn arbitrary_with((): ()) -> Self::Strategy {
        use proptest::strategy::Strategy;
        (-Self::MATE.0..=Self::MATE.0).prop_map(Score::new).boxed()
    }
}

const _: () = assert!(
    Score::LONGEST_MATE.0 < Score::MATE.0 - u8::MAX as i32 && Score::MATE.0 < Score::INFINITY.0
);

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::Score;

    const PLIES: u8 = 3;
    const TWO_MOVES: i32 = 2;

    #[test]
    fn a_mate_distance_in_moves_survives_the_trip_through_a_score_and_a_quicker_mate_scores_higher()
    {
        proptest!(|(moves: i8)| {
            prop_assume!(moves != 0);
            let moves = i32::from(moves);
            prop_assert_eq!(
                (Score::mating_in_moves(moves).mate_in_moves(), (-Score::mating_in_moves(moves)).mate_in_moves()),
                (Some(moves), Some(-moves))
            );
            prop_assert!(Score::mating_in_moves(moves) > Score::mating_in_moves(moves + 1));
        });
        assert_eq!(
            (Score::mating_in_moves(TWO_MOVES), Score::mating_in_moves(0)),
            (Score::mate_in(PLIES), Score::mated_in(0))
        );
    }

    #[test]
    fn centipawns_never_read_as_a_mate_and_a_table_scores_each_entry() {
        proptest!(|(score: Score, centipawns: [i32; 4])| {
            prop_assert_eq!(score.mate_in_moves().is_none(), score > -Score::LONGEST_MATE && score < Score::LONGEST_MATE);
            prop_assert_eq!(Score::new(score.centipawns()), score);
            prop_assert!(Score::table(centipawns).iter().zip(centipawns).all(|(scored, raw)| *scored == Score::new(raw)));
        });
    }
}
