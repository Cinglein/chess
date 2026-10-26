use super::{Leaper, Leaps};
use crate::direction::Direction;

pub struct King;

impl Leaper for King {
    const LEAPS: Leaps = Leaps::new(&[
        &[Direction::NORTH],
        &[Direction::NORTH_EAST],
        &[Direction::EAST],
        &[Direction::SOUTH_EAST],
        &[Direction::SOUTH],
        &[Direction::SOUTH_WEST],
        &[Direction::WEST],
        &[Direction::NORTH_WEST],
    ]);
}
