use super::MoveKind;
use crate::file::File;
use crate::piece_placement::PiecePlacement;
use crate::square::Square;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DoublePush {
    pub origin: Square,
    pub destination: Square,
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
