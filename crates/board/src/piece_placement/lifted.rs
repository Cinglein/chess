use super::PiecePlacement;
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
    pub fn capture(self, square: Square) -> Lifted {
        Lifted {
            placement: self.placement.cleared(square),
            ..self
        }
    }

    #[must_use]
    pub fn land(self, square: Square) -> PiecePlacement {
        self.placement.with(self.piece, square)
    }
}
