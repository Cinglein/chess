mod empty;
mod hand;
mod holding;
mod placed_piece;
mod rank_placement;
mod rank_token;

pub use empty::Empty;
pub use hand::Hand;
pub use holding::Holding;
pub use placed_piece::PlacedPiece;

use core::fmt;
use core::str::FromStr;

use enum_map::EnumMap;
use fen::{Fen, FenError};
use itertools::{Itertools, process_results};
use strum::{EnumCount, IntoEnumIterator};

use crate::bitboard::Bitboard;
use crate::color::Color;
use crate::leaper::{BlackPawn, King, Knight, Leaper, WhitePawn};
use crate::piece::Piece;
use crate::piece_kind::PieceKind;
use crate::promotion_piece::PromotionPiece;
use crate::rank::Rank;
use crate::slider::{Bishop, Rook, Slider};
use crate::square::Square;
use crate::state::State;
use crate::zobrist::Zobrist;
use crate::zobrist_keys::ZobristKeys;
use rank_placement::RankPlacement;

pub type PiecePlacement = Placement<Empty>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Placement<H: Hand> {
    pieces: EnumMap<Color, EnumMap<PieceKind, Bitboard>>,
    hand: H,
    hash: Zobrist,
}

impl State for Placement<Empty> {}

impl State for Placement<Holding> {}

impl<H: Hand> Placement<H> {
    #[must_use]
    pub fn pieces(&self, color: Color, kind: PieceKind) -> Bitboard {
        self.pieces[color][kind]
    }

    #[must_use]
    pub const fn hash(&self) -> Zobrist {
        self.hash
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
            .find(|piece| self.pieces[piece.color()][piece.kind()].contains(square))
    }

    #[must_use]
    pub fn attackers(&self, square: Square, by: Color, occupied: Bitboard) -> Bitboard {
        let pawn_attacks = match by {
            Color::White => BlackPawn::attacks(square),
            Color::Black => WhitePawn::attacks(square),
        };
        let queens = self.pieces(by, PieceKind::Queen);
        (pawn_attacks & self.pieces(by, PieceKind::Pawn))
            | (Knight::attacks(square) & self.pieces(by, PieceKind::Knight))
            | (King::attacks(square) & self.pieces(by, PieceKind::King))
            | (Bishop::attacks(square, occupied) & (self.pieces(by, PieceKind::Bishop) | queens))
            | (Rook::attacks(square, occupied) & (self.pieces(by, PieceKind::Rook) | queens))
    }

    fn rank_placement(&self, rank: Rank) -> RankPlacement {
        RankPlacement::new(EnumMap::from_fn(|file| {
            self.piece_at(Square::new(file, rank))
        }))
    }
}

impl Placement<Empty> {
    pub const EMPTY: PiecePlacement = Placement {
        pieces: EnumMap::from_array([
            EnumMap::from_array([Bitboard::EMPTY; PieceKind::COUNT]),
            EnumMap::from_array([Bitboard::EMPTY; PieceKind::COUNT]),
        ]),
        hand: Empty,
        hash: Zobrist::EMPTY,
    };

    pub const START: PiecePlacement = Placement {
        pieces: Self::START_PIECES,
        hand: Empty,
        hash: ZobristKeys::KEYS.hash_of(&Self::START_PIECES),
    };

    const START_PIECES: EnumMap<Color, EnumMap<PieceKind, Bitboard>> = EnumMap::from_array([
        EnumMap::from_array([
            Bitboard::rank(Rank::Two),
            Bitboard::from_square(Square::B1).including(Square::G1),
            Bitboard::from_square(Square::C1).including(Square::F1),
            Bitboard::from_square(Square::A1).including(Square::H1),
            Bitboard::from_square(Square::D1),
            Bitboard::from_square(Square::E1),
        ]),
        EnumMap::from_array([
            Bitboard::rank(Rank::Seven),
            Bitboard::from_square(Square::B8).including(Square::G8),
            Bitboard::from_square(Square::C8).including(Square::F8),
            Bitboard::from_square(Square::A8).including(Square::H8),
            Bitboard::from_square(Square::D8),
            Bitboard::from_square(Square::E8),
        ]),
    ]);

    #[must_use]
    pub fn lift(self, square: Square) -> Option<Placement<Holding>> {
        self.piece_at(square).map(|piece| {
            let mut pieces = self.pieces;
            pieces[piece.color()][piece.kind()] &= !Bitboard::from_square(square);
            Placement {
                pieces,
                hand: Holding::new(piece),
                hash: self.hash ^ ZobristKeys::KEYS.piece(piece, square),
            }
        })
    }
}

impl Placement<Holding> {
    #[must_use]
    pub const fn piece(&self) -> Piece {
        self.hand.piece()
    }

    #[must_use]
    pub fn promote(self, promotion: PromotionPiece) -> Placement<Holding> {
        Placement {
            hand: Holding::new(Piece::new(self.piece().color(), promotion.into())),
            ..self
        }
    }

    #[must_use]
    pub fn land(self, square: Square) -> PiecePlacement {
        let piece = self.piece();
        let mut placement = Placement {
            pieces: self.pieces,
            hand: Empty,
            hash: self.hash,
        };
        if let Some(occupant) = placement.lift(square) {
            placement.pieces = occupant.pieces;
            placement.hash = occupant.hash;
        }
        placement.pieces[piece.color()][piece.kind()] |= Bitboard::from_square(square);
        placement.hash ^= ZobristKeys::KEYS.piece(piece, square);
        placement
    }
}

impl FromIterator<PlacedPiece> for PiecePlacement {
    fn from_iter<I: IntoIterator<Item = PlacedPiece>>(pieces: I) -> PiecePlacement {
        pieces
            .into_iter()
            .fold(PiecePlacement::EMPTY, |mut placement, placed| {
                placement.pieces[placed.piece().color()][placed.piece().kind()] |=
                    Bitboard::from_square(placed.square());
                placement.hash ^= ZobristKeys::KEYS.piece(placed.piece(), placed.square());
                placement
            })
    }
}

impl Fen for PiecePlacement {}

impl fmt::Display for PiecePlacement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ranks = Rank::iter().rev().map(|rank| self.rank_placement(rank));
        write!(formatter, "{}", ranks.format("/"))
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
                .map(|placement| placement.pieces(rank))
        });
        process_results(ranks, |ranks| ranks.flatten().collect())
    }
}

#[cfg(test)]
mod tests {
    use fen::FenError;
    use proptest::prelude::*;
    use proptest::sample::select;
    use strum::VariantArray;

    use super::{PiecePlacement, PlacedPiece};
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
            let placement: PiecePlacement = [PlacedPiece::new(square, piece)].into_iter().collect();
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
