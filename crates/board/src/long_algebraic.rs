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

    #[must_use]
    pub const fn promotion(&self) -> Option<PromotionPiece> {
        self.promotion
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

#[cfg(test)]
mod tests {
    use super::LongAlgebraic;
    use crate::board::Board;
    use crate::perft_position::PerftPosition;

    #[test]
    fn every_legal_move_roundtrips_through_its_text_and_resolves_to_itself() {
        for position in &PerftPosition::REFERENCE {
            let board: Board = position.fen().parse().unwrap();
            for legal in board.legal_moves() {
                let notation = LongAlgebraic::from(legal);
                assert_eq!(notation.to_string().parse(), Ok(notation));
                assert_eq!(board.resolve_move(notation), Some(legal), "{notation}");
            }
        }
    }
}
