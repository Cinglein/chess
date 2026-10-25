mod elo;
mod option_name;
mod option_word;
mod switch;

pub use elo::Elo;
pub use switch::Switch;

use core::fmt;

use option_name::OptionName;
use option_word::OptionWord;

use crate::uci_error::UciError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EngineOption<'line> {
    LimitStrength(Switch),
    Elo(Elo),
    Other {
        name: &'line str,
        setting: &'line str,
    },
}

impl<'line> TryFrom<&'line str> for EngineOption<'line> {
    type Error = UciError;

    fn try_from(rest: &'line str) -> Result<EngineOption<'line>, UciError> {
        let (name, setting) = rest
            .trim()
            .strip_prefix(<&str>::from(OptionWord::Name))
            .and_then(|named| named.split_once(<&str>::from(OptionWord::Value)))
            .ok_or(UciError::UnknownOption)?;
        let setting = setting.trim();
        match name.trim().parse::<OptionName>() {
            Ok(OptionName::LimitStrength) => setting
                .parse()
                .map(EngineOption::LimitStrength)
                .map_err(|_| UciError::UnknownOption),
            Ok(OptionName::Elo) => setting
                .parse()
                .map(EngineOption::Elo)
                .map_err(|_| UciError::UnknownOption),
            Err(_) => Ok(EngineOption::Other {
                name: name.trim(),
                setting,
            }),
        }
    }
}

impl fmt::Display for EngineOption<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} ", OptionWord::Name)?;
        match self {
            EngineOption::LimitStrength(switch) => {
                write!(
                    formatter,
                    "{} {} {switch}",
                    OptionName::LimitStrength,
                    OptionWord::Value
                )
            }
            EngineOption::Elo(elo) => {
                write!(formatter, "{} {} {elo}", OptionName::Elo, OptionWord::Value)
            }
            EngineOption::Other { name, setting } => {
                write!(formatter, "{name} {} {setting}", OptionWord::Value)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::{Elo, EngineOption, Switch};

    const OTHER: EngineOption<'static> = EngineOption::Other {
        name: "Hash",
        setting: "16",
    };

    #[test]
    fn typed_and_foreign_options_print_back_to_themselves() {
        proptest!(|(rating: u16, limited: bool)| {
            let switch = if limited { Switch::True } else { Switch::False };
            for option in [EngineOption::Elo(Elo::new(rating)), EngineOption::LimitStrength(switch), OTHER] {
                let line = option.to_string();
                prop_assert_eq!(EngineOption::try_from(line.as_str()), Ok(option));
            }
        });
    }
}
