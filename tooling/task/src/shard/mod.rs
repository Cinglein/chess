mod slice;

pub use slice::Slice;

use std::collections::BTreeSet;
use std::fmt;
use std::str::FromStr;

use crate::command_line::{CargoArgument, MutantsArgument};
use crate::crate_name::CrateName;
use crate::failure::Failure;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Shard {
    Whole(BTreeSet<CrateName>),
    Slice { crate_name: CrateName, slice: Slice },
}

impl Shard {
    const JOIN: char = '+';
    const CUT: char = '/';

    #[must_use]
    pub fn crates(&self) -> BTreeSet<CrateName> {
        match self {
            Shard::Whole(crates) => crates.clone(),
            Shard::Slice { crate_name, .. } => BTreeSet::from([crate_name.clone()]),
        }
    }

    #[must_use]
    pub fn runs(&self) -> Vec<Shard> {
        match self {
            Shard::Whole(crates) => crates
                .iter()
                .map(|name| Shard::Whole(BTreeSet::from([name.clone()])))
                .collect(),
            Shard::Slice { .. } => vec![self.clone()],
        }
    }

    #[must_use]
    pub fn arguments(&self) -> Vec<String> {
        let packages = self
            .crates()
            .into_iter()
            .flat_map(|name| [CargoArgument::Package.to_string(), name.to_string()]);
        match self {
            Shard::Whole(_) => packages.collect(),
            Shard::Slice { slice, .. } => packages
                .chain([MutantsArgument::Shard.to_string(), slice.to_string()])
                .collect(),
        }
    }
}

impl fmt::Display for Shard {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Shard::Whole(crates) => {
                let names: Vec<String> = crates.iter().map(ToString::to_string).collect();
                formatter.write_str(&names.join(&Self::JOIN.to_string()))
            }
            Shard::Slice { crate_name, slice } => {
                write!(formatter, "{crate_name}{}{slice}", Self::CUT)
            }
        }
    }
}

impl FromStr for Shard {
    type Err = Failure;

    fn from_str(text: &str) -> Result<Shard, Failure> {
        let malformed = || {
            Failure::Usage(format!(
                "{text} is not a shard; expected a+b+c or crate/index/count"
            ))
        };
        match text.split_once(Self::CUT) {
            Some((crate_name, slice)) => {
                let Ok(crate_name) = crate_name.parse::<CrateName>();
                Ok(Shard::Slice {
                    crate_name,
                    slice: slice.parse().map_err(|_| malformed())?,
                })
            }
            None => Ok(Shard::Whole(
                text.split(Self::JOIN)
                    .filter(|name| !name.is_empty())
                    .filter_map(|name| name.parse().ok())
                    .collect(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Shard;
    const SLICE: &str = "board/0/4";
    const WHOLE: &str = "eval+uci";
    const SLICE_ARGUMENTS: [&str; 4] = ["--package", "board", "--shard", "0/4"];
    const WHOLE_ARGUMENTS: [&str; 4] = ["--package", "eval", "--package", "uci"];

    #[test]
    fn a_shard_prints_back_to_its_spelling_and_names_its_packages_and_slice() {
        let slice: Shard = SLICE.parse().unwrap();
        let whole: Shard = WHOLE.parse().unwrap();
        assert!(
            slice.to_string() == SLICE
                && whole.to_string() == WHOLE
                && "a/b".parse::<Shard>().is_err()
        );
        assert!(slice.arguments() == SLICE_ARGUMENTS && whole.arguments() == WHOLE_ARGUMENTS);
        assert_eq!((whole.runs().len(), slice.runs().len()), (2, 1));
    }
}
