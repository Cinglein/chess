use std::fmt;
use std::str::FromStr;

use super::slice_count::SliceCount;
use crate::failure::Failure;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Slice {
    index: usize,
    count: SliceCount,
}

impl Slice {
    const CUT: char = '/';

    #[must_use]
    pub const fn at_index(index: usize, count: SliceCount) -> Slice {
        Slice { index, count }
    }

    #[must_use]
    pub fn picked<T>(&self, items: Vec<T>) -> Vec<T> {
        items
            .into_iter()
            .enumerate()
            .filter(|(position, _)| position % usize::from(self.count) == self.index)
            .map(|(_, item)| item)
            .collect()
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

#[cfg(test)]
mod tests {
    use super::{Slice, SliceCount};
    const SECOND_OF_THREE: Slice = Slice::at_index(1, SliceCount::new(3));
    const ITEMS: [char; 5] = ['a', 'b', 'c', 'd', 'e'];
    #[test]
    fn a_slice_takes_every_third_item_from_its_index_and_nothing_from_fewer_items_than_its_index() {
        let picked = SECOND_OF_THREE.picked(ITEMS.to_vec());
        assert_eq!(
            (picked, SECOND_OF_THREE.picked(ITEMS[..1].to_vec())),
            (vec!['b', 'e'], Vec::new())
        );
    }
}
