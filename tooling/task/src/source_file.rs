use std::fs;
use std::path::{Component, Path, PathBuf};

use syn::ItemMod;

use crate::failure::Failure;
use crate::site::Site;
use crate::workspace::Workspace;

pub struct SourceFile {
    path: PathBuf,
    text: String,
    syntax: syn::File,
}

impl SourceFile {
    pub fn read(workspace: &Workspace, file: &Path) -> Result<Self, Failure> {
        let path = workspace.relative(file);
        let text = fs::read_to_string(file).map_err(|error| Failure::Io {
            site: Site::File(path.clone()),
            error,
        })?;
        Self::parse(path, text)
    }

    pub fn parse(path: impl Into<PathBuf>, text: String) -> Result<Self, Failure> {
        let path = path.into();
        let syntax = syn::parse_file(&text).map_err(|error| Failure::Parse {
            site: Site::File(path.clone()),
            error,
        })?;
        Ok(Self { path, text, syntax })
    }

    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    #[must_use]
    pub fn crate_name(&self) -> Option<&str> {
        let mut components = self.path.components();
        match components.next() {
            Some(Component::Normal(root)) if root == "crates" || root == "tooling" => components
                .map(Component::as_os_str)
                .find(|name| *name != "lints")
                .and_then(|name| name.to_str()),
            Some(Component::Normal(root)) if root == "xtask" => Some("xtask"),
            _ => None,
        }
    }

    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    #[must_use]
    pub fn syntax(&self) -> &syn::File {
        &self.syntax
    }

    #[must_use]
    pub fn is_test_module(module: &ItemMod) -> bool {
        module.attrs.iter().any(|attribute| {
            attribute.path().is_ident("cfg")
                && attribute
                    .parse_args::<syn::Ident>()
                    .is_ok_and(|ident| ident == "test")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::SourceFile;

    const NAMED: [(&str, Option<&str>); 5] = [
        ("crates/board/src/square.rs", Some("board")),
        ("tooling/lints/fn_shape/src/lib.rs", Some("fn_shape")),
        ("tooling/task/src/lib.rs", Some("task")),
        ("xtask/src/main.rs", Some("xtask")),
        ("other/board/src/lib.rs", None),
    ];
    const MODULES: &str = "#[cfg(test)] mod tests {} #[cfg(feature = \"proptest\")] mod strategies {} #[allow(test)] mod odd {}";
    const TEST_MODULES: usize = 1;

    #[test]
    fn a_file_names_its_crate_from_its_path_and_only_a_cfg_test_module_is_a_test_module() {
        let names = NAMED.map(|(path, _)| {
            SourceFile::parse(path.to_owned(), String::new())
                .expect("empty is valid")
                .crate_name()
                .map(str::to_owned)
        });
        assert_eq!(names, NAMED.map(|(_, name)| name.map(str::to_owned)));
        let parsed = syn::parse_file(MODULES).expect("valid rust");
        let test_modules = parsed
            .items
            .iter()
            .filter(
                |item| matches!(item, syn::Item::Mod(module) if SourceFile::is_test_module(module)),
            )
            .count();
        assert_eq!(test_modules, TEST_MODULES);
    }
}
