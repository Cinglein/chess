use std::collections::BTreeMap;

use serde::Deserialize;

use task::CrateName;
use task::Failure;

#[derive(Debug, Deserialize)]
pub struct ListedMutant {
    package: CrateName,
}

impl ListedMutant {
    pub fn tally(json: &str) -> Result<BTreeMap<CrateName, usize>, Failure> {
        let listed: Vec<ListedMutant> = serde_json::from_str(json).map_err(Failure::MutantList)?;
        Ok(listed
            .into_iter()
            .fold(BTreeMap::new(), |mut counts, mutant| {
                *counts.entry(mutant.package).or_default() += 1;
                counts
            }))
    }
}

#[cfg(test)]
mod tests {
    use super::ListedMutant;
    const LISTED: &str = r#"[{"package": "board", "file": "crates/board/src/lib.rs"},
        {"package": "eval"}, {"package": "board"}]"#;
    const BOARD: usize = 2;

    #[test]
    fn a_mutant_list_is_tallied_per_package_whatever_else_each_entry_carries() {
        let counts = ListedMutant::tally(LISTED).unwrap();
        assert_eq!(
            (counts[&"board".parse().unwrap()], counts.len()),
            (BOARD, BOARD)
        );
        assert!(ListedMutant::tally("{").is_err());
    }
}
