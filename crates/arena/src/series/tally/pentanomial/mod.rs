mod elo_delta;
mod elo_estimate;
mod pair_score;
mod score_fraction;

pub use elo_estimate::EloEstimate;
pub(super) use pair_score::PairScore;

use enum_map::EnumMap;
use score_fraction::ScoreFraction;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct Pentanomial(EnumMap<PairScore, usize>);

impl Pentanomial {
    pub(super) fn counted(mut self, score: PairScore) -> Pentanomial {
        self.0[score] += 1;
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

#[cfg(test)]
mod tests {
    use super::{PairScore, Pentanomial};

    const SPLIT_PAIRS: usize = 4;

    #[test]
    fn split_pairs_estimate_even_and_a_sweep_estimates_a_gain_with_a_bound_above_even() {
        let even = (0..SPLIT_PAIRS).fold(Pentanomial::default(), |pairs, _| {
            pairs.counted(PairScore::Split)
        });
        let sweep = (0..SPLIT_PAIRS).fold(Pentanomial::default(), |pairs, _| {
            pairs.counted(PairScore::TwoWins)
        });
        assert!(!even.estimate().excludes_even());
        assert!(sweep.estimate().excludes_even());
    }
}
