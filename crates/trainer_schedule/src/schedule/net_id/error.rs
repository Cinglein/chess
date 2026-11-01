use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Error)]
pub enum NetIdError {
    #[error("a net id is not empty")]
    Empty,
    #[error("a net id holds letters, digits, '_' or '-'")]
    Character,
}
