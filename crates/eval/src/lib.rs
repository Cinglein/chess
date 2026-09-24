#![cfg_attr(not(test), no_std)]

mod piece_square_tables;
mod score;

pub use piece_square_tables::{Evaluator, PieceKindValue, PieceSquareTables};
pub use score::Score;
