use core::fmt;
use core::iter;
use core::str::FromStr;

use enum_map::EnumMap;
use fen::FenError;
use itertools::{Itertools, process_results};

use crate::file::File;
use crate::piece::Piece;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RankPlacement(EnumMap<File, Option<Piece>>);

impl RankPlacement {
    pub fn new(squares: EnumMap<File, Option<Piece>>) -> Self {
        RankPlacement(squares)
    }

    pub fn pieces(&self) -> impl Iterator<Item = (File, Piece)> + '_ {
        self.0
            .iter()
            .filter_map(|(file, piece)| piece.map(|piece| (file, piece)))
    }

    fn squares_for(letter: char) -> impl Iterator<Item = Result<Option<Piece>, FenError>> {
        let (square, count) = match letter.to_digit(10) {
            Some(empties) => (Ok(None), empties as usize),
            None => (
                letter
                    .encode_utf8(&mut [0; 4])
                    .parse::<Piece>()
                    .map(Some)
                    .map_err(|_| FenError::Piece(letter)),
                1,
            ),
        };
        iter::repeat_n(square, count)
    }
}

impl fmt::Display for RankPlacement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
            .as_slice()
            .chunk_by(|left, right| left.is_none() && right.is_none())
            .try_for_each(|run| match run {
                [Some(piece)] => write!(formatter, "{piece}"),
                empties => write!(formatter, "{}", empties.len()),
            })
    }
}

impl FromStr for RankPlacement {
    type Err = FenError;

    fn from_str(text: &str) -> Result<Self, FenError> {
        process_results(text.chars().flat_map(Self::squares_for), |squares| {
            squares.collect_array()
        })?
        .map(|squares| RankPlacement(EnumMap::from_array(squares)))
        .ok_or(FenError::RankWidth)
    }
}

#[cfg(test)]
mod tests {
    use fen::FenError;

    use super::RankPlacement;

    const ROUNDTRIPS: [&str; 5] = ["rnbqkbnr", "8", "r3k2r", "4P3", "p6P"];
    const REJECTED: [(&str, FenError); 4] = [
        ("9", FenError::RankWidth),
        ("ppppppp", FenError::RankWidth),
        ("4P4", FenError::RankWidth),
        ("RNBQKBNX", FenError::Piece('X')),
    ];

    #[test]
    fn ranks_roundtrip_with_empty_runs_counted() {
        for text in ROUNDTRIPS {
            assert_eq!(text.parse::<RankPlacement>().unwrap().to_string(), text);
        }
    }

    #[test]
    fn ranks_with_the_wrong_width_or_an_unknown_letter_are_rejected() {
        for (text, error) in REJECTED {
            assert_eq!(text.parse::<RankPlacement>(), Err(error), "{text}");
        }
    }
}
