use core::fmt;
use core::str::FromStr;

use strum::ParseError;

use crate::color::Color;
use crate::piece_kind::PieceKind;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Piece {
    color: Color,
    kind: PieceKind,
}

impl Piece {
    #[must_use]
    pub const fn new(color: Color, kind: PieceKind) -> Piece {
        Piece { color, kind }
    }

    #[must_use]
    pub const fn color(self) -> Color {
        self.color
    }

    #[must_use]
    pub const fn kind(self) -> PieceKind {
        self.kind
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

#[cfg(any(test, feature = "proptest"))]
impl proptest::arbitrary::Arbitrary for Piece {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Piece>;

    fn arbitrary_with((): ()) -> Self::Strategy {
        use proptest::strategy::Strategy;
        (
            proptest::arbitrary::any::<Color>(),
            proptest::arbitrary::any::<PieceKind>(),
        )
            .prop_map(|(color, kind)| Piece::new(color, kind))
            .boxed()
    }
}
