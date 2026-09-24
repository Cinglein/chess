use std::fs;
use std::path::Path;

use syn::ItemMod;

use crate::task::failure::Failure;
use crate::task::site::Site;
use crate::task::workspace::Workspace;

pub struct SourceFile {
    path: String,
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

    pub fn parse(path: String, text: String) -> Result<Self, Failure> {
        let syntax = syn::parse_file(&text).map_err(|error| Failure::Parse {
            site: Site::File(path.clone()),
            error,
        })?;
        Ok(Self { path, text, syntax })
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn syntax(&self) -> &syn::File {
        &self.syntax
    }

    pub fn is_test_module(module: &ItemMod) -> bool {
        module.attrs.iter().any(|attribute| {
            attribute.path().is_ident("cfg")
                && attribute
                    .parse_args::<syn::Ident>()
                    .is_ok_and(|ident| ident == "test")
        })
    }
}
