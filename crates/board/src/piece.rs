use core::fmt;
use core::ops::Not;

use strum::{EnumCount, EnumIter, FromRepr, VariantArray};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumCount, EnumIter, FromRepr, VariantArray)]
pub enum Color {
    White,
    Black,
}

impl Color {
    #[must_use]
    pub const fn index(self) -> usize {
        self as usize
    }

    #[must_use]
    pub const fn opposite(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

impl Not for Color {
    type Output = Color;

    fn not(self) -> Color {
        self.opposite()
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    EnumCount,
    EnumIter,
    FromRepr,
    VariantArray,
)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceKind {
    #[must_use]
    pub const fn index(self) -> usize {
        self as usize
    }

    #[must_use]
    pub const fn letter(self) -> char {
        match self {
            PieceKind::Pawn => 'p',
            PieceKind::Knight => 'n',
            PieceKind::Bishop => 'b',
            PieceKind::Rook => 'r',
            PieceKind::Queen => 'q',
            PieceKind::King => 'k',
        }
    }

    #[must_use]
    pub const fn from_letter(letter: char) -> Option<PieceKind> {
        match letter.to_ascii_lowercase() {
            'p' => Some(PieceKind::Pawn),
            'n' => Some(PieceKind::Knight),
            'b' => Some(PieceKind::Bishop),
            'r' => Some(PieceKind::Rook),
            'q' => Some(PieceKind::Queen),
            'k' => Some(PieceKind::King),
            _ => None,
        }
    }
}

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

    use super::{Color, Piece, PieceKind};

    #[test]
    fn each_color_is_the_opposite_of_the_other() {
        assert_eq!(Color::White.opposite(), Color::Black);
        assert_eq!(!Color::Black, Color::White);
    }

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
