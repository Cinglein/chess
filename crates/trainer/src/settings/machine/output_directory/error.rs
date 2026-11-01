use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Error)]
pub enum OutputDirectoryError {
    #[error("an output directory is not empty")]
    Empty,
    #[error("an output directory holds no NUL")]
    Nul,
}
