use core::iter::Sum;
use core::ops::{Add, Mul};

use super::super::weight::Weight;
use crate::network_format::NetworkFormat;
use crate::score::Score;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::saved_network) struct Signal(i32);

impl Signal {
    pub(super) const ZERO: Signal = Signal(0);
    const MOST_ACTIVATION: Signal = Signal(NetworkFormat::HIDDEN_QUANTISATION.factor() as i32);
    const ACTIVATION_UNITS: i32 = NetworkFormat::HIDDEN_QUANTISATION.factor() as i32;
    const OUTPUT_UNITS: i32 = NetworkFormat::OUTPUT_BIAS_QUANTISATION.factor() as i32;
    const CENTIPAWNS_PER_UNIT: i32 = NetworkFormat::EVAL_SCALE.centipawns().centipawns();
    const MOST_UNITS: i32 =
        (Score::LONGEST_MATE.centipawns() - 1) * Self::OUTPUT_UNITS / Self::CENTIPAWNS_PER_UNIT;

    pub(super) fn screlu(self) -> Signal {
        let clamped = self.0.clamp(Self::ZERO.0, Self::MOST_ACTIVATION.0);
        Signal(clamped * clamped)
    }

    pub(in crate::saved_network) fn output_centipawns(self, bias: Weight) -> Score {
        let units = (self.0 / Self::ACTIVATION_UNITS).saturating_add(i32::from(bias.units()));
        Score::new(
            units.clamp(-Self::MOST_UNITS, Self::MOST_UNITS) * Self::CENTIPAWNS_PER_UNIT
                / Self::OUTPUT_UNITS,
        )
    }
}

impl From<Weight> for Signal {
    fn from(weight: Weight) -> Signal {
        Signal(i32::from(weight.units()))
    }
}

impl Add for Signal {
    type Output = Signal;

    fn add(self, other: Signal) -> Signal {
        Signal(self.0.saturating_add(other.0))
    }
}

impl Add<Weight> for Signal {
    type Output = Signal;

    fn add(self, weight: Weight) -> Signal {
        Signal(self.0 + i32::from(weight.units()))
    }
}

impl Mul<Weight> for Signal {
    type Output = Signal;

    fn mul(self, weight: Weight) -> Signal {
        Signal(self.0.saturating_mul(i32::from(weight.units())))
    }
}

impl Sum for Signal {
    fn sum<I: Iterator<Item = Signal>>(signals: I) -> Signal {
        signals.fold(Signal::ZERO, Add::add)
    }
}
