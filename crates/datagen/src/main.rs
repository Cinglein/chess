use std::process::ExitCode;

use datagen::CommandLine;

fn main() -> ExitCode {
    match CommandLine::generate() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
