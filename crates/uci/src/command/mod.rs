mod command_word;
mod engine_option;
mod go_limits;
mod position;

pub use engine_option::EngineOption;
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
    use core::time::Duration;

    use board::Color;

    use super::Command;
    use crate::command::{EngineOption, GoLimits, Position};
    use crate::receiver::Receiver;

    const GO: &str = "go wtime 60 btime 40 winc 1 binc 1";
    const POSITION: &str = "position startpos moves e2e4 e7e5";
    const LINES: [&str; 12] = [
        "uci",
        "isready",
        "setoption name UCI_Elo value 1320",
        "ucinewgame",
        "position startpos",
        "position fen 8/8/8/8/8/8/8/K6k w - - 0 1 moves a1a2",
        "go infinite",
        "go depth 3",
        "go nodes 1000",
        "go movetime 300",
        "stop",
        "quit",
    ];

    #[derive(Debug, PartialEq, Eq)]
    struct Trace(&'static str);

    impl Receiver<'_> for Trace {
        fn identify(self) -> Self {
            Trace("identify")
        }

        fn confirm_ready(self) -> Self {
            Trace("confirm_ready")
        }

        fn configure(self, _: EngineOption<'_>) -> Self {
            Trace("configure")
        }

        fn reset_game(self) -> Self {
            Trace("reset_game")
        }

        fn place(self, _: Position<'_>) -> Self {
            Trace("place")
        }

        fn start_search(self, _: GoLimits) -> Self {
            Trace("start_search")
        }

        fn halt(self) -> Self {
            Trace("halt")
        }

        fn shut_down(self) -> Self {
            Trace("shut_down")
        }
    }

    #[test]
    fn a_clock_go_command_parses_both_sides_and_is_delivered_as_a_search() {
        let command = Command::try_from(GO).unwrap();
        let Command::Go(limits) = command else {
            unreachable!();
        };
        let clock = limits.clock().unwrap();
        assert_eq!(clock.remaining(Color::Black), Duration::from_millis(40));
        assert_eq!(command.deliver_to(Trace("halt")), Trace("start_search"));
    }

    #[test]
    fn a_position_command_carries_its_moves_and_unknown_words_are_rejected() {
        let Command::Position(position) = Command::try_from(POSITION).unwrap() else {
            panic!("{POSITION}");
        };
        assert_eq!((position.fen(), position.moves().count()), (None, 2));
        assert!(Command::try_from("dance").is_err());
    }

    #[test]
    fn every_command_prints_back_to_the_line_it_was_parsed_from() {
        for line in LINES.into_iter().chain([GO, POSITION]) {
            assert_eq!(Command::try_from(line).unwrap().to_string(), line);
        }
    }
}
