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
