use std::collections::BTreeMap;
use std::iter;

use syn::visit::Visit;
use syn::{ItemUse, Path, UseTree};

#[derive(Default)]
pub struct UseAliases(BTreeMap<String, String>);

impl UseAliases {
    pub fn in_file(file: &syn::File) -> UseAliases {
        let mut aliases = UseAliases::default();
        aliases.visit_file(file);
        aliases
    }

    pub fn resolve(&self, path: &Path) -> String {
        let segments: Vec<String> = path
            .segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect();
        match segments.split_first() {
            Some((first, rest)) => iter::once(self.0.get(first).unwrap_or(first).clone())
                .chain(rest.iter().cloned())
                .collect::<Vec<String>>()
                .join("::"),
            None => String::new(),
        }
    }

    fn walk(&mut self, prefix: &[String], tree: &UseTree) {
        let joined = |name: &str| {
            prefix
                .iter()
                .map(String::as_str)
                .chain(iter::once(name))
                .collect::<Vec<&str>>()
                .join("::")
        };
        match tree {
            UseTree::Path(path) => {
                let longer: Vec<String> = prefix
                    .iter()
                    .cloned()
                    .chain(iter::once(path.ident.to_string()))
                    .collect();
                self.walk(&longer, &path.tree);
            }
            UseTree::Group(group) => group.items.iter().for_each(|item| self.walk(prefix, item)),
            UseTree::Name(name) => {
                self.0
                    .insert(name.ident.to_string(), joined(&name.ident.to_string()));
            }
            UseTree::Rename(rename) => {
                self.0
                    .insert(rename.rename.to_string(), joined(&rename.ident.to_string()));
            }
            UseTree::Glob(_) => {}
        }
    }
}

impl<'ast> Visit<'ast> for UseAliases {
    fn visit_item_use(&mut self, item: &'ast ItemUse) {
        self.walk(&[], &item.tree);
    }
}
