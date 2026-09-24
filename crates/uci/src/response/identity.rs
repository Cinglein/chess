use core::fmt;

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
        match kind {
            "name" => Ok(Identity::Name(text.trim())),
            "author" => Ok(Identity::Author(text.trim())),
            _ => Err(UciError::UnknownResponse),
        }
    }
}

impl fmt::Display for Identity<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Identity::Name(name) => write!(formatter, "name {name}"),
            Identity::Author(author) => write!(formatter, "author {author}"),
        }
    }
}
