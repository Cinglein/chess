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
    fn a_mate_stored_at_one_distance_reads_at_another_as_the_same_mate_from_the_root() {
        proptest!(|(stored_at: u8, beyond: u8, read_at: u8, score: Score)| {
            prop_assume!(stored_at.checked_add(beyond).is_some() && beyond.checked_add(read_at).is_some());
            let at = |plies: u8| (0..plies).fold(RootDistance::ROOT, |distance, _| distance.deeper());
            prop_assert_eq!(
                (
                    StoredScore::new(Score::mate_in(stored_at + beyond), at(stored_at)).seen_from(at(read_at)),
                    StoredScore::new(Score::mated_in(stored_at + beyond), at(stored_at)).seen_from(at(read_at))
                ),
                (Score::mate_in(beyond + read_at), Score::mated_in(beyond + read_at))
            );
            prop_assume!(score.mate_in_moves().is_none());
            prop_assert_eq!(StoredScore::new(score, at(stored_at)).seen_from(at(read_at)), score);
        });
    }
}
