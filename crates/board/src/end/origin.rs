use super::End;
use crate::square::Square;

pub struct Origin;

impl End for Origin {
    fn pick(origin: Square, _: Square) -> Square {
        origin
    }
}
