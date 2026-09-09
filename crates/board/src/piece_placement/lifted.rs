use super::PiecePlacement;
use crate::bitboard::Bitboard;
use crate::piece::Piece;
use crate::promotion::Promotion;
use crate::square::Square;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Lifted {
    pub(super) placement: PiecePlacement,
    pub(super) piece: Piece,
}

impl Lifted {
    #[must_use]
    pub const fn piece(&self) -> Piece {
        self.piece
    }

    #[must_use]
    pub fn promote(self, promotion: Promotion) -> Lifted {
        Lifted {
            piece: Piece::new(self.piece.color, promotion.into()),
            ..self
        }
    }

    #[must_use]
    pub fn land(self, square: Square) -> PiecePlacement {
        let mut placement = self
            .placement
            .lift(square)
            .map_or(self.placement, |occupant| occupant.placement);
        placement.pieces[self.piece.color][self.piece.kind] |= Bitboard::from_square(square);
        placement
    }
}
