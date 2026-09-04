use std::fs;
use std::path::Path;

use crate::workspace::Workspace;

pub struct SourceFile {
    pub path: String,
    pub text: String,
    pub syntax: syn::File,
}

impl SourceFile {
    pub fn read(workspace: &Workspace, file: &Path) -> Result<Self, String> {
        let text =
            fs::read_to_string(file).map_err(|error| format!("{}: {error}", file.display()))?;
        let syntax =
            syn::parse_file(&text).map_err(|error| format!("{}: {error}", file.display()))?;
        Ok(Self {
            path: workspace.relative(file),
            text,
            syntax,
        })
    }
}
