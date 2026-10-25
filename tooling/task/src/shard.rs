use std::collections::BTreeSet;
use std::fmt;
use std::str::FromStr;

use crate::command_line::{CargoArgument, MutantsArgument};
use crate::crate_name::CrateName;
use crate::failure::Failure;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Shard {
    Whole(BTreeSet<CrateName>),
    Slice {
        crate_name: CrateName,
        index: usize,
        count: usize,
    },
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
    pub fn arguments(&self) -> Vec<String> {
        let packages = self
            .crates()
            .into_iter()
            .flat_map(|name| [CargoArgument::Package.to_string(), name.to_string()]);
        match self {
            Shard::Whole(_) => packages.collect(),
            Shard::Slice { index, count, .. } => packages
                .chain([
                    MutantsArgument::Shard.to_string(),
                    format!("{index}{}{count}", Self::CUT),
                ])
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
            Shard::Slice {
                crate_name,
                index,
                count,
            } => write!(
                formatter,
                "{crate_name}{cut}{index}{cut}{count}",
                cut = Self::CUT
            ),
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
        match text.split(Self::CUT).collect::<Vec<&str>>().as_slice() {
            [crate_name, index, count] => {
                let Ok(crate_name) = crate_name.parse::<CrateName>();
                Ok(Shard::Slice {
                    crate_name,
                    index: index.parse().map_err(|_| malformed())?,
                    count: count.parse().map_err(|_| malformed())?,
                })
            }
            [whole] => Ok(Shard::Whole(
                whole
                    .split(Self::JOIN)
                    .filter(|name| !name.is_empty())
                    .filter_map(|name| name.parse().ok())
                    .collect(),
            )),
            _ => Err(malformed()),
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
        assert_eq!(
            (slice.to_string(), whole.to_string()),
            (SLICE.to_owned(), WHOLE.to_owned())
        );
        assert_eq!(
            (slice.arguments(), whole.arguments()),
            (
                SLICE_ARGUMENTS.map(String::from).to_vec(),
                WHOLE_ARGUMENTS.map(String::from).to_vec()
            )
        );
        assert!("a/b".parse::<Shard>().is_err());
    }
}
