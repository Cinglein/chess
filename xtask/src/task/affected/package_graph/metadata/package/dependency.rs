use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Dependency {
    name: String,
    path: Option<String>,
}

impl Dependency {
    pub fn local_name(&self) -> Option<&str> {
        self.path.as_ref().map(|_| self.name.as_str())
    }
}
