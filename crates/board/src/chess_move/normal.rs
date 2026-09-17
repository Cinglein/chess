use super::MoveKind;
use crate::placement::PiecePlacement;
use crate::square::Square;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Normal {
    origin: Square,
    destination: Square,
}

impl Normal {
    #[must_use]
    pub const fn new(origin: Square, destination: Square) -> Normal {
        Normal {
            origin,
            destination,
        }
    }
}

impl MoveKind for Normal {
    fn origin(&self) -> Square {
        self.origin
    }

    fn destination(&self) -> Square {
        self.destination
    }

    fn play(self, placement: PiecePlacement) -> Option<PiecePlacement> {
        Some(placement.lift(self.origin)?.land(self.destination))
    }
}
