use syn::{Item, UseTree, Visibility};

use crate::task::source_file::SourceFile;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Export {
    crate_name: String,
    name: String,
    line: usize,
}

impl Export {
    pub fn in_root(file: &SourceFile) -> Vec<Export> {
        let Some(crate_name) = file
            .crate_name()
            .filter(|_| file.path().ends_with("/src/lib.rs"))
        else {
            return Vec::new();
        };
        file.syntax()
            .items
            .iter()
            .filter_map(|item| match item {
                Item::Use(used) if !matches!(used.vis, Visibility::Inherited) => Some(used),
                _ => None,
            })
            .flat_map(|used| Self::leaves(crate_name, &used.tree))
            .collect()
    }

    pub fn crate_name(&self) -> &str {
        &self.crate_name
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn line(&self) -> usize {
        self.line
    }

    fn leaves(crate_name: &str, tree: &UseTree) -> Vec<Export> {
        match tree {
            UseTree::Path(path) => Self::leaves(crate_name, &path.tree),
            UseTree::Group(group) => group
                .items
                .iter()
                .flat_map(|item| Self::leaves(crate_name, item))
                .collect(),
            UseTree::Name(name) => vec![Export {
                crate_name: crate_name.to_owned(),
                name: name.ident.to_string(),
                line: name.ident.span().start().line,
            }],
            UseTree::Rename(rename) => vec![Export {
                crate_name: crate_name.to_owned(),
                name: rename.rename.to_string(),
                line: rename.rename.span().start().line,
            }],
            UseTree::Glob(_) => Vec::new(),
        }
    }
}
