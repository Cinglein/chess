mod chess_task;

use std::error::Error;
use std::process::ExitCode;

use meta_xtask::Task;

use chess_task::ChessTask;

pub struct XtaskLine {
    arguments: Vec<String>,
}

impl XtaskLine {
    #[must_use]
    pub fn new(arguments: Vec<String>) -> XtaskLine {
        XtaskLine { arguments }
    }

    #[must_use]
    pub fn exit_code(&self) -> ExitCode {
        self.outcome().map_or_else(
            |failure| {
                eprintln!("{failure}");
                ExitCode::FAILURE
            },
            |()| ExitCode::SUCCESS,
        )
    }

    fn outcome(&self) -> Result<(), Box<dyn Error>> {
        let (name, flags) = self.arguments.split_first().ok_or_else(Task::usage)?;
        match name.parse::<ChessTask>() {
            Ok(task) => task.run(flags),
            Err(_) => Ok(name
                .parse::<Task>()
                .map_err(|_| Task::usage())?
                .run(flags.iter().cloned())?),
        }
    }
}
