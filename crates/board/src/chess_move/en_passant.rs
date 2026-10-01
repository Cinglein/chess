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

#[cfg(test)]
mod tests {
    use super::{EnPassant, MoveKind, PiecePlacement, Square};

    const BEFORE: &str = "rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR";
    const AFTER: &str = "rnbqkbnr/ppp1pppp/3P4/8/8/8/PPPP1PPP/RNBQKBNR";
    const CAPTURE: EnPassant = EnPassant::new(Square::E5, Square::D6);

    #[test]
    fn an_en_passant_capture_takes_the_pawn_that_passed_rather_than_the_square_it_lands_on() {
        let placement: PiecePlacement = BEFORE.parse().unwrap();
        assert_eq!(CAPTURE.victim(&placement), placement.piece_at(Square::D5));
        assert_eq!(CAPTURE.play(placement).unwrap().to_string(), AFTER);
    }
}
