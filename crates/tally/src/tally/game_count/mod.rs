mod half_points;

use board::Incremented;
use derive_more::{Add, Display, Sum};
use half_points::HalfPoints;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Add, Sum, Display)]
pub struct GameCount(usize);

impl GameCount {
    const HALF_POINTS_PER_WIN: usize = 2;

    pub(super) fn half_points(self, draws: GameCount) -> HalfPoints {
        HalfPoints::from(Self::HALF_POINTS_PER_WIN * self.0 + draws.0)
    }
}

impl Incremented for GameCount {
    fn incremented(self) -> GameCount {
        GameCount(self.0 + 1)
    }
}
