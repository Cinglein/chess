mod test_word;

use std::path::PathBuf;
use std::time::Instant;

use test_word::TestWord;

use super::test_timing::TestTiming;
use crate::task::command_line::{CargoWord, CommandLine, Program};
use crate::task::failure::Failure;
use crate::task::selection::Selection;
use crate::task::workspace::Workspace;

pub(super) struct TestBinary {
    path: PathBuf,
}

impl TestBinary {
    const EXECUTABLE: &str = "Executable";
    const TEST_SUFFIX: &str = ": test";

    pub(super) fn discover(
        workspace: &Workspace,
        selection: &Selection,
    ) -> Result<Vec<TestBinary>, Failure> {
        let built = workspace
            .line(Program::Cargo)
            .word(CargoWord::Test)
            .words(selection.package_words())
            .word(CargoWord::AllFeatures)
            .word(CargoWord::NoRun)
            .word(CargoWord::Color)
            .word(CargoWord::Never)
            .stderr()?;
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
        let listed = self
            .line()
            .word(TestWord::List)
            .word(TestWord::Format)
            .word(TestWord::Terse)
            .stdout()?;
        Ok(listed
            .lines()
            .filter_map(|line| line.strip_suffix(Self::TEST_SUFFIX))
            .map(str::to_owned)
            .collect())
    }

    fn timed(&self, test: String) -> Result<TestTiming, Failure> {
        let started = Instant::now();
        self.line()
            .word(TestWord::Exact)
            .word(&test)
            .word(TestWord::Quiet)
            .stdout()?;
        Ok(TestTiming::new(self.name(), test, started.elapsed()))
    }

    fn line(&self) -> CommandLine {
        CommandLine::new(Program::Executable(self.path.clone()))
    }

    fn name(&self) -> String {
        self.path
            .file_name()
            .map(|name| name.to_string_lossy())
            .and_then(|name| name.rsplit_once('-').map(|(stem, _)| stem.to_owned()))
            .unwrap_or_default()
    }
}
