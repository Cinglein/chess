mod destination;
mod origin;

pub use destination::Destination;
pub use origin::Origin;

use crate::square::Square;

pub trait End {
    fn pick(origin: Square, destination: Square) -> Square;
}
