use strum::{EnumCount, EnumIter, VariantArray};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumCount, EnumIter, VariantArray)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub const ORTHOGONAL: [Direction; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    pub const DIAGONAL: [Direction; 4] = [
        Direction::NorthEast,
        Direction::SouthEast,
        Direction::SouthWest,
        Direction::NorthWest,
    ];

    #[must_use]
    pub const fn file_delta(self) -> isize {
        match self {
            Direction::North | Direction::South => 0,
            Direction::NorthEast | Direction::East | Direction::SouthEast => 1,
            Direction::SouthWest | Direction::West | Direction::NorthWest => -1,
        }
    }

    #[must_use]
    pub const fn rank_delta(self) -> isize {
        match self {
            Direction::East | Direction::West => 0,
            Direction::North | Direction::NorthEast | Direction::NorthWest => 1,
            Direction::South | Direction::SouthEast | Direction::SouthWest => -1,
        }
    }
}
