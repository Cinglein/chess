use enum_map::EnumMap;

use super::elo_delta::EloDelta;
use super::pair_score::PairScore;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub(super) struct ScoreFraction(f64);

impl ScoreFraction {
    const Z_95: f64 = 1.96;
    const LEAST: f64 = 0.000_5;

    pub(super) const fn new(fraction: f64) -> ScoreFraction {
        ScoreFraction(fraction)
    }

    pub(super) fn mean_of(counts: &EnumMap<PairScore, usize>) -> ScoreFraction {
        ScoreFraction(Self::weighted(counts, |fraction| fraction.0) / Self::total(counts))
    }

    pub(super) fn margin_around(self, counts: &EnumMap<PairScore, usize>) -> f64 {
        let variance =
            Self::weighted(counts, |fraction| (fraction.0 - self.0).powi(2)) / Self::total(counts);
        Self::Z_95 * (variance / Self::total(counts)).sqrt()
    }

    pub(super) fn shifted(self, by: f64) -> ScoreFraction {
        ScoreFraction(self.0 + by)
    }

    pub(super) fn elo(self) -> EloDelta {
        let clamped = self.0.clamp(Self::LEAST, 1.0 - Self::LEAST);
        EloDelta::new(400.0 * (clamped / (1.0 - clamped)).log10())
    }

    fn weighted(counts: &EnumMap<PairScore, usize>, measure: impl Fn(ScoreFraction) -> f64) -> f64 {
        counts
            .iter()
            .map(|(score, count)| measure(score.fraction()) * Self::as_float(*count))
            .sum()
    }

    fn total(counts: &EnumMap<PairScore, usize>) -> f64 {
        Self::as_float(counts.values().sum::<usize>()).max(1.0)
    }

    fn as_float(count: usize) -> f64 {
        f64::from(u32::try_from(count).unwrap_or(u32::MAX))
    }
}
