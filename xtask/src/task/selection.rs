use std::collections::BTreeSet;
use std::fmt;

use crate::task::affected::Affected;
use crate::task::failure::Failure;
use crate::task::workspace::Workspace;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Selection {
    Workspace,
    Crates(BTreeSet<String>),
}

impl Selection {
    const AFFECTED_FLAG: &str = "--affected";

    pub fn from_flags(
        mut flags: impl Iterator<Item = String>,
        workspace: &Workspace,
    ) -> Result<Selection, Failure> {
        flags.try_fold(Selection::Workspace, |_, flag| {
            if flag == Self::AFFECTED_FLAG {
                Affected::reached(workspace).map(Selection::Crates)
            } else {
                Err(Failure::Usage(format!(
                    "unknown flag {flag}; the only flag is {}",
                    Self::AFFECTED_FLAG
                )))
            }
        })
    }

    pub fn package_arguments(&self) -> Vec<String> {
        match self {
            Selection::Workspace => vec![String::from("--workspace")],
            Selection::Crates(_) => self.explicit_packages(),
        }
    }

    pub fn explicit_packages(&self) -> Vec<String> {
        match self {
            Selection::Workspace => Vec::new(),
            Selection::Crates(crates) => crates
                .iter()
                .flat_map(|name| [String::from("--package"), name.clone()])
                .collect(),
        }
    }

    pub fn includes(&self, crate_name: &str) -> bool {
        match self {
            Selection::Workspace => true,
            Selection::Crates(crates) => crates.contains(crate_name),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Selection::Workspace => false,
            Selection::Crates(crates) => crates.is_empty(),
        }
    }
}

impl fmt::Display for Selection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Selection::Workspace => formatter.write_str("the whole workspace"),
            Selection::Crates(crates) if crates.is_empty() => formatter.write_str("no crate"),
            Selection::Crates(crates) => {
                write!(
                    formatter,
                    "{}",
                    crates.iter().cloned().collect::<Vec<String>>().join(", ")
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::Selection;

    const EVAL: &str = "eval";
    const SEARCH: &str = "search";
    const BOARD: &str = "board";
    const PACKAGES: [&str; 4] = ["--package", EVAL, "--package", SEARCH];
    const WORKSPACE: [&str; 1] = ["--workspace"];

    #[test]
    fn a_crate_selection_names_each_package_and_the_workspace_names_itself() {
        let crates = Selection::Crates(BTreeSet::from([SEARCH.to_owned(), EVAL.to_owned()]));
        assert_eq!(crates.package_arguments(), PACKAGES);
        assert_eq!(
            (
                Selection::Workspace.package_arguments(),
                crates.includes(EVAL),
                crates.includes(BOARD)
            ),
            (WORKSPACE.map(String::from).to_vec(), true, false)
        );
    }
}
