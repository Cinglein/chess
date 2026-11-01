mod elo_delta;
mod elo_estimate;
mod pair_count;
mod pair_score;
mod score_fraction;

pub use elo_estimate::EloEstimate;
pub(super) use pair_score::PairScore;

use enum_map::EnumMap;
use pair_count::PairCount;
use score_fraction::ScoreFraction;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct Pentanomial(EnumMap<PairScore, PairCount>);

impl Pentanomial {
    pub(super) fn counted(mut self, score: PairScore) -> Pentanomial {
        self.0[score] = self.0[score].incremented();
        self
    }

    pub(super) fn merged(self, other: Pentanomial) -> Pentanomial {
        Pentanomial(EnumMap::from_fn(|score| self.0[score] + other.0[score]))
    }

    pub(super) fn estimate(&self) -> EloEstimate {
        let mean = ScoreFraction::mean_of(&self.0);
        let margin = mean.margin_around(&self.0);
        EloEstimate::new(
            mean.shifted(-margin).elo(),
            mean.elo(),
            mean.shifted(margin).elo(),
        )
    }
}
