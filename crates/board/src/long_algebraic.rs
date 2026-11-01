use core::fmt;
use core::str::FromStr;

use strum::ParseError;

use crate::chess_move::{ChessMove, MoveKind};
use crate::end::End;
use crate::promotion_piece::PromotionPiece;
use crate::square::Square;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LongAlgebraic {
    origin: Square,
    destination: Square,
    promotion: Option<PromotionPiece>,
}

impl LongAlgebraic {
    const SQUARE_TEXT_LENGTH: usize = 2;

    #[must_use]
    pub const fn new(
        origin: Square,
        destination: Square,
        promotion: Option<PromotionPiece>,
    ) -> LongAlgebraic {
        LongAlgebraic {
            origin,
            destination,
            promotion,
        }
    }

    #[must_use]
    pub fn square<E: End>(&self) -> Square {
        E::pick(self.origin, self.destination)
    }
}

impl From<ChessMove> for LongAlgebraic {
    fn from(chess_move: ChessMove) -> LongAlgebraic {
        LongAlgebraic {
            origin: chess_move.origin(),
            destination: chess_move.destination(),
            promotion: chess_move.promotion_piece(),
        }
    }
}

impl fmt::Display for LongAlgebraic {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}{}", self.origin, self.destination)?;
        self.promotion
            .map_or(Ok(()), |piece| write!(formatter, "{piece}"))
    }
}

impl FromStr for LongAlgebraic {
    type Err = ParseError;

    fn from_str(text: &str) -> Result<LongAlgebraic, ParseError> {
        let (origin, rest) = text
            .split_at_checked(Self::SQUARE_TEXT_LENGTH)
            .ok_or(ParseError::VariantNotFound)?;
        let (destination, promotion) = rest
            .split_at_checked(Self::SQUARE_TEXT_LENGTH)
            .ok_or(ParseError::VariantNotFound)?;
        Ok(LongAlgebraic {
            origin: origin.parse()?,
            destination: destination.parse()?,
            promotion: match promotion {
                "" => None,
                letter => Some(letter.parse()?),
            },
        })
    }
}

#[cfg(feature = "proptest")]
impl proptest::arbitrary::Arbitrary for LongAlgebraic {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<LongAlgebraic>;

    fn arbitrary_with((): ()) -> Self::Strategy {
        use proptest::strategy::Strategy;
        (
            proptest::arbitrary::any::<Square>(),
            proptest::arbitrary::any::<Square>(),
            proptest::arbitrary::any::<Option<PromotionPiece>>(),
        )
            .prop_map(|(origin, destination, promotion)| {
                LongAlgebraic::new(origin, destination, promotion)
            })
            .boxed()
    }
}
