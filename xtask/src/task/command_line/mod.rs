mod cargo_word;
mod git_word;
mod mutants_word;
mod program;

pub use cargo_word::CargoWord;
pub use git_word::GitWord;
pub use mutants_word::MutantsWord;
pub use program::Program;

use std::fmt;
use std::path::PathBuf;
use std::process::{Child, Command, Output};

use crate::task::failure::Failure;

#[derive(Clone, Debug)]
pub struct CommandLine {
    program: Program,
    words: Vec<String>,
    directory: Option<PathBuf>,
}

impl CommandLine {
    pub fn new(program: Program) -> CommandLine {
        CommandLine {
            program,
            words: Vec::new(),
            directory: None,
        }
    }

    pub fn in_directory(self, directory: PathBuf) -> CommandLine {
        CommandLine {
            directory: Some(directory),
            ..self
        }
    }

    pub fn word(mut self, word: impl fmt::Display) -> CommandLine {
        self.words.push(word.to_string());
        self
    }

    pub fn words(self, words: impl IntoIterator<Item = impl fmt::Display>) -> CommandLine {
        words.into_iter().fold(self, CommandLine::word)
    }

    pub fn run(&self) -> Result<(), Failure> {
        self.command()
            .status()
            .ok()
            .filter(std::process::ExitStatus::success)
            .map(|_| ())
            .ok_or_else(|| Failure::Command(self.to_string()))
    }

    pub fn spawn(&self) -> Result<Child, Failure> {
        self.command()
            .spawn()
            .map_err(|_| Failure::Command(self.to_string()))
    }

    pub fn stdout(&self) -> Result<String, Failure> {
        self.captured()
            .map(|output| String::from_utf8_lossy(&output.stdout).into_owned())
    }

    pub fn stderr(&self) -> Result<String, Failure> {
        self.captured()
            .map(|output| String::from_utf8_lossy(&output.stderr).into_owned())
    }

    fn captured(&self) -> Result<Output, Failure> {
        let output = self
            .command()
            .output()
            .map_err(|_| Failure::Command(self.to_string()))?;
        if output.status.success() {
            Ok(output)
        } else {
            Err(Failure::Command(format!(
                "{self}: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            )))
        }
    }

    fn command(&self) -> Command {
        let mut command = Command::new(self.program.to_string());
        command.args(&self.words);
        if let Some(directory) = &self.directory {
            command.current_dir(directory);
        }
        command
    }
}

impl fmt::Display for CommandLine {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} {}", self.program, self.words.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::{CargoWord, CommandLine, Program};

    const SHOWN: &str = "git fmt --all";

    #[test]
    fn a_command_line_prints_its_program_and_words_in_order() {
        let line = CommandLine::new(Program::Git)
            .word(CargoWord::Fmt)
            .words([CargoWord::All]);
        assert_eq!(line.to_string(), SHOWN);
    }
}
