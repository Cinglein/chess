use core::fmt;
use core::str::FromStr;

use fen::{Fen, FenError};
use strum::{EnumCount, IntoEnumIterator};

use super::PiecePlacement;
use crate::file::File;
use crate::piece::Piece;
use crate::rank::Rank;
use crate::square::Square;

impl Fen for PiecePlacement {}

impl PiecePlacement {
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

    const START: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

    #[test]
    fn the_start_position_roundtrips_through_fen() {
        assert_eq!(PiecePlacement::START.to_string(), START);
        assert_eq!(START.parse::<PiecePlacement>(), Ok(PiecePlacement::START));
    }

    #[test]
    fn empty_squares_inside_a_rank_are_counted_in_both_directions() {
        let text = "r3k2r/8/8/3pP3/8/8/8/R3K2R";
        assert_eq!(text.parse::<PiecePlacement>().unwrap().to_string(), text);
    }

    #[test]
    fn malformed_placements_are_rejected() {
        let cases = [
            ("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP", FenError::RankCount),
            (
                "rnbqkbnr/pppppppp/9/8/8/8/PPPPPPPP/RNBQKBNR",
                FenError::RankWidth,
            ),
            (
                "rnbqkbnr/ppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR",
                FenError::RankWidth,
            ),
            (
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNX",
                FenError::Piece('X'),
            ),
        ];
        for (text, error) in cases {
            assert_eq!(text.parse::<PiecePlacement>(), Err(error), "{text}");
        }
    }
}
