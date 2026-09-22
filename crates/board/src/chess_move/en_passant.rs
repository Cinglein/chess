use super::MoveKind;
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

    fn captures(&self, _: &PiecePlacement) -> bool {
        true
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
