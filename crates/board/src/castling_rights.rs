use core::fmt;
use core::str::FromStr;

use enum_map::EnumMap;
use enumset::{EnumSet, enum_set_union};
use fen::{DashOr, Fen, FenError};
use strum::{EnumCount, VariantArray};

use crate::castling_right::CastlingRight;
use crate::castling_squares::CastlingSquares;
use crate::square::Square;
use crate::state::State;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CastlingRights(EnumSet<CastlingRight>);

impl State for CastlingRights {}

impl CastlingRights {
    pub const NONE: CastlingRights = CastlingRights(EnumSet::empty());
    pub const ALL: CastlingRights = CastlingRights(EnumSet::all());
    const REVOKED_BY: EnumMap<Square, CastlingRights> = {
        let mut table = [CastlingRights::NONE; Square::COUNT];
        let mut squares = Square::VARIANTS;
        while let [square, rest @ ..] = squares {
            table[*square as usize] = Self::revoked_by(*square);
            squares = rest;
        }
        EnumMap::from_array(table)
    };

    #[must_use]
    pub fn is_none(self) -> bool {
        self.0.is_empty()
    }

    #[must_use]
    pub fn contains(self, right: CastlingRight) -> bool {
        self.0.contains(right)
    }

    #[must_use]
    pub fn without_touching(self, square: Square) -> CastlingRights {
        CastlingRights(self.0.difference(Self::REVOKED_BY[square].0))
    }

    const fn revoked_by(square: Square) -> CastlingRights {
        let mut revoked: EnumSet<CastlingRight> = EnumSet::empty();
        let mut rights = CastlingRight::VARIANTS;
        while let [right, rest @ ..] = rights {
            if CastlingSquares::new(*right).footprint().contains(square) {
                let right = *right;
                revoked = enum_set_union!(revoked, right);
            }
            rights = rest;
        }
        CastlingRights(revoked)
    }
}

impl Fen for CastlingRights {}

impl From<CastlingRights> for DashOr<CastlingRights> {
    fn from(rights: CastlingRights) -> Self {
        if rights.is_none() {
            DashOr::Dash
        } else {
            DashOr::Value(rights)
        }
    }
}

impl From<DashOr<CastlingRights>> for CastlingRights {
    fn from(field: DashOr<CastlingRights>) -> Self {
        match field {
            DashOr::Dash => CastlingRights::NONE,
            DashOr::Value(rights) => rights,
        }
    }
}

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

#[cfg(any(test, feature = "proptest"))]
impl proptest::arbitrary::Arbitrary for CastlingRights {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<CastlingRights>;

    fn arbitrary_with((): ()) -> Self::Strategy {
        use proptest::strategy::Strategy;
        proptest::arbitrary::any::<u8>()
            .prop_map(|bits| CastlingRights(EnumSet::from_u8_truncated(bits)))
            .boxed()
    }
}

#[cfg(test)]
mod tests {
    use fen::{DashOr, FenError};
    use proptest::prelude::*;

    use super::CastlingRights;

    const REJECTED: [&str; 3] = ["KK", "x", ""];

    #[test]
    fn any_set_of_rights_prints_as_a_fen_field_that_parses_back() {
        proptest!(|(rights: CastlingRights)| {
            let field = DashOr::from((!rights.is_none()).then_some(rights)).to_string();
            let parsed: Option<CastlingRights> = field.parse::<DashOr<CastlingRights>>().unwrap().into();
            prop_assert_eq!(parsed.unwrap_or(CastlingRights::NONE), rights);
        });
    }

    #[test]
    fn repeated_unknown_or_missing_letters_are_rejected() {
        for text in REJECTED {
            assert_eq!(
                text.parse::<CastlingRights>(),
                Err(FenError::CastlingRights),
                "{text}"
            );
        }
    }
}
