#![cfg_attr(not(test), no_std)]

mod bitboard;
mod color;
mod diagonal;
mod direction;
mod file;
mod r#move;
mod move_kind;
mod orthogonal;
mod piece;
mod piece_kind;
mod promotion;
mod rank;
mod square;
mod square_iter;

pub use bitboard::Bitboard;
pub use color::Color;
pub use diagonal::Diagonal;
pub use direction::Direction;
pub use file::File;
pub use r#move::Move;
pub use move_kind::MoveKind;
pub use orthogonal::Orthogonal;
pub use piece::Piece;
pub use piece_kind::PieceKind;
pub use promotion::Promotion;
pub use rank::Rank;
pub use square::Square;
pub use square_iter::SquareIter;
