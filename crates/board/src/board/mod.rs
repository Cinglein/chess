mod fen;

use crate::castling_rights::CastlingRights;
use crate::color::Color;
use crate::piece_placement::PiecePlacement;
use crate::square::Square;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Board {
    placement: PiecePlacement,
    side_to_move: Color,
    castling_rights: CastlingRights,
    en_passant: Option<Square>,
    halfmove_clock: u8,
    fullmove_number: u16,
}

impl Board {
    pub const START: Board = Board {
        placement: PiecePlacement::START,
        side_to_move: Color::White,
        castling_rights: CastlingRights::ALL,
        en_passant: None,
        halfmove_clock: 0,
        fullmove_number: 1,
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
    pub const fn en_passant(&self) -> Option<Square> {
        self.en_passant
    }

    #[must_use]
    pub const fn halfmove_clock(&self) -> u8 {
        self.halfmove_clock
    }

    #[must_use]
    pub const fn fullmove_number(&self) -> u16 {
        self.fullmove_number
    }
}
