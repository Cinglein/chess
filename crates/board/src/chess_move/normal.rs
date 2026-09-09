use super::MoveKind;
use crate::piece_placement::PiecePlacement;
use crate::square::Square;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Normal {
    pub origin: Square,
    pub destination: Square,
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
