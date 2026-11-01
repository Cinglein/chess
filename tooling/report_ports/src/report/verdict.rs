use super::missed::Missed;
use super::shards::Shards;

#[derive(Clone, Copy)]
pub(super) enum Verdict {
    Green,
    Red,
}

impl Verdict {
    pub(super) fn from_run(shards: Shards, missed: &Missed) -> Self {
        match (shards, missed.any()) {
            (Shards::Finished, None) => Self::Green,
            _ => Self::Red,
        }
    }
}
