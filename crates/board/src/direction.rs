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

    #[must_use]
    pub const fn file_step(self) -> i8 {
        match self {
            Direction::Orthogonal(Orthogonal::North | Orthogonal::South) => 0,
            Direction::Orthogonal(Orthogonal::East)
            | Direction::Diagonal(Diagonal::NorthEast | Diagonal::SouthEast) => 1,
            Direction::Orthogonal(Orthogonal::West)
            | Direction::Diagonal(Diagonal::NorthWest | Diagonal::SouthWest) => -1,
        }
    }

    #[must_use]
    pub const fn rank_step(self) -> i8 {
        match self {
            Direction::Orthogonal(Orthogonal::East | Orthogonal::West) => 0,
            Direction::Orthogonal(Orthogonal::North)
            | Direction::Diagonal(Diagonal::NorthEast | Diagonal::NorthWest) => 1,
            Direction::Orthogonal(Orthogonal::South)
            | Direction::Diagonal(Diagonal::SouthEast | Diagonal::SouthWest) => -1,
        }
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
