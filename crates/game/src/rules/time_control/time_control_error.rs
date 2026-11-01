use std::num::ParseFloatError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum TimeControlError {
    #[error("a time control is seconds+increment, such as 10+0.1")]
    Shape,
    #[error("{0}")]
    Number(#[from] ParseFloatError),
}
