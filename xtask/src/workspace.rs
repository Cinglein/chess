use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct Workspace {
    root: PathBuf,
}

impl Workspace {
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

    pub fn cargo(&self, args: &[&str]) -> Result<(), String> {
        let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
        let status = Command::new(cargo)
            .args(args)
            .current_dir(&self.root)
            .status()
            .map_err(|error| format!("failed to run cargo {}: {error}", args.join(" ")))?;
        if status.success() {
            Ok(())
        } else {
            Err(format!("cargo {} failed", args.join(" ")))
        }
    }

    pub fn rust_files(&self) -> Result<Vec<PathBuf>, String> {
        let mut files = Vec::new();
        Self::collect_rust_files(&self.root, &mut files)?;
        files.sort();
        Ok(files)
    }

    pub fn relative(&self, path: &Path) -> String {
        path.strip_prefix(&self.root)
            .unwrap_or(path)
            .display()
            .to_string()
    }

    fn collect_rust_files(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
        let entries = fs::read_dir(dir).map_err(|error| format!("{}: {error}", dir.display()))?;
        for entry in entries {
            let entry = entry.map_err(|error| format!("{}: {error}", dir.display()))?;
            let path = entry.path();
            if path.is_dir() {
                let name = entry.file_name();
                if name != "target" && name != ".git" {
                    Self::collect_rust_files(&path, out)?;
                }
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                out.push(path);
            }
        }
        Ok(())
    }
}
