use core::fmt;
use core::str::FromStr;

use fen::{DashOr, Fen, FenError};

use super::Board;
use crate::castling_rights::CastlingRights;
use crate::color::Color;
use crate::file::File;
use crate::square::Square;

impl Board {
    fn parse_en_passant(side_to_move: Color, text: &str) -> Result<Option<File>, FenError> {
        match text.parse::<DashOr<Square>>() {
            Ok(DashOr::Dash) => Ok(None),
            Ok(DashOr::Value(square)) if square.rank() == Self::en_passant_rank(side_to_move) => {
                Ok(Some(square.file()))
            }
            _ => Err(FenError::EnPassant),
        }
    }
}

impl Fen for Board {}

impl fmt::Display for Board {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} {} {} {} {} {}",
            self.placement,
            self.side_to_move,
            DashOr::from(self.castling_rights),
            DashOr::from(self.en_passant_square()),
            self.halfmove_clock,
            self.fullmove_number
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
        let castling_rights = CastlingRights::from(field()?.parse::<DashOr<CastlingRights>>()?);
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
    const ROUNDTRIPS: [&str; 2] = [
        "rnbqkbnr/pppp1ppp/8/4p3/4PP2/8/PPPP2PP/RNBQKBNR b KQkq f3 0 2",
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
    ];
    const REJECTED: [(&str, &str, FenError); 8] = [
        (" 0 1", " 0", FenError::FieldCount),
        (" 0 1", " 0 1 extra", FenError::FieldCount),
        (" w ", " x ", FenError::SideToMove),
        ("KQkq", "KQkx", FenError::CastlingRights),
        (" - ", " z9 ", FenError::EnPassant),
        (" - ", " e3 ", FenError::EnPassant),
        (" 0 1", " -1 1", FenError::HalfmoveClock),
        (" 0 1", " 0 0", FenError::FullmoveNumber),
    ];

    #[test]
    fn positions_roundtrip_through_fen() {
        assert_eq!(Board::START.to_string(), START);
        assert_eq!(START.parse::<Board>(), Ok(Board::START));
        for text in ROUNDTRIPS {
            assert_eq!(text.parse::<Board>().unwrap().to_string(), text);
        }
    }

    #[test]
    fn malformed_fields_are_rejected_with_the_field_named() {
        for (valid, broken, error) in REJECTED {
            let text = START.replace(valid, broken);
            assert_eq!(text.parse::<Board>(), Err(error), "{text}");
        }
    }
}
