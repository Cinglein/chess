#![cfg_attr(not(test), no_std)]

mod dash_or;
mod fen;

pub use dash_or::DashOr;
pub use fen::{Fen, FenError};
