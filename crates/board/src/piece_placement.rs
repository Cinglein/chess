mod fen;

use enum_map::EnumMap;
use strum::{EnumCount, IntoEnumIterator};

use crate::bitboard::Bitboard;
use crate::color::Color;
use crate::piece::Piece;
use crate::piece_kind::PieceKind;
use crate::rank::Rank;
use crate::square::Square;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PiecePlacement {
    pieces: EnumMap<Color, EnumMap<PieceKind, Bitboard>>,
}

impl PiecePlacement {
    pub const EMPTY: PiecePlacement = PiecePlacement {
        pieces: EnumMap::from_array([
            EnumMap::from_array([Bitboard::EMPTY; PieceKind::COUNT]),
            EnumMap::from_array([Bitboard::EMPTY; PieceKind::COUNT]),
        ]),
    };

    pub const START: PiecePlacement = PiecePlacement {
        pieces: EnumMap::from_array([
            EnumMap::from_array([
                Bitboard::rank(Rank::Two),
                Bitboard::from_square(Square::B1).with(Square::G1),
                Bitboard::from_square(Square::C1).with(Square::F1),
                Bitboard::from_square(Square::A1).with(Square::H1),
                Bitboard::from_square(Square::D1),
                Bitboard::from_square(Square::E1),
            ]),
            EnumMap::from_array([
                Bitboard::rank(Rank::Seven),
                Bitboard::from_square(Square::B8).with(Square::G8),
                Bitboard::from_square(Square::C8).with(Square::F8),
                Bitboard::from_square(Square::A8).with(Square::H8),
                Bitboard::from_square(Square::D8),
                Bitboard::from_square(Square::E8),
            ]),
        ]),
    };

    #[must_use]
    pub fn pieces(&self, color: Color, kind: PieceKind) -> Bitboard {
        self.pieces[color][kind]
    }

    #[must_use]
    pub fn occupied_by(&self, color: Color) -> Bitboard {
        self.pieces[color]
            .values()
            .fold(Bitboard::EMPTY, |occupied, pieces| occupied | *pieces)
    }

    #[must_use]
    pub fn occupied(&self) -> Bitboard {
        self.occupied_by(Color::White) | self.occupied_by(Color::Black)
    }

    #[must_use]
    pub fn piece_at(&self, square: Square) -> Option<Piece> {
        Color::iter()
            .flat_map(|color| PieceKind::iter().map(move |kind| Piece::new(color, kind)))
            .find(|piece| self.pieces[piece.color][piece.kind].contains(square))
    }

    #[must_use]
    pub fn with(mut self, piece: Piece, square: Square) -> PiecePlacement {
        self.pieces[piece.color][piece.kind] |= Bitboard::from_square(square);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::PiecePlacement;
    use crate::bitboard::Bitboard;
    use crate::color::Color;
    use crate::piece::Piece;
    use crate::piece_kind::PieceKind;
    use crate::square::Square;

    #[test]
    fn the_start_position_has_thirty_two_pieces_with_the_kings_on_e1_and_e8() {
        let start = PiecePlacement::START;
        assert_eq!(start.occupied().count(), 32);
        assert_eq!(start.occupied_by(Color::White).count(), 16);
        assert_eq!(
            start.piece_at(Square::E1),
            Some(Piece::new(Color::White, PieceKind::King))
        );
        assert_eq!(
            start.piece_at(Square::E8),
            Some(Piece::new(Color::Black, PieceKind::King))
        );
        assert_eq!(start.piece_at(Square::E4), None);
    }

    #[test]
    fn placing_a_piece_adds_it_to_its_own_bitboard_only() {
        let knight = Piece::new(Color::Black, PieceKind::Knight);
        let placement = PiecePlacement::EMPTY.with(knight, Square::F6);
        assert_eq!(placement.piece_at(Square::F6), Some(knight));
        assert_eq!(placement.pieces(Color::Black, PieceKind::Knight).count(), 1);
        assert_eq!(placement.occupied_by(Color::White), Bitboard::EMPTY);
    }
}
