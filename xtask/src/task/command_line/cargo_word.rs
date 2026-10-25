use strum::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display)]
#[repr(u8)]
pub enum CargoWord {
    #[strum(serialize = "fmt")]
    Fmt,
    #[strum(serialize = "clippy")]
    Clippy,
    #[strum(serialize = "test")]
    Test,
    #[strum(serialize = "metadata")]
    Metadata,
    #[strum(serialize = "mutants")]
    Mutants,
    #[strum(serialize = "--all")]
    All,
    #[strum(serialize = "--check")]
    Check,
    #[strum(serialize = "--workspace")]
    Workspace,
    #[strum(serialize = "--package")]
    Package,
    #[strum(serialize = "--all-targets")]
    AllTargets,
    #[strum(serialize = "--all-features")]
    AllFeatures,
    #[strum(serialize = "--target")]
    Target,
    #[strum(serialize = "wasm32-unknown-unknown")]
    Wasm32,
    #[strum(serialize = "--no-run")]
    NoRun,
    #[strum(serialize = "--color")]
    Color,
    #[strum(serialize = "never")]
    Never,
    #[strum(serialize = "--lib")]
    Lib,
    #[strum(serialize = "--format-version=1")]
    FormatVersionOne,
    #[strum(serialize = "--no-deps")]
    NoDeps,
    #[strum(serialize = "--")]
    ToolArguments,
    #[strum(serialize = "-D")]
    Deny,
    #[strum(serialize = "warnings")]
    Warnings,
}
