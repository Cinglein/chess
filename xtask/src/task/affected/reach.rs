use std::path::Path;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Reach {
    Everything,
    Nothing,
    Package(String),
}

impl Reach {
    const CRATES: &str = "crates";
    const INERT_DIRECTORIES: [&str; 1] = [".claude"];
    const INERT_EXTENSIONS: [&str; 1] = ["md"];

    pub fn reached_by(path: &str) -> Reach {
        let path = Path::new(path);
        match path
            .strip_prefix(Self::CRATES)
            .ok()
            .and_then(Self::first_component)
        {
            Some(name) => Reach::Package(name),
            None if Self::is_inert(path) => Reach::Nothing,
            None => Reach::Everything,
        }
    }

    pub fn package(&self) -> Option<String> {
        match self {
            Reach::Package(name) => Some(name.clone()),
            Reach::Everything | Reach::Nothing => None,
        }
    }

    fn is_inert(path: &Path) -> bool {
        Self::INERT_DIRECTORIES
            .iter()
            .any(|directory| path.starts_with(directory))
            || path.extension().is_some_and(|extension| {
                Self::INERT_EXTENSIONS
                    .iter()
                    .any(|inert| extension == *inert)
            })
    }

    fn first_component(inside: &Path) -> Option<String> {
        inside
            .components()
            .next()
            .map(|component| component.as_os_str().to_string_lossy().into_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::Reach;

    const REACHES: [(&str, Reach); 6] = [
        ("crates/eval/src/score.rs", Reach::Package(String::new())),
        ("xtask/src/main.rs", Reach::Everything),
        ("Cargo.lock", Reach::Everything),
        (".github/workflows/ci.yml", Reach::Everything),
        ("README.md", Reach::Nothing),
        (".claude/settings.json", Reach::Nothing),
    ];

    #[test]
    fn a_path_reaches_its_crate_everything_or_nothing() {
        assert!(REACHES.iter().all(|(path, expected)| match expected {
            Reach::Package(_) => Reach::reached_by(path) == Reach::Package("eval".to_owned()),
            other => Reach::reached_by(path) == *other,
        }));
    }
}
