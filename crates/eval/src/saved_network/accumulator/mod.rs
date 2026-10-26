mod feature;
mod signal;

use core::array;

use board::{Board, Color};
use feature::Feature;
use signal::Signal;

use super::SavedNetwork;
use super::side::Side;
use super::table::Table;
use crate::network_format::NetworkFormat;

pub(super) struct Accumulator([Signal; NetworkFormat::HIDDEN_SIZE]);

impl Accumulator {
    pub(super) fn fresh(
        network: &SavedNetwork<'_>,
        perspective: Color,
        board: &Board,
    ) -> Accumulator {
        let placement = board.placement();
        placement
            .occupied()
            .into_iter()
            .filter_map(|square| {
                placement
                    .piece_at(square)
                    .map(|piece| Feature::seen_from(perspective, piece, square))
            })
            .fold(Accumulator::biased(network), |accumulator, feature| {
                accumulator.added(network, feature)
            })
    }

    fn biased(network: &SavedNetwork<'_>) -> Accumulator {
        Accumulator(array::from_fn(|index| {
            Signal::from(network.weight(Table::HiddenBiases, index))
        }))
    }

    fn added(self, network: &SavedNetwork<'_>, feature: Feature) -> Accumulator {
        Accumulator(array::from_fn(|index| {
            self.0[index] + network.weight(Table::HiddenWeights, feature.weight_index(index))
        }))
    }

    pub(super) fn screlu_dot(&self, network: &SavedNetwork<'_>, side: Side) -> Signal {
        self.0
            .iter()
            .enumerate()
            .map(|(index, signal)| {
                signal.screlu() * network.weight(Table::OutputWeights, side.half_index() + index)
            })
            .sum()
    }
}
