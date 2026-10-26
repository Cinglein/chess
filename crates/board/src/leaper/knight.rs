use super::{Leaper, Leaps};
use crate::direction::Direction;

pub struct Knight;

impl Leaper for Knight {
    const LEAPS: Leaps = Leaps::new(&[
        &[Direction::NORTH, Direction::NORTH, Direction::EAST],
        &[Direction::NORTH, Direction::NORTH, Direction::WEST],
        &[Direction::SOUTH, Direction::SOUTH, Direction::EAST],
        &[Direction::SOUTH, Direction::SOUTH, Direction::WEST],
        &[Direction::EAST, Direction::EAST, Direction::NORTH],
        &[Direction::EAST, Direction::EAST, Direction::SOUTH],
        &[Direction::WEST, Direction::WEST, Direction::NORTH],
        &[Direction::WEST, Direction::WEST, Direction::SOUTH],
    ]);
}
