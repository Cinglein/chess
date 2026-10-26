use super::{Leaper, Leaps, Pawn};
use crate::color::Color;
use crate::direction::Direction;
use crate::orthogonal::Orthogonal;
use crate::rank::Rank;

pub struct WhitePawn;

impl Leaper for WhitePawn {
    const LEAPS: Leaps = Leaps::new(&[&[Direction::NORTH_EAST], &[Direction::NORTH_WEST]]);
}

impl Pawn for WhitePawn {
    const COLOR: Color = Color::White;
    const PUSH: Orthogonal = Orthogonal::North;
    const START_RANK: Rank = Rank::Two;
    const PROMOTION_RANK: Rank = Rank::Eight;
}
