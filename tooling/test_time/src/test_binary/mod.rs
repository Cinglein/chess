mod test_argument;

use std::path::PathBuf;
use std::time::Instant;

use test_argument::TestArgument;

use super::test_timing::TestTiming;
use task::Failure;
use task::Workspace;
use task::{CargoArgument, CommandLine, Program, Stderr, Stdout};
use task::{CargoTool, Selection};

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
            .argument(CargoArgument::Test)
            .arguments(selection.arguments::<CargoTool>())
            .argument(CargoArgument::AllFeatures)
            .argument(CargoArgument::NoRun)
            .argument(CargoArgument::Color)
            .argument(CargoArgument::Never)
            .captured::<Stderr>()?;
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
            .argument(TestArgument::List)
            .argument(TestArgument::Format)
            .argument(TestArgument::Terse)
            .captured::<Stdout>()?;
        Ok(listed
            .lines()
            .filter_map(|line| line.strip_suffix(Self::TEST_SUFFIX))
            .map(str::to_owned)
            .collect())
    }

    fn timed(&self, test: String) -> Result<TestTiming, Failure> {
        let started = Instant::now();
        self.line()
            .argument(TestArgument::Exact)
            .argument(&test)
            .argument(TestArgument::Quiet)
            .captured::<Stdout>()?;
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
