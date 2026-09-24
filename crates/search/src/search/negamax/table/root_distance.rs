use eval::Score;

use super::stored_score::StoredScore;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct RootDistance(u8);

impl RootDistance {
    pub(crate) const ROOT: RootDistance = RootDistance(0);

    pub(crate) const fn plies(self) -> u8 {
        self.0
    }

    pub(crate) const fn deeper(self) -> RootDistance {
        RootDistance(self.0.saturating_add(1))
    }

    pub(crate) fn mated_here(self) -> Score {
        Score::mated_in(self.0)
    }

    pub(crate) fn store(self, score: Score) -> StoredScore {
        StoredScore::from_root_relative(score.stored_from_root_distance(self.0))
    }

    pub(crate) fn recall(self, stored: StoredScore) -> Score {
        stored.root_relative().seen_from_root_distance(self.0)
    }
}
