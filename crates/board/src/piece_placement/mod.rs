mod lifted;
mod rank_placement;

pub use lifted::Lifted;

use core::fmt;
use core::str::FromStr;

use enum_map::EnumMap;
use fen::{Fen, FenError};
use itertools::process_results;
use strum::{EnumCount, IntoEnumIterator};

use crate::bitboard::Bitboard;
use crate::color::Color;
use crate::piece::Piece;
use crate::piece_kind::PieceKind;
use crate::rank::Rank;
use crate::square::Square;
use rank_placement::RankPlacement;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PiecePlacement {
    pieces: EnumMap<Color, EnumMap<PieceKind, Bitboard>>,
}

impl PiecePlacement {
    pub const EMPTY: PiecePlacement = PiecePlacement {
        pieces: EnumMap::from_array([
            EnumMap::from_array([Bitboard::EMPTY; PieceKind::COUNT]),
            EnumMap::from_array([Bitboard::EMPTY; PieceKind::COUNT]),
        ]),
    };

    pub const START: PiecePlacement = PiecePlacement {
        pieces: EnumMap::from_array([
            EnumMap::from_array([
                Bitboard::rank(Rank::Two),
                Bitboard::from_square(Square::B1).with(Square::G1),
                Bitboard::from_square(Square::C1).with(Square::F1),
                Bitboard::from_square(Square::A1).with(Square::H1),
                Bitboard::from_square(Square::D1),
                Bitboard::from_square(Square::E1),
            ]),
            EnumMap::from_array([
                Bitboard::rank(Rank::Seven),
                Bitboard::from_square(Square::B8).with(Square::G8),
                Bitboard::from_square(Square::C8).with(Square::F8),
                Bitboard::from_square(Square::A8).with(Square::H8),
                Bitboard::from_square(Square::D8),
                Bitboard::from_square(Square::E8),
            ]),
        ]),
    };

    #[must_use]
    pub fn pieces(&self, color: Color, kind: PieceKind) -> Bitboard {
        self.pieces[color][kind]
    }

    #[must_use]
    pub fn occupied_by(&self, color: Color) -> Bitboard {
        self.pieces[color]
            .values()
            .fold(Bitboard::EMPTY, |occupied, pieces| occupied | *pieces)
    }

    #[must_use]
    pub fn occupied(&self) -> Bitboard {
        self.occupied_by(Color::White) | self.occupied_by(Color::Black)
    }

    #[must_use]
    pub fn piece_at(&self, square: Square) -> Option<Piece> {
        Color::iter()
            .flat_map(|color| PieceKind::iter().map(move |kind| Piece::new(color, kind)))
            .find(|piece| self.pieces[piece.color][piece.kind].contains(square))
    }

    #[must_use]
    pub fn lift(self, square: Square) -> Option<Lifted> {
        self.piece_at(square).map(|piece| {
            let mut placement = self;
            placement.pieces[piece.color][piece.kind] &= !Bitboard::from_square(square);
            Lifted { placement, piece }
        })
    }

    fn rank_placement(&self, rank: Rank) -> RankPlacement {
        RankPlacement::new(EnumMap::from_fn(|file| {
            self.piece_at(Square::new(file, rank))
        }))
    }
}

impl FromIterator<(Square, Piece)> for PiecePlacement {
    fn from_iter<I: IntoIterator<Item = (Square, Piece)>>(pieces: I) -> PiecePlacement {
        pieces
            .into_iter()
            .fold(PiecePlacement::EMPTY, |mut placement, (square, piece)| {
                placement.pieces[piece.color][piece.kind] |= Bitboard::from_square(square);
                placement
            })
    }
}

impl Fen for PiecePlacement {}

impl fmt::Display for PiecePlacement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ranks = Rank::iter().rev();
        if let Some(rank) = ranks.next() {
            write!(formatter, "{}", self.rank_placement(rank))?;
        }
        ranks.try_for_each(|rank| write!(formatter, "/{}", self.rank_placement(rank)))
    }
}

impl FromStr for PiecePlacement {
    type Err = FenError;

    fn from_str(text: &str) -> Result<PiecePlacement, FenError> {
        if text.split('/').count() != Rank::COUNT {
            return Err(FenError::RankCount);
        }
        let ranks = Rank::iter().rev().zip(text.split('/')).map(|(rank, text)| {
            text.parse::<RankPlacement>()
                .map(|placement| (rank, placement))
        });
        process_results(ranks, |ranks| {
            ranks
                .flat_map(|(rank, placement)| {
                    placement
                        .pieces()
                        .map(move |(file, piece)| (Square::new(file, rank), piece))
                })
                .collect()
        })
    }
}

#[cfg(test)]
mod tests {
    use fen::FenError;
    use proptest::prelude::*;
    use proptest::sample::select;
    use strum::VariantArray;

    use super::PiecePlacement;
    use crate::bitboard::Bitboard;
    use crate::color::Color;
    use crate::piece::Piece;
    use crate::piece_kind::PieceKind;
    use crate::square::Square;

    const START: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

    #[test]
    fn a_placed_piece_is_found_on_its_square_and_nowhere_else() {
        proptest!(|(color in select(Color::VARIANTS), kind in select(PieceKind::VARIANTS), square in select(Square::VARIANTS))| {
            let piece = Piece::new(color, kind);
            let placement: PiecePlacement = [(square, piece)].into_iter().collect();
            prop_assert_eq!(placement.piece_at(square), Some(piece));
            prop_assert_eq!(placement.pieces(color, kind), Bitboard::from_square(square));
            prop_assert_eq!(placement.occupied(), Bitboard::from_square(square));
        });
    }

    #[test]
    fn placements_roundtrip_through_fen() {
        assert_eq!(PiecePlacement::START.to_string(), START);
        assert_eq!(START.parse::<PiecePlacement>(), Ok(PiecePlacement::START));
        let mixed = "r3k2r/8/8/3pP3/8/8/8/R3K2R";
        assert_eq!(mixed.parse::<PiecePlacement>().unwrap().to_string(), mixed);
    }

    #[test]
    fn a_placement_needs_exactly_eight_ranks() {
        assert_eq!(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP".parse::<PiecePlacement>(),
            Err(FenError::RankCount)
        );
    }
}
