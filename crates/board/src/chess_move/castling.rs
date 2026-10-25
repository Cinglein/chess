use super::MoveKind;
use crate::castling_right::CastlingRight;
use crate::castling_squares::CastlingSquares;
use crate::end::Destination;
use crate::end::Origin;
use crate::leaper::King;
use crate::placement::PiecePlacement;
use crate::slider::Rook;
use crate::square::Square;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Castling(CastlingRight);

impl Castling {
    #[must_use]
    pub const fn new(right: CastlingRight) -> Castling {
        Castling(right)
    }
}

impl MoveKind for Castling {
    fn origin(&self) -> Square {
        CastlingSquares::new(self.0).square::<King, Origin>()
    }

    fn destination(&self) -> Square {
        CastlingSquares::new(self.0).square::<King, Destination>()
    }

    fn play(self, placement: PiecePlacement) -> Option<PiecePlacement> {
        let squares = CastlingSquares::new(self.0);
        Some(
            placement
                .lift(squares.square::<King, Origin>())?
                .land(squares.square::<King, Destination>())
                .lift(squares.square::<Rook, Origin>())?
                .land(squares.square::<Rook, Destination>()),
        )
    }
}
