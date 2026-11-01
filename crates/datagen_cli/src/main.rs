use std::process::ExitCode;

use datagen_cli::CommandLine;

fn main() -> ExitCode {
    match CommandLine::generate() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
