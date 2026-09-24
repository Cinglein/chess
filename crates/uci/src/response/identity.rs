use core::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Identity {
    Name(&'static str),
    Author(&'static str),
}

impl fmt::Display for Identity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Identity::Name(name) => write!(formatter, "name {name}"),
            Identity::Author(author) => write!(formatter, "author {author}"),
        }
    }
}
