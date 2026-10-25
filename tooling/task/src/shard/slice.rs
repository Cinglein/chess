use std::fmt;
use std::str::FromStr;

use crate::failure::Failure;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Slice {
    index: usize,
    count: usize,
}

impl Slice {
    const CUT: char = '/';

    #[must_use]
    pub const fn new(index: usize, count: usize) -> Slice {
        Slice { index, count }
    }

    #[must_use]
    pub fn picked<T>(&self, items: Vec<T>) -> Option<Vec<T>> {
        (items.len() >= self.count).then(|| {
            items
                .into_iter()
                .enumerate()
                .filter(|(position, _)| position % self.count == self.index)
                .map(|(_, item)| item)
                .collect()
        })
    }
}

impl fmt::Display for Slice {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}{}{}", self.index, Self::CUT, self.count)
    }
}

impl FromStr for Slice {
    type Err = Failure;

    fn from_str(text: &str) -> Result<Slice, Failure> {
        let malformed = || Failure::Usage(format!("{text} is not a slice; expected index/count"));
        let (index, count) = text.split_once(Self::CUT).ok_or_else(malformed)?;
        Ok(Slice {
            index: index.parse().map_err(|_| malformed())?,
            count: count.parse().map_err(|_| malformed())?,
        })
    }
}
