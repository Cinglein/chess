mod dependency;

use dependency::Dependency;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Package {
    name: String,
    dependencies: Vec<Dependency>,
}

impl Package {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn local_dependencies(&self) -> impl Iterator<Item = &str> {
        self.dependencies.iter().filter_map(Dependency::local_name)
    }
}
