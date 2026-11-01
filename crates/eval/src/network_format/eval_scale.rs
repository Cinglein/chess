use derive_more::Display;
use newtype::Newtype;

use crate::score::Score;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, Newtype)]
pub struct EvalScale(i16);

impl EvalScale {
    #[must_use]
    pub fn centipawns(self) -> Score {
        Score::new(i32::from(self.0))
    }

    #[must_use]
    pub fn per_unit(self) -> f32 {
        f32::from(self.0)
    }
}
