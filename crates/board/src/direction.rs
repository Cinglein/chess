use strum::IntoEnumIterator;

use crate::diagonal::Diagonal;
use crate::orthogonal::Orthogonal;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Orthogonal(Orthogonal),
    Diagonal(Diagonal),
}

impl Direction {
    pub const NORTH: Direction = Direction::Orthogonal(Orthogonal::North);
    pub const EAST: Direction = Direction::Orthogonal(Orthogonal::East);
    pub const SOUTH: Direction = Direction::Orthogonal(Orthogonal::South);
    pub const WEST: Direction = Direction::Orthogonal(Orthogonal::West);
    pub const NORTH_EAST: Direction = Direction::Diagonal(Diagonal::NorthEast);
    pub const SOUTH_EAST: Direction = Direction::Diagonal(Diagonal::SouthEast);
    pub const SOUTH_WEST: Direction = Direction::Diagonal(Diagonal::SouthWest);
    pub const NORTH_WEST: Direction = Direction::Diagonal(Diagonal::NorthWest);

    pub fn iter() -> impl Iterator<Item = Direction> {
        Orthogonal::iter()
            .map(Direction::Orthogonal)
            .chain(Diagonal::iter().map(Direction::Diagonal))
    }
}

impl From<Orthogonal> for Direction {
    fn from(orthogonal: Orthogonal) -> Direction {
        Direction::Orthogonal(orthogonal)
    }
}

impl From<Diagonal> for Direction {
    fn from(diagonal: Diagonal) -> Direction {
        Direction::Diagonal(diagonal)
    }
}
