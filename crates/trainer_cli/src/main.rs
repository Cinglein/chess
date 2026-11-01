use std::process::ExitCode;

use trainer_cli::CommandLine;

fn main() -> ExitCode {
    match CommandLine::train() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
