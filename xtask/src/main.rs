use std::env;
use std::process::ExitCode;

use xtask::XtaskLine;

fn main() -> ExitCode {
    XtaskLine::new(env::args().skip(1).collect()).exit_code()
}
