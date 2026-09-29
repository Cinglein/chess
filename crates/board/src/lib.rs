#![cfg_attr(not(test), no_std)]

mod bitboard;
mod board;
mod castling_right;
mod castling_rights;
mod castling_squares;
mod chess_move;
mod color;
mod diagonal;
mod direction;
mod file;
mod leaper;
mod long_algebraic;
mod orthogonal;
#[cfg(test)]
mod perft_position;
mod piece;
mod piece_kind;
mod placement;
mod promotion_piece;
mod rank;
mod slider;
mod square;
mod state;
mod zobrist;
mod zobrist_keys;

pub use bitboard::{Bitboard, SubsetIter};
pub use board::{Board, HalfmoveClock, MoveList};
pub use chess_move::{ChessMove, MoveKind};
pub use color::Color;
pub use long_algebraic::LongAlgebraic;
pub use piece_kind::PieceKind;
pub use slider::{Bishop, Magic, Rook, Slider};
pub use square::Square;
pub use state::State;
pub use zobrist::Zobrist;
