use super::MoveKind;
use crate::piece_placement::PiecePlacement;
use crate::promotion_piece::PromotionPiece;
use crate::square::Square;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Promotion {
    pub origin: Square,
    pub destination: Square,
    pub piece: PromotionPiece,
}

impl MoveKind for Promotion {
    fn origin(&self) -> Square {
        self.origin
    }

    fn destination(&self) -> Square {
        self.destination
    }

    fn play(self, placement: PiecePlacement) -> Option<PiecePlacement> {
        Some(
            placement
                .lift(self.origin)?
                .promote(self.piece)
                .land(self.destination),
        )
    }

    fn promotion_piece(&self) -> Option<PromotionPiece> {
        Some(self.piece)
    }
}
