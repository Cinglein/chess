use crate::task::failure::Failure;
use crate::task::selection::Selection;
use crate::task::workspace::Workspace;

pub struct Clippy;

impl Clippy {
    pub fn run(workspace: &Workspace, selection: &Selection) -> Result<(), Failure> {
        if selection.is_empty() {
            println!("clippy: no crate affected");
            return Ok(());
        }
        let arguments: Vec<String> = [String::from("clippy")]
            .into_iter()
            .chain(selection.package_arguments())
            .chain(
                ["--all-targets", "--all-features", "--", "-D", "warnings"]
                    .into_iter()
                    .map(String::from),
            )
            .collect();
        let borrowed: Vec<&str> = arguments.iter().map(String::as_str).collect();
        workspace.cargo(&borrowed)
    }
}
