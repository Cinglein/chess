mod command_word;
mod engine_option;
mod go_limits;
mod position;

pub use engine_option::{Elo, EngineOption, Switch};
pub use go_limits::{Clock, GoLimits};
pub use position::Position;

use core::fmt;

use command_word::CommandWord;

use crate::receiver::Receiver;
use crate::uci_error::UciError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Command<'line> {
    Uci,
    IsReady,
    SetOption(EngineOption<'line>),
    UciNewGame,
    Position(Position<'line>),
    Go(GoLimits),
    Stop,
    Quit,
}

impl<'line> Command<'line> {
    #[must_use]
    pub const fn interrupts(&self) -> bool {
        matches!(self, Command::Stop | Command::Quit)
    }

    const fn word(&self) -> CommandWord {
        match self {
            Command::Uci => CommandWord::Uci,
            Command::IsReady => CommandWord::IsReady,
            Command::SetOption(_) => CommandWord::SetOption,
            Command::UciNewGame => CommandWord::UciNewGame,
            Command::Position(_) => CommandWord::Position,
            Command::Go(_) => CommandWord::Go,
            Command::Stop => CommandWord::Stop,
            Command::Quit => CommandWord::Quit,
        }
    }

    pub fn deliver_to<R: Receiver<'line>>(self, receiver: R) -> R {
        match self {
            Command::Uci => receiver.identify(),
            Command::IsReady => receiver.confirm_ready(),
            Command::SetOption(option) => receiver.configure(option),
            Command::UciNewGame => receiver.reset_game(),
            Command::Position(position) => receiver.place(position),
            Command::Go(limits) => receiver.start_search(limits),
            Command::Stop => receiver.halt(),
            Command::Quit => receiver.shut_down(),
        }
    }
}

impl<'line> TryFrom<&'line str> for Command<'line> {
    type Error = UciError;

    fn try_from(line: &'line str) -> Result<Command<'line>, UciError> {
        let trimmed = line.trim();
        let (head, rest) = trimmed
            .split_once(char::is_whitespace)
            .unwrap_or((trimmed, ""));
        match head
            .parse::<CommandWord>()
            .map_err(|_| UciError::UnknownCommand)?
        {
            CommandWord::Uci => Ok(Command::Uci),
            CommandWord::IsReady => Ok(Command::IsReady),
            CommandWord::SetOption => EngineOption::try_from(rest).map(Command::SetOption),
            CommandWord::UciNewGame => Ok(Command::UciNewGame),
            CommandWord::Position => Position::try_from(rest).map(Command::Position),
            CommandWord::Go => Ok(Command::Go(GoLimits::from(rest))),
            CommandWord::Stop => Ok(Command::Stop),
            CommandWord::Quit => Ok(Command::Quit),
        }
    }
}

impl fmt::Display for Command<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.word())?;
        match self {
            Command::SetOption(option) => write!(formatter, " {option}"),
            Command::Position(position) => write!(formatter, " {position}"),
            Command::Go(limits) => write!(formatter, " {limits}"),
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Command;

    const LINES: [&str; 11] = [
        "uci",
        "isready",
        "setoption name UCI_Elo value 1320",
        "ucinewgame",
        "position startpos",
        "position startpos moves e2e4 e7e5",
        "position fen 8/8/8/8/8/8/8/K6k w - - 0 1 moves a1a2",
        "go infinite",
        "go wtime 1 btime 2 winc 3 binc 4",
        "stop",
        "quit",
    ];

    #[test]
    fn every_command_prints_back_to_the_line_it_was_parsed_from_and_unknown_words_are_rejected() {
        for line in LINES {
            let command = Command::try_from(line).unwrap();
            assert_eq!(command.to_string(), line);
            assert_eq!(command.interrupts(), line == "stop" || line == "quit");
        }
        assert!(Command::try_from("dance").is_err());
    }
}
