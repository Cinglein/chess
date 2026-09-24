use super::MoveKind;
use crate::piece::Piece;
use crate::placement::PiecePlacement;
use crate::square::Square;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EnPassant {
    origin: Square,
    destination: Square,
}

impl EnPassant {
    #[must_use]
    pub const fn new(origin: Square, destination: Square) -> EnPassant {
        EnPassant {
            origin,
            destination,
        }
    }
}

impl MoveKind for EnPassant {
    fn origin(&self) -> Square {
        self.origin
    }

    fn destination(&self) -> Square {
        self.destination
    }

    fn victim(&self, placement: &PiecePlacement) -> Option<Piece> {
        placement.piece_at(Square::new(self.destination.file(), self.origin.rank()))
    }

    fn play(self, placement: PiecePlacement) -> Option<PiecePlacement> {
        let passed = Square::new(self.destination.file(), self.origin.rank());
        Some(
            placement
                .lift(self.origin)?
                .land(passed)
                .lift(passed)?
                .land(self.destination),
        )
    }
}
