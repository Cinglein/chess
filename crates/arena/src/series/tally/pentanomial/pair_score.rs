use enum_map::Enum;
use strum::{EnumCount, VariantArray};

use super::score_fraction::ScoreFraction;
use crate::series::tally::game_score::GameScore;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Enum, EnumCount, VariantArray)]
#[repr(u8)]
pub(in crate::series::tally) enum PairScore {
    TwoLosses,
    LossAndDraw,
    Split,
    WinAndDraw,
    TwoWins,
}

impl PairScore {
    pub(in crate::series::tally) const fn new(
        as_white: GameScore,
        as_black: GameScore,
    ) -> PairScore {
        match (as_white, as_black) {
            (GameScore::Loss, GameScore::Loss) => PairScore::TwoLosses,
            (GameScore::Loss, GameScore::Draw) | (GameScore::Draw, GameScore::Loss) => {
                PairScore::LossAndDraw
            }
            (GameScore::Win, GameScore::Loss)
            | (GameScore::Loss, GameScore::Win)
            | (GameScore::Draw, GameScore::Draw) => PairScore::Split,
            (GameScore::Win, GameScore::Draw) | (GameScore::Draw, GameScore::Win) => {
                PairScore::WinAndDraw
            }
            (GameScore::Win, GameScore::Win) => PairScore::TwoWins,
        }
    }

    pub(super) const fn fraction(self) -> ScoreFraction {
        match self {
            PairScore::TwoLosses => ScoreFraction::new(0.0),
            PairScore::LossAndDraw => ScoreFraction::new(0.25),
            PairScore::Split => ScoreFraction::new(0.5),
            PairScore::WinAndDraw => ScoreFraction::new(0.75),
            PairScore::TwoWins => ScoreFraction::new(1.0),
        }
    }
}
