mod module_path;

use std::collections::{BTreeMap, BTreeSet};

use module_path::{Ancestor, InsideSibling, ModulePath};

use task::Site;
use task::SourceFile;
use task::UseEdges;
use task::Violation;

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
        files.iter().fold(empty, ModuleTree::absorbed)
    }

    pub fn violations(&self) -> impl Iterator<Item = Violation> + '_ {
        let deep_uses = self.deep_uses.iter().cloned();
        self.modules
            .iter()
            .filter_map(|module| {
                self.home_of(module).map(|home| {
                    Violation::new(
                        Site::File(module.file().into()),
                        format!("used only from {home}; move it inside that module"),
                    )
                })
            })
            .chain(deep_uses)
    }

    pub fn crowded(&self, max: usize) -> impl Iterator<Item = Violation> + '_ {
        self.reexports
            .iter()
            .filter(|(module, _)| !module.segments().is_empty())
            .filter(move |(_, count)| **count > max)
            .map(move |(module, count)| {
                Violation::new(
                    Site::File(module.file().into()),
                    format!("re-exports {count} items, at most {max} allowed; split the module"),
                )
            })
    }

    fn absorbed(mut self, file: &SourceFile) -> ModuleTree {
        match ModulePath::of_file(file.path()) {
            Some(module) => {
                self.reexports
                    .insert(module.clone(), UseEdges::count_reexports(file.syntax()));
                UseEdges::in_file(file.syntax())
                    .iter()
                    .fold(self, |tree, path| tree.absorbing(&module, path))
            }
            None => self,
        }
    }

    fn absorbing(mut self, module: &ModulePath, path: &[String]) -> ModuleTree {
        match ModulePath::from_use(module, path, &self.modules) {
            Some(target) => {
                let used = ModulePath::deepest_module_containing(&self.modules, &target, module);
                self.record_user(used.cloned(), module);
                self.deep_uses.extend(
                    target
                        .crosses_directory_boundary(module, &self.modules)
                        .then(|| {
                            Violation::new(
                                Site::File(module.file().into()),
                                format!(
                                    "reaches into {target}; import the parent's re-export instead"
                                ),
                            )
                        }),
                );
                self
            }
            None => self,
        }
    }

    fn record_user(&mut self, used: Option<ModulePath>, module: &ModulePath) -> bool {
        match used {
            Some(used) => self.users.entry(used).or_default().insert(module.clone()),
            None => false,
        }
    }

    fn home_of(&self, module: &ModulePath) -> Option<ModulePath> {
        let outsiders: Vec<&ModulePath> = self
            .users
            .get(module)?
            .iter()
            .filter(|user| !module.relates::<Ancestor>(user))
            .collect();
        outsiders
            .iter()
            .all(|user| user.relates::<InsideSibling>(module))
            .then(|| {
                let homes: BTreeSet<ModulePath> = outsiders
                    .iter()
                    .map(|user| user.truncated(module.segments().len()))
                    .collect();
                (homes.len() == 1)
                    .then(|| homes.into_iter().next())
                    .flatten()
            })
            .flatten()
    }
}
