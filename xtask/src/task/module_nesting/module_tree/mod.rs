mod module_path;
mod use_edge;

use std::collections::{BTreeMap, BTreeSet};

use module_path::ModulePath;
use use_edge::UseEdges;

use crate::task::site::Site;
use crate::task::source_file::SourceFile;
use crate::task::violation::Violation;

pub struct ModuleTree {
    modules: Vec<ModulePath>,
    users: BTreeMap<ModulePath, BTreeSet<ModulePath>>,
    deep_uses: Vec<Violation>,
    reexports: BTreeMap<ModulePath, usize>,
}

impl ModuleTree {
    pub fn collect(files: &[SourceFile]) -> ModuleTree {
        let empty = ModuleTree {
            modules: files
                .iter()
                .filter_map(|file| ModulePath::of_file(file.path()))
                .collect(),
            users: BTreeMap::new(),
            deep_uses: Vec::new(),
            reexports: BTreeMap::new(),
        };
        files.iter().fold(empty, |mut tree, file| {
            tree.absorb(file);
            tree
        })
    }

    pub fn misplaced(&self) -> impl Iterator<Item = Violation> + '_ {
        self.modules.iter().filter_map(|module| {
            self.home_of(module).map(|home| {
                Violation::new(
                    Site::File(module.file().to_owned()),
                    format!("used only from {home}; move it inside that module"),
                )
            })
        })
    }

    pub fn deep_uses(&self) -> impl Iterator<Item = Violation> + '_ {
        self.deep_uses.iter().cloned()
    }

    pub fn crowded(&self, max: usize) -> impl Iterator<Item = Violation> + '_ {
        self.reexports
            .iter()
            .filter(|(module, _)| !module.segments().is_empty())
            .filter(move |(_, count)| **count > max)
            .map(move |(module, count)| {
                Violation::new(
                    Site::File(module.file().to_owned()),
                    format!("re-exports {count} items, at most {max} allowed; split the module"),
                )
            })
    }

    fn absorb(&mut self, file: &SourceFile) {
        let Some(module) = ModulePath::of_file(file.path()) else {
            return;
        };
        self.reexports
            .insert(module.clone(), UseEdges::count_reexports(file.syntax()));
        for path in UseEdges::in_file(file.syntax()) {
            self.absorb_use(&module, &path);
        }
    }

    fn absorb_use(&mut self, module: &ModulePath, path: &[String]) {
        let Some(target) = ModulePath::from_use(module, path, &self.modules) else {
            return;
        };
        if let Some(used) = ModulePath::deepest_module_containing(&self.modules, &target, module) {
            self.users
                .entry(used.clone())
                .or_default()
                .insert(module.clone());
        }
        if target.crosses_directory_boundary(module, &self.modules) {
            self.deep_uses.push(Violation::new(
                Site::File(module.file().to_owned()),
                format!("reaches into {target}; import the parent's re-export instead"),
            ));
        }
    }

    fn home_of(&self, module: &ModulePath) -> Option<ModulePath> {
        let outsiders: Vec<&ModulePath> = self
            .users
            .get(module)?
            .iter()
            .filter(|user| !module.is_ancestor_of(user))
            .collect();
        if outsiders
            .iter()
            .any(|user| !user.is_inside_a_sibling_of(module))
        {
            return None;
        }
        let homes: BTreeSet<ModulePath> = outsiders
            .iter()
            .map(|user| user.truncated(module.segments().len()))
            .collect();
        (homes.len() == 1)
            .then(|| homes.into_iter().next())
            .flatten()
    }
}
