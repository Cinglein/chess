use core::marker::PhantomData;
use core::ops::Neg;

use eval::Score;

use super::limit::Limit;
use super::lower::Lower;
use super::upper::Upper;

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

#[cfg(test)]
mod tests {
    use eval::Score;
    use proptest::prelude::*;

    use super::{Bound, Lower};

    #[test]
    fn a_raised_lower_bound_admits_up_to_the_higher_score_and_negates_into_the_matching_upper_bound()
     {
        proptest!(|(bound: i16, raise: i16, probe: i16)| {
            let (bound, raise, probe) = (Score::new(i32::from(bound)), Score::new(i32::from(raise)), Score::new(i32::from(probe)));
            let lower = Bound::<Lower>::new(bound);
            prop_assert_eq!(lower.raised(raise).admits_no_more_than(probe), probe <= bound.max(raise));
            prop_assert_eq!((-lower).excludes(probe), probe >= -bound);
            prop_assert_eq!(-(-lower), lower);
        });
    }
}
