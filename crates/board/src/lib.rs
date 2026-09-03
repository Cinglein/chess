#![cfg_attr(not(test), no_std)]

mod piece;
mod square;

pub use piece::{Color, Piece, PieceKind};
pub use square::{Direction, File, Rank, Square};
