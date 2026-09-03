use core::fmt;

use crate::color::Color;
use crate::piece_kind::PieceKind;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Piece {
    pub color: Color,
    pub kind: PieceKind,
}

impl Piece {
    #[must_use]
    pub const fn new(color: Color, kind: PieceKind) -> Piece {
        Piece { color, kind }
    }

    #[must_use]
    pub const fn letter(self) -> char {
        match self.color {
            Color::White => self.kind.letter().to_ascii_uppercase(),
            Color::Black => self.kind.letter(),
        }
    }

    #[must_use]
    pub const fn from_letter(letter: char) -> Option<Piece> {
        let color = if letter.is_ascii_uppercase() {
            Color::White
        } else {
            Color::Black
        };
        match PieceKind::from_letter(letter) {
            Some(kind) => Some(Piece::new(color, kind)),
            None => None,
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.letter())
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::Piece;
    use crate::color::Color;
    use crate::piece_kind::PieceKind;

    #[test]
    fn white_pieces_use_uppercase_letters_and_black_lowercase() {
        assert_eq!(Piece::new(Color::White, PieceKind::Knight).letter(), 'N');
        assert_eq!(Piece::new(Color::Black, PieceKind::Knight).letter(), 'n');
        assert_eq!(
            Piece::from_letter('Q'),
            Some(Piece::new(Color::White, PieceKind::Queen))
        );
        assert_eq!(
            Piece::from_letter('k'),
            Some(Piece::new(Color::Black, PieceKind::King))
        );
        assert_eq!(Piece::from_letter('x'), None);
    }

    #[test]
    fn every_piece_letter_roundtrips() {
        for color in Color::iter() {
            for kind in PieceKind::iter() {
                let piece = Piece::new(color, kind);
                assert_eq!(Piece::from_letter(piece.letter()), Some(piece));
            }
        }
    }
}
