mod fen;
mod make_move;

use crate::castling_rights::CastlingRights;
use crate::color::Color;
use crate::file::File;
use crate::fullmove_number::FullmoveNumber;
use crate::halfmove_clock::HalfmoveClock;
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

    const fn en_passant_rank(side_to_move: Color) -> Rank {
        match side_to_move {
            Color::White => Rank::Six,
            Color::Black => Rank::Three,
        }
    }

    #[must_use]
    pub const fn halfmove_clock(&self) -> HalfmoveClock {
        self.halfmove_clock
    }

    #[must_use]
    pub const fn fullmove_number(&self) -> FullmoveNumber {
        self.fullmove_number
    }
}
