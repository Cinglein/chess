use std::process::ExitCode;

use arena::CommandLine;

fn main() -> ExitCode {
    match CommandLine::play() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
