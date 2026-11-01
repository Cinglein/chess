use std::fmt;

use derive_more::From;

#[derive(Clone, Copy, Debug, PartialEq, Eq, From)]
pub(in crate::tally) struct HalfPoints(usize);

impl HalfPoints {
    const PER_POINT: usize = 2;
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
