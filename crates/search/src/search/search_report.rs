use eval::Score;

use super::depth::Depth;

pub trait SearchReport {
    #[must_use]
    fn depth(&self) -> Depth;

    #[must_use]
    fn score(&self) -> Score;
}
