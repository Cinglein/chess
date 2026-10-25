use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};

use crate::task::command_line::{CommandLine, Program};
use crate::task::failure::Failure;
use crate::task::site::Site;
use crate::task::source_file::SourceFile;

pub struct Workspace {
    root: PathBuf,
}

impl Workspace {
    const EXCLUDED_DIRECTORIES: [&str; 2] = ["target", ".git"];

    pub fn locate() -> Workspace {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("xtask lives directly under the workspace root")
            .to_path_buf();
        Workspace { root }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

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

    pub fn relative(&self, path: &Path) -> String {
        path.strip_prefix(&self.root)
            .unwrap_or(path)
            .display()
            .to_string()
    }

    fn rust_files_under(directory: &Path) -> Result<Vec<PathBuf>, Failure> {
        let io = |error| Failure::Io {
            site: Site::File(directory.display().to_string()),
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
        if path.is_dir() {
            return if Self::EXCLUDED_DIRECTORIES
                .iter()
                .any(|excluded| entry.file_name() == *excluded)
            {
                Ok(Vec::new())
            } else {
                Self::rust_files_under(&path)
            };
        }
        Ok(path
            .extension()
            .is_some_and(|extension| extension == "rs")
            .then_some(path)
            .into_iter()
            .collect())
    }
}
