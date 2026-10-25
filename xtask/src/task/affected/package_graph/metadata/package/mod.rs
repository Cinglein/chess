mod dependency;

use dependency::Dependency;
use serde::Deserialize;

use task::CrateName;

#[derive(Debug, Deserialize)]
pub struct Package {
    name: CrateName,
    dependencies: Vec<Dependency>,
}

impl Package {
    pub fn name(&self) -> &CrateName {
        &self.name
    }

    pub fn local_dependencies(&self) -> impl Iterator<Item = &CrateName> {
        self.dependencies.iter().filter_map(Dependency::local_name)
    }
}
