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

#[cfg(test)]
mod tests {
    use super::{PairScore, Pentanomial};

    const SPLIT_PAIRS: usize = 4;
    const MIXED: [PairScore; 4] = [
        PairScore::TwoWins,
        PairScore::TwoLosses,
        PairScore::Split,
        PairScore::Split,
    ];
    const ESTIMATED: &str = "Elo +0 (-297 to +297) inconclusive";

    impl Pentanomial {
        fn repeating(score: PairScore) -> Pentanomial {
            (0..SPLIT_PAIRS).fold(Pentanomial::default(), |pairs, _| pairs.counted(score))
        }
    }

    #[test]
    fn split_pairs_estimate_even_and_a_sweep_either_way_is_decisive() {
        assert!(
            !Pentanomial::repeating(PairScore::Split)
                .estimate()
                .excludes_even()
        );
        assert!(
            Pentanomial::repeating(PairScore::TwoWins)
                .estimate()
                .excludes_even()
        );
        assert!(
            Pentanomial::repeating(PairScore::TwoLosses)
                .estimate()
                .excludes_even()
        );
    }

    #[test]
    fn a_mixed_sample_estimates_even_with_a_symmetric_margin_at_ninety_five_percent() {
        let mixed = MIXED
            .into_iter()
            .fold(Pentanomial::default(), Pentanomial::counted);
        assert_eq!(mixed.estimate().to_string(), ESTIMATED);
    }
}
