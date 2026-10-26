use super::{Leaper, Leaps, Pawn};
use crate::color::Color;
use crate::direction::Direction;
use crate::orthogonal::Orthogonal;
use crate::rank::Rank;

pub struct BlackPawn;

impl Leaper for BlackPawn {
    const LEAPS: Leaps = Leaps::new(&[&[Direction::SOUTH_EAST], &[Direction::SOUTH_WEST]]);
}

impl Pawn for BlackPawn {
    const COLOR: Color = Color::Black;
    const PUSH: Orthogonal = Orthogonal::South;
    const START_RANK: Rank = Rank::Seven;
    const PROMOTION_RANK: Rank = Rank::One;
}
