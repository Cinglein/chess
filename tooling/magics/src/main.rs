use std::path::Path;
use std::process::ExitCode;

use magics::Magics;

fn main() -> ExitCode {
    match Magics::run(Path::new(".")) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
