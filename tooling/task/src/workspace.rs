use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};

use crate::command_line::{CommandLine, Program};
use crate::failure::Failure;
use crate::site::Site;
use crate::source_file::SourceFile;

pub struct Workspace {
    root: PathBuf,
}

impl Workspace {
    const EXCLUDED_DIRECTORIES: [&str; 3] = ["target", ".git", ".claude"];
    const MANIFEST: &str = "Cargo.toml";
    const WORKSPACE_TABLE: &str = "[workspace]";

    #[must_use]
    pub fn locate() -> Workspace {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .find(|directory| {
                fs::read_to_string(directory.join(Self::MANIFEST))
                    .is_ok_and(|manifest| manifest.contains(Self::WORKSPACE_TABLE))
            })
            .expect("the tooling lives inside the workspace")
            .to_path_buf();
        Workspace { root }
    }

    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    #[must_use]
    pub fn line(&self, program: Program) -> CommandLine {
        CommandLine::new(program).in_directory(self.root.clone())
    }

    pub fn source_files(&self) -> Result<Vec<SourceFile>, Failure> {
        let mut files = Self::rust_files_under(&self.root)?;
        files.sort();
        files
            .iter()
            .map(|file| SourceFile::read(self, file))
            .collect()
    }

    #[must_use]
    pub fn relative(&self, path: &Path) -> PathBuf {
        path.strip_prefix(&self.root).unwrap_or(path).to_path_buf()
    }

    fn rust_files_under(directory: &Path) -> Result<Vec<PathBuf>, Failure> {
        let io = |error| Failure::Io {
            site: Site::File(directory.to_path_buf()),
            error,
        };
        fs::read_dir(directory)
            .map_err(io)?
            .map(|entry| {
                entry
                    .map_err(io)
                    .and_then(|entry| Self::rust_files_in(&entry))
            })
            .collect::<Result<Vec<Vec<PathBuf>>, Failure>>()
            .map(|nested| nested.into_iter().flatten().collect())
    }

    fn rust_files_in(entry: &DirEntry) -> Result<Vec<PathBuf>, Failure> {
        let path = entry.path();
        if !path.is_dir() {
            Ok(path
                .extension()
                .is_some_and(|extension| extension == "rs")
                .then_some(path)
                .into_iter()
                .collect())
        } else if Self::EXCLUDED_DIRECTORIES
            .iter()
            .any(|excluded| entry.file_name() == *excluded)
        {
            Ok(Vec::new())
        } else {
            Self::rust_files_under(&path)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Workspace;
    use std::fs;
    use std::path::{Path, PathBuf};
    const TREE: [&str; 4] = ["a.rs", "nested/b.rs", "target/skipped.rs", "notes.txt"];
    const FOUND: [&str; 2] = ["a.rs", "nested/b.rs"];
    const FIRST: &str = "a.rs";

    #[test]
    fn rust_files_are_found_below_the_root_outside_target_and_named_relative_to_it() {
        let root = std::env::temp_dir().join(format!("chess-workspace-{}", std::process::id()));
        TREE.iter().map(|file| root.join(file)).for_each(|path| {
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(&path, "pub struct Found;").unwrap();
        });
        let workspace = Workspace { root };
        let files: Vec<PathBuf> = workspace
            .source_files()
            .unwrap()
            .iter()
            .map(|file| file.path().to_path_buf())
            .collect();
        assert_eq!(files, FOUND.map(PathBuf::from));
        assert_eq!(
            workspace.relative(&workspace.root().join(FIRST)),
            Path::new(FIRST)
        );
        fs::remove_dir_all(workspace.root()).unwrap();
    }
}
