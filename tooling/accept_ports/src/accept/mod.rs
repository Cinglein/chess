mod code;
mod codes;

use std::env;
use std::ffi::OsString;
use std::process::{Command, ExitCode};

use code::Code;
use codes::Codes;

pub struct Accept {
    codes: Codes,
    program: OsString,
    arguments: Vec<OsString>,
}

impl Accept {
    #[must_use]
    pub fn from_env() -> Option<Self> {
        let given: Vec<OsString> = env::args_os().skip(1).collect();
        let codes = env::var("ACCEPT").ok()?.parse().ok()?;
        given.split_first().map(|(program, arguments)| Self {
            codes,
            program: program.clone(),
            arguments: arguments.to_vec(),
        })
    }

    #[must_use]
    pub fn run(self) -> ExitCode {
        let code = Command::new(&self.program)
            .args(&self.arguments)
            .status()
            .map_or(Code::FAILED, Code::from);
        if self.codes.contains(code) {
            ExitCode::SUCCESS
        } else {
            code.exit()
        }
    }
}
