use core::ops::Add;

use super::Square;
use crate::axis::Axis;
use crate::direction::Direction;
use crate::file::File;
use crate::rank::Rank;

impl<D: Into<Direction>> Add<D> for Square {
    type Output = Option<Square>;

    fn add(self, direction: D) -> Option<Square> {
        let direction = direction.into();
        let file_index = (self.file() as u8).checked_add_signed(direction.step(Axis::File))?;
        let rank_index = (self.rank() as u8).checked_add_signed(direction.step(Axis::Rank))?;
        Some(Square::new(
            File::from_repr(file_index)?,
            Rank::from_repr(rank_index)?,
        ))
    }
}
