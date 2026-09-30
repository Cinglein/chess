use board::FullmoveNumber;
use eval::Score;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DrawAdjudication {
    after_move: FullmoveNumber,
    within_centipawns: Score,
    for_plies: usize,
}

impl DrawAdjudication {
    #[must_use]
    pub const fn new(
        after_move: FullmoveNumber,
        within_centipawns: Score,
        for_plies: usize,
    ) -> DrawAdjudication {
        DrawAdjudication {
            after_move,
            within_centipawns,
            for_plies,
        }
    }

    #[must_use]
    pub fn is_level(&self, score: Score) -> bool {
        score <= self.within_centipawns && score >= -self.within_centipawns
    }

    #[must_use]
    pub fn reached(&self, level_plies: usize, move_number: FullmoveNumber) -> bool {
        level_plies >= self.for_plies && move_number >= self.after_move
    }
}
