use std::path::Path;
use std::process::ExitCode;

use magics::{MagicTables, XorShift};

fn main() -> ExitCode {
    match MagicTables::find(&mut XorShift::from(MagicTables::SEED)).write(Path::new(".")) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
