mod identity_word;

use core::fmt;

use identity_word::IdentityWord;

use crate::uci_error::UciError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Identity<'line> {
    Name(&'line str),
    Author(&'line str),
}

impl<'line> TryFrom<&'line str> for Identity<'line> {
    type Error = UciError;

    fn try_from(rest: &'line str) -> Result<Identity<'line>, UciError> {
        let trimmed = rest.trim();
        let (kind, text) = trimmed
            .split_once(char::is_whitespace)
            .unwrap_or((trimmed, ""));
        match kind
            .parse::<IdentityWord>()
            .map_err(|_| UciError::UnknownResponse)?
        {
            IdentityWord::Name => Ok(Identity::Name(text.trim())),
            IdentityWord::Author => Ok(Identity::Author(text.trim())),
        }
    }
}

impl fmt::Display for Identity<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Identity::Name(name) => write!(formatter, "{} {name}", IdentityWord::Name),
            Identity::Author(author) => write!(formatter, "{} {author}", IdentityWord::Author),
        }
    }
}
