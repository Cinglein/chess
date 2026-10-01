use eval::Score;

use super::super::root_distance::RootDistance;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StoredScore(Score);

impl StoredScore {
    pub const DRAW: StoredScore = StoredScore(Score::DRAW);

    pub(crate) fn new(score: Score, distance: RootDistance) -> StoredScore {
        let plies = Score::new(i32::from(distance.plies()));
        if score >= Score::LONGEST_MATE {
            StoredScore(score + plies)
        } else if score <= -Score::LONGEST_MATE {
            StoredScore(score - plies)
        } else {
            StoredScore(score)
        }
    }

    pub(crate) fn seen_from(self, distance: RootDistance) -> Score {
        let plies = Score::new(i32::from(distance.plies()));
        if self.0 >= Score::LONGEST_MATE {
            self.0 - plies
        } else if self.0 <= -Score::LONGEST_MATE {
            self.0 + plies
        } else {
            self.0
        }
    }
}

#[cfg(test)]
mod tests {
    use eval::Score;
    use proptest::prelude::*;

    use super::{RootDistance, StoredScore};

    #[test]
    fn storing_and_recalling_a_score_at_the_same_distance_is_the_identity() {
        proptest!(|(score: Score, ply: u8)| {
            let distance = (0..ply).fold(RootDistance::ROOT, |distance, _| distance.deeper());
            prop_assert_eq!(StoredScore::new(score, distance).seen_from(distance), score);
        });
    }
}
