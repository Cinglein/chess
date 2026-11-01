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
