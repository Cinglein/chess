use std::process::ExitCode;

use report_ports::Report;

fn main() -> ExitCode {
    Report::from_env().map_or(ExitCode::FAILURE, Report::run)
}
