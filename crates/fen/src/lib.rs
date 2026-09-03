#![cfg_attr(not(test), no_std)]

mod fen;
mod fen_error;

pub use fen::Fen;
pub use fen_error::FenError;
