use core::fmt;

use fen::{Fen, FenError};

use super::Board;
use crate::square::Square;

impl Board {
    fn parse_en_passant(text: &str) -> Result<Option<Square>, FenError> {
        match text {
            "-" => Ok(None),
            square => square.parse().map(Some).map_err(|_| FenError::EnPassant),
        }
    }
}

impl Fen for Board {
    fn fmt_fen(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.placement.fmt_fen(formatter)?;
        write!(formatter, " {} ", self.side_to_move)?;
        self.castling_rights.fmt_fen(formatter)?;
        match self.en_passant {
            Some(square) => write!(formatter, " {square}")?,
            None => formatter.write_str(" -")?,
        }
        write!(
            formatter,
            " {} {}",
            self.halfmove_clock, self.fullmove_number
        )
    }

    fn from_fen(text: &str) -> Result<Board, FenError> {
        let mut fields = text.split_whitespace();
        let mut field = || fields.next().ok_or(FenError::FieldCount);
        let placement = Fen::from_fen(field()?)?;
        let side_to_move = field()?.parse().map_err(|_| FenError::SideToMove)?;
        let castling_rights = Fen::from_fen(field()?)?;
        let en_passant = Self::parse_en_passant(field()?)?;
        let halfmove_clock = field()?.parse().map_err(|_| FenError::HalfmoveClock)?;
        let fullmove_number = field()?
            .parse()
            .ok()
            .filter(|number| *number > 0)
            .ok_or(FenError::FullmoveNumber)?;
        if fields.next().is_some() {
            return Err(FenError::FieldCount);
        }
        Ok(Board {
            placement,
            side_to_move,
            castling_rights,
            en_passant,
            halfmove_clock,
            fullmove_number,
        })
    }
}

#[cfg(test)]
mod tests {
    use fen::{Fen, FenError};

    use super::Board;
    use crate::castling_rights::CastlingRights;
    use crate::color::Color;
    use crate::square::Square;

    const START: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    const AFTER_E4_E5_F4: &str = "rnbqkbnr/pppp1ppp/8/4p3/4PP2/8/PPPP2PP/RNBQKBNR b KQkq f3 0 2";
    const KIWIPETE: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";

    #[test]
    fn the_start_position_is_the_standard_fen() {
        assert_eq!(Board::START.fen().to_string(), START);
        assert_eq!(Board::from_fen(START), Ok(Board::START));
    }

    #[test]
    fn every_field_roundtrips() {
        let board = Board::from_fen(AFTER_E4_E5_F4).unwrap();
        assert_eq!(board.side_to_move(), Color::Black);
        assert_eq!(board.castling_rights(), CastlingRights::ALL);
        assert_eq!(board.en_passant(), Some(Square::F3));
        assert_eq!(board.halfmove_clock(), 0);
        assert_eq!(board.fullmove_number(), 2);
        assert_eq!(board.fen().to_string(), AFTER_E4_E5_F4);
        assert_eq!(
            Board::from_fen(KIWIPETE).unwrap().fen().to_string(),
            KIWIPETE
        );
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
            assert_eq!(Board::from_fen(text), Err(error), "{text}");
        }
    }
}
