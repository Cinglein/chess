use super::MoveKind;
use crate::piece_placement::PiecePlacement;
use crate::square::Square;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EnPassant {
    pub origin: Square,
    pub destination: Square,
}

impl MoveKind for EnPassant {
    fn origin(&self) -> Square {
        self.origin
    }

    fn destination(&self) -> Square {
        self.destination
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
