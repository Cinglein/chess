use core::fmt;
use core::str::FromStr;

use enum_map::EnumMap;
use fen::{Fen, FenError};
use strum::{EnumCount, IntoEnumIterator};

use super::PiecePlacement;
use super::rank_placement::RankPlacement;
use crate::rank::Rank;
use crate::square::Square;

impl Fen for PiecePlacement {}

impl PiecePlacement {
    fn rank_placement(&self, rank: Rank) -> RankPlacement {
        RankPlacement::new(EnumMap::from_fn(|file| {
            self.piece_at(Square::new(file, rank))
        }))
    }

    fn with_rank(self, rank: Rank, placement: &RankPlacement) -> PiecePlacement {
        placement.pieces().fold(self, |board, (file, piece)| {
            board.with(piece, Square::new(file, rank))
        })
    }
}

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
        Rank::iter()
            .rev()
            .zip(text.split('/'))
            .try_fold(PiecePlacement::EMPTY, |placement, (rank, text)| {
                Ok(placement.with_rank(rank, &text.parse()?))
            })
    }
}

#[cfg(test)]
mod tests {
    use fen::FenError;

    use super::PiecePlacement;

    const START: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

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
