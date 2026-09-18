use super::Leaper;
use crate::color::Color;
use crate::orthogonal::Orthogonal;
use crate::rank::Rank;

pub trait Pawn: Leaper {
    const COLOR: Color;
    const PUSH: Orthogonal;
    const START_RANK: Rank;
    const PROMOTION_RANK: Rank;
}
