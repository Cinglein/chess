use crate::square::Square;

pub trait Step {
    #[must_use]
    fn between(origin: Square, destination: Square) -> Self;
}
