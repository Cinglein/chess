use super::Direction;
use crate::axis::Axis;
use crate::diagonal::Diagonal;
use crate::orthogonal::Orthogonal;

impl Direction {
    #[must_use]
    pub const fn step(self, axis: Axis) -> i8 {
        match (axis, self) {
            (Axis::File, Direction::Orthogonal(Orthogonal::North | Orthogonal::South))
            | (Axis::Rank, Direction::Orthogonal(Orthogonal::East | Orthogonal::West)) => 0,
            (
                Axis::File,
                Direction::Orthogonal(Orthogonal::East)
                | Direction::Diagonal(Diagonal::NorthEast | Diagonal::SouthEast),
            )
            | (
                Axis::Rank,
                Direction::Orthogonal(Orthogonal::North)
                | Direction::Diagonal(Diagonal::NorthEast | Diagonal::NorthWest),
            ) => 1,
            (
                Axis::File,
                Direction::Orthogonal(Orthogonal::West)
                | Direction::Diagonal(Diagonal::NorthWest | Diagonal::SouthWest),
            )
            | (
                Axis::Rank,
                Direction::Orthogonal(Orthogonal::South)
                | Direction::Diagonal(Diagonal::SouthEast | Diagonal::SouthWest),
            ) => -1,
        }
    }
}
