use core::fmt;
use core::str::FromStr;

use enumset::EnumSet;
use fen::{Fen, FenError};

use crate::castling_right::CastlingRight;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CastlingRights(EnumSet<CastlingRight>);

impl CastlingRights {
    pub const NONE: CastlingRights = CastlingRights(EnumSet::empty());
    pub const ALL: CastlingRights = CastlingRights(EnumSet::all());

    #[must_use]
    pub fn is_none(self) -> bool {
        self.0.is_empty()
    }

    #[must_use]
    pub fn contains(self, right: CastlingRight) -> bool {
        self.0.contains(right)
    }
}

impl Fen for CastlingRights {}

impl fmt::Display for CastlingRights {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
            .iter()
            .try_for_each(|right| write!(formatter, "{right}"))
    }
}

impl FromStr for CastlingRights {
    type Err = FenError;

    fn from_str(text: &str) -> Result<CastlingRights, FenError> {
        let rights: EnumSet<CastlingRight> = text
            .chars()
            .map(|letter| letter.encode_utf8(&mut [0; 4]).parse::<CastlingRight>())
            .collect::<Result<_, _>>()
            .map_err(|_| FenError::CastlingRights)?;
        (rights.len() == text.len() && !rights.is_empty())
            .then_some(CastlingRights(rights))
            .ok_or(FenError::CastlingRights)
    }
}

#[cfg(test)]
mod tests {
    use fen::FenError;

    use super::CastlingRights;
    use crate::castling_right::CastlingRight;

    #[test]
    fn rights_display_and_parse_as_fen_letters() {
        assert_eq!(CastlingRights::ALL.to_string(), "KQkq");
        let partial = "qK".parse::<CastlingRights>().unwrap();
        assert!(partial.contains(CastlingRight::WhiteKingside));
        assert!(!partial.contains(CastlingRight::WhiteQueenside));
        assert_eq!(partial.to_string(), "Kq");
    }

    #[test]
    fn repeated_unknown_or_missing_letters_are_rejected() {
        for text in ["KK", "x", ""] {
            assert_eq!(
                text.parse::<CastlingRights>(),
                Err(FenError::CastlingRights),
                "{text}"
            );
        }
    }
}
