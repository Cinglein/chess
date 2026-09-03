use core::fmt;
use core::str::FromStr;

use strum::ParseError;

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
}

impl fmt::Display for Piece {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for letter in self.kind.as_ref().chars() {
            let letter = match self.color {
                Color::White => letter.to_ascii_uppercase(),
                Color::Black => letter,
            };
            write!(formatter, "{letter}")?;
        }
        Ok(())
    }
}

impl FromStr for Piece {
    type Err = ParseError;

    fn from_str(text: &str) -> Result<Piece, ParseError> {
        let kind = text.parse()?;
        let color = if text.bytes().all(|byte| byte.is_ascii_uppercase()) {
            Color::White
        } else {
            Color::Black
        };
        Ok(Piece::new(color, kind))
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::Piece;
    use crate::color::Color;
    use crate::piece_kind::PieceKind;

    #[test]
    fn white_pieces_are_uppercase_letters_and_black_lowercase() {
        assert_eq!(Piece::new(Color::White, PieceKind::Knight).to_string(), "N");
        assert_eq!(Piece::new(Color::Black, PieceKind::Knight).to_string(), "n");
        assert_eq!("Q".parse(), Ok(Piece::new(Color::White, PieceKind::Queen)));
        assert_eq!("k".parse(), Ok(Piece::new(Color::Black, PieceKind::King)));
        assert!("x".parse::<Piece>().is_err());
        assert!("".parse::<Piece>().is_err());
    }

    #[test]
    fn every_piece_roundtrips_through_its_letter() {
        for color in Color::iter() {
            for kind in PieceKind::iter() {
                let piece = Piece::new(color, kind);
                assert_eq!(piece.to_string().parse(), Ok(piece));
            }
        }
    }
}
