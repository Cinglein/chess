#![cfg_attr(not(test), no_std)]

mod dash_or;
mod fen;
mod fen_error;

pub use dash_or::DashOr;
pub use fen::Fen;
pub use fen_error::FenError;
