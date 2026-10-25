use std::path::{Path, PathBuf};

use syn::ItemMod;
use syn::visit::Visit;

use crate::task::source_file::SourceFile;

#[derive(Default)]
pub struct SupportModules {
    names: Vec<String>,
}

impl SupportModules {
    pub fn paths(file: &SourceFile) -> Vec<PathBuf> {
        let mut modules = SupportModules::default();
        modules.visit_file(file.syntax());
        let directory = Self::directory_of(file.path());
        modules
            .names
            .iter()
            .flat_map(|name| {
                [
                    directory.join(format!("{name}.rs")),
                    directory.join(name).join("mod.rs"),
                ]
            })
            .collect()
    }

    fn directory_of(path: &Path) -> PathBuf {
        match path.file_name().and_then(|name| name.to_str()) {
            Some("mod.rs" | "lib.rs" | "main.rs") => {
                path.parent().map(Path::to_path_buf).unwrap_or_default()
            }
            _ => path.with_extension(""),
        }
    }
}

impl<'ast> Visit<'ast> for SupportModules {
    fn visit_item_mod(&mut self, module: &'ast ItemMod) {
        if module.content.is_none() && SourceFile::is_test_module(module) {
            self.names.push(module.ident.to_string());
        }
        syn::visit::visit_item_mod(self, module);
    }
}
