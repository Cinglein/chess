use strum::{EnumString, VariantNames};

use crate::magic_tables::MagicTables;
use crate::no_comments::NoComments;
use crate::no_free_fns::NoFreeFns;
use crate::test_budget::TestBudget;
use crate::workspace::Workspace;
use crate::xor_shift::XorShift;

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumString, VariantNames)]
#[strum(serialize_all = "kebab-case")]
pub enum Task {
    Ci,
    Lint,
    Magics,
    NoComments,
    NoFreeFns,
    TestBudget,
    Wasm,
}

impl Task {
    const WASM_CRATES: &[&str] = &["board", "fen"];

    pub fn usage() -> String {
        format!("usage: cargo xtask <{}>", Self::VARIANTS.join("|"))
    }

    pub fn run(self) -> Result<(), String> {
        let workspace = Workspace::locate();
        match self {
            Task::Ci => Self::ci(&workspace),
            Task::Lint => Self::lint(&workspace),
            Task::Magics => Self::magics(&workspace),
            Task::NoComments => NoComments::check(&workspace.source_files()?),
            Task::NoFreeFns => NoFreeFns::check(&workspace.source_files()?),
            Task::TestBudget => TestBudget::check(&workspace.source_files()?),
            Task::Wasm => Self::wasm(&workspace),
        }
    }

    fn ci(workspace: &Workspace) -> Result<(), String> {
        workspace.cargo(&["fmt", "--all", "--check"])?;
        workspace.cargo(&[
            "clippy",
            "--workspace",
            "--all-targets",
            "--all-features",
            "--",
            "-D",
            "warnings",
        ])?;
        Self::wasm(workspace)?;
        workspace.cargo(&["test", "--workspace", "--all-features"])?;
        Self::lint(workspace)
    }

    fn lint(workspace: &Workspace) -> Result<(), String> {
        let files = workspace.source_files()?;
        let failures: Vec<String> = [
            NoComments::check(&files),
            NoFreeFns::check(&files),
            TestBudget::check(&files),
        ]
        .into_iter()
        .filter_map(Result::err)
        .collect();
        failures
            .is_empty()
            .then_some(())
            .ok_or_else(|| failures.join("\n\n"))
    }

    fn magics(workspace: &Workspace) -> Result<(), String> {
        MagicTables::find(&mut XorShift::new(MagicTables::SEED)).write(workspace.root())?;
        workspace.cargo(&["fmt", "--package", "board"])
    }

    fn wasm(workspace: &Workspace) -> Result<(), String> {
        Self::WASM_CRATES.iter().try_for_each(|crate_name| {
            workspace.cargo(&[
                "clippy",
                "--package",
                crate_name,
                "--target",
                "wasm32-unknown-unknown",
                "--",
                "-D",
                "warnings",
            ])
        })
    }
}
