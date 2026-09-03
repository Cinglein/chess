use strum::IntoEnumIterator;

use crate::diagonal::Diagonal;
use crate::orthogonal::Orthogonal;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Orthogonal(Orthogonal),
    Diagonal(Diagonal),
}

impl Direction {
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
