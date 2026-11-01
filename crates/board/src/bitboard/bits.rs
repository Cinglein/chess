use strum::EnumCount;

use super::Bitboard;
use crate::axis::Axis;
use crate::direction::Direction;
use crate::file::File;

impl Bitboard {
    #[must_use]
    pub const fn shift(self, direction: Direction) -> Bitboard {
        let vertical = match direction.step(Axis::Rank) {
            1 => self.0 << File::COUNT,
            -1 => self.0 >> File::COUNT,
            _ => self.0,
        };
        let horizontal = match direction.step(Axis::File) {
            1 => (vertical & !Self::file(File::H).0) << 1,
            -1 => (vertical & !Self::file(File::A).0) >> 1,
            _ => vertical,
        };
        Bitboard(horizontal)
    }
}
