use core::fmt;
use core::str::FromStr;

use bitfield_struct::bitfield;
use fen::{Fen, FenError};

#[bitfield(u8, new = false, default = false)]
#[derive(PartialEq, Eq, Hash)]
pub struct CastlingRights {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
    #[bits(4)]
    __: u8,
}

impl CastlingRights {
    pub const NONE: CastlingRights = CastlingRights(0);
    pub const ALL: CastlingRights = CastlingRights::NONE
        .with_white_kingside(true)
        .with_white_queenside(true)
        .with_black_kingside(true)
        .with_black_queenside(true);

    const LETTERS: [(char, CastlingRights); 4] = [
        ('K', CastlingRights::NONE.with_white_kingside(true)),
        ('Q', CastlingRights::NONE.with_white_queenside(true)),
        ('k', CastlingRights::NONE.with_black_kingside(true)),
        ('q', CastlingRights::NONE.with_black_queenside(true)),
    ];

    #[must_use]
    pub const fn is_none(self) -> bool {
        self.0 == 0
    }

    #[must_use]
    pub const fn contains(self, other: CastlingRights) -> bool {
        self.0 & other.0 == other.0
    }

    #[must_use]
    pub const fn union(self, other: CastlingRights) -> CastlingRights {
        CastlingRights(self.0 | other.0)
    }
}

impl Fen for CastlingRights {}

impl fmt::Display for CastlingRights {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_none() {
            return formatter.write_str("-");
        }
        Self::LETTERS
            .iter()
            .filter(|(_, right)| self.contains(*right))
            .try_for_each(|(letter, _)| write!(formatter, "{letter}"))
    }
}

impl FromStr for CastlingRights {
    type Err = FenError;

    fn from_str(text: &str) -> Result<CastlingRights, FenError> {
        match text {
            "-" => return Ok(CastlingRights::NONE),
            "" => return Err(FenError::CastlingRights),
            _ => {}
        }
        text.chars()
            .try_fold(CastlingRights::NONE, |rights, letter| {
                Self::LETTERS
                    .iter()
                    .find(|(candidate, _)| *candidate == letter)
                    .map(|(_, right)| *right)
                    .filter(|right| !rights.contains(*right))
                    .map(|right| rights.union(right))
                    .ok_or(FenError::CastlingRights)
            })
    }
}

#[cfg(test)]
mod tests {
    use fen::{Fen, FenError};

    use super::CastlingRights;

    #[test]
    fn rights_display_and_parse_as_fen_letters() {
        assert_eq!(CastlingRights::ALL.to_string(), "KQkq");
        assert_eq!(CastlingRights::NONE.to_string(), "-");
        let partial = CastlingRights::from_fen("Kq").unwrap();
        assert!(partial.white_kingside());
        assert!(!partial.white_queenside());
        assert!(!partial.black_kingside());
        assert!(partial.black_queenside());
        assert_eq!(partial.to_string(), "Kq");
        assert_eq!(CastlingRights::from_fen("qK"), Ok(partial));
    }

    #[test]
    fn repeated_or_unknown_letters_are_rejected() {
        assert_eq!(
            CastlingRights::from_fen("KK"),
            Err(FenError::CastlingRights)
        );
        assert_eq!(CastlingRights::from_fen("x"), Err(FenError::CastlingRights));
        assert_eq!(CastlingRights::from_fen(""), Err(FenError::CastlingRights));
    }
}
