use core::fmt;
use core::str::FromStr;

use enum_map::EnumMap;
use fen::{Fen, FenError};
use strum::{EnumCount, IntoEnumIterator};

use crate::bitboard::Bitboard;
use crate::color::Color;
use crate::file::File;
use crate::piece::Piece;
use crate::piece_kind::PieceKind;
use crate::rank::Rank;
use crate::square::Square;

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
    pub fn with(mut self, piece: Piece, square: Square) -> PiecePlacement {
        self.pieces[piece.color][piece.kind] |= Bitboard::from_square(square);
        self
    }

    fn parse_rank(self, rank: Rank, text: &str) -> Result<PiecePlacement, FenError> {
        let (placement, files) =
            text.chars()
                .try_fold((self, 0usize), |(placement, files), letter| {
                    letter
                        .to_digit(10)
                        .filter(|digit| (1..=9).contains(digit))
                        .map_or_else(
                            || placement.place_letter(rank, files, letter),
                            |skipped| Ok((placement, files + skipped as usize)),
                        )
                })?;
        if files == File::COUNT {
            Ok(placement)
        } else {
            Err(FenError::RankWidth)
        }
    }

    fn place_letter(
        self,
        rank: Rank,
        files: usize,
        letter: char,
    ) -> Result<(PiecePlacement, usize), FenError> {
        let piece = letter
            .encode_utf8(&mut [0; 4])
            .parse::<Piece>()
            .map_err(|_| FenError::Piece(letter))?;
        let file = u8::try_from(files)
            .ok()
            .and_then(File::from_repr)
            .ok_or(FenError::RankWidth)?;
        Ok((self.with(piece, Square::new(file, rank)), files + 1))
    }

    fn fmt_rank(&self, formatter: &mut fmt::Formatter<'_>, rank: Rank) -> fmt::Result {
        let empties = File::iter().try_fold(0, |empties, file| {
            match self.piece_at(Square::new(file, rank)) {
                Some(piece) => {
                    if empties > 0 {
                        write!(formatter, "{empties}")?;
                    }
                    write!(formatter, "{piece}").map(|()| 0)
                }
                None => Ok(empties + 1),
            }
        })?;
        if empties > 0 {
            write!(formatter, "{empties}")?;
        }
        Ok(())
    }
}

impl Fen for PiecePlacement {}

impl fmt::Display for PiecePlacement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ranks = Rank::iter().rev();
        if let Some(rank) = ranks.next() {
            self.fmt_rank(formatter, rank)?;
        }
        ranks.try_for_each(|rank| {
            formatter.write_str("/")?;
            self.fmt_rank(formatter, rank)
        })
    }
}

impl FromStr for PiecePlacement {
    type Err = FenError;

    fn from_str(text: &str) -> Result<PiecePlacement, FenError> {
        if text.split('/').count() != Rank::COUNT {
            return Err(FenError::RankCount);
        }
        Rank::iter()
            .rev()
            .zip(text.split('/'))
            .try_fold(PiecePlacement::EMPTY, |placement, (rank, text)| {
                placement.parse_rank(rank, text)
            })
    }
}

#[cfg(test)]
mod tests {
    use fen::FenError;

    use super::PiecePlacement;
    use crate::color::Color;
    use crate::piece::Piece;
    use crate::piece_kind::PieceKind;
    use crate::square::Square;

    const START: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

    #[test]
    fn the_start_position_roundtrips_through_fen() {
        assert_eq!(PiecePlacement::START.to_string(), START);
        assert_eq!(START.parse::<PiecePlacement>(), Ok(PiecePlacement::START));
        assert_eq!(PiecePlacement::START.occupied().count(), 32);
        assert_eq!(
            PiecePlacement::START.piece_at(Square::E1),
            Some(Piece::new(Color::White, PieceKind::King))
        );
        assert_eq!(PiecePlacement::START.piece_at(Square::E4), None);
    }

    #[test]
    fn empty_squares_inside_a_rank_are_counted_in_both_directions() {
        let text = "r3k2r/8/8/3pP3/8/8/8/R3K2R";
        let placement = text.parse::<PiecePlacement>().unwrap();
        assert_eq!(placement.to_string(), text);
        assert_eq!(placement.occupied().count(), 8);
        assert_eq!(
            placement.piece_at(Square::D5),
            Some(Piece::new(Color::Black, PieceKind::Pawn))
        );
    }

    #[test]
    fn malformed_placements_are_rejected() {
        assert_eq!(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP".parse::<PiecePlacement>(),
            Err(FenError::RankCount)
        );
        assert_eq!(
            "rnbqkbnr/pppppppp/9/8/8/8/PPPPPPPP/RNBQKBNR".parse::<PiecePlacement>(),
            Err(FenError::RankWidth)
        );
        assert_eq!(
            "rnbqkbnr/ppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".parse::<PiecePlacement>(),
            Err(FenError::RankWidth)
        );
        assert_eq!(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNX".parse::<PiecePlacement>(),
            Err(FenError::Piece('X'))
        );
    }
}
