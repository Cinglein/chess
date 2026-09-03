mod black_pawn;
mod king;
mod knight;
mod leaps;
mod white_pawn;

pub use black_pawn::BlackPawn;
pub use king::King;
pub use knight::Knight;
pub use leaps::Leaps;
pub use white_pawn::WhitePawn;

use enum_map::EnumMap;

use crate::bitboard::Bitboard;
use crate::square::Square;

pub trait Leaper {
    const LEAPS: Leaps;
    const TABLE: EnumMap<Square, Bitboard> = Self::LEAPS.table();

    #[must_use]
    fn attacks(square: Square) -> Bitboard {
        Self::TABLE[square]
    }
}
