#![cfg_attr(not(test), no_std)]

pub mod attacks;
mod bitboard;
mod moves;
mod piece;
mod square;

pub use bitboard::{Bitboard, Squares, Subsets};
pub use moves::{Move, MoveKind, Promotion};
pub use piece::{Color, Piece, PieceKind};
pub use square::{Direction, File, ParseSquareError, Rank, Square};
