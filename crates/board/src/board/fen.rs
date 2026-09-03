use core::fmt;
use core::str::FromStr;

use fen::{Fen, FenError};

use super::Board;
use crate::color::Color;
use crate::file::File;
use crate::square::Square;

impl Board {
    fn parse_en_passant(side_to_move: Color, text: &str) -> Result<Option<File>, FenError> {
        if text == "-" {
            return Ok(None);
        }
        text.parse::<Square>()
            .ok()
            .filter(|square| square.rank() == Self::en_passant_rank(side_to_move))
            .map(|square| Some(square.file()))
            .ok_or(FenError::EnPassant)
    }
}

impl Fen for Board {}

impl fmt::Display for Board {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} {} {}",
            self.placement, self.side_to_move, self.castling_rights
        )?;
        match self.en_passant_square() {
            Some(square) => write!(formatter, " {square}")?,
            None => formatter.write_str(" -")?,
        }
        write!(
            formatter,
            " {} {}",
            self.halfmove_clock, self.fullmove_number
        )
    }
}

impl FromStr for Board {
    type Err = FenError;

    fn from_str(text: &str) -> Result<Board, FenError> {
        let mut fields = text.split_whitespace();
        let mut field = || fields.next().ok_or(FenError::FieldCount);
        let placement = field()?.parse()?;
        let side_to_move = field()?.parse().map_err(|_| FenError::SideToMove)?;
        let castling_rights = field()?.parse()?;
        let en_passant_file = Self::parse_en_passant(side_to_move, field()?)?;
        let halfmove_clock = field()?.parse().map_err(|_| FenError::HalfmoveClock)?;
        let fullmove_number = field()?.parse().map_err(|_| FenError::FullmoveNumber)?;
        if fields.next().is_some() {
            return Err(FenError::FieldCount);
        }
        Ok(Board {
            placement,
            side_to_move,
            castling_rights,
            en_passant_file,
            halfmove_clock,
            fullmove_number,
        })
    }
}

#[cfg(test)]
mod tests {
    use fen::FenError;

    use super::Board;

    const START: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    const AFTER_E4_E5_F4: &str = "rnbqkbnr/pppp1ppp/8/4p3/4PP2/8/PPPP2PP/RNBQKBNR b KQkq f3 0 2";
    const KIWIPETE: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";

    #[test]
    fn positions_roundtrip_through_fen() {
        assert_eq!(Board::START.to_string(), START);
        assert_eq!(START.parse::<Board>(), Ok(Board::START));
        for text in [AFTER_E4_E5_F4, KIWIPETE] {
            assert_eq!(text.parse::<Board>().unwrap().to_string(), text);
        }
    }

    #[test]
    fn malformed_fields_are_rejected_with_the_field_named() {
        let cases = [
            (
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0",
                FenError::FieldCount,
            ),
            (
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 extra",
                FenError::FieldCount,
            ),
            (
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR x KQkq - 0 1",
                FenError::SideToMove,
            ),
            (
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkx - 0 1",
                FenError::CastlingRights,
            ),
            (
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq z9 0 1",
                FenError::EnPassant,
            ),
            (
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - -1 1",
                FenError::HalfmoveClock,
            ),
            (
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 0",
                FenError::FullmoveNumber,
            ),
        ];
        for (text, error) in cases {
            assert_eq!(text.parse::<Board>(), Err(error), "{text}");
        }
    }
}
