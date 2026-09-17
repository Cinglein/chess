use super::Leaper;
use crate::orthogonal::Orthogonal;

pub trait Pawn: Leaper {
    const PUSH: Orthogonal;
}
