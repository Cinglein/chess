mod outcomes;
mod tally;

use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::num::NonZeroU64;
use std::path::{Path, PathBuf};

use outcomes::Outcomes;
use tally::Tally;

use task::CrateName;
use task::Failure;
use task::Site;

pub struct MutantCosts {
    tallies: BTreeMap<CrateName, Tally>,
}

impl MutantCosts {
    const OUTCOMES: &str = "outcomes.json";
    const STEP: NonZeroU64 = NonZeroU64::new(100).unwrap();

    pub fn measure(directory: &Path) -> Result<MutantCosts, Failure> {
        let io = |error| Failure::Io {
            site: Site::File(directory.to_path_buf()),
            error,
        };
        Self::outcome_files(directory)?
            .iter()
            .map(|file| fs::read_to_string(file).map_err(io))
            .map(|json| Outcomes::parse(&json?))
            .collect::<Result<Vec<Outcomes>, Failure>>()
            .map(Self::tally)
    }

    #[must_use]
    pub fn tally(all: impl IntoIterator<Item = Outcomes>) -> MutantCosts {
        MutantCosts {
            tallies: all.into_iter().fold(BTreeMap::new(), |tallies, outcomes| {
                outcomes.tallied_into(tallies)
            }),
        }
    }

    fn outcome_files(path: &Path) -> Result<Vec<PathBuf>, Failure> {
        let io = |error| Failure::Io {
            site: Site::File(path.to_path_buf()),
            error,
        };
        if path.is_dir() {
            fs::read_dir(path)
                .map_err(io)?
                .map(|entry| Self::outcome_files(&entry.map_err(io)?.path()))
                .collect::<Result<Vec<Vec<PathBuf>>, Failure>>()
                .map(|nested| nested.into_iter().flatten().collect())
        } else {
            Ok(path
                .file_name()
                .is_some_and(|name| name == Self::OUTCOMES)
                .then(|| path.to_path_buf())
                .into_iter()
                .collect())
        }
    }
}

impl fmt::Display for MutantCosts {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let overall = self
            .tallies
            .values()
            .fold(Tally::default(), |all, tally| all.plus(*tally));
        writeln!(formatter, "[mutant_millis]")?;
        writeln!(
            formatter,
            "default = {}",
            overall.mean().rounded_to(Self::STEP)
        )?;
        writeln!(formatter)?;
        writeln!(formatter, "[mutant_millis.crates]")?;
        self.tallies.iter().try_for_each(|(name, tally)| {
            writeln!(
                formatter,
                "{name} = {}",
                tally.mean().rounded_to(Self::STEP)
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{MutantCosts, Outcomes};

    const OUTCOMES: &str = r#"{"outcomes": [{"scenario": "Baseline", "phase_results": [{"phase": "Build", "duration": 9.0}]}, {"scenario": {"Mutant": {"package": "board", "file": "x"}}, "phase_results": [{"phase": "Build", "duration": 1.5}, {"phase": "Test", "duration": 0.54}]}, {"scenario": {"Mutant": {"package": "eval"}}, "phase_results": [{"phase": "Build", "duration": 1.0}]}]}"#;
    const TABLE: &str =
        "[mutant_millis]\ndefault = 1500\n\n[mutant_millis.crates]\nboard = 2000\neval = 1000\n";

    #[test]
    fn mutant_phases_are_averaged_per_crate_and_overall_then_rounded_to_a_hundred_millis() {
        let costs = MutantCosts::tally([Outcomes::parse(OUTCOMES).unwrap()]);
        assert_eq!(costs.to_string(), TABLE);
    }
}
