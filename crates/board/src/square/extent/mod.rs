mod between;
mod through;

pub use between::Between;
pub use through::Through;

use crate::bitboard::Bitboard;
use crate::square::Square;

pub trait Extent {
    fn trace(from: Square, to: Square) -> Bitboard;
}
