use eval::Score;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResignAdjudication {
    beyond_centipawns: Score,
    for_plies: usize,
}

impl ResignAdjudication {
    #[must_use]
    pub const fn new(beyond_centipawns: Score, for_plies: usize) -> ResignAdjudication {
        ResignAdjudication {
            beyond_centipawns,
            for_plies,
        }
    }

    #[must_use]
    pub fn is_lost(&self, score: Score) -> bool {
        score <= -self.beyond_centipawns
    }

    #[must_use]
    pub fn reached(&self, losing_plies: usize) -> bool {
        losing_plies >= self.for_plies
    }
}
