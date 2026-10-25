use crate::task::failure::Failure;
use crate::task::selection::Selection;
use crate::task::workspace::Workspace;

pub struct Test;

impl Test {
    pub fn run(workspace: &Workspace, selection: &Selection) -> Result<(), Failure> {
        if selection.is_empty() {
            println!("test: no crate affected");
            return Ok(());
        }
        let arguments: Vec<String> = [String::from("test")]
            .into_iter()
            .chain(selection.package_arguments())
            .chain([String::from("--all-features")])
            .collect();
        let borrowed: Vec<&str> = arguments.iter().map(String::as_str).collect();
        workspace.cargo(&borrowed)
    }
}
