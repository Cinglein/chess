use std::process::ExitCode;

use accept_ports::Accept;

fn main() -> ExitCode {
    Accept::from_env().map_or(ExitCode::FAILURE, Accept::run)
}
