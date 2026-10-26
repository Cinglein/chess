use board::{Color, Piece, PieceKind, Square};
use strum::EnumCount;

use super::super::side::Side;
use crate::network_format::NetworkFormat;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Feature(usize);

impl Feature {
    const PER_SIDE: usize = PieceKind::COUNT * Square::COUNT;

    pub(super) const fn seen_from(perspective: Color, piece: Piece, square: Square) -> Feature {
        let side = match Side::relative_to(perspective, piece.color()) {
            Side::Ours => 0,
            Side::Theirs => Self::PER_SIDE,
        };
        let shown = match perspective {
            Color::White => square,
            Color::Black => square.mirrored(),
        };
        Feature(side + piece.kind() as usize * Square::COUNT + shown as usize)
    }

    pub(super) const fn weight_index(self, index: usize) -> usize {
        self.0 * NetworkFormat::HIDDEN_SIZE + index
    }
}
