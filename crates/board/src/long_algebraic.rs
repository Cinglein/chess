use core::fmt;
use core::str::FromStr;

use strum::ParseError;

use crate::chess_move::{ChessMove, MoveKind};
use crate::promotion_piece::PromotionPiece;
use crate::square::Square;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LongAlgebraic {
    origin: Square,
    destination: Square,
    promotion: Option<PromotionPiece>,
}

impl LongAlgebraic {
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
    pub const fn origin(&self) -> Square {
        self.origin
    }

    #[must_use]
    pub const fn destination(&self) -> Square {
        self.destination
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
            .split_at_checked(2)
            .ok_or(ParseError::VariantNotFound)?;
        let (destination, promotion) = rest
            .split_at_checked(2)
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

#[cfg(any(test, feature = "proptest"))]
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

#[cfg(test)]
mod tests {
    use super::LongAlgebraic;
    use crate::board::Board;
    use proptest::prelude::*;

    const POSITIONS: [&str; 4] = [
        "r3k3/1P6/8/8/8/8/8/4K3 w q - 0 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
    ];

    #[test]
    fn any_notation_prints_and_parses_back_to_itself() {
        proptest!(|(notation: LongAlgebraic)| {
            prop_assert_eq!(notation.to_string().parse(), Ok(notation));
        });
    }

    #[test]
    fn every_legal_move_roundtrips_through_its_text_and_resolves_to_itself() {
        for fen in POSITIONS {
            let board: Board = fen.parse().unwrap();
            for legal in board.legal_moves() {
                let notation = LongAlgebraic::from(legal);
                assert_eq!(legal.to_string(), notation.to_string());
                assert_eq!(board.resolve_move(notation), Some(legal), "{notation}");
            }
        }
    }
}
