use syn::{Item, ItemUse, UseTree, Visibility};

use task::CrateName;
use task::Name;
use task::SourceFile;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Export {
    crate_name: CrateName,
    name: Name,
    line: usize,
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

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn line(&self) -> usize {
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
                name: Name::from(&name.ident),
                line: name.ident.span().start().line,
            }],
            UseTree::Rename(rename) => vec![Export {
                crate_name: crate_name.clone(),
                name: Name::from(&rename.rename),
                line: rename.rename.span().start().line,
            }],
            UseTree::Glob(_) => Vec::new(),
        }
    }
}
