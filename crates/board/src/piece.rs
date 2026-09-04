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
    fn every_piece_roundtrips_through_a_letter_whose_case_is_its_colour() {
        for color in Color::iter() {
            for kind in PieceKind::iter() {
                let piece = Piece::new(color, kind);
                let text = piece.to_string();
                assert_eq!(text.parse(), Ok(piece));
                assert_eq!(
                    text.chars().all(|letter| letter.is_ascii_uppercase()),
                    color == Color::White
                );
            }
        }
        assert!("x".parse::<Piece>().is_err());
    }
}
