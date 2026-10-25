use crate::bitboard::Bitboard;
use crate::castling_right::CastlingRight;
use crate::square::Square;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CastlingSquares {
    king_origin: Square,
    king_destination: Square,
    rook_origin: Square,
    rook_destination: Square,
}

impl CastlingSquares {
    #[must_use]
    pub const fn new(right: CastlingRight) -> CastlingSquares {
        let (king_origin, king_destination, rook_origin, rook_destination) = match right {
            CastlingRight::WhiteKingside => (Square::E1, Square::G1, Square::H1, Square::F1),
            CastlingRight::WhiteQueenside => (Square::E1, Square::C1, Square::A1, Square::D1),
            CastlingRight::BlackKingside => (Square::E8, Square::G8, Square::H8, Square::F8),
            CastlingRight::BlackQueenside => (Square::E8, Square::C8, Square::A8, Square::D8),
        };
        CastlingSquares {
            king_origin,
            king_destination,
            rook_origin,
            rook_destination,
        }
    }

    #[must_use]
    pub const fn king_origin(self) -> Square {
        self.king_origin
    }

    #[must_use]
    pub const fn king_destination(self) -> Square {
        self.king_destination
    }

    #[must_use]
    pub const fn rook_origin(self) -> Square {
        self.rook_origin
    }

    #[must_use]
    pub const fn rook_destination(self) -> Square {
        self.rook_destination
    }

    #[must_use]
    pub const fn footprint(self) -> Bitboard {
        Bitboard::from_square(self.king_origin).including(self.rook_origin)
    }
}
