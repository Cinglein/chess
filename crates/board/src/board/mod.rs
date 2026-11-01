mod fullmove_number;
mod halfmove_clock;
mod king_safety;
mod move_generator;
#[cfg(feature = "proptest")]
mod playout;

pub use fullmove_number::FullmoveNumber;
pub use halfmove_clock::HalfmoveClock;
pub use move_generator::MoveGenerator;

use core::fmt;
use core::str::FromStr;

use arrayvec::ArrayVec;
use fen::{DashOr, Fen, FenError};
use itertools::Itertools;

use crate::castling_rights::CastlingRights;
use crate::chess_move::{ChessMove, MoveKind};
use crate::color::Color;
use crate::file::File;
use crate::leaper::{BlackPawn, WhitePawn};
use crate::long_algebraic::LongAlgebraic;
use crate::piece_kind::PieceKind;
use crate::placement::PiecePlacement;
#[cfg(feature = "proptest")]
use crate::ply_count::PlyCount;
use crate::rank::Rank;
use crate::square::Square;
use crate::state::State;
use crate::zobrist::Zobrist;
#[cfg(feature = "proptest")]
use crate::zobrist_keys::SplitMix64;
use crate::zobrist_keys::ZobristKeys;
use king_safety::KingSafety;
#[cfg(feature = "proptest")]
use playout::Playout;

pub type MoveList = ArrayVec<ChessMove, { Board::LEGAL_MOVE_CAPACITY }>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Board {
    placement: PiecePlacement,
    side_to_move: Color,
    castling_rights: CastlingRights,
    en_passant_file: Option<File>,
    halfmove_clock: HalfmoveClock,
    fullmove_number: FullmoveNumber,
}

impl State for Board {}

impl Board {
    pub const LEGAL_MOVE_CAPACITY: usize = 218;
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
    pub fn hash(&self) -> Zobrist {
        Zobrist::from(&self.placement)
            ^ ZobristKeys::KEYS.castling(self.castling_rights)
            ^ ZobristKeys::KEYS.en_passant(self.en_passant_file)
            ^ ZobristKeys::KEYS.side_to_move(self.side_to_move)
    }

    #[must_use]
    pub fn in_check(&self) -> bool {
        KingSafety::new(&self.placement, self.side_to_move).is_some_and(|safety| safety.in_check())
    }

    #[must_use]
    pub fn legal_moves(&self) -> MoveList {
        match self.side_to_move {
            Color::White => MoveGenerator::<WhitePawn>::new(self)
                .map_or_else(MoveList::new, MoveGenerator::legal_moves),
            Color::Black => MoveGenerator::<BlackPawn>::new(self)
                .map_or_else(MoveList::new, MoveGenerator::legal_moves),
        }
    }

    #[must_use]
    pub fn resolve_move(&self, notation: LongAlgebraic) -> Option<ChessMove> {
        self.legal_moves()
            .into_iter()
            .find(|chess_move| LongAlgebraic::from(*chess_move) == notation)
    }

    #[must_use]
    pub fn make_move(self, chess_move: ChessMove) -> Option<Board> {
        let piece = self
            .placement
            .piece_at(chess_move.origin())
            .filter(|piece| piece.color() == self.side_to_move)?;
        let placement = chess_move.play(self.placement)?;
        Some(Board {
            placement,
            side_to_move: !self.side_to_move,
            castling_rights: self
                .castling_rights
                .without_touching(chess_move.origin())
                .without_touching(chess_move.destination()),
            en_passant_file: chess_move.en_passant_file(),
            halfmove_clock: if piece.kind() == PieceKind::Pawn
                || placement.occupied().count() < self.placement.occupied().count()
            {
                HalfmoveClock::ZERO
            } else {
                self.halfmove_clock.incremented()
            },
            fullmove_number: match self.side_to_move {
                Color::White => self.fullmove_number,
                Color::Black => self.fullmove_number.following(),
            },
        })
    }

    const fn en_passant_rank(side_to_move: Color) -> Rank {
        match side_to_move {
            Color::White => Rank::Six,
            Color::Black => Rank::Three,
        }
    }

    #[cfg(feature = "proptest")]
    const LONGEST_PLAYOUT: PlyCount = PlyCount::new(60);

    #[cfg(feature = "proptest")]
    fn random_playout(generator: SplitMix64, plies: PlyCount) -> Board {
        (0..plies.plies())
            .fold(Playout::new(generator), |playout, _| playout.advanced())
            .board()
    }

    fn parse_en_passant(side_to_move: Color, text: &str) -> Result<Option<File>, FenError> {
        text.parse::<DashOr<Square>>()
            .map(Option::<Square>::from)
            .map_err(|_| FenError::EnPassant)?
            .map(|square| {
                (square.rank() == Self::en_passant_rank(side_to_move))
                    .then_some(square.file())
                    .ok_or(FenError::EnPassant)
            })
            .transpose()
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
        let [
            placement,
            side_to_move,
            castling_rights,
            en_passant,
            halfmove_clock,
            fullmove_number,
        ] = text
            .split_whitespace()
            .collect_array()
            .ok_or(FenError::FieldCount)?;
        let side_to_move: Color = side_to_move.parse().map_err(|_| FenError::SideToMove)?;
        Ok(Board {
            placement: placement.parse()?,
            side_to_move,
            castling_rights: CastlingRights::from(
                castling_rights.parse::<DashOr<CastlingRights>>()?,
            ),
            en_passant_file: Self::parse_en_passant(side_to_move, en_passant)?,
            halfmove_clock: halfmove_clock
                .parse()
                .map_err(|_| FenError::HalfmoveClock)?,
            fullmove_number: fullmove_number
                .parse()
                .map_err(|_| FenError::FullmoveNumber)?,
        })
    }
}

#[cfg(feature = "proptest")]
impl proptest::arbitrary::Arbitrary for Board {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Board>;

    fn arbitrary_with((): ()) -> Self::Strategy {
        use proptest::strategy::Strategy;
        (
            proptest::arbitrary::any::<u64>(),
            0..=Self::LONGEST_PLAYOUT.plies(),
        )
            .prop_map(|(seed, plies)| {
                Board::random_playout(SplitMix64::new(seed), PlyCount::new(plies))
            })
            .boxed()
    }
}
