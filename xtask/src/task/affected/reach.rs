use std::path::{Path, PathBuf};

use crate::task::crate_name::CrateName;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Reach {
    Everything,
    Nothing,
    Package(CrateName),
    Tool(PathBuf),
}

impl Reach {
    const CRATES: &str = "crates";
    const TOOL: &str = "xtask";
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
            None if path.starts_with(Self::TOOL) => Reach::Tool(path.to_path_buf()),
            None if Self::is_inert(path) => Reach::Nothing,
            None => Reach::Everything,
        }
    }

    pub fn package(&self) -> Option<CrateName> {
        match self {
            Reach::Package(name) => Some(name.clone()),
            Reach::Everything | Reach::Nothing | Reach::Tool(_) => None,
        }
    }

    pub fn tool_file(&self) -> Option<&Path> {
        match self {
            Reach::Tool(path) => Some(path),
            Reach::Everything | Reach::Nothing | Reach::Package(_) => None,
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

    fn first_component(inside: &Path) -> Option<CrateName> {
        inside
            .components()
            .next()
            .and_then(|component| component.as_os_str().to_string_lossy().parse().ok())
    }
}

#[cfg(test)]
mod tests {
    use super::Reach;

    const PACKAGE_PATHS: [&str; 1] = ["crates/eval/src/score.rs"];
    const TOOL_PATHS: [&str; 2] = ["xtask/src/task/lint.rs", "xtask/policy.toml"];
    const EVERYTHING_PATHS: [&str; 2] = ["Cargo.lock", ".github/ci.yml"];
    const NOTHING_PATHS: [&str; 2] = ["README.md", ".claude/settings.json"];

    #[test]
    fn a_path_reaches_its_crate_the_tool_everything_or_nothing() {
        assert!(
            PACKAGE_PATHS
                .iter()
                .all(|path| Reach::reached_by(path).package() == "eval".parse().ok())
        );
        assert!(
            TOOL_PATHS
                .iter()
                .all(|path| Reach::reached_by(path).tool_file().is_some())
        );
        assert!(
            EVERYTHING_PATHS
                .iter()
                .all(|path| Reach::reached_by(path) == Reach::Everything)
                && NOTHING_PATHS
                    .iter()
                    .all(|path| Reach::reached_by(path) == Reach::Nothing)
        );
    }
}
