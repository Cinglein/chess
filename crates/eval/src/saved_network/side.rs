use board::Color;
use enum_map::{Enum, EnumMap};

use crate::network_format::NetworkFormat;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Enum)]
pub(super) enum Side {
    Ours,
    Theirs,
}

impl Side {
    const HALF_INDEX: EnumMap<Side, usize> = EnumMap::from_array([0, NetworkFormat::HIDDEN_SIZE]);

    pub(super) const fn relative_to(perspective: Color, color: Color) -> Side {
        match (perspective, color) {
            (Color::White, Color::White) | (Color::Black, Color::Black) => Side::Ours,
            (Color::White, Color::Black) | (Color::Black, Color::White) => Side::Theirs,
        }
    }

    pub(super) fn half_index(self) -> usize {
        Self::HALF_INDEX[self]
    }
}
