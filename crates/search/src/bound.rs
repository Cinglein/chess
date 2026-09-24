use core::marker::PhantomData;
use core::ops::Neg;

use eval::Score;

use crate::limit::Limit;
use crate::lower::Lower;
use crate::upper::Upper;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Bound<L: Limit> {
    score: Score,
    limit: PhantomData<L>,
}

impl<L: Limit> Bound<L> {
    pub(crate) const fn new(score: Score) -> Bound<L> {
        Bound {
            score,
            limit: PhantomData,
        }
    }
}

impl Bound<Lower> {
    pub(crate) const LOWEST: Bound<Lower> = Bound::new(Score::INFINITY.negated());

    pub(crate) fn raised(self, score: Score) -> Bound<Lower> {
        Bound::new(self.score.max(score))
    }

    pub(crate) fn admits_no_more_than(self, score: Score) -> bool {
        score <= self.score
    }
}

impl Bound<Upper> {
    pub(crate) const HIGHEST: Bound<Upper> = Bound::new(Score::INFINITY);

    pub(crate) fn excludes(self, score: Score) -> bool {
        score >= self.score
    }
}

impl Neg for Bound<Lower> {
    type Output = Bound<Upper>;

    fn neg(self) -> Bound<Upper> {
        Bound::new(-self.score)
    }
}

impl Neg for Bound<Upper> {
    type Output = Bound<Lower>;

    fn neg(self) -> Bound<Lower> {
        Bound::new(-self.score)
    }
}
