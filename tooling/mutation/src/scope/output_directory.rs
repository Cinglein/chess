use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use task::Failure;
use task::Shard;
use task::Site;
use task::Workspace;

pub struct OutputDirectory(PathBuf);

impl OutputDirectory {
    const ROOT: &str = "target/mutants";

    pub fn cleared(workspace: &Workspace) -> Result<OutputDirectory, Failure> {
        let root = workspace.root().join(Self::ROOT);
        match fs::remove_dir_all(&root) {
            Err(error) if error.kind() != ErrorKind::NotFound => Err(Failure::Io {
                site: Site::File(Self::ROOT.into()),
                error,
            }),
            _ => Ok(OutputDirectory(root)),
        }
    }

    pub fn root(&self) -> &Path {
        &self.0
    }

    pub fn prepared_for(&self, run: &Shard) -> Result<PathBuf, Failure> {
        let crates: Vec<String> = run.crates().iter().map(ToString::to_string).collect();
        let directory = self.0.join(crates.join("+"));
        fs::create_dir_all(&directory).map_err(|error| Failure::Io {
            site: Site::File(directory.clone()),
            error,
        })?;
        Ok(directory)
    }
}
