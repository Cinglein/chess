use enum_map::{Enum, EnumMap};

use crate::network_format::NetworkFormat;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Enum)]
pub(super) enum Table {
    HiddenWeights,
    HiddenBiases,
    OutputWeights,
    OutputBias,
}

impl Table {
    const HIDDEN_BIASES_INDEX: usize = NetworkFormat::INPUT_SIZE * NetworkFormat::HIDDEN_SIZE;
    const OUTPUT_WEIGHTS_INDEX: usize = Self::HIDDEN_BIASES_INDEX + NetworkFormat::HIDDEN_SIZE;
    const OUTPUT_BIAS_INDEX: usize =
        Self::OUTPUT_WEIGHTS_INDEX + NetworkFormat::BOTH_PERSPECTIVES_SIZE;
    const START_INDEX: EnumMap<Table, usize> = EnumMap::from_array([
        0,
        Self::HIDDEN_BIASES_INDEX,
        Self::OUTPUT_WEIGHTS_INDEX,
        Self::OUTPUT_BIAS_INDEX,
    ]);
    pub(super) const WEIGHT_LENGTH: usize = Self::OUTPUT_BIAS_INDEX + NetworkFormat::OUTPUT_SIZE;

    pub(super) fn start_index(self) -> usize {
        Self::START_INDEX[self]
    }
}
