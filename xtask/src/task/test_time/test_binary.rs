use std::path::PathBuf;
use std::process::Command;
use std::time::Instant;

use super::test_timing::TestTiming;
use crate::task::failure::Failure;
use crate::task::workspace::Workspace;

pub(super) struct TestBinary {
    path: PathBuf,
}

impl TestBinary {
    const EXECUTABLE: &str = "Executable";
    const TEST_SUFFIX: &str = ": test";

    pub(super) fn discover(workspace: &Workspace) -> Result<Vec<TestBinary>, Failure> {
        let built = workspace.cargo_output(&[
            "test",
            "--workspace",
            "--all-features",
            "--no-run",
            "--color",
            "never",
        ])?;
        Ok(built
            .lines()
            .filter_map(|line| line.split_once(Self::EXECUTABLE))
            .map(|(_, rest)| rest)
            .filter_map(|rest| rest.rsplit_once('('))
            .map(|(_, path)| TestBinary {
                path: workspace.root().join(path.trim_end_matches(')')),
            })
            .collect())
    }

    pub(super) fn timings(&self) -> Result<Vec<TestTiming>, Failure> {
        self.tests()?
            .into_iter()
            .map(|test| self.timed(test))
            .collect()
    }

    fn tests(&self) -> Result<Vec<String>, Failure> {
        let listed = self.output(&["--list", "--format", "terse"])?;
        Ok(listed
            .lines()
            .filter_map(|line| line.strip_suffix(Self::TEST_SUFFIX))
            .map(str::to_owned)
            .collect())
    }

    fn timed(&self, test: String) -> Result<TestTiming, Failure> {
        let started = Instant::now();
        self.output(&["--exact", &test, "--quiet"])?;
        Ok(TestTiming::new(self.name(), test, started.elapsed()))
    }

    fn output(&self, args: &[&str]) -> Result<String, Failure> {
        let failure = || Failure::Cargo(format!("test {} {}", self.name(), args.join(" ")));
        let output = Command::new(&self.path)
            .args(args)
            .output()
            .map_err(|_| failure())?;
        output
            .status
            .success()
            .then(|| String::from_utf8_lossy(&output.stdout).into_owned())
            .ok_or_else(failure)
    }

    fn name(&self) -> String {
        self.path
            .file_name()
            .map(|name| name.to_string_lossy())
            .and_then(|name| name.rsplit_once('-').map(|(stem, _)| stem.to_owned()))
            .unwrap_or_default()
    }
}
