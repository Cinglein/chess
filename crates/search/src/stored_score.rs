use eval::Score;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StoredScore(Score);

impl StoredScore {
    pub(crate) const fn from_root_relative(score: Score) -> StoredScore {
        StoredScore(score)
    }

    pub(crate) const fn root_relative(self) -> Score {
        self.0
    }
}
