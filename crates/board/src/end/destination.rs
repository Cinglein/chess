use super::End;
use crate::square::Square;

pub struct Destination;

impl End for Destination {
    fn pick(_: Square, destination: Square) -> Square {
        destination
    }
}
