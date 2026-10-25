use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::series::tally) struct HalfPoints(usize);

impl HalfPoints {
    const PER_POINT: usize = 2;

    pub(super) const fn new(half_points: usize) -> HalfPoints {
        HalfPoints(half_points)
    }
}

impl fmt::Display for HalfPoints {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0 / Self::PER_POINT)?;
        if self.0 % Self::PER_POINT == 1 {
            formatter.write_str(".5")
        } else {
            Ok(())
        }
    }
}
