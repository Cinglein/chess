mod cargo_tool;
mod mutation_tool;
mod tool;

pub use cargo_tool::CargoTool;
pub use mutation_tool::MutationTool;
pub use tool::Tool;

use std::collections::BTreeSet;
use std::fmt;

use crate::task::affected::Affected;
use crate::task::command_line::CargoArgument;
use crate::task::crate_name::CrateName;
use crate::task::failure::Failure;
use crate::task::workspace::Workspace;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Selection {
    Workspace,
    Crates(BTreeSet<CrateName>),
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

    pub fn arguments<T: Tool>(&self) -> Vec<String> {
        match self {
            Selection::Workspace => T::whole_workspace()
                .map(|argument| argument.to_string())
                .into_iter()
                .collect(),
            Selection::Crates(crates) => crates
                .iter()
                .flat_map(|name| [CargoArgument::Package.to_string(), name.to_string()])
                .collect(),
        }
    }

    pub fn includes(&self, crate_name: &CrateName) -> bool {
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
                    crates
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::{CargoTool, Selection};

    const EVAL: &str = "eval";
    const SEARCH: &str = "search";
    const BOARD: &str = "board";
    const PACKAGES: [&str; 4] = ["--package", EVAL, "--package", SEARCH];
    const WORKSPACE: [&str; 1] = ["--workspace"];

    #[test]
    fn a_crate_selection_names_each_package_and_the_workspace_names_itself() {
        let crates = Selection::Crates(BTreeSet::from([
            SEARCH.parse().unwrap(),
            EVAL.parse().unwrap(),
        ]));
        assert_eq!(crates.arguments::<CargoTool>(), PACKAGES);
        assert_eq!(
            (
                Selection::Workspace.arguments::<CargoTool>(),
                crates.includes(&EVAL.parse().unwrap()),
                crates.includes(&BOARD.parse().unwrap())
            ),
            (WORKSPACE.map(String::from).to_vec(), true, false)
        );
    }
}
