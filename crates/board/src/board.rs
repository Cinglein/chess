use core::fmt;
use core::str::FromStr;

use fen::{DashOr, Fen, FenError};

use crate::castling_rights::CastlingRights;
use crate::chess_move::ChessMove;
use crate::color::Color;
use crate::file::File;
use crate::fullmove_number::FullmoveNumber;
use crate::halfmove_clock::HalfmoveClock;
use crate::leaper::{BlackPawn, Pawn, WhitePawn};
use crate::piece::Piece;
use crate::piece_kind::PieceKind;
use crate::piece_placement::PiecePlacement;
use crate::rank::Rank;
use crate::square::Square;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Board {
    placement: PiecePlacement,
    side_to_move: Color,
    castling_rights: CastlingRights,
    en_passant_file: Option<File>,
    halfmove_clock: HalfmoveClock,
    fullmove_number: FullmoveNumber,
}

impl Board {
    pub const START: Board = Board {
        placement: PiecePlacement::START,
        side_to_move: Color::White,
        castling_rights: CastlingRights::ALL,
        en_passant_file: None,
        halfmove_clock: HalfmoveClock::ZERO,
        fullmove_number: FullmoveNumber::FIRST,
    };

    #[must_use]
    pub const fn placement(&self) -> &PiecePlacement {
        &self.placement
    }

    #[must_use]
    pub const fn side_to_move(&self) -> Color {
        self.side_to_move
    }

    #[must_use]
    pub const fn castling_rights(&self) -> CastlingRights {
        self.castling_rights
    }

    #[must_use]
    pub const fn en_passant_file(&self) -> Option<File> {
        self.en_passant_file
    }

    #[must_use]
    pub fn en_passant_square(&self) -> Option<Square> {
        self.en_passant_file
            .map(|file| Square::new(file, Self::en_passant_rank(self.side_to_move)))
    }

    #[must_use]
    pub const fn halfmove_clock(&self) -> HalfmoveClock {
        self.halfmove_clock
    }

    #[must_use]
    pub const fn fullmove_number(&self) -> FullmoveNumber {
        self.fullmove_number
    }

    #[must_use]
    pub fn make_move(self, chess_move: ChessMove) -> Option<Board> {
        let piece = self
            .placement
            .piece_at(chess_move.origin())
            .filter(|piece| piece.color == self.side_to_move)?;
        let placement = self.placement.apply(chess_move)?;
        let captured = placement.occupied().count() < self.placement.occupied().count();
        Some(Board {
            placement,
            side_to_move: !self.side_to_move,
            castling_rights: self
                .castling_rights
                .without_touching(chess_move.origin())
                .without_touching(chess_move.destination()),
            en_passant_file: Self::double_push_file(piece, chess_move),
            halfmove_clock: if piece.kind == PieceKind::Pawn || captured {
                HalfmoveClock::ZERO
            } else {
                self.halfmove_clock.incremented()
            },
            fullmove_number: match self.side_to_move {
                Color::White => self.fullmove_number,
                Color::Black => self.fullmove_number.incremented(),
            },
        })
    }

    const fn en_passant_rank(side_to_move: Color) -> Rank {
        match side_to_move {
            Color::White => Rank::Six,
            Color::Black => Rank::Three,
        }
    }

    fn double_push_file(piece: Piece, chess_move: ChessMove) -> Option<File> {
        let forward = match piece.color {
            Color::White => WhitePawn::PUSH,
            Color::Black => BlackPawn::PUSH,
        };
        let origin = chess_move.origin();
        (piece.kind == PieceKind::Pawn
            && (origin + forward) + forward == Some(chess_move.destination()))
        .then_some(origin.file())
    }

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
    use proptest::prelude::*;
    use proptest::sample::select;
    use strum::VariantArray;

    use super::Board;
    use crate::castling_right::CastlingRight;
    use crate::chess_move::ChessMove;
    use crate::color::Color;
    use crate::promotion::Promotion;
    use crate::square::Square;

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
    const TRANSITIONS: [(&str, ChessMove, &str); 7] = [
        (
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            ChessMove::Normal {
                origin: Square::E2,
                destination: Square::E4,
            },
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
        ),
        (
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
            ChessMove::Normal {
                origin: Square::G8,
                destination: Square::F6,
            },
            "rnbqkb1r/pppppppp/5n2/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 1 2",
        ),
        (
            "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1",
            ChessMove::Castling(CastlingRight::WhiteKingside),
            "r3k2r/8/8/8/8/8/8/R4RK1 b kq - 1 1",
        ),
        (
            "r3k2r/8/8/8/8/8/8/R4RK1 b kq - 1 1",
            ChessMove::Castling(CastlingRight::BlackQueenside),
            "2kr3r/8/8/8/8/8/8/R4RK1 w - - 2 2",
        ),
        (
            "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1",
            ChessMove::Normal {
                origin: Square::A1,
                destination: Square::A8,
            },
            "R3k2r/8/8/8/8/8/8/4K2R b Kk - 0 1",
        ),
        (
            "rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 3",
            ChessMove::EnPassant {
                origin: Square::E5,
                destination: Square::D6,
            },
            "rnbqkbnr/ppp1pppp/3P4/8/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 3",
        ),
        (
            "r3k3/1P6/8/8/8/8/8/4K3 w q - 0 1",
            ChessMove::Promotion {
                origin: Square::B7,
                destination: Square::A8,
                piece: Promotion::Queen,
            },
            "Q3k3/8/8/8/8/8/8/4K3 b - - 0 1",
        ),
    ];

    #[test]
    fn positions_roundtrip_through_fen_and_malformed_fields_are_named() {
        assert_eq!(START.parse::<Board>(), Ok(Board::START));
        for text in [START].into_iter().chain(ROUNDTRIPS) {
            assert_eq!(text.parse::<Board>().unwrap().to_string(), text);
        }
        for (valid, broken, error) in REJECTED {
            let text = START.replace(valid, broken);
            assert_eq!(text.parse::<Board>(), Err(error), "{text}");
        }
    }

    #[test]
    fn making_a_move_produces_the_position_fen_describes() {
        for (before, chess_move, after) in TRANSITIONS {
            let board = before.parse::<Board>().unwrap().make_move(chess_move);
            let played = board.map(|board| board.to_string());
            assert_eq!(played.as_deref(), Some(after), "{before} {chess_move}");
        }
    }

    #[test]
    fn a_move_is_applied_exactly_when_the_side_to_move_owns_the_origin() {
        proptest!(|(origin in select(Square::VARIANTS), destination in select(Square::VARIANTS))| {
            let owned = Board::START.placement().occupied_by(Color::White).contains(origin);
            prop_assert_eq!(Board::START.make_move(ChessMove::Normal { origin, destination }).is_some(), owned);
        });
    }
}
