use syn::{Item, ItemUse, UseTree, Visibility};

use task::CrateName;
use task::LineNumber;
use task::SourceFile;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Export {
    crate_name: CrateName,
    name: String,
    line: LineNumber,
}

impl Export {
    pub fn in_root(file: &SourceFile) -> Vec<Export> {
        file.crate_name()
            .filter(|_| file.path().ends_with("src/lib.rs"))
            .and_then(|name| name.parse::<CrateName>().ok())
            .map_or_else(Vec::new, |crate_name| {
                Self::reexports(file)
                    .flat_map(|used| Self::leaves(&crate_name, &used.tree))
                    .collect()
            })
    }

    fn reexports(file: &SourceFile) -> impl Iterator<Item = &ItemUse> {
        file.syntax().items.iter().filter_map(|item| match item {
            Item::Use(used) if !matches!(used.vis, Visibility::Inherited) => Some(used),
            _ => None,
        })
    }

    pub fn crate_name(&self) -> &CrateName {
        &self.crate_name
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn line(&self) -> LineNumber {
        self.line
    }

    fn leaves(crate_name: &CrateName, tree: &UseTree) -> Vec<Export> {
        match tree {
            UseTree::Path(path) => Self::leaves(crate_name, &path.tree),
            UseTree::Group(group) => group
                .items
                .iter()
                .flat_map(|item| Self::leaves(crate_name, item))
                .collect(),
            UseTree::Name(name) => vec![Export {
                crate_name: crate_name.clone(),
                name: name.ident.to_string(),
                line: LineNumber::from(name.ident.span()),
            }],
            UseTree::Rename(rename) => vec![Export {
                crate_name: crate_name.clone(),
                name: rename.rename.to_string(),
                line: LineNumber::from(rename.rename.span()),
            }],
            UseTree::Glob(_) => Vec::new(),
        }
    }
}
