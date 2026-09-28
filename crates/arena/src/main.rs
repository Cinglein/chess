mod command_line;

use std::process::ExitCode;

use command_line::CommandLine;

fn main() -> ExitCode {
    match CommandLine::play() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
