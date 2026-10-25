mod package;

use package::Package;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Metadata {
    packages: Vec<Package>,
}

impl Metadata {
    pub fn packages(&self) -> &[Package] {
        &self.packages
    }
}
