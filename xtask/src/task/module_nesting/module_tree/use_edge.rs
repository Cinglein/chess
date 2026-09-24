use syn::visit::Visit;
use syn::{Item, ItemUse, UseTree, Visibility};

#[derive(Default)]
pub struct UseEdges(Vec<Vec<String>>);

impl UseEdges {
    pub fn in_file(file: &syn::File) -> Vec<Vec<String>> {
        let mut edges = UseEdges::default();
        edges.visit_file(file);
        edges.0
    }

    pub fn count_reexports(file: &syn::File) -> usize {
        file.items
            .iter()
            .filter_map(|item| match item {
                Item::Use(used) if !matches!(used.vis, Visibility::Inherited) => {
                    Some(Self::count_leaves(&used.tree))
                }
                _ => None,
            })
            .sum()
    }

    fn count_leaves(tree: &UseTree) -> usize {
        match tree {
            UseTree::Path(path) => Self::count_leaves(&path.tree),
            UseTree::Group(group) => group.items.iter().map(Self::count_leaves).sum(),
            UseTree::Name(_) | UseTree::Rename(_) | UseTree::Glob(_) => 1,
        }
    }

    fn walk(&mut self, prefix: &[String], tree: &UseTree) {
        match tree {
            UseTree::Path(path) => {
                let longer: Vec<String> = prefix
                    .iter()
                    .cloned()
                    .chain(Some(path.ident.to_string()))
                    .collect();
                self.walk(&longer, &path.tree);
            }
            UseTree::Group(group) => group.items.iter().for_each(|item| self.walk(prefix, item)),
            UseTree::Name(_) | UseTree::Rename(_) | UseTree::Glob(_) => {
                self.0.push(prefix.to_vec());
            }
        }
    }
}

impl<'ast> Visit<'ast> for UseEdges {
    fn visit_item_use(&mut self, item: &'ast ItemUse) {
        self.walk(&[], &item.tree);
    }
}
