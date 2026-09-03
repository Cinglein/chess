#![cfg_attr(not(test), no_std)]

mod bitboard;
mod color;
mod diagonal;
mod direction;
mod file;
mod orthogonal;
mod piece;
mod piece_kind;
mod rank;
mod square;
mod squares;

pub use bitboard::Bitboard;
pub use color::Color;
pub use diagonal::Diagonal;
pub use direction::Direction;
pub use file::File;
pub use orthogonal::Orthogonal;
pub use piece::Piece;
pub use piece_kind::PieceKind;
pub use rank::Rank;
pub use square::Square;
pub use squares::Squares;
