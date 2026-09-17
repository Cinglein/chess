use super::MoveKind;
use crate::file::File;
use crate::placement::PiecePlacement;
use crate::square::Square;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DoublePush {
    origin: Square,
    destination: Square,
}

impl DoublePush {
    #[must_use]
    pub const fn new(origin: Square, destination: Square) -> DoublePush {
        DoublePush {
            origin,
            destination,
        }
    }
}

impl MoveKind for DoublePush {
    fn origin(&self) -> Square {
        self.origin
    }

    fn destination(&self) -> Square {
        self.destination
    }

    fn play(self, placement: PiecePlacement) -> Option<PiecePlacement> {
        Some(placement.lift(self.origin)?.land(self.destination))
    }

    fn en_passant_file(&self) -> Option<File> {
        Some(self.origin.file())
    }
}
