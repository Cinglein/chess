mod option_word;

use core::fmt;

use option_word::OptionWord;

use crate::uci_error::UciError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EngineOption<'line> {
    name: &'line str,
    setting: &'line str,
}

impl<'line> EngineOption<'line> {
    #[must_use]
    pub const fn new(name: &'line str, setting: &'line str) -> EngineOption<'line> {
        EngineOption { name, setting }
    }

    #[must_use]
    pub const fn name(&self) -> &'line str {
        self.name
    }

    #[must_use]
    pub const fn setting(&self) -> &'line str {
        self.setting
    }
}

impl<'line> TryFrom<&'line str> for EngineOption<'line> {
    type Error = UciError;

    fn try_from(rest: &'line str) -> Result<EngineOption<'line>, UciError> {
        let (name, setting) = rest
            .trim()
            .strip_prefix(<&str>::from(OptionWord::Name))
            .and_then(|named| named.split_once(<&str>::from(OptionWord::Value)))
            .ok_or(UciError::UnknownOption)?;
        Ok(EngineOption {
            name: name.trim(),
            setting: setting.trim(),
        })
    }
}

impl fmt::Display for EngineOption<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} {} {} {}",
            OptionWord::Name,
            self.name,
            OptionWord::Value,
            self.setting
        )
    }
}
